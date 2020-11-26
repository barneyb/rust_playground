use crate::cli;
use crate::intcode;

#[cfg(test)]
mod test;

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_07.txt");
    let orig_prog = intcode::read_from_file(filename);

    println!("{:?}", find_optimial_phase_settings(&orig_prog));
}

#[derive(Debug)]
struct OptimalPhases {
    phase_settings: [i32; 5],
    signal: i32,
}

fn find_optimial_phase_settings(prog: &Vec<i32>) -> OptimalPhases {
    let mut best = OptimalPhases {
        phase_settings: [0, 0, 0, 0, 0],
        signal: 0,
    };
    for a in 0..5 {
        for b in 0..5 {
            if b == a { continue }
            for c in 0..5 {
                if c == a { continue }
                if c == b { continue }
                for d in 0..5 {
                    if d == a { continue }
                    if d == b { continue }
                    if d == c { continue }
                    for e in 0..5 {
                        if e == a { continue }
                        if e == b { continue }
                        if e == c { continue }
                        if e == d { continue }
                        let phase_settings = [a, b, c, d, e];
                        let signal = thruster_signal(&prog, phase_settings);
                        if signal > best.signal {
                            best = OptimalPhases {
                                phase_settings,
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

fn thruster_signal(orig_prog: &Vec<i32>, phase_settings: [i32; 5]) -> i32 {
    let mut signal = 0;
    let mut input = Vec::new();
    let mut output = Vec::new();
    for phase in phase_settings.iter() {
        input.push(*phase);
        input.push(signal);
        let mut prog = orig_prog.clone();
        let mut m = intcode::Machine::new(&mut prog);
        m.stdin(&mut input);
        m.stdout(&mut output);
        m.run();
        signal = output.remove(0);
    }
    signal
}
