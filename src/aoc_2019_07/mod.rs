use crate::cli;
use crate::intcode;
use std::collections::LinkedList;

#[cfg(test)]
mod test;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_07.txt");
    let orig_prog = intcode::read_from_file(filename);

    println!("{:?}", find_optimial_phase_settings(&orig_prog));
}

type Phases = [i32; 5];

#[derive(Debug)]
struct OptimalPhases {
    settings: Phases,
    signal: i32,
}

fn find_optimial_phase_settings(prog: &Vec<i32>) -> OptimalPhases {
    let mut settings = [0, 0, 0, 0, 0];
    let mut best = OptimalPhases {
        settings,
        signal: -1,
    };
    for a in 0..5 {
        settings[0] = a;
        for b in 0..5 {
            if b == a { continue }
            settings[1] = b;
            for c in 0..5 {
                if c == a { continue }
                if c == b { continue }
                settings[2] = c;
                for d in 0..5 {
                    if d == a { continue }
                    if d == b { continue }
                    if d == c { continue }
                    settings[3] = d;
                    for e in 0..5 {
                        if e == a { continue }
                        if e == b { continue }
                        if e == c { continue }
                        if e == d { continue }
                        settings[4] = e;
                        let signal = thruster_signal(&prog, &settings);
                        if signal > best.signal {
                            best = OptimalPhases {
                                settings, // since i32 is copy, so is [i32;n], so the dupe is free
                                signal,
                            }
                        }
                    }
                }
            }
        }
    }
    best
}

fn thruster_signal(orig_prog: &Vec<i32>, phase_settings: &Phases) -> i32 {
    let mut signal = 0;
    let mut input = LinkedList::new();
    let mut output = LinkedList::new();
    for phase in phase_settings.iter() {
        input.push_back(*phase);
        input.push_back(signal);
        let mut prog = orig_prog.clone();
        let mut m = intcode::Machine::new(&mut prog);
        m.stdin(&mut input);
        m.stdout(&mut output);
        m.run();
        signal = output.pop_front().unwrap();
    }
    signal
}
