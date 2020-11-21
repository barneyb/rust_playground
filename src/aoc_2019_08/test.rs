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

