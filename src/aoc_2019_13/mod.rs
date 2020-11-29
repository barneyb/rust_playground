use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;

use crate::cli;
use crate::intcode;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_13.txt");
    let mut prog = intcode::read_from_file(filename);
    prog[0] = 2;
    let proc = intcode::Processor::new(prog);
    let mut block_count = 0;
    let park_time = Duration::from_millis(1);
    let mut score = -1;
    let mut paddle_x = -1;
    let mut ball_x = -1;

    'each_triple: loop {
        let x = loop {
            match proc.stdout.try_recv() {
                Ok(n) => break n,
                Err(TryRecvError::Disconnected) => break 'each_triple,
                Err(TryRecvError::Empty) => thread::park_timeout(park_time),
            }
        };
        let y = proc.stdout.recv().expect("Failed to receive 'y'");
        let z = proc.stdout.recv().expect("Failed to receive 'z'");
        match z {
            2 => block_count += 1,
            3 => paddle_x = x,
            4 => ball_x = x,
            _ => {}
        }
        if x == -1 && y == 0 {
            score = z;
        }

        if paddle_x >= 0 && ball_x >= 0 {
            let position = if paddle_x < ball_x {
                paddle_x = -1;
                1
            } else if paddle_x > ball_x {
                paddle_x = -1;
                -1
            } else {
                0
            };
            proc.stdin
                .send(position)
                .expect("failed to send joystick position");
            ball_x = -1;
        }
    }
    proc.join().expect("Failed to join thread");
    println!("Block Count: {:5}", block_count);
    println!("Final Score: {:5}", score);
}
