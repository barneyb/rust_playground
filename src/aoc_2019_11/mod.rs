use ship::Ship;

use crate::cli;
use crate::geom2d::Point;
use crate::geom2d::{Dir, Turn};
use crate::intcode;
use crate::intcode::{Program, Processor};

mod ship;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_11.txt");
    let prog = intcode::read_from_file(filename);

    let mut ship = Ship::new();
    run_bot(&prog, &mut ship);
    println!("Panels painted: {}", ship.painted_panel_count());

    let mut ship = Ship::new();
    run_bot(&prog, &mut ship);
    println!("{}", ship);

    let mut ship = Ship::new();
    ship.paint(Point::origin(), Color::White);
    run_bot(&prog, &mut ship);
    println!("{}", ship);
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White,
}

fn run_bot(orig_prog: &Program, ship: &mut Ship) {
    let proc = Processor::new(orig_prog.clone());
    let mut pos = Point::origin();
    let mut facing = Dir::Up;
    loop {
        let curr_color = match ship.get_color(pos) {
            Color::Black => 0,
            Color::White => 1,
        };
        // For reasons I don't really understand, stdin lasts longer than stdout, so it may still be
        // open to send while stdout will be closed to receive immediately afterward. Perhaps this
        // is just a race between the deallocation of the streams (in the thread) and here?
        //
        // The net is that a failure must be checked at either point, though it's far more likely at
        // the receive site. Because of the mem::take in Machine::run?
        match proc.stdin.send(curr_color) {
            Ok(_) => {}
            Err(_) => break,
        }
        match proc.stdout.recv() {
            Ok(c) => ship.paint(
                pos,
                match c {
                    0 => Color::Black,
                    1 => Color::White,
                    _ => panic!("Unrecognized color: {}", c),
                },
            ),
            // Err(e) => break,
            Err(_) => break,
        }
        match proc.stdout.recv() {
            Ok(d) => {
                facing = facing.turn(match d {
                    0 => Turn::CounterClockWise,
                    1 => Turn::ClockWise,
                    _ => panic!("Unrecognized turn direction: {}", d),
                });
                pos = pos.step(&facing);
            }
            Err(e) => panic!("Failed to read turn direction: {}", e),
        }
    }
    match proc.join() {
        Ok(_) => {}
        Err(e) => panic!("Failed to join thread: {:?}", e),
    }
}
