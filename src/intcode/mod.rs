use std::ops::{Add, Mul};

pub struct Machine<'a> {
    ip: usize,
    program: &'a mut Vec<i32>,
}

impl<'a> Machine<'a> {

    pub fn new(program: &mut Vec<i32>) -> Machine {
        Machine {
            ip: 0,
            program,
        }
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
            99 => self.ip = usize::max_value(),
            opcode => panic!("Unknown opcode {} (at position {})", opcode, self.ip),
        };
    }

    fn next(&mut self) -> i32 {
        let v = self.program[self.ip];
        self.ip += 1;
        v
    }

    fn binary_op<F>(&mut self, mut f: F)
        where F: FnMut(i32, i32) -> i32
    {
        let a = self.next() as usize;
        let b = self.next() as usize;
        let c = self.next() as usize;
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

}
