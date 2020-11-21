use super::*;

#[test]
fn example_one() {
    let img = parse(3, 2, "123456789012");
    assert_eq!(3, img.width);
    assert_eq!(2, img.height);
    assert_eq!(2, img.layer_count());

    let l = img.get_layer(0);
    assert_eq!(3, l.width);
    assert_eq!(2, l.height);
    assert_eq!(vec![1, 2, 3, 4, 5, 6], l.data);

    let l = img.get_layer(1);
    assert_eq!(3, l.width);
    assert_eq!(2, l.height);
    assert_eq!(vec![7, 8, 9, 0, 1, 2], l.data);
}

#[test]
fn layers_next() {
    // three 2x2 layers
    let img = parse(2, 2, "111122223333");
    let mut itr = img.layers();
    match itr.next() {
        Some(_) => {},
        _ => panic!("premature exhaustion!"),
    }
    match itr.next() {
        Some(_) => {},
        _ => panic!("premature exhaustion!"),
    }
    match itr.next() {
        Some(_) => {},
        _ => panic!("premature exhaustion!"),
    }
    for _ in 0..2 {
        match itr.next() {
            None => {},
            _ => panic!("delayed exhaustion!"),
        }
    }
}

#[test]
fn layers_size_hint() {
    // three 2x2 layers
    let img = parse(2, 2, "111122223333");
    let mut itr = img.layers();
    assert_eq!((3, Some(3)), itr.size_hint());
    itr.next();
    assert_eq!((2, Some(2)), itr.size_hint());
    itr.next();
    assert_eq!((1, Some(1)), itr.size_hint());
    for _ in 0..2 {
        itr.next();
        assert_eq!((0, Some(0)), itr.size_hint());
    }
}

#[test]
fn layers_nth() {
    // three 2x2 layers
    let img = parse(2, 2, "111122223333");
    let mut itr = img.layers();
    match itr.nth(1) {
        Some(l) => assert_eq!(4, l.count_of(2)),
        _ => panic!("premature exhaustion!"),
    }
    match itr.nth(0) {
        Some(l) => assert_eq!(4, l.count_of(3)),
        _ => panic!("premature exhaustion!"),
    }
    match itr.nth(0) {
        None => {},
        _ => panic!("delayed exhaustion!"),
    }
}

#[test]
fn layers_nth_out_of_bounds() {
    // three 2x2 layers
    let img = parse(2, 2, "111122223333");
    let mut itr = img.layers();
    match itr.nth(42) {
        None => {},
        _ => panic!("delayed exhaustion!"),
    }
}

#[test]
fn example_two() {
    let img = parse(2, 2, "0222112222120000");
    println!("{}", render_layer(&img.get_layer(0)));
    println!("{}", render_layer(&img.get_layer(1)));
    println!("{}", render_layer(&img.get_layer(2)));
    println!("{}", render_layer(&img.get_layer(3)));
    let flat = img.flatten();
    println!("{}", render_layer(&flat.get_layer(0)));
    assert_eq!("+----+\n\
                |  # |\n\
                | #  |\n\
                +----+\n", part_two(&img))
}
