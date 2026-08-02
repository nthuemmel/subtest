fn main() {}

use assert2::assert;
use subtest::subtest;

#[subtest]
// #[test] // if marked as test, the function is removed before the diagnostic shows, because trybuild doesn't compile with `--test`
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();

    #[subtest]
    fn value_can_be_received() {
        let value = receiver.recv().unwrap();
        assert!(value == "Hello!");
    }

    drop(receiver);
}
