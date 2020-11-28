use std::any::Any;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_13.txt");
    let mut prog = intcode::read_from_file(filename);
    let (tx, stdout) = mpsc::channel();
    let thread = thread::spawn(move || {
        let mut m = intcode::Machine::new(&prog);
        m.with_stdout(tx);
        m.run();
    });
    // now read!
    let mut output = Vec::new();
    for v in stdout {
        output.push(v)
    }
    let mut count = 0;
    for i in (2..output.len()).step_by(3) {
        if output[i] == 2 {
            count += 1;
        }
    }
    println!("{}", count);
    thread.join().expect("Failed to join thread");
}
