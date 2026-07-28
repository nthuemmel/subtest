#[subtest]
#[tokio::test]
async fn value_can_be_sent_async() {
    let (sender, receiver) = tokio::sync::mpsc::channel(5);
    sender.send("Hello!").await.unwrap();

    #[subtest]
    async fn value_can_be_received() {
        let mut receiver = receiver;
        let value = receiver.recv().await.unwrap();
        assert_eq!(value, "Hello!");
    }

    drop(receiver);
}
