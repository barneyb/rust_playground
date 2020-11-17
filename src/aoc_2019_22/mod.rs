use crate::fs;
use crate::cli;

#[cfg(test)]
mod test;

static DECK_SIZE: i64 = 119315717514047;
#[allow(dead_code)]
static ITR_COUNT: i64 = 101741582076661;

pub fn run() {
    let ops = parse();

    let rev_ops = invert_ops(&ops, 10007);
    let it = slam_shuffle(&ops, 10007, 2019);
    println!("{}", it);
    let it = slam_shuffle(&rev_ops, 10007, it);
    println!("{}", it);

    let rev_ops = invert_ops(&ops, DECK_SIZE);
    let it = slam_shuffle(&rev_ops, DECK_SIZE, 2020);
    println!("{}", it);
    let it = slam_shuffle(&ops, DECK_SIZE, it);
    println!("{}", it);
}

fn invert_ops(ops: &Vec<Op>, deck_size: i64) -> Vec<Op> {
    let mut rev_ops = ops.to_vec();
    rev_ops.reverse();
    for i in 0..(rev_ops.len()) {
        rev_ops[i] = rev_ops[i].invert(deck_size);
    }
    rev_ops
}

fn slam_shuffle(ops: &Vec<Op>, deck_size: i64, card: i64) -> i64 {
    ops.iter().fold(card, |idx, op| op.perform(deck_size, idx))
}

fn multiply(mut a: i64, mut b: i64, modulus: i64) -> i64 {
    let mut result = 0;
    a %= modulus; // this'll be unneeded for us, but useful in general
    while b > 0 {
        if b % 2 == 1 {
            result = (result + a) % modulus;
        }
        a = (a * 2) % modulus;
        b /= 2;
    }
    result
}

#[allow(dead_code)]
fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a < b {
        return gcd(b, a);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn inverse(a: i64, n: i64) -> i64 {
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
        panic!("{} is not invertible mod {}", a, n);
    }
    if t < 0 {
        t = t + n;
    }
    t
}

#[derive(Debug, Clone)]
enum Op {
    Reverse(),
    Cut(i64),
    Deal(i64),
}

impl Op {
    fn perform(&self, deck_size: i64, idx: i64) -> i64 {
        match self {
            Op::Reverse() => (deck_size - idx - 1) % deck_size,
            Op::Cut(n) => (deck_size + idx - n) % deck_size,
            Op::Deal(n) => multiply(idx, *n, deck_size),
        }
    }

    fn invert(&self, deck_size: i64) -> Op {
        match self {
            Op::Reverse() => Op::Reverse(),
            Op::Cut(n) => Op::Cut(-1 * n),
            Op::Deal(n) => Op::Deal(inverse(*n, deck_size)),
        }
    }
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
