use super::*;

#[test]
fn example_one() {
    let prog = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
    let sig = thruster_signal(&prog, &[4,3,2,1,0]);
    assert_eq!(43210, sig);
    let opt = find_optimial_phase_settings(&prog);
    assert_eq!(43210, opt.signal);
    assert_eq!([4,3,2,1,0], opt.settings);
}

#[test]
fn example_two() {
    let prog = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    let sig = thruster_signal(&prog, &[0,1,2,3,4]);
    assert_eq!(54321, sig);
    let opt = find_optimial_phase_settings(&prog);
    assert_eq!(54321, opt.signal);
    assert_eq!([0,1,2,3,4], opt.settings);
}

#[test]
fn example_three() {
    let prog = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    let sig = thruster_signal(&prog, &[1,0,4,3,2]);
    assert_eq!(65210, sig);
    let opt = find_optimial_phase_settings(&prog);
    assert_eq!(65210, opt.signal);
    assert_eq!([1,0,4,3,2], opt.settings);
}
