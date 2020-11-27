// use self::io::{InStream, OutStream};
use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Mul};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

mod io;

#[cfg(test)]
mod test;

pub type Program = Vec<i32>;

pub fn read_from_file(filename: String) -> Program {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|a| a.parse().expect(&format!("couldn't parse '{}'", a)))
        .collect()
}

// struct Input<'a> {
//     buffer: Option<&'a mut dyn io::InStream<i32>>,
// }
//
// impl<'a> Input<'a> {
//     fn read(&mut self) -> i32 {
//         match &mut self.buffer {
//             Some(b) => b.read(),
//             None => panic!("No input is available!")
//         }
//     }
// }
//
// struct Output<'a> {
//     buffer: Option<&'a mut dyn io::OutStream<i32>>,
// }
//
// impl<'a> Output<'a> {
//     fn write(&mut self, n: i32) {
//         if let Some(b) = &mut self.buffer {
//             b.write(n)
//         }
//     }
// }

pub struct Machine {
    ip: usize,
    modes: i32,
    rel_base: i32,
    program: Program,
    memory: HashMap<usize, i32>,
    stdin: Option<Receiver<i32>>,
    stdout: Option<Sender<i32>>,
}

#[allow(dead_code)]
pub fn one_off_machine(prog: &Program, input: Option<Vec<i32>>) -> Machine {
    one_off(prog, input).0
}

pub fn one_off_output(prog: &Program, input: Option<Vec<i32>>) -> Vec<i32> {
    one_off(prog, input).1
}

fn one_off(prog: &Program, input: Option<Vec<i32>>) -> (Machine, Vec<i32>) {
    let mut m = Machine::new(&prog);

    if let Some(input) = input {
        let (tx, rx) = mpsc::channel();
        for v in input {
            tx.send(v).expect("failed to send");
        }
        m.with_stdin(rx);
    }

    let (tx, rx) = mpsc::channel();
    m.with_stdout(tx);
    m.run();
    let mut output = Vec::new();
    for v in rx {
        output.push(v)
    }
    (m, output)
}

impl Machine {

    pub fn new(program: &Program) -> Machine {
        Machine {
            ip: 0,
            modes: 0,
            rel_base: 0,
            program: program.clone(),
            memory: HashMap::new(),
            stdin: None,
            stdout: None,
        }
    }

    pub fn with_stdin(&mut self, rx: Receiver<i32>) {
        self.stdin = Some(rx);
    }

    pub fn with_stdout(&mut self, tx: Sender<i32>) {
        self.stdout = Some(tx);
    }

    pub fn run(&mut self) {
        while !self.halted() {
            self.step();
        }
        self.stdin = None;
        self.stdout = None;
    }

    fn step(&mut self) {
        if self.halted() {
            panic!("Can't step when already halted");
        }
        match self.next_op() {
            1 => self.binary_op(i32::add),
            2 => self.binary_op(i32::mul),
            3 => {
                let pos = self.next_position();
                self.program[pos] = match &self.stdin {
                    Some(rx) => rx.recv().expect("Failed to read from STDIN"),
                    None => panic!("No STDIN is connected"),
                };
            },
            4 => {
                let value = self.next_param();
                match &self.stdout {
                    Some(tx) => tx.send(value).expect("Failed to send to STDOUT"),
                    None => println!("{}", value),
                }
            },
            5 => self.conditional_jump_op(|a| a != 0),
            6 => self.conditional_jump_op(|a| a == 0),
            7 => self.binary_op(|a, b| if a < b { 1 } else { 0 }),
            8 => self.binary_op(|a, b| if a == b { 1 } else { 0 }),
            // 9 => {
            //     let a = self.next_param();
            //     self.rel_base += a;
            // }
            99 => self.ip = usize::max_value(),
            opcode => panic!("Unknown opcode {} (at position {})", opcode, self.ip - 1),
        };
    }

    pub fn read_addr(&self, addr: usize) -> i32 {
        if addr < self.program.len() {
            self.program[addr]
        } else {
            match self.memory.get(&addr) {
                Some(v) => *v,
                None => 0,
            }
        }
    }

    pub fn write_addr(&mut self, addr: usize, value: i32) {
        if addr < self.program.len() {
            self.program[addr] = value;
        } else {
            self.memory.insert(addr, value);
        }
    }

    fn next(&mut self) -> i32 {
        let v = self.read_addr(self.ip);
        self.ip += 1;
        v
    }

    fn next_op(&mut self) -> i32 {
        let i = self.next();
        self.modes = i / 100;
        i % 100
    }

    fn next_param(&mut self) -> i32 {
        let mut v = self.next();
        v = match self.modes % 10 {
            0 => self.read_addr(v as usize), // position
            1 => v, // immediate
            2 => self.read_addr((self.rel_base + v) as usize), // position
            m => panic!("Unknown parameter mode {}", m),
        };
        self.modes /= 10;
        v
    }

    fn next_position(&mut self) -> usize {
        self.next() as usize
    }

    fn binary_op<F>(&mut self, mut op: F)
        where F: FnMut(i32, i32) -> i32
    {
        let a = self.next_param();
        let b = self.next_param();
        let c = self.next_position();
        self.write_addr(c, op(a, b));
    }

    fn conditional_jump_op<F>(&mut self, mut test: F)
        where F: FnMut(i32) -> bool
    {
        let a = self.next_param();
        let b = self.next_param();
        if test(a) {
            self.ip = b as usize;
        }
    }

    fn halted(&self) -> bool {
        self.ip >= self.program.len()
    }

}
