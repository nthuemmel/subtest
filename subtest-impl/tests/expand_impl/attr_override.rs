#[subtest]
#[test]
#[should_panic(expected = "my failure")]
fn parent() {
    #[subtest]
    #[test]
    fn child() {
        #[subtest]
        #[test]
        #[ignore]
        fn grandchild() {}
    }
}
