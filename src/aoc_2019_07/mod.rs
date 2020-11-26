use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;

use crate::cli;
use crate::intcode;
use crate::intcode::{Machine, Program};

#[cfg(test)]
mod test;

const PHASES_SINGLE: [i32; 5] = [0, 1, 2, 3, 4];
const PHASES_FEEDBACK: [i32; 5] = [5, 6, 7, 8, 9];

pub fn run() {
    let filename = cli::aoc_filename("aoc_2019_07.txt");
    let orig_prog = intcode::read_from_file(filename);

    println!("{:?}", find_optimal_phase_settings(&orig_prog, PHASES_SINGLE, thruster_signal));
    println!("{:?}", find_optimal_phase_settings(&orig_prog, PHASES_FEEDBACK, thruster_signal_with_feedback));
}

type PhaseSettings = [i32; 5];

#[derive(Debug)]
struct OptimalPhaseSettings {
    settings: PhaseSettings,
    signal: i32,
}

fn find_optimal_phase_settings<F>(prog: &Program, settings: PhaseSettings, generator: F) -> OptimalPhaseSettings
where F: Fn(&Program, &PhaseSettings) -> i32
{
    let mut best = OptimalPhaseSettings {
        settings: [0, 0, 0, 0, 0],
        signal: -1,
    };
    for settings in AllPhaseSettings::from(settings) {
        let signal = generator(&prog, &settings);
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
    fn from(settings: PhaseSettings) -> AllPhaseSettings {
        AllPhaseSettings {
            setting: settings,
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

fn thruster_signal(orig_prog: &Program, phase_settings: &PhaseSettings) -> i32 {
    let mut signal = 0;
    let mut input = VecDeque::new();
    let mut output = VecDeque::new();
    for phase in phase_settings.iter() {
        input.push_back(*phase);
        input.push_back(signal);
        let mut prog = orig_prog.clone();
        let mut m = Machine::new(&mut prog);
        m.stdin(&mut input);
        m.stdout(&mut output);
        m.run();
        signal = output.pop_front().unwrap();
    }
    signal
}

fn thruster_signal_with_feedback(orig_prog: &Program, phase_settings: &PhaseSettings) -> i32 {
    let mut senders = VecDeque::new();
    let mut receivers = VecDeque::new();
    for _ in 0..5 {
        let (s, r) = mpsc::channel();
        senders.push_back(s);
        receivers.push_back(r);
    }

    // rotate the senders around so they're indexed right
    senders.rotate_right(1);

    // the phase settings
    for (s, p) in senders.iter().zip(phase_settings) {
        s.send(*p).expect("failed to send phase setting")
    }

    senders.front().unwrap()
        .send(0)
        .expect("failed to send initial signal");

    // rotate the senders back...
    senders.rotate_left(1);

    // ...and the receivers forward
    receivers.rotate_right(1);

    let mut threads = Vec::new();
    for (i, (mut s, mut r)) in senders
        .into_iter()
        .zip(receivers)
        .enumerate() {
        let mut prog = orig_prog.clone();
        threads.push(thread::spawn(move || {
            let mut m = Machine::new(&mut prog);
            m.stdin(&mut r);
            m.stdout(&mut s);
            m.run();
            if i == 0 {
                r.recv().unwrap() // the final signal
            } else {
                0
            }
        }));
    }

    let mut sig = -1;
    for t in threads {
        let s = t.join();
        if sig < 0 {
            sig = s.unwrap()
        }
    }

    sig // the final signal
}
