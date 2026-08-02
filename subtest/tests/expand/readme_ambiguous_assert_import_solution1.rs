use subtest::subtest;

#[subtest]
#[test]
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();

    #[subtest]
    fn value_can_be_received() {
        use assert2::assert;
        let value = receiver.recv().unwrap();
        assert!(value == "Hello!");
    }

    drop(receiver);
}
