#[subtest]
#[test]
fn parent() {
    let value = 1;

    #[subtest]
    fn child() {
        // several items in a row at the start of the body: they only land after statements
        // because the parent's `let value` is prepended above them
        use std::fmt::Write as _;
        const SUFFIX: &str = "!";
        fn render() {}

        let rendered = render();

        // follows a statement the subtest declares itself, so it stays reported
        fn after_own_statement() {}
    }
}
