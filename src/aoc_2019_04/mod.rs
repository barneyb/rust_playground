use std::fs;
use crate::cli;

pub fn run() {
    let (start, end) = parse();
    println!(" from {}", start);
    println!("   to {}", end);
    let n = (start..=end)
        .filter(|n| {
            let mut last = n % 10;
            let mut rest = n / 10;
            let mut double = false;
            while rest > 0 {
                let next = rest % 10;
                if next > last {
                    return false
                }
                double = double || next == last;
                last = next;
                rest = rest / 10;
            }
            double
        })
        .count();
    println!("count {}", n);
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
