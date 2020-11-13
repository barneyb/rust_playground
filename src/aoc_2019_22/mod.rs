use crate::fs;
use crate::cli;

#[cfg(test)]
mod test;

pub fn run() {
    let ops = parse();
    let it = slam_shuffle(&ops, 10007, 2019);
    println!("{}", it);
    let it = unslam_shuffle(&ops, 10007, it);
    println!("{}", it);
}

fn slam_shuffle(ops: &Vec<Op>, deck_size: i32, card: i32) -> i32 {
    ops.iter().fold(card, |idx, op| shuffle(op, deck_size, idx))
}

fn unslam_shuffle(ops: &Vec<Op>, deck_size: i32, card: i32) -> i32 {
    let mut rev_ops = ops.to_vec();
    rev_ops.reverse();
    rev_ops.iter().fold(card, |idx, op| unshuffle(op, deck_size, idx))
}

fn shuffle(op: &Op, deck_size: i32, idx: i32) -> i32 {
    match op {
        Op::Reverse() => (deck_size - idx - 1) % deck_size,
        Op::Cut(n) => (deck_size + idx - n) % deck_size,
        Op::Deal(n) => (idx * n) % deck_size,
    }
}

fn unshuffle(op: &Op, deck_size: i32, idx: i32) -> i32 {
    match op {
        Op::Reverse() => (deck_size - idx - 1) % deck_size,
        Op::Cut(n) => (deck_size + idx + n) % deck_size,
        Op::Deal(n) => idx * inverse(*n, deck_size) % deck_size,
    }
}

fn inverse(a: i32, n: i32) -> i32 {
    let mut t = 0;
    let mut newt = 1;
    let mut r = n;
    let mut newr = a;

    while newr != 0 {
        let quotient = r / newr;
        let pt = t;
        t = newt;
        newt = pt  - quotient * newt;
        let pr = r;
        r = newr;
        newr = pr - quotient * newr;
    }

    if r > 1 {
        panic!("a '{}' is not invertible mod {}", a, n);
    }
    if t < 0 {
        t = t + n;
    }
    t
}

#[derive(Debug, Clone)]
enum Op {
    Reverse(),
    Cut(i32),
    Deal(i32),
}

fn parse() -> Vec<Op> {
    fs::read_lines(
        cli::aoc_filename("aoc_2019_22.txt"),
        |s| {
            if s == "deal into new stack" {
                Op::Reverse()
            } else if s.starts_with("cut") {
                Op::Cut(s.split(' ').last().unwrap().parse().unwrap())
            } else if s.starts_with("deal") {
                Op::Deal(s.split(' ').last().unwrap().parse().unwrap())
            } else {
                panic!("Unrecognized shuffle '{}'", s)
            }
        }
    ).unwrap()
}
