use crate::cli;
use std::fs;

mod machine;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_02.txt");
    let prog = read_program(filename);
    println!("prog len {} ({} codes)", prog.len(), prog.len() as f32 / 4.0);
}

fn read_program(filename: String) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|a| a.parse().expect(&format!("couldn't parse '{}'", a)))
        .collect()
}
