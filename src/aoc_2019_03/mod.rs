use crate::fs;
use crate::cli;
use crate::point2d::{Dir, Point};
use std::collections::HashSet;

pub fn run() {
    let wires: Vec<Vec<Step>> = fs::read_lines(
        cli::aoc_filename("aoc_2019_03.txt"),
        parse_line
    ).unwrap();

    let mut points = HashSet::new();
    walk_wire(&wires[0], |p| {
        points.insert(p);
    });

    let mut best = i32::max_value();
    walk_wire(&wires[1], |p| {
        if points.contains(&p) {
            best = best.min(p.manhattan_distance())
        }
    });
    println!("closest intersection: {}", best);
}

fn walk_wire<F>(wire: &Vec<Step>, mut f: F)
    where F: FnMut(Point)
{
    let mut curr = Point::origin();
    for s in wire.iter() {
        for _ in 0..s.count {
            curr = curr.step(&s.dir);
            &f(curr);
        }
    }
}

fn parse_line(l: &str) -> Vec<Step> {
    l.split(',')
        .map(Step::parse)
        .collect()
}

impl Dir {

    fn parse(c: char) -> Dir {
        match c {
            'U' | 'u' => Dir::Up,
            'D' | 'd' => Dir::Down,
            'R' | 'r' => Dir::Right,
            'L' | 'l' => Dir::Left,
            _ => panic!(),
        }
    }

}

#[derive(Debug)]
struct Step {
    dir: Dir,
    count: usize,
}

impl Step {

    fn parse(s: &str) -> Step {
        let dir = match s.chars().next() {
            Some(c) => Dir::parse(c),
            _ => panic!(),
        };
        Step {
            dir,
            count: s[1..].parse().unwrap(),
        }
    }
}

