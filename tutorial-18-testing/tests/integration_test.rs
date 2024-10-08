use tutorial_18_testing;

mod common;

#[test]
fn it_adds_two_integration_test() {
    common::setup();
    let result = tutorial_18_testing::add(2, 2);
    assert_eq!(result, 4);
    // assert_eq!(result, 5);
}
