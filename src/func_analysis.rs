use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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
        write!(f, "({}, {})", self.x, self.y)
    }
}
pub struct Expression<F> {
    expr: Box<F>,
    roots: Vec<Option<f64>>,
    extrs: HashMap<String, Point>,
    precision: f64,
}
impl<F> Expression<F>
    where F: (Fn(f64) -> f64) + Copy
{
    pub fn new(func: F) -> Self
    {
        Self {
            expr: Box::new(func),
            roots: Default::default(),
            extrs: Default::default(),
            precision: 10_000.0,
        }
    }
    pub fn find_roots(&mut self) {
        let y = *self.expr;
        for i in -10000..10000 {
            let x = i as f64;
            if is_equal(&y(x), &0.0, 1.0 / self.precision) {
                self.roots.push(Some(x))
            }
        }
        if self.roots.is_empty() { self.roots.push(None) }
    }

    pub fn find_extremums(&mut self, x_min: f64, x_max: f64) -> Option<&HashMap<String, Point>> {
        let scale = 100.0;
        let mut x_range = (x_min * scale) as i32..=(x_max * scale) as i32;
        if x_min > x_max {
            x_range = (x_max * scale) as i32..=(x_min * scale) as i32;
        }
        let mut extrs = HashMap::<i32, i64>::new();
        let y = *self.expr;

        for i in x_range {
            let x = i as f64 / scale;
            let y_x = y(x);
            if y_x.is_nan() { continue }
            extrs.insert(i, (y_x * self.precision) as i64);
        }

        if extrs.is_empty() {
            self.extrs.insert("NONE".to_owned(), Point::default());  // Flag
            return None
        }

        self._find_global_min_max(&extrs);

        // Other style of data returning
        self.extremums()
    }

    fn _find_global_min_max(&mut self, extrs: &HashMap<i32, i64>) {
        // Here minx and miny will have definitely get the values.
        let miny = extrs.values().sorted().min().unwrap();
        let minx = extrs.iter().find_map(|(x, y)| if y == miny {Some(x)} else {None}).unwrap();

        let min = Point{
            x: *minx as f64 / 100.0,
            y: *miny as f64 / self.precision
        };
        self.extrs.insert("min".to_owned(), min);

        // Here maxx and maxy will have definitely get the values.
        let maxy = extrs.values().sorted().max().unwrap();
        let maxx = extrs.iter().find_map(|(x, y)| if y == maxy {Some(x)} else {None}).unwrap();

        let max = Point{
            x: *maxx as f64 / 100.0,
            y: *maxy as f64 / self.precision
        };
        self.extrs.insert("max".to_owned(), max);
    }

    pub fn extremums(&self) -> Option<&HashMap<String, Point>> {
        if self.extrs.is_empty() || self.extrs.get("NONE").is_some() { return None }
        Some(&self.extrs)
    }

    pub fn max(&self) -> Option<&Point> {
        match self.extrs.is_empty() {
            true  => { None },
            false => {
                self.extrs.get("max")
            }
        }
    }

    pub fn min(&self) -> Option<&Point> {
        match self.extrs.is_empty() {
            true  => { None },
            false => {
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
                    let min = self.extrs.get("min").unwrap();
                    println!("Min F(x)={:.2}, x={:.2}", min.y, min.x);

                    let max = self.extrs.get("max").unwrap();
                    println!("Max F(x)={:.2}, x={:.2}", max.y, max.x);
                }
            }
        }
    }
}
