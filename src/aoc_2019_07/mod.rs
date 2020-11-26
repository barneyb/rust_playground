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

type PhaseSettings = [i32; 5];

#[derive(Debug)]
struct OptimalPhaseSettings {
    settings: PhaseSettings,
    signal: i32,
}

fn find_optimial_phase_settings(prog: &Vec<i32>) -> OptimalPhaseSettings {
    let mut best = OptimalPhaseSettings {
        settings: [0, 0, 0, 0, 0],
        signal: -1,
    };
    for settings in AllPhaseSettings::new() {
        let signal = thruster_signal(&prog, &settings);
        if signal > best.signal {
            best = OptimalPhaseSettings {
                settings, // i32 is copy, thus is [i32;n] also, so the clone is free
                signal,
            }
        }
    }
    best
}

#[derive(Debug)]
struct AllPhaseSettings {
    setting: PhaseSettings,
    stack: [usize; 5],
    ptr: usize,
    started: bool,
}

impl AllPhaseSettings {
    fn new() -> AllPhaseSettings {
        AllPhaseSettings {
            setting: [0, 1, 2, 3, 4],
            stack: [0, 0, 0, 0, 0],
            ptr: 0,
            started: false,
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let t = self.setting[i];
        self.setting[i] = self.setting[j];
        self.setting[j] = t;
    }
}

impl Iterator for AllPhaseSettings {
    type Item = PhaseSettings;

    // Heap's algorithm, iteratively, w/ suspension. generators are coming...
    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            return Some(self.setting);
        }
        while self.ptr < 5 {
            if self.stack[self.ptr] < self.ptr {
                if self.ptr % 2 == 0 {
                    self.swap(0, self.ptr);
                } else {
                    self.swap(self.stack[self.ptr], self.ptr);
                }
                // return it, but first set up for reentrance
                self.stack[self.ptr] += 1;
                self.ptr = 0;
                return Some(self.setting);
            } else {
                // on to the next!
                self.stack[self.ptr] = 0;
                self.ptr += 1;
            }
        }
        None
    }

}

fn thruster_signal(orig_prog: &Vec<i32>, phase_settings: &PhaseSettings) -> i32 {
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
