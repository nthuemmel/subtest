#[subtest]
#[test]
#[ignore = "my reason"]
#[should_panic(expected = "my failure")]
fn parent() {
    #[subtest]
    fn child() {
        #[subtest]
        fn grandchild() {}
    }
}
