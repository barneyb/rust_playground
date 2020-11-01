use crate::cli;
use std::fs;

mod machine;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_02.txt");
    let mut prog = read_program(filename);
    prog[1] = 12;
    prog[2] = 2;
    machine::run(&mut prog);
    println!("Result: {}", prog[0]);
}

fn read_program(filename: String) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|a| a.parse().expect(&format!("couldn't parse '{}'", a)))
        .collect()
}
