use crate::cli;
use std::fs;

mod machine;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_02.txt");
    let prog = read_program(filename);
    println!("Part One: {}", eval_noun_verb(&prog, 12, 2));

    let mut result: i32 = -1;
    'outer: for n in 0..=99 {
        for v in 0..=99 {
            if eval_noun_verb(&prog, n, v) == 19690720 {
                result = n * 100 + v;
                break 'outer;
            }
        }
    };
    println!("Part Two: {}", result);
}

fn eval_noun_verb(prog: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut prog = prog.clone();
    prog[1] = noun;
    prog[2] = verb;
    machine::run(&mut prog);
    prog[0]
}

fn read_program(filename: String) -> Vec<i32> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|a| a.parse().expect(&format!("couldn't parse '{}'", a)))
        .collect()
}
