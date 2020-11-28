use crate::cli;
use crate::fs;

pub fn run() {
    let masses: Vec<usize> = fs::read_lines(cli::aoc_filename("aoc_2019_01.txt"), |l| {
        l.parse::<usize>().unwrap()
    })
    .unwrap();

    let fuel: usize = masses.iter().map(needed_fuel).sum();

    println!("Fuel needed: {}", fuel);

    let fuel: usize = masses.iter().map(actually_needed_fuel).sum();

    println!("Fuel ACTUALLY needed: {}", fuel);
}

fn needed_fuel(mass: &usize) -> usize {
    mass / 3 - 2
}

fn actually_needed_fuel(mass: &usize) -> usize {
    let mut curr = needed_fuel(mass);
    let mut total = curr;
    while curr > 8 {
        curr = needed_fuel(&curr);
        total += curr;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_mass_cases() {
        assert_eq!(needed_fuel(&12), 2);
        assert_eq!(needed_fuel(&14), 2);
        assert_eq!(needed_fuel(&1969), 654);
        assert_eq!(needed_fuel(&100756), 33583);
    }

    #[test]
    fn part_two_mass_cases() {
        assert_eq!(actually_needed_fuel(&14), 2);
        assert_eq!(actually_needed_fuel(&1969), 966);
        assert_eq!(actually_needed_fuel(&100756), 50346);
    }
}
