#[subtest]
#[test]
fn parent() {
    #[subtest]
    #[should_panic(expected = "my failure")]
    fn child_should_panic() {
        #[subtest(inherit_attributes = false)]
        #[test]
        fn grandchild_reset() {}

        #[subtest]
        fn grandchild_should_panic() {}
    }
}
