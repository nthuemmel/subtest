#[subtest]
#[test]
fn parent() -> anyhow::Result<()> {
    #[subtest]
    fn child() {
        #[subtest]
        fn grandchild() {}
    }
}
