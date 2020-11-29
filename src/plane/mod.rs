use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::geom2d::Point;

pub trait ToChar {
    fn to_char(&self) -> char;
}

pub enum BW {
    Black,
    White,
}

impl ToChar for BW {
    fn to_char(&self) -> char {
        match self {
            BW::Black => ' ',
            BW::White => '#',
        }
    }
}

impl ToChar for char {
    fn to_char(&self) -> char {
        *self
    }
}

pub struct Plane<C> {
    panels: HashMap<Point, C>,
    default_paint: C,
    min: Point,
    max: Point,
}

impl<C> Plane<C> {
    pub fn new(default_paint: C) -> Plane<C> {
        Plane {
            panels: HashMap::new(),
            default_paint,
            min: Point::origin(),
            max: Point::origin(),
        }
    }

    pub fn paint(&mut self, p: Point, c: C) {
        if let None = self.panels.insert(p, c) {
            self.min = p.min(self.min);
            self.max = p.max(self.max);
        }
    }

    pub fn get_paint(&self, p: Point) -> &C {
        match self.panels.get(&p) {
            Some(c) => c,
            None => &self.default_paint,
        }
    }

    pub fn paint_count(&self) -> usize {
        self.panels.len()
    }
}

impl<C: ToChar> Display for Plane<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = (self.max.x - self.min.x + 1) as usize;
        let height = (self.max.y - self.min.y + 1) as usize;
        let mut result = format!("+- {} x {} ", width, height);
        if result.len() < width {
            result.push_str(&"-".repeat(width - result.len() + 2));
        }
        result.push_str("-+\n");
        for y in (self.min.y..=self.max.y).rev() {
            result.push_str("| ");
            for x in self.min.x..=self.max.x {
                let p = Point::new(x, y);
                result.push(match self.panels.get(&p) {
                    Some(c) => c.to_char(),
                    None => ' ',
                })
            }
            result.push_str(" |\n");
        }
        result.push_str("+-");
        result.push_str(&"-".repeat(width));
        result.push_str("-+\n");
        f.write_str(&result)
    }
}
