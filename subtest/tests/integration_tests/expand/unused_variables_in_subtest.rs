use subtest::subtest;

#[subtest]
#[test]
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();

    #[subtest]
    fn another_value_can_be_sent() {
        // the subtest inherits `receiver`, but does not use it - which must not be reported as an
        // unused variable, because the parent test function below does use it
        sender.send("Hello again!").unwrap();
    }

    let value = receiver.recv().unwrap();
    assert_eq!(value, "Hello!");
}
