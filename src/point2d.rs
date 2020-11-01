use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Dir {
    Up, Down, Right, Left,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[allow(dead_code)]
impl Point {

    pub fn origin() -> Point {
        Point::at(0, 0)
    }

    pub fn at(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn step(&self, dir: &Dir) -> Point {
        self.step_by(dir, 1)
    }

    pub fn step_by(&self, dir: &Dir, count: i32) -> Point {
        match dir {
            Dir::Up => Point {
                x: self.x,
                y: self.y + count,
            },
            Dir::Down => Point {
                x: self.x,
                y: self.y - count,
            },
            Dir::Right => Point {
                x: self.x + count,
                y: self.y,
            },
            Dir::Left => Point {
                x: self.x - count,
                y: self.y,
            },
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn manhattan_distance_from(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

}
