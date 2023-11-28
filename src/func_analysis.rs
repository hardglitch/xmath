use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use itertools;
use itertools::{Itertools};

#[derive(Debug, PartialEq)]
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
    scale: f64,
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
            scale: 100.0,
        }
    }
    pub fn find_roots(&mut self) {
        let y = *self.expr;
        for i in -10000..10000 {
            let x = i as f64;
            if (0.0 - y(x)).abs() <= 0.01 {
                self.roots.push(Some(x))
            }
        }
        if self.roots.is_empty() { self.roots.push(None) }
    }

    pub fn find_extremums(&mut self, x_min: i32, x_max: i32) -> Option<&HashMap<String, Point>> {
        let mut x_range = x_min * self.scale as i32..=x_max * self.scale as i32;
        if x_min > x_max {
            x_range = x_max * self.scale as i32..=x_min * self.scale as i32;
        }
        let mut extrs = HashMap::<i32, i32>::new();
        let y = *self.expr;

        for i in x_range {
            let x = i as f64 / self.scale;
            let y_x = y(x);
            if y_x.is_nan() { continue }
            extrs.insert(i, (y_x * self.scale) as i32);
        }

        if extrs.is_empty() {
            self.extrs.insert("NONE".to_owned(), Point{x:0.0, y:0.0});  // Flag
            return None
        }

        let miny = extrs.values().sorted().min();
        let minx = extrs
            .iter()
            .find_map(|(k, v)| if Some(v) == miny {Some(k)} else {None});

        let min = Point{
            x: *minx.unwrap() as f64 / self.scale,
            y: *miny.unwrap() as f64 / self.scale
        };
        self.extrs.insert("min".to_owned(), min);

        let maxy = extrs.values().sorted().max();
        let maxx = extrs
            .iter()
            .find_map(|(k, v)| if Some(v) == maxy {Some(k)} else {None});

        let max = Point{
            x: *maxx.unwrap() as f64 / self.scale,
            y: *maxy.unwrap() as f64 / self.scale
        };
        self.extrs.insert("max".to_owned(), max);

        // Other style of data returning
        self.extremums()
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
