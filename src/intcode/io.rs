use std::collections::LinkedList;

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
