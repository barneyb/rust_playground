use bot::Bot;
use ship::Ship;

use crate::cli;
use crate::geom2d::Point;
use crate::intcode;

mod ship;
mod bot;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_11.txt");
    let prog = intcode::read_from_file(filename);

    let mut ship = Ship::new();
    let mut bot = Bot::new(&prog, &mut ship);
    bot.run();
    println!("Panels painted: {}", ship.painted_panel_count());

    let mut ship = Ship::new();
    ship.paint(Point::origin(), Color::White);
    let mut bot = Bot::new(&prog, &mut ship);
    bot.run();
    println!("{}", ship);
}

#[derive(Copy, Clone)]
pub enum Color {
    Black, White,
}

