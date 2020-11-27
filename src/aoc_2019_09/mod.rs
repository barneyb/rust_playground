use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_09.txt");
    let prog = intcode::read_from_file(filename);
    println!("{:?}", intcode::one_off_output(&prog, Some(vec![1])));
    println!("{:?}", intcode::one_off_output(&prog, Some(vec![2])));
}
