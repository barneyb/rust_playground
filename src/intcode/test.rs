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

// #[test]
// fn basic_io() {
//     let mut prog = vec![3,0,4,0,99];
//     let mut m = Machine::new(&mut prog);
//     let mut input = vec![42, 1, 2, 3];
//     let mut output = vec![4, 5, 6];
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(prog[0], 42); // the temp storage
//     assert_eq!(input[0], 1); // read from the head
//     assert_eq!(output[3], 42); // wrote to the tail
// }
//
// #[test]
// fn parameter_modes() {
//     let mut prog = vec![1002,4,3,4,33];
//     Machine::new(&mut prog).run();
//     assert_eq!(prog[4], 99);
// }
//
// #[test]
// fn equals_position() {
//     let mut prog = vec![3,9,8,9,10,9,4,9,99,-1,8];
//     let mut input = vec![4, 8, 12];
//     let mut output = vec![];
//     let mut m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 0); // less than is not equal
//     m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 1); // equal is equal
//     m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 0); // greater than is not equal
// }
//
// #[test]
// fn equals_immediate() {
//     let mut prog = vec![3,3,1108,-1,8,3,4,3,99];
//     let mut input = vec![4, 8, 12];
//     let mut output = vec![];
//     let mut m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 0); // less than is not equal
//     m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 1); // equal is equal
//     m = Machine::new(&mut prog);
//     m.stdin(&mut input);
//     m.stdout(&mut output);
//     m.run();
//     assert_eq!(output.remove(0), 0); // greater than is not equal
// }
