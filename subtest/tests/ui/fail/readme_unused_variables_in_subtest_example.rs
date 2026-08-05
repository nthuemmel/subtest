#![deny(unused_variables)]

fn main() {}

use subtest::subtest;

#[subtest(allow_missing_test_attribute)]
// #[test] // if marked as test, the function is removed before the diagnostic shows, because trybuild doesn't compile with `--test`
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();

    #[subtest]
    fn another_value_can_be_sent() {
        sender.send("Hello again!").unwrap();
    }

    let value = receiver.recv().unwrap();
    assert_eq!(value, "Hello!");
}
