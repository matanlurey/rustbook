mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, automated_tests::add_two(2));
}
