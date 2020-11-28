use super::*;

#[test]
fn part_one_discussion() {
    let prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(3), 70);
    assert_eq!(m.read_addr(0), 3500);
}

#[test]
fn part_one_example_one() {
    let prog = vec![1, 0, 0, 0, 99];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(0), 2);
}

#[test]
fn part_one_example_two() {
    let prog = vec![2, 3, 0, 3, 99];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(3), 6);
}

#[test]
fn part_one_example_three() {
    let prog = vec![2, 4, 4, 5, 99, 0];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(5), 9801);
}

#[test]
fn part_one_example_four() {
    let prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(0), 30);
    assert_eq!(m.read_addr(4), 2);
}

#[test]
fn basic_io() {
    let prog = vec![3, 0, 4, 0, 99];
    let (m, output) = one_off(&prog, Some(vec![42, 1, 2, 3]));
    assert_eq!(m.read_addr(0), 42); // the temp storage
    assert_eq!(output, vec![42]); // wrote it out
}

#[test]
fn parameter_modes() {
    let prog = vec![1002, 4, 3, 4, 33];
    let m = one_off_machine(&prog, None);
    assert_eq!(m.read_addr(4), 99);
}

#[test]
fn equals_position() {
    let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    let output = one_off_output(&prog, Some(vec![4]));
    assert_eq!(output, vec![0]);
    let output = one_off_output(&prog, Some(vec![8]));
    assert_eq!(output, vec![1]);
    let output = one_off_output(&prog, Some(vec![12]));
    assert_eq!(output, vec![0]);
}

#[test]
fn equals_immediate() {
    let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
    let output = one_off_output(&prog, Some(vec![4]));
    assert_eq!(output, vec![0]);
    let output = one_off_output(&prog, Some(vec![8]));
    assert_eq!(output, vec![1]);
    let output = one_off_output(&prog, Some(vec![12]));
    assert_eq!(output, vec![0]);
}

#[test]
fn quine() {
    let prog = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];
    let output = one_off_output(&prog, None);
    assert_eq!(prog, output)
}

#[test]
fn compute_16_digit_number() {
    let prog = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
    let output = one_off_output(&prog, None);
    assert_eq!(output[0].to_string().len(), 16)
}

#[test]
fn output_huge_number() {
    let n = 1125899906842624;
    let prog = vec![104, n, 99];
    let output = one_off_output(&prog, None);
    assert_eq!(output[0], n)
}
