use subtest::subtest;

#[subtest]
#[tokio::test]
async fn value_can_be_sent_async() -> anyhow::Result<()> {
    let (sender, receiver) = tokio::sync::mpsc::channel(5);
    sender.send("Hello!").await?;

    #[subtest]
    async fn value_can_be_received_inherit() {
        let mut receiver = receiver;
        let value = receiver.try_recv()?;
        assert_eq!(value, "Hello!");
        Ok(())
    }

    #[subtest]
    #[tokio::test]
    async fn value_can_be_received_repeat() -> anyhow::Result<()> {
        let mut receiver = receiver;
        let value = receiver.try_recv()?;
        assert_eq!(value, "Hello!");
        Ok(())
    }

    drop(receiver);
    Ok(())
}

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
