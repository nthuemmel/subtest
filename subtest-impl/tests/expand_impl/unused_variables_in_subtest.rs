#[subtest]
#[test]
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
