use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_02.txt");
    let prog = intcode::read_from_file(filename);
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
    let mut m = intcode::Machine::new(&prog);
    m.write_addr(1, noun);
    m.write_addr(2, verb);
    m.run();
    m.read_addr(0)
}
