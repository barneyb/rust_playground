use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::aoc_2019_11::Color;
use crate::geom2d::{Dir, Point, Turn};
use crate::intcode::{Int, Machine, Program};

use super::ship::Ship;

pub struct Bot<'a> {
    ship: &'a mut Ship,
    pos: Point,
    facing: Dir,
    stdin: Sender<Int>,
    stdout: Receiver<Int>,
}

impl Bot<'_> {

    pub fn new<'a>(orig_prog: &Program, ship: &'a mut Ship) -> Bot<'a> {
        let (stdin, rx) = mpsc::channel();
        let (tx, stdout) = mpsc::channel();
        let prog = orig_prog.clone();
        thread::spawn(move || {
            let mut machine = Machine::new(&prog);
            machine.with_stdin(rx);
            machine.with_stdout(tx);
            machine.run();
        });
        Bot {
            ship: ship,
            stdin,
            stdout,
            pos: Point::origin(),
            facing: Dir::Up,
        }
    }

    pub fn run(&mut self) {
        loop {
            let curr_color = match self.ship.get_color(self.pos) {
                Color::Black => 0,
                Color::White => 1,
            };
            self.stdin
                .send(curr_color)
                .expect("failed to send from camera");
            match self.stdout.recv() {
                Ok(c) => self.ship.paint(self.pos, match c {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => panic!("Unrecognized color: {}", c),
                }),
                Err(_) => break, // blindly assume it's because the machine completed...
            }
            match self.stdout.recv() {
                Ok(d) => {
                    self.facing = self.facing.turn(match d {
                        0 => Turn::CounterClockWise,
                        1 => Turn::ClockWise,
                        _ => panic!("Unrecognized turn direction: {}", d),
                    });
                    self.pos = self.pos.step(&self.facing);
                },
                Err(e) => panic!("Failed to read turn direction: {}", e),
            }
        }
    }

}
