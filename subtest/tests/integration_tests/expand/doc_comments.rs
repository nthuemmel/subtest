#[subtest::subtest]
#[test]
fn doc_comments() {
    let i = 1;
    assert_eq!(i, 1);

    /// A doc comment must not override the inherited `#[test]` attribute - otherwise this subtest
    /// would silently never run.
    #[subtest]
    fn documented() {
        let i = i + 1;
        assert_eq!(i, 2);
    }

    /// Neither must a lint attribute.
    #[subtest]
    #[allow(clippy::eq_op)]
    fn documented_with_lint_attr() {
        assert_eq!(i, i);
    }

    /// Explicit test attributes still override.
    #[subtest(inherit_attributes = false)]
    #[test]
    #[should_panic(expected = "my failure")]
    fn documented_with_override() {
        panic!("my failure");
    }
}
