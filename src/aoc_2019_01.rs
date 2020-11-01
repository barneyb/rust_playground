use crate::fs;
use crate::cli;

const DEFAULT_FILENAME: &'static str = "aoc_2019_01.txt";

pub fn run() {
    let masses: Vec<usize> = fs::read_lines(
        get_filename(),
        |l| l.parse::<usize>().unwrap()
    ).unwrap();

    let fuel: usize = masses
        .iter()
        .map(needed_fuel)
        .sum();

    println!("Fuel needed: {}", fuel)
}

fn get_filename() -> String {
    if let Some(n) = cli::util_args().next() {
        n
    } else {
        String::from(DEFAULT_FILENAME)
    }
}

fn needed_fuel(mass: &usize) -> usize {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_on_mass_cases() {
        assert_eq!(needed_fuel(&12), 2);
        assert_eq!(needed_fuel(&14), 2);
        assert_eq!(needed_fuel(&1969), 654);
        assert_eq!(needed_fuel(&100756), 33583);
    }
}
