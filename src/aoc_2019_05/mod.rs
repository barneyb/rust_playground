use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_05.txt");
    let prog = intcode::read_from_file(filename);
    let mut prog = prog.clone();
    let mut m = intcode::Machine::new(&mut prog);
    let mut input = vec![1];
    m.stdin(&mut input);
    let mut output = vec![];
    m.stdout(&mut output);
    m.run();
    println!("{:?}", output)
}
