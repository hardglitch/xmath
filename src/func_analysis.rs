use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::thread::JoinHandle;
use crate::utils::is_equal;


#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub(crate) x: f64,
    pub(crate) y: f64,
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



#[derive(Debug, Copy, Clone)]
pub struct RawPoint {
    pub(crate) x: i64,
    pub(crate) y: i64,
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
// impl Settings {
//     pub fn precision(&self) -> f64 {
//         self.precision
//     }
// }

pub struct Expression<F> {
    expr: Box<F>,
    roots: Vec<Option<f64>>,
    extrs: Vec<RawPoint>,
    settings: Settings,
}
impl<F> Expression<F>
    where for<'a> F: (Fn(f64) -> f64) + Copy + Send + Sync + 'a
{
    pub fn new(func: F) -> Self
    {
        Self {
            expr: Box::new(func),
            roots: Default::default(),
            extrs: Default::default(),
            settings: Default::default(),
        }
    }

    pub fn find_roots(&mut self) -> std::io::Result<()> {
        let upscaled_x_min = (self.settings.x_min / self.settings.precision) as i64;
        let upscaled_x_max = (self.settings.x_max / self.settings.precision) as i64;
        let max_threads = std::thread::available_parallelism()?.get();
        let one_thread_tasks = ((upscaled_x_max - upscaled_x_min).abs() / max_threads as i64) + 1;
        let mut handels = Vec::<JoinHandle<()>>::new();
        let (tx, rx) = std::sync::mpsc::channel();

        for th in 0..max_threads {
            let tx_th = tx.clone();
            let expr = *self.expr;
            let precision = self.settings.precision;

            let handle = std::thread::spawn(move || {
                let upscaled_x_min_shifted = upscaled_x_min + one_thread_tasks * th as i64;
                let upscaled_x_max_shifted = upscaled_x_min + one_thread_tasks * (th + 1) as i64;
                for upscaled_x in upscaled_x_min_shifted..=upscaled_x_max_shifted {
                    let downscaled_x = upscaled_x as f64 * precision;
                    if is_equal(&expr(downscaled_x), &0.0, precision / 10.0) {
                        tx_th.send(upscaled_x).unwrap();
                    }
                }
            });
            handels.push(handle);
        }
        for handle in handels { handle.join().unwrap(); }
        drop(tx);

        for upscaled_x in rx {
            let downscaled_x = upscaled_x as f64 * self.settings.precision;
            self.roots.push(Some(downscaled_x));
        }
        if self.roots.is_empty() { self.roots.push(None); }
        Ok(())
    }

    pub fn find_extremums(&mut self, x_min: f64, x_max: f64) -> std::io::Result<Option<Vec<Point>>> {
        let mut upscaled_x_min = (x_min / self.settings.precision) as i64;
        let mut upscaled_x_max = (x_max / self.settings.precision) as i64;
        if upscaled_x_min > upscaled_x_max {
            std::mem::swap(&mut upscaled_x_min, &mut upscaled_x_max);
        }

        let max_threads = std::thread::available_parallelism()?.get();
        let one_thread_tasks = ((upscaled_x_max - upscaled_x_min).abs() / max_threads as i64) + 1;
        let mut handels = Vec::<JoinHandle<()>>::new();
        let (tx, rx) = std::sync::mpsc::channel();

        for th in 0..max_threads {
            let tx_th = tx.clone();
            let expr = *self.expr;
            let precision = self.settings.precision;

            let handle = std::thread::spawn(move || {
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
            handels.push(handle);
        }
        for handle in handels { handle.join().unwrap(); }
        drop(tx);

        let mut raw_data = Vec::<RawPoint>::new();
        for (min_point, max_point) in rx {
            raw_data.push(min_point);
            raw_data.push(max_point);
        }
        if raw_data.is_empty() {
            return Ok(None)
        }
        raw_data.sort();
        self.extrs.push(*raw_data.first().unwrap());  // min point
        self.extrs.push(*raw_data.last().unwrap());   // max point

        // Other style of data returning
        Ok(self.extremums())
    }

    pub fn extremums(&self) -> Option<Vec<Point>> {
        match self.extrs.is_empty() {
            true => None,
            false => {
                let points = self.extrs.iter()
                    .map(|p| p.convert_to_point())
                    .collect();
                Some(points)
            }
        }
    }

    pub fn max(&self) -> Option<Point> {
        match self.extrs.is_empty() {
            true => None,
            false => Some(self.extrs.last().unwrap().convert_to_point())
        }
    }

    pub fn min(&self) -> Option<Point> {
        match self.extrs.is_empty() {
            true => None,
            false => Some(self.extrs.first().unwrap().convert_to_point())
        }
    }

    pub fn roots(&self) -> &[Option<f64>] {
        &self.roots[..]
    }

    pub fn print_result(&self) {
        if !self.roots.is_empty() {
            match self.roots[0].is_none() {
                true  => println!("No roots"),
                false => {
                    for (i, root) in self.roots.iter().enumerate() {
                        println!("x{}={:.2}", i+1, root.unwrap());
                    }
                }
            }
        }

        if !self.extrs.is_empty() {
            match self.min().is_none() {
                true => println!("No extremums"),
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
