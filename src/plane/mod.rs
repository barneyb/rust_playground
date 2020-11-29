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
}

impl<C> Plane<C> {
    pub fn new(default_paint: C) -> Plane<C> {
        Plane {
            panels: HashMap::new(),
            default_paint,
        }
    }

    pub fn paint(&mut self, p: Point, c: C) {
        self.panels.insert(p, c);
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
        let (min, max) =
            self.panels
                .keys()
                .fold((Point::origin(), Point::origin()), |(min, max), p| {
                    (
                        Point::new(min.x.min(p.x), min.y.min(p.y)),
                        Point::new(max.x.max(p.x), max.y.max(p.y)),
                    )
                });
        let mut bar = "-".repeat((max.x - min.x + 1) as usize) + "-+\n";
        bar.insert_str(0, "+-");
        let mut result = bar.clone();
        for y in (min.y..=max.y).rev() {
            result.push_str("| ");
            for x in min.x..=max.x {
                let p = Point::new(x, y);
                result.push(match self.panels.get(&p) {
                    Some(c) => c.to_char(),
                    None => ' ',
                })
            }
            result.push_str(" |\n");
        }
        result.push_str(&bar);
        f.write_str(&result)
    }
}
