use crate::cli;
use crate::intcode;
use crate::intcode::{Int, Program};

const MAGIC_TARGET_VALUE: Int = 19690720;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_02.txt");
    let prog = intcode::read_from_file(filename);
    println!("Part One: {}", eval_noun_verb(&prog, 12, 2));

    let mut result: Int = -1;
    'outer: for n in 0..=99 {
        for v in 0..=99 {
            if eval_noun_verb(&prog, n, v) == MAGIC_TARGET_VALUE {
                result = n * 100 + v;
                break 'outer;
            }
        }
    }
    println!("Part Two: {}", result);
}

fn eval_noun_verb(prog: &Program, noun: Int, verb: Int) -> Int {
    let mut m = intcode::Machine::new(&prog);
    m.write_addr(1, noun);
    m.write_addr(2, verb);
    m.run();
    m.read_addr(0)
}
