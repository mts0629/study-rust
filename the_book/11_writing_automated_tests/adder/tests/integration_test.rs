use adder::add_two;

mod common;

#[test]
fn it_add_two() {
    common::setup(); // Test setup

    let result = add_two(2);
    assert_eq!(result, 4);
}
