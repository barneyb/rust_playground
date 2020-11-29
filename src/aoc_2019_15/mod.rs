use crate::cli;
use crate::geom2d::Point;
use crate::geom2d::{Dir, Turn};
use crate::intcode;
use crate::intcode::{Processor, Program};
use crate::plane::{Plane, BW, BW::*, ToChar};

enum Markers {
    Unknown,
    Droid,
    Wall,
    Visited,
}
use Markers::*;

impl ToChar for Markers {
    fn to_char(&self) -> char {
        match self {
            Unknown => ' ',
            Droid => 'D',
            Wall => '#',
            Visited => '.',
        }
    }
}

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_15.txt");
    let prog = intcode::read_from_file(filename);

    let mut ship = Plane::new(Unknown);
    // let proc = Processor::new(prog.clone());
    let mut pos = Point::origin();
    ship.paint(pos, Droid);
    println!("{}", ship);
}
