#[subtest]
#[test]
#[expect(
    unused_variables,
    reason = "the lint fires on the statement following the subtest"
)]
fn a_test_function_triggering_the_lint_after_the_subtest() {
    #[subtest]
    fn a_subtest_inheriting_nothing() {
        // this subtest inherits none of the code the lint fires on, so an inherited `expect`
        // could not be fulfilled here (but it passes since we turn it into an `allow`)
        assert_eq!(1 + 1, 2);
    }

    let unused = 1;
}
