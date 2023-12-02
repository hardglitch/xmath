use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::thread::JoinHandle;
use itertools;
use itertools::Itertools;
use crate::utils::is_equal;


#[derive(Debug, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x={}, y={})", self.x, self.y)
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

pub struct Expression<F> {
    expr: Box<F>,
    roots: Vec<Option<f64>>,
    extrs: HashMap<String, Point>,
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
                        tx_th.send(Some(upscaled_x)).unwrap();
                    }
                }
            });
            handels.push(handle);
        }
        for handle in handels { handle.join().unwrap(); }
        drop(tx);

        for r in rx {
            let upscaled_x = r.expect("Nothing has been received.");
            let downscaled_x = upscaled_x as f64 * self.settings.precision;
            self.roots.push(Some(downscaled_x));
        }
        if self.roots.is_empty() { self.roots.push(None); }
        Ok(())
    }

    pub fn find_extremums(&mut self, x_min: f64, x_max: f64) -> std::io::Result<Option<&HashMap<String, Point>>> {
        let mut upscaled_x_min = (x_min / self.settings.precision) as i64;
        let mut upscaled_x_max = (x_max / self.settings.precision) as i64;
        if upscaled_x_min > upscaled_x_max {
            std::mem::swap(&mut upscaled_x_min, &mut upscaled_x_max);
        }

        let max_threads = std::thread::available_parallelism()?.get();
        let one_thread_tasks = ((upscaled_x_max - upscaled_x_min).abs() / max_threads as i64) + 1;
        let mut handels = Vec::<JoinHandle<()>>::new();
        let mut extrs = HashMap::<i64, i64>::new();
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
                    let y = expr(downscaled_x);
                    if y.is_nan() { continue }
                    let upscaled_y = (y / precision) as i64;
                    tx_th.send(Some((upscaled_x, upscaled_y))).unwrap();
                }
            });
            handels.push(handle);
        }
        for handle in handels { handle.join().unwrap(); }
        drop(tx);

        for r in rx {
            let (x, y) = r.expect("Nothing has been received.");
            extrs.insert(x, y);
        }

        if extrs.is_empty() {
            self.extrs.insert("NONE".to_owned(), Point::default());  // Flag
            return Ok(None)
        }

        self._find_global_min_max(&extrs);

        // Other style of data returning
        Ok(self.extremums())
    }

    fn _find_global_min_max(&mut self, extrs: &HashMap<i64, i64>) {
        // Here minx and miny will have definitely get the values.
        let miny = extrs.values().sorted().min().unwrap();
        let minx = extrs.iter().find_map(|(x, y)| if y == miny {Some(x)} else {None}).unwrap();

        let min = Point{
            x: *minx as f64 * self.settings.precision,
            y: *miny as f64 * self.settings.precision,
        };
        self.extrs.insert("min".to_owned(), min);

        // Here maxx and maxy will have definitely get the values.
        let maxy = extrs.values().sorted().max().unwrap();
        let maxx = extrs.iter().find_map(|(x, y)| if y == maxy {Some(x)} else {None}).unwrap();

        let max = Point{
            x: *maxx as f64 * self.settings.precision,
            y: *maxy as f64 * self.settings.precision,
        };
        self.extrs.insert("max".to_owned(), max);
    }

    pub fn extremums(&self) -> Option<&HashMap<String, Point>> {
        if self.extrs.is_empty() || self.extrs.get("NONE").is_some() { return None }
        Some(&self.extrs)
    }

    pub fn max(&self) -> Option<&Point> {
        if self.extrs.is_empty() { return None; }
        match self.extrs.get("NONE") {
            Some(_)  => { None },
            None => {
                self.extrs.get("max")
            }
        }
    }

    pub fn min(&self) -> Option<&Point> {
        if self.extrs.is_empty() { return None; }
        match self.extrs.get("NONE") {
            Some(_)  => { None },
            None => {
                self.extrs.get("min")
            }
        }
    }

    pub fn roots(&self) -> &Vec<Option<f64>> {
        &self.roots
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
            match self.extrs.get("NONE") {
                Some(_) => println!("No extremums"),
                None => {
                    if self.extrs.get("min") == self.extrs.get("max") {
                        let min_max = self.extrs.get("min").unwrap();
                        println!("Min=Max F(x)={:.2}, x={:.2}", min_max.y, min_max.x);
                    } else {
                        let min = self.extrs.get("min").unwrap();
                        println!("Min F(x)={:.2}, x={:.2}", min.y, min.x);

                        let max = self.extrs.get("max").unwrap();
                        println!("Max F(x)={:.2}, x={:.2}", max.y, max.x);
                    }
                }
            }
        }
    }
}
