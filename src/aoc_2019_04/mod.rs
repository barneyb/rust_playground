use std::fs;
use crate::cli;

pub fn run() {
    let (mut curr, end) = parse();
    println!(" from {}", curr);
    println!("   to {}", end);
    let mut count = 0;
    let mut iterations = 0;
    'outer: while curr <= end {
        iterations += 1;
        let mut last = curr % 10;
        let mut rest = curr / 10;
        let mut mult = 1;
        let mut double = false;
        while rest > 0 {
            let next = rest % 10;
            if next > last {
                curr += (next - last) * mult;
                continue 'outer
            }
            rest /= 10;
            mult *= 10;
            double = double || next == last;
            last = next;
        }
        if double {
            count += 1;
        }
        curr += 1;
    }
    println!("count {} in {} iterations", count, iterations);
}

fn parse() -> (i32, i32) {
    let parts = fs::read_to_string(
        cli::aoc_filename("aoc_2019_04.txt")
    )
        .unwrap()
        .trim()
        .split('-')
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    (parts[0], parts[1])
}
