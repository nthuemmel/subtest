use subtest::subtest;

#[subtest]
#[test]
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();

    assert2::assert!(let Ok(value) = receiver.recv());

    #[subtest]
    fn another_value_can_be_sent() {
        // the subtest inherits `value`, but does not use it - which must not be reported as an
        // unused variable, because the parent test function does use it below
        sender.send("Hello again!").unwrap();
    }

    assert_eq!(value, "Hello!");
}
