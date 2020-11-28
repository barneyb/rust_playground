use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use crate::cli;
use crate::fs;
use crate::geom2d::{Dir, Point};

pub fn run() {
    let wires: Vec<Vec<Step>> = fs::read_lines(
        cli::aoc_filename("aoc_2019_03.txt"),
        parse_line
    ).unwrap();

    let points = points_on_wire(&wires[0]);

    let stats = cross_stats(&points, &wires[1]);

    println!("closest intersection: {} steps", stats.closest);
    println!("fastest intersection: {} steps", stats.fastest);
}

struct CrossStats {
    closest: i32,
    fastest: i32,
}

fn cross_stats(points: &HashMap<Point, i32, RandomState>, wire: &Vec<Step>) -> CrossStats {
    let mut closest = i32::max_value();
    let mut fastest = i32::max_value();
    let mut point_count = 0;
    walk_wire_by_point(&wire, |p| {
        point_count += 1;
        if let Some(first_count) = points.get(&p) {
            closest = closest.min(p.manhattan_distance());
            fastest = fastest.min(first_count + point_count);
        }
    });
    CrossStats {
        closest,
        fastest,
    }
}

fn points_on_wire(wire: &Vec<Step>) -> HashMap<Point, i32, RandomState> {
    let mut points = HashMap::with_capacity(
        wire_length(wire));

    let mut point_count = 0;
    walk_wire_by_point(wire, |p| {
        point_count += 1;
        points.entry(p).or_insert(point_count);
    });

    points
}

fn wire_length(wire: &Vec<Step>) -> usize {
    let mut count = 1;
    walk_wire_by_step(wire, |s| count += s.count);
    count
}

fn walk_wire_by_point<F>(wire: &Vec<Step>, mut f: F)
    where F: FnMut(Point)
{
    let mut curr = Point::origin();
    walk_wire_by_step(wire, |s| {
        for _ in 0..s.count {
            curr = curr.step(&s.dir);
            f(curr);
        }
    });
}

fn walk_wire_by_step<F>(wire: &Vec<Step>, mut f: F)
    where F: FnMut(&Step)
{
    for s in wire.iter() {
        f(s);
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

