#[subtest]
#[test]
fn parent() {
    #[subtest]
    #[allow(clippy::eq_op)]
    #[should_panic(expected = "my failure")]
    fn child_with_added_attrs() {
        #[subtest(inherit_attributes = false)]
        #[test]
        fn grandchild_reset() {}

        #[subtest]
        fn grandchild_inheriting_only_the_allowance() {}
    }
}
