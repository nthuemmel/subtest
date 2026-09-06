#[subtest]
#[test]
fn parent() -> anyhow::Result<()> {
    // resets the return type, because the parent's `anyhow::Result<()>` is inherited otherwise
    #[subtest]
    fn child() -> () {
        // inherits the `-> ()` of `child`
        #[subtest]
        fn grandchild() {}
    }
}
