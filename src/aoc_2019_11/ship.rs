use std::collections::HashMap;

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
