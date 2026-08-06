#[subtest]
#[test]
#[allow(clippy::eq_op)]
fn parent() {
    /// A doc comment next to overriding attributes is kept, but does not affect the override.
    #[subtest(inherit_attributes = false)]
    #[test]
    fn child() {}
}
