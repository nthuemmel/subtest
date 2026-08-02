#[subtest]
#[tokio::test]
async fn value_can_be_sent_async() -> anyhow::Result<()> {
    let (sender, receiver) = tokio::sync::mpsc::channel(5);
    sender.send("Hello!").await?;

    #[subtest]
    #[tokio::test]
    async fn value_can_be_received() -> anyhow::Result<()> {
        let mut receiver = receiver;
        let value = receiver.try_recv()?;
        assert_eq!(value, "Hello!");
        Ok(())
    }

    drop(receiver);
    Ok(())
}
