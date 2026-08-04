#[subtest]
#[test]
#[should_panic(expected = "my failure")]
fn parent() {
    #[subtest]
    fn child() {
        #[subtest(inherit_attributes = true)]
        fn grandchild() {}
    }
}
