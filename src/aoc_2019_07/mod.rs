use crate::cli;
use crate::intcode;
use std::collections::LinkedList;
use std::sync::mpsc;
use std::thread;

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

fn find_optimal_phase_settings<F>(prog: &Vec<i32>, settings: PhaseSettings, generator: F) -> OptimalPhaseSettings
where F: Fn(&Vec<i32>, &PhaseSettings) -> i32
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

fn thruster_signal_with_feedback(orig_prog: &Vec<i32>, phase_settings: &PhaseSettings) -> i32 {

    // all this mutability means I have the wrong stuff on IntCode's IO types
    let (mut aout, mut bin) = mpsc::channel();
    let (mut bout, mut cin) = mpsc::channel();
    let (mut cout, mut din) = mpsc::channel();
    let (mut dout, mut ein) = mpsc::channel();
    let (mut eout, mut ain) = mpsc::channel();

        let mut aprog = orig_prog.clone();
        let mut bprog = orig_prog.clone();
        let mut cprog = orig_prog.clone();
        let mut dprog = orig_prog.clone();
        let mut eprog = orig_prog.clone();

    let toa = eout.clone();
    let tob = aout.clone();
    let toc = bout.clone();
    let tod = cout.clone();
    let toe = dout.clone();

    toa.send(phase_settings[0]);
    tob.send(phase_settings[1]);
    toc.send(phase_settings[2]);
    tod.send(phase_settings[3]);
    toe.send(phase_settings[4]);

    toa.send(0);

    let mut threads = Vec::new();
    threads.push(thread::spawn(move || {
        let mut a = intcode::Machine::new(&mut aprog);

        a.stdin(&mut ain);
        a.stdout(&mut aout);
        a.run();
        ain.recv().unwrap() // the final signal
    }));
    threads.push(thread::spawn(move || {
        let mut b = intcode::Machine::new(&mut bprog);

        b.stdin(&mut bin);
        b.stdout(&mut bout);
        b.run();
        0
    }));
    threads.push(thread::spawn(move || {
        let mut c = intcode::Machine::new(&mut cprog);

        c.stdin(&mut cin);
        c.stdout(&mut cout);
        c.run();
        0
    }));
    threads.push(thread::spawn(move || {
        let mut d = intcode::Machine::new(&mut dprog);

        d.stdin(&mut din);
        d.stdout(&mut dout);
        d.run();
        0
    }));
    threads.push(thread::spawn(move || {
        let mut e = intcode::Machine::new(&mut eprog);
        e.stdin(&mut ein);
        e.stdout(&mut eout);
        e.run();
        0
    }));

    let mut sig = -1;
    for t in threads {
        let s = t.join();
        if sig < 0 {
            sig = s.unwrap()
        }
    }

    sig // the final signal
}
