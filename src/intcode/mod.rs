use std::ops::{Add, Mul};
use self::io::{InStream, OutStream};

mod io;

struct Input<'a> {
    buffer: Option<&'a mut dyn io::InStream<i32>>,
}

impl<'a> Input<'a> {
    fn read(&mut self) -> i32 {
        match &mut self.buffer {
            Some(b) => b.read(),
            None => panic!("No input is available!")
        }
    }
}

struct Output<'a> {
    buffer: Option<&'a mut dyn io::OutStream<i32>>,
}

impl<'a> Output<'a> {
    fn write(&mut self, n: i32) {
        if let Some(b) = &mut self.buffer {
            b.write(n)
        }
    }
}

pub struct Machine<'a> {
    ip: usize,
    program: &'a mut Vec<i32>,
    stdin: Input<'a>,
    stdout: Output<'a>,
}

impl<'a> Machine<'a> {

    pub fn new(program: &mut Vec<i32>) -> Machine {
        Machine {
            ip: 0,
            program,
            stdin: Input { buffer: None },
            stdout: Output { buffer: None },
        }
    }

    pub fn stdin(&mut self, buffer: &'a mut dyn InStream<i32>) {
        self.stdin = Input { buffer: Some(buffer) }
    }

    pub fn stdout(&mut self, buffer: &'a mut dyn OutStream<i32>) {
        self.stdout = Output { buffer: Some(buffer) }
    }

    pub fn run(&mut self) {
        while !self.halted() {
            self.step();
        }
    }

    fn step(&mut self) {
        if self.halted() {
            panic!("Can't step when already halted");
        }
        match self.next() {
            1 => self.binary_op(i32::add),
            2 => self.binary_op(i32::mul),
            3 => {
                let pos = self.next_pos();
                self.program[pos] = self.stdin.read()
            },
            4 => {
                let pos = self.next_pos();
                self.stdout.write(self.program[pos])
            },
            99 => self.ip = usize::max_value(),
            opcode => panic!("Unknown opcode {} (at position {})", opcode, self.ip),
        };
    }

    fn next(&mut self) -> i32 {
        let v = self.program[self.ip];
        self.ip += 1;
        v
    }

    fn next_pos(&mut self) -> usize {
        self.next() as usize
    }

    fn binary_op<F>(&mut self, mut f: F)
        where F: FnMut(i32, i32) -> i32
    {
        let a = self.next_pos();
        let b = self.next_pos();
        let c = self.next_pos();
        self.program[c] = f(self.program[a], self.program[b]);
    }

    fn halted(&self) -> bool {
        self.ip >= self.program.len()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_discussion() {
        let mut prog = vec![1,9,10,3,2,3,11,0,99,30,40,50];
        Machine::new(&mut prog).run();
        assert_eq!(prog[3], 70);
        assert_eq!(prog[0], 3500);
    }

    #[test]
    fn part_one_example_one() {
        let mut prog = vec![1,0,0,0,99];
        Machine::new(&mut prog).run();
        assert_eq!(prog[0], 2);
    }

    #[test]
    fn part_one_example_two() {
        let mut prog = vec![2,3,0,3,99];
        Machine::new(&mut prog).run();
        assert_eq!(prog[3], 6);
    }

    #[test]
    fn part_one_example_three() {
        let mut prog = vec![2,4,4,5,99,0];
        Machine::new(&mut prog).run();
        assert_eq!(prog[5], 9801);
    }

    #[test]
    fn part_one_example_four() {
        let mut prog = vec![1,1,1,4,99,5,6,0,99];
        Machine::new(&mut prog).run();
        assert_eq!(prog[0], 30);
        assert_eq!(prog[4], 2);
    }

    #[test]
    fn basic_io() {
        let mut prog = vec![3,0,4,0,99];
        let mut m = Machine::new(&mut prog);
        let mut input = vec![42, 1, 2, 3];
        let mut output = vec![4, 5, 6];
        m.stdin(&mut input);
        m.stdout(&mut output);
        m.run();
        assert_eq!(prog[0], 42); // the temp storage
        assert_eq!(input[0], 1); // read from the head
        assert_eq!(output[3], 42); // wrote to the tail
    }

}
