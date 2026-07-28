use subtest::subtest;

fn main() {}

#[subtest]
// #[tokio::test] // if marked as test, the function is removed before the diagnostic shows, because trybuild doesn't compile with `--test`
async fn value_can_be_sent_async() {
    let (sender, receiver) = tokio::sync::mpsc::channel(5);
    sender.send("Hello!").await.unwrap();

    #[subtest]
    // #[test]
    fn value_can_be_received() {
        let mut receiver = receiver;
        let value = receiver.try_recv().unwrap();
        assert_eq!(value, "Hello!");
    }

    drop(receiver);
}
