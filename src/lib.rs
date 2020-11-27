use std::collections::HashMap;

pub mod cli;
pub mod fs;
mod guess_number;
mod temp_convert;
mod aoc_2019_01;
mod aoc_2019_02;
mod aoc_2019_03;
mod aoc_2019_04;
mod aoc_2019_05;
mod aoc_2019_07;
mod aoc_2019_08;
mod aoc_2019_22;
mod point2d;
mod intcode;

pub struct Utilities<'a> {
    utilities: HashMap<&'a str, fn()>,
    names: Vec<&'a str>,
}

impl<'a> Utilities<'a> {

    pub fn new() -> Utilities<'a> {
        let mut utilities: HashMap<&str, fn()> = HashMap::new();
        // todo: "add Sally to Engineering" / "show Engineering" interface
        // todo: convert strings to pig latin
        // todo: mean/median/mode of list of integers
        // todo: 12 days of christmas
        // todo: fibonacci
        utilities.insert("temp", temp_convert::run);
        utilities.insert("guess", guess_number::run);
        utilities.insert("aoc_2019_01", aoc_2019_01::run);
        utilities.insert("aoc_2019_02", aoc_2019_02::run);
        utilities.insert("aoc_2019_03", aoc_2019_03::run);
        utilities.insert("aoc_2019_04", aoc_2019_04::run);
        utilities.insert("aoc_2019_05", aoc_2019_05::run);
        utilities.insert("aoc_2019_07", aoc_2019_07::run);
        utilities.insert("aoc_2019_08", aoc_2019_08::run);
        utilities.insert("aoc_2019_22", aoc_2019_22::run);
        let mut names: Vec<&str> = utilities.iter()
            .map(|(k, _)| *k)
            .collect();
        names.sort_unstable();
        Utilities {
            utilities,
            names,
        }
    }

    pub fn names(&self) -> &Vec<&str> {
        &self.names
    }

    pub fn by_name(&self, name: &str) -> Option<&fn()> {
        self.utilities.get(name)
    }

}
