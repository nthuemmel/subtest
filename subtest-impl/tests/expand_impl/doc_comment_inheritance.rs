#[subtest]
#[test]
#[allow(clippy::eq_op)]
/// Documents the parent test fn.
fn parent() {
    /// Documents the child subtest fn.
    ///
    /// Doc comments must not count as an attribute override, and must not be inherited themselves.
    #[subtest]
    fn child() {
        #[subtest]
        fn grandchild() {}
    }
}
