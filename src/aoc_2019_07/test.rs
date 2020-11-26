use super::*;

#[test]
fn all_phases_iterator() {
    let mut aps = AllPhaseSettings::new();
    println!("{:?}", aps);
    println!("{:?}", aps.next());
    println!("{:?}", aps.next());
    println!("{:?}", aps.next());
    assert_eq!(aps.count(), 5 * 4 * 3 * 2 * 1 - 3) // 5!, minus the three already printed
}

#[test]
fn example_one() {
    let prog = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    let sig = thruster_signal(&prog, &[4,3,2,1,0]);
    assert_eq!(43210, sig);
    let opt = find_optimal_phase_settings(&prog, [0, 1, 2, 3, 4], thruster_signal);
    assert_eq!(43210, opt.signal);
    assert_eq!([4,3,2,1,0], opt.settings);
}

#[test]
fn example_two() {
    let prog = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    let sig = thruster_signal(&prog, &[0,1,2,3,4]);
    assert_eq!(54321, sig);
    let opt = find_optimal_phase_settings(&prog, [0, 1, 2, 3, 4], thruster_signal);
    assert_eq!(54321, opt.signal);
    assert_eq!([0,1,2,3,4], opt.settings);
}

#[test]
fn example_three() {
    let prog = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    let sig = thruster_signal(&prog, &[1,0,4,3,2]);
    assert_eq!(65210, sig);
    let opt = find_optimal_phase_settings(&prog, [0, 1, 2, 3, 4], thruster_signal);
    assert_eq!(65210, opt.signal);
    assert_eq!([1,0,4,3,2], opt.settings);
}

#[test]
fn example_four() {
    let prog = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    let sig = thruster_signal_with_feedback(&prog, &[9,8,7,6,5]);
    assert_eq!(139629729, sig);
    let opt = find_optimal_phase_settings(&prog, [5, 6, 7, 8, 9], thruster_signal_with_feedback);
    assert_eq!(139629729, opt.signal);
    assert_eq!([9,8,7,6,5], opt.settings);
}

#[test]
fn example_five() {
    let prog = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
    let sig = thruster_signal_with_feedback(&prog, &[9,7,8,5,6]);
    assert_eq!(18216, sig);
    let opt = find_optimal_phase_settings(&prog, [5, 6, 7, 8, 9], thruster_signal_with_feedback);
    assert_eq!(18216, opt.signal);
    assert_eq!([9,7,8,5,6], opt.settings);
}
