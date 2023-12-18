use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::utils::AdvancedEQ;


#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x(), self.y())
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
}


#[derive(Copy, Clone)]
pub struct RawPoint {
    x: i64,
    y: i64,
}
impl PartialEq for RawPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for RawPoint {}
impl Ord for RawPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y)
    }
}
impl PartialOrd for RawPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.y.cmp(&other.y))
    }
}
impl RawPoint {
    pub(crate) fn convert_to_point(&self) -> Point {
        Point {
            x: self.x as f64 * Settings::default().precision,
            y: self.y as f64 * Settings::default().precision,
        }
    }
}


struct Settings {
    x_min: f64,
    x_max: f64,
    precision: f64,
}
impl Default for Settings {
    fn default() -> Self {
        Self{
            x_min: -1000.0,
            x_max: 1000.0,
            precision: 0.0001,
        }
    }
}


#[derive(Default)]
struct Flags {
    is_roots: bool,
    is_extrs: bool,
}


pub struct Expression<F> {
    expr: Box<F>,
    roots: Vec<f64>,
    extrs: Vec<RawPoint>,
    settings: Settings,
    flags: Flags,
}
impl<F> Expression<F>
    where for<'a> F: (Fn(f64) -> f64) + Copy + Send + 'a
{
    pub fn new(func: F) -> Self
    {
        Self {
            expr: Box::new(func),
            roots: Default::default(),
            extrs: Default::default(),
            settings: Default::default(),
            flags: Default::default(),
        }
    }

    pub fn find_roots(&mut self) -> std::io::Result<()> {
        self.flags.is_roots = true;
        let max_threads = std::thread::available_parallelism()?.get();
        let upscaled_x_min = (self.settings.x_min / self.settings.precision) as i64;
        let upscaled_x_max = (self.settings.x_max / self.settings.precision) as i64;
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::scope(|s| {
            let one_thread_tasks = ((upscaled_x_max - upscaled_x_min).abs() / max_threads as i64) + 1;

            for th in 0..max_threads {
                let tx_th = tx.clone();
                let expr = *self.expr;
                let precision = self.settings.precision;

                s.spawn(move || {
                    let upscaled_x_min_shifted = upscaled_x_min + one_thread_tasks * th as i64;
                    let upscaled_x_max_shifted = upscaled_x_min + one_thread_tasks * (th + 1) as i64;
                    for upscaled_x in upscaled_x_min_shifted..=upscaled_x_max_shifted {
                        let downscaled_x = upscaled_x as f64 * precision;
                        if expr(downscaled_x).is_equal(0.0, precision / 10.0) {
                            tx_th.send(upscaled_x).unwrap();
                        }
                    }
                });
            }
        });
        drop(tx);

        for upscaled_x in rx {
            let downscaled_x = upscaled_x as f64 * self.settings.precision;
            self.roots.push(downscaled_x);
        }
        Ok(())
    }

    pub fn find_extremums(&mut self, x_min: f64, x_max: f64) -> std::io::Result<Option<Vec<Point>>> {
        self.flags.is_extrs = true;
        let mut upscaled_x_min = (x_min / self.settings.precision) as i64;
        let mut upscaled_x_max = (x_max / self.settings.precision) as i64;
        if upscaled_x_min > upscaled_x_max {
            std::mem::swap(&mut upscaled_x_min, &mut upscaled_x_max);
        }

        let max_threads = std::thread::available_parallelism()?.get();
        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::scope(|s| {
            let one_thread_tasks = ((upscaled_x_max - upscaled_x_min).abs() / max_threads as i64) + 1;

            for th in 0..max_threads {
                let tx_th = tx.clone();
                let expr = *self.expr;
                let precision = self.settings.precision;

                s.spawn(move || {
                    let mut th_raw_data = Vec::<RawPoint>::new();
                    let upscaled_x_min_shifted = upscaled_x_min + one_thread_tasks * th as i64;
                    let upscaled_x_max_shifted = upscaled_x_min + one_thread_tasks * (th + 1) as i64;
                    for upscaled_x in upscaled_x_min_shifted..=upscaled_x_max_shifted {
                        let downscaled_x = upscaled_x as f64 * precision;
                        let y = expr(downscaled_x);
                        if y.is_nan() { continue }
                        let upscaled_y = (y / precision) as i64;
                        th_raw_data.push(
                            RawPoint{ x: upscaled_x, y: upscaled_y }
                        );
                    }
                    if !th_raw_data.is_empty() {
                        th_raw_data.sort();
                        let min_point = *th_raw_data.first().unwrap();
                        let max_point = *th_raw_data.last().unwrap();
                        tx_th.send((min_point, max_point)).unwrap();
                    }
                });
            }
        });
        drop(tx);

        let mut raw_data = Vec::<RawPoint>::new();
        for (min, max) in rx {
            raw_data.push(min);
            raw_data.push(max);
        }
        if raw_data.is_empty() { return Ok(None) }
        raw_data.sort();
        self.extrs.push(*raw_data.first().unwrap());  // min point
        self.extrs.push(*raw_data.last().unwrap());   // max point

        // Other style of data returning
        Ok(self.extremums())
    }

    pub fn extremums(&self) -> Option<Vec<Point>> {
        match self.flags.is_extrs && !self.extrs.is_empty() {
            false => None,
            true => {
                let points = self.extrs.iter()
                    .map(|p| p.convert_to_point())
                    .collect();
                Some(points)
            }
        }
    }

    pub fn max(&self) -> Option<Point> {
        match self.flags.is_extrs && !self.extrs.is_empty() {
            false => None,
            true => Some(self.extrs.last().unwrap().convert_to_point())
        }
    }

    pub fn min(&self) -> Option<Point> {
        match self.flags.is_extrs && !self.extrs.is_empty() {
            false => None,
            true => Some(self.extrs.first().unwrap().convert_to_point())
        }
    }

    pub fn roots(&self) -> Option<&[f64]> {
        match self.flags.is_roots && !self.roots.is_empty() {
            false => None,
            true => Some(&self.roots[..])
        }
    }

    pub fn print_result(&self) {
        if self.flags.is_roots {
            match self.roots.is_empty() {
                true  => println!("No roots"),
                false => {
                    for (i, root) in self.roots.iter().enumerate() {
                        println!("x{}={:.2}", i+1, root);
                    }
                }
            }
        }

        if self.flags.is_extrs {
            match self.extrs.is_empty() {
                true  => println!("No extremums"),
                false => {
                    if self.min() == self.max() {
                        println!("Min=Max F(x)={:.2}, x={:.2}", self.min().unwrap().y, self.min().unwrap().x);
                    } else {
                        println!("Min F(x)={:.2}, x={:.2}", self.min().unwrap().y, self.min().unwrap().x);
                        println!("Max F(x)={:.2}, x={:.2}", self.max().unwrap().y, self.max().unwrap().x);
                    }
                }
            }
        }
    }
}
