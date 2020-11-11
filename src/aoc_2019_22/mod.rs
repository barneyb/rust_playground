use crate::fs;
use crate::cli;

pub fn run() {
    let ops = parse();
    let it = slam_shuffle(&ops, 10007, 2019);
    println!("{}", it)
}

fn slam_shuffle(ops: &Vec<Op>, deck_size: i32, card: i32) -> i32 {
    ops.iter().fold(card, |idx, op| shuffle(op, deck_size, idx))
}

fn shuffle(op: &Op, deck_size: i32, idx: i32) -> i32 {
    match op {
        Op::Reverse() => (deck_size - idx - 1) % deck_size,
        Op::Cut(n) => (deck_size + idx - n) % deck_size,
        Op::Deal(n) => (idx * n) % deck_size,
    }
}

#[derive(Debug)]
enum Op {
    Reverse(),
    Cut(i32),
    Deal(i32),
}

fn parse() -> Vec<Op> {
    fs::read_lines(
        cli::aoc_filename("aoc_2019_22.txt"),
        |s| {
            if s == "deal into new stack" {
                Op::Reverse()
            } else if s.starts_with("cut") {
                Op::Cut(s.split(' ').last().unwrap().parse().unwrap())
            } else if s.starts_with("deal") {
                Op::Deal(s.split(' ').last().unwrap().parse().unwrap())
            } else {
                panic!("Unrecognized shuffle '{}'", s)
            }
        }
    ).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse() {
        assert_eq!(shuffle(&Op::Reverse(), 10, 9), 0);
    }

    #[test]
    fn cut_positive() {
        assert_eq!(shuffle(&Op::Cut(3), 10, 0), 7);
        assert_eq!(shuffle(&Op::Cut(3), 10, 2), 9);
        assert_eq!(shuffle(&Op::Cut(3), 10, 3), 0);
        assert_eq!(shuffle(&Op::Cut(3), 10, 9), 6);
    }

    #[test]
    fn cut_negative() {
        assert_eq!(shuffle(&Op::Cut(-4), 10, 0), 4);
        assert_eq!(shuffle(&Op::Cut(-4), 10, 5), 9);
        assert_eq!(shuffle(&Op::Cut(-4), 10, 6), 0);
        assert_eq!(shuffle(&Op::Cut(-4), 10, 9), 3);
    }

    #[test]
    fn deal() {
        assert_eq!(shuffle(&Op::Deal(3), 10, 0), 0);
        assert_eq!(shuffle(&Op::Deal(3), 10, 1), 3);
        assert_eq!(shuffle(&Op::Deal(3), 10, 2), 6);
        assert_eq!(shuffle(&Op::Deal(3), 10, 3), 9);
        assert_eq!(shuffle(&Op::Deal(3), 10, 4), 2);
        assert_eq!(shuffle(&Op::Deal(3), 10, 9), 7);
    }

    #[test]
    fn example_one() {
        let ops = vec![
            Op::Deal(7),
            Op::Reverse(),
            Op::Reverse(),
        ];
        assert_eq!(slam_shuffle(&ops, 10, 0), 0);
        assert_eq!(slam_shuffle(&ops, 10, 3), 1);
        assert_eq!(slam_shuffle(&ops, 10, 6), 2);
        assert_eq!(slam_shuffle(&ops, 10, 9), 3);
        assert_eq!(slam_shuffle(&ops, 10, 2), 4);
        assert_eq!(slam_shuffle(&ops, 10, 5), 5);
        assert_eq!(slam_shuffle(&ops, 10, 8), 6);
        assert_eq!(slam_shuffle(&ops, 10, 1), 7);
        assert_eq!(slam_shuffle(&ops, 10, 4), 8);
        assert_eq!(slam_shuffle(&ops, 10, 7), 9);
    }

}
