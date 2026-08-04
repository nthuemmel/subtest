#[subtest]
#[test]
fn parent() {
    #[subtest(inherit_attributes = false)]
    #[test]
    fn nested_sync() {}

    #[subtest(inherit_attributes = false)]
    #[tokio::test]
    async fn nested_async() -> anyhow::Result<()> {}

    #[subtest(inherit_attributes = false)]
    #[rstest]
    #[case(0, 0)]
    #[case(1, 1)]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(4, 3)]
    fn nested_rstest(#[case] input: u32, #[case] expected: u32) {}
}
