use std::fs;
use crate::util_args;

pub fn run() {
    let filename = if let Some(n) = util_args().next() {
        n
    } else {
        String::from("aoc_2019_01.txt")
    };

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", &contents[0..20]);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2, 1 + 1);
    }
}
