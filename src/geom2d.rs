use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Turn {
    CounterClockWise, ClockWise,
}

#[derive(Debug)]
pub enum Dir {
    Up, Down, Right, Left,
}

impl Dir {

    pub fn turn(&self, t: Turn) -> Dir {
        match t {
            Turn::CounterClockWise => match self {
                Dir::Up => Dir::Left,
                Dir::Down => Dir::Right,
                Dir::Right => Dir::Up,
                Dir::Left => Dir::Down,
            },
            Turn::ClockWise => match self {
                Dir::Up => Dir::Right,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Down,
                Dir::Left => Dir::Up,
            },
        }
    }

}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Point {

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn step(&self, dir: &Dir) -> Point {
        self.step_by(dir, 1)
    }

    pub fn step_by(&self, dir: &Dir, count: i32) -> Point {
        match dir {
            Dir::Up    => Point::new(self.x, self.y + count),
            Dir::Down  => Point::new(self.x, self.y - count),
            Dir::Right => Point::new(self.x + count, self.y),
            Dir::Left  => Point::new(self.x - count, self.y),
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

}
