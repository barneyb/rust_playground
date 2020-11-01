use crate::fs;
use crate::cli;
use crate::point2d::{Dir, Point};
use std::collections::HashMap;

pub fn run() {
    let wires: Vec<Vec<Step>> = fs::read_lines(
        cli::aoc_filename("aoc_2019_03.txt"),
        parse_line
    ).unwrap();

    let mut points = HashMap::new();
    let mut count = 0;
    walk_wire(&wires[0], |p| {
        count += 1;
        points.insert(p, count);
    });

    let mut closest = i32::max_value();
    let mut fastest = i32::max_value();
    count = 0;
    walk_wire(&wires[1], |p| {
        count += 1;
        if let Some(first_count) = points.get(&p) {
            closest = closest.min(p.manhattan_distance());
            fastest = fastest.min(first_count + count);
        }
    });
    println!("closest intersection: {} steps", closest);
    println!("fastest intersection: {} steps", fastest);
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

