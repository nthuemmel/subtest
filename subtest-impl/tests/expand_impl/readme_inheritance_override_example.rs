#[subtest]
#[test]
fn value_can_be_sent_and_received() {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(5);
    sender.try_send("Hello!").unwrap();
    receiver.try_recv().unwrap();

    #[subtest]
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Empty")]
    fn value_cannot_be_received_a_second_time() {
        receiver.try_recv().unwrap();
    }

    #[subtest]
    #[test]
    #[ignore]
    fn value_can_be_sent_a_second_time() {
        unimplemented!()
    }

    drop(receiver);
}
