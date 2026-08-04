#[subtest]
#[test]
#[should_panic(expected = "my failure")]
fn parent() {
    #[subtest(inherit_attributes = false)]
    #[test]
    fn child() {
        #[subtest(inherit_attributes = false)]
        #[test]
        #[ignore]
        fn grandchild() {}
    }
}
