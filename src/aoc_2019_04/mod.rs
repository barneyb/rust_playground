use std::fs;
use crate::cli;

pub fn run() {
    let (mut curr, end) = parse();
    println!(" from {}", curr);
    println!("   to {}", end);
    println!("range {}", end - curr + 1);
    let mut count = 0;
    let mut iterations = 0;
    while curr <= end {
        iterations += 1;
        let r = test(curr);
        if r.accept {
            count += 1;
        }
        curr += r.skip;
    }
    println!("count {} in {} iterations", count, iterations);
}

struct Result {
    accept: bool,
    skip: i32,
}

fn test(n: i32) -> Result {
    let mut last = n % 10;
    let mut rest = n / 10;
    let mut place = 1;
    let mut accept = false;
    while rest > 0 {
        let curr = rest % 10;
        // if we're ascending, skip to the next possibility
        if curr > last {
            return Result {
                accept: false,
                skip: (curr - last) * place,
            }
        }
        rest /= 10;
        place *= 10;
        // check for a run
        if curr == last {
            accept = true;
        }
        last = curr;
    }
    return Result {
        accept,
        skip: 1,
    }
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
