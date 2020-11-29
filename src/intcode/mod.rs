use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Mul};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::intcode::Mode::{Immediate, Position, Relative};

#[cfg(test)]
mod test;

pub type Int = i64;
pub type Program = Vec<Int>;
pub type Buffer = Vec<Int>;
pub type TxInt = mpsc::Sender<Int>;
pub type RxInt = mpsc::Receiver<Int>;
use std::thread::JoinHandle;

pub fn read_from_file(filename: String) -> Program {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split(',')
        .map(|a| a.parse().expect(&format!("couldn't parse '{}'", a)))
        .collect()
}

pub struct Processor {
    pub stdin: TxInt,
    pub stdout: RxInt,
    thread: JoinHandle<Machine>,
}

/// I encapsulate an Intcode machine in an isolated thread, with STDIN and STDOUT channels.
impl Processor {
    pub fn new(program: Program) -> Processor {
        let (stdin, rx) = mpsc::channel();
        let (tx, stdout) = mpsc::channel();
        let thread = thread::spawn(move || {
            let mut m = Machine::new(&program);
            m.with_stdin(rx);
            m.with_stdout(tx);
            m.run();
            m
        });
        Processor {
            stdin,
            stdout,
            thread,
        }
    }

    pub fn join(self) -> thread::Result<Machine> {
        self.thread.join()
    }
}

pub struct Machine {
    ip: usize,
    modes: Int,
    rel_base: Int,
    program: Program,
    memory: HashMap<usize, Int>,
    stdin: Option<RxInt>,
    stdout: Option<TxInt>,
}

enum Mode {
    Position,
    Immediate,
    Relative,
}

#[allow(dead_code)]
pub fn one_off_machine(prog: &Program, input: Option<Buffer>) -> Machine {
    one_off(prog, input).0
}

pub fn one_off_output(prog: &Program, input: Option<Buffer>) -> Buffer {
    one_off(prog, input).1
}

fn one_off(prog: &Program, input: Option<Buffer>) -> (Machine, Buffer) {
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

    pub fn with_stdin(&mut self, rx: RxInt) {
        self.stdin = Some(rx);
    }

    pub fn with_stdout(&mut self, tx: TxInt) {
        self.stdout = Some(tx);
    }

    /// I execute the program, and return my STDIN
    pub fn run(&mut self) -> Option<RxInt> {
        while self.running() {
            self.step();
        }
        self.stdout = None;
        std::mem::take(&mut self.stdin)
    }

    fn step(&mut self) {
        if self.halted() {
            panic!("Can't step when already halted");
        }
        match self.next_op() {
            1 => self.binary_op(Int::add),
            2 => self.binary_op(Int::mul),
            3 => {
                let pos = self.next_position();
                self.write_addr(
                    pos,
                    match &self.stdin {
                        // I'm just gonna decree that nothing should take more than one second to catch
                        // back up with another concurrent piece, so if we hit that threshold it means
                        // there's a race/deadlock in the _logic_.
                        Some(rx) => rx
                            .recv_timeout(Duration::from_secs(1))
                            .expect("Failed to read from STDIN"),
                        None => panic!("No STDIN is connected"),
                    },
                );
            }
            4 => {
                let value = self.next_param();
                match &self.stdout {
                    Some(tx) => tx.send(value).expect("Failed to send to STDOUT"),
                    None => println!("{}", value),
                }
            }
            5 => self.conditional_jump_op(|a| a != 0),
            6 => self.conditional_jump_op(|a| a == 0),
            7 => self.binary_op(|a, b| if a < b { 1 } else { 0 }),
            8 => self.binary_op(|a, b| if a == b { 1 } else { 0 }),
            9 => {
                let a = self.next_param();
                self.rel_base += a;
            }
            99 => self.ip = usize::max_value(),
            opcode => panic!("Unknown opcode {} (at position {})", opcode, self.ip - 1),
        };
    }

    pub fn read_addr(&self, addr: usize) -> Int {
        if addr < self.program.len() {
            self.program[addr]
        } else {
            match self.memory.get(&addr) {
                Some(v) => *v,
                None => 0,
            }
        }
    }

    pub fn write_addr(&mut self, addr: usize, value: Int) {
        if addr < self.program.len() {
            self.program[addr] = value;
        } else {
            self.memory.insert(addr, value);
        }
    }

    fn next(&mut self) -> Int {
        let v = self.read_addr(self.ip);
        self.ip += 1;
        v
    }

    fn next_op(&mut self) -> Int {
        let i = self.next();
        self.modes = i / 100;
        i % 100
    }

    fn next_mode(&mut self) -> Mode {
        let m = self.modes % 10;
        self.modes /= 10;
        match m {
            0 => Position,
            1 => Immediate, // immediate
            2 => Relative,  // relative
            md => panic!("Unknown parameter mode {}", md),
        }
    }

    fn next_param(&mut self) -> Int {
        let v = self.next();
        match self.next_mode() {
            Position => self.read_addr(v as usize),
            Immediate => v,
            Relative => self.read_addr((self.rel_base + v) as usize),
        }
    }

    fn next_position(&mut self) -> usize {
        let v = self.next();
        (match self.next_mode() {
            Position => v,
            Immediate => panic!("Positions cannot use immediate mode"),
            Relative => self.rel_base + v,
        }) as usize
    }

    fn binary_op<F>(&mut self, mut op: F)
    where
        F: FnMut(Int, Int) -> Int,
    {
        let a = self.next_param();
        let b = self.next_param();
        let c = self.next_position();
        self.write_addr(c, op(a, b));
    }

    fn conditional_jump_op<F>(&mut self, mut test: F)
    where
        F: FnMut(Int) -> bool,
    {
        let a = self.next_param();
        let b = self.next_param();
        if test(a) {
            self.ip = b as usize;
        }
    }

    pub fn halted(&self) -> bool {
        !self.running()
    }

    pub fn running(&self) -> bool {
        self.ip < self.program.len()
    }
}
