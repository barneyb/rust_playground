// use self::io::{InStream, OutStream};
use std::collections::HashMap;
use std::fs;
use std::ops::{Add, Mul};

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
    // stdin: Input<'a>,
    // stdout: Output<'a>,
}

impl Machine {

    pub fn new(program: &Program) -> Machine {
        Machine {
            ip: 0,
            modes: 0,
            rel_base: 0,
            program: program.clone(),
            memory: HashMap::new(),
            // stdin: Input { buffer: None },
            // stdout: Output { buffer: None },
        }
    }

    // pub fn stdin(&mut self, buffer: &'a mut dyn InStream<i32>) {
    //     self.stdin = Input { buffer: Some(buffer) }
    // }
    //
    // pub fn stdout(&mut self, buffer: &'a mut dyn OutStream<i32>) {
    //     self.stdout = Output { buffer: Some(buffer) }
    // }

    pub fn run(&mut self) {
        while !self.halted() {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.halted() {
            panic!("Can't step when already halted");
        }
        match self.next_op() {
            1 => self.binary_op(i32::add),
            2 => self.binary_op(i32::mul),
            // 3 => {
            //     let pos = self.next_position();
            //     self.program[pos] = self.stdin.read()
            // },
            // 4 => {
            //     let value = self.next_param();
            //     self.stdout.write(value)
            // },
            // 5 => {
            //     let a = self.next_param();
            //     let b = self.next_param();
            //     if a != 0 {
            //         self.ip = b as usize;
            //     }
            // },
            // 6 => {
            //     let a = self.next_param();
            //     let b = self.next_param();
            //     if a == 0 {
            //         self.ip = b as usize;
            //     }
            // },
            // 7 => {
            //     let a = self.next_param();
            //     let b = self.next_param();
            //     let c = self.next_position();
            //     self.write_addr(c, if a < b { 1 } else { 0 });
            // },
            // 8 => {
            //     let a = self.next_param();
            //     let b = self.next_param();
            //     let c = self.next_position();
            //     self.write_addr(c, if a == b { 1 } else { 0 });
            // },
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

    fn binary_op<F>(&mut self, mut f: F)
        where F: FnMut(i32, i32) -> i32
    {
        let a = self.next_param();
        let b = self.next_param();
        let c = self.next_position();
        self.write_addr(c, f(a, b));
    }

    fn halted(&self) -> bool {
        self.ip >= self.program.len()
    }

}
