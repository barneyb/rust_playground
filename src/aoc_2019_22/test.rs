use super::*;

#[test]
fn reverse() {
    assert_eq!(shuffle(&Op::Reverse(), 10, 9), 0);
}

#[test]
fn unreverse() {
    assert_eq!(unshuffle(&Op::Reverse(), 10, 0), 9);
}

#[test]
fn cut_positive() {
    assert_eq!(shuffle(&Op::Cut(3), 10, 0), 7);
    assert_eq!(shuffle(&Op::Cut(3), 10, 2), 9);
    assert_eq!(shuffle(&Op::Cut(3), 10, 3), 0);
    assert_eq!(shuffle(&Op::Cut(3), 10, 9), 6);
}

#[test]
fn uncut_positive() {
    assert_eq!(unshuffle(&Op::Cut(3), 10, 7), 0);
    assert_eq!(unshuffle(&Op::Cut(3), 10, 9), 2);
    assert_eq!(unshuffle(&Op::Cut(3), 10, 0), 3);
    assert_eq!(unshuffle(&Op::Cut(3), 10, 6), 9);
}

#[test]
fn cut_negative() {
    assert_eq!(shuffle(&Op::Cut(-4), 10, 0), 4);
    assert_eq!(shuffle(&Op::Cut(-4), 10, 5), 9);
    assert_eq!(shuffle(&Op::Cut(-4), 10, 6), 0);
    assert_eq!(shuffle(&Op::Cut(-4), 10, 9), 3);
}

#[test]
fn uncut_negative() {
    assert_eq!(unshuffle(&Op::Cut(-4), 10, 4), 0);
    assert_eq!(unshuffle(&Op::Cut(-4), 10, 9), 5);
    assert_eq!(unshuffle(&Op::Cut(-4), 10, 0), 6);
    assert_eq!(unshuffle(&Op::Cut(-4), 10, 3), 9);
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

fn deal_symmetry(ds: i64, n: i64) {
    let op = Op::Deal(n);
    for i in 0..ds {
        let si = shuffle(&op, ds, i);
        let usi = unshuffle(&op, ds, i);
        assert_eq!(unshuffle(&op, ds, si), i);
        assert_eq!(shuffle(&op, ds, usi), i);
    }
}

#[test]
fn undeal() {
    deal_symmetry(7, 3)
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

#[test]
fn test_inverse() {
    assert_eq!(inverse(3, 7), 5);
    assert_eq!(inverse(4, 7), 2);
}
