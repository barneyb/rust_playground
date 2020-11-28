use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_13.txt");
    let prog = intcode::read_from_file(filename);
    let output = intcode::one_off_output(&prog, None);
    let mut count = 0;
    for i in (2..output.len()).step_by(3) {
        if output[i] == 2 {
            count += 1;
        }
    }
    println!("{}", count);
}
