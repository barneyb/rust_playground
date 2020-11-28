use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fmt;

use crate::geom2d::Point;

use super::Color;

pub struct Ship {
    panels: HashMap<Point, Color>,
}

impl Ship {

    pub fn new() -> Ship {
        Ship {
            panels: HashMap::new(),
        }
    }

    pub fn paint(&mut self, p: Point, c: Color) {
        self.panels.insert(p, c);
    }

    pub fn get_color(&self, p: Point) -> Color {
        match self.panels.get(&p) {
            Some(&c) => c,
            None => Color::Black,
        }
    }

    pub fn painted_panel_count(&self) -> usize {
        self.panels.len()
    }

}

impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (min, max) = self.panels.keys()
            .fold((Point::origin(), Point::origin()), |(min, max), p| (
                Point::new(min.x.min(p.x), min.y.min(p.y)),
                Point::new(max.x.max(p.x), max.y.max(p.y)),
            ));
        let mut bar = "-".repeat((max.x - min.x + 1) as usize) + "-+\n";
        bar.insert_str(0, "+-");
        let mut result = bar.clone();
        for y in (min.y..=max.y).rev() {
            result.push_str("| ");
            for x in min.x..=max.x {
                let p = Point::new(x, y);
                result.push(match self.panels.get(&p) {
                    Some(c) => match c {
                        Color::Black => ' ',
                        Color::White => '#',
                    },
                    None => ' '
                })
            }
            result.push_str(" |\n");
        }
        result.push_str(&bar);
        f.write_str(&result)
    }
}
