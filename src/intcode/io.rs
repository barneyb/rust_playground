use std::collections::{LinkedList, VecDeque};
use std::sync::mpsc::{Receiver, Sender, SyncSender};

pub trait InStream<T> {
    fn read(&mut self) -> T;
}

impl<T> InStream<T> for Vec<T> {
    fn read(&mut self) -> T {
        if self.len() == 0 {
            panic!("No input is available")
        }
        self.remove(0)
    }
}

impl<T> InStream<T> for LinkedList<T> {
    fn read(&mut self) -> T {
        match self.pop_front() {
            Some(v) => v,
            None => panic!("No input is available"),
        }
    }
}

impl<T> InStream<T> for VecDeque<T> {
    fn read(&mut self) -> T {
        match self.pop_front() {
            Some(v) => v,
            None => panic!("No input is available"),
        }
    }
}

impl<T> InStream<T> for Receiver<T> {
    fn read(&mut self) -> T {
        self.recv().unwrap()
    }
}

pub trait OutStream<T> {
    fn write(&mut self, n: T);
}

impl<T> OutStream<T> for Vec<T> {
    fn write(&mut self, n: T) {
        self.push(n)
    }
}

impl<T> OutStream<T> for LinkedList<T> {
    fn write(&mut self, n: T) {
        self.push_back(n)
    }
}

impl<T> OutStream<T> for VecDeque<T> {
    fn write(&mut self, n: T) {
        self.push_back(n)
    }
}

impl<T> OutStream<T> for Sender<T> {
    fn write(&mut self, n: T) {
        self.send(n).expect("Failed to send")
    }
}

impl<T> OutStream<T> for SyncSender<T> {
    fn write(&mut self, n: T) {
        self.send(n).expect("Failed to send")
    }
}
