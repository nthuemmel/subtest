#[subtest]
#[test]
#[expect(unused_variables, reason = "kept for a later commit")]
fn a_test_function_with_an_unused_variable() {
    let unused = 1;

    #[subtest]
    fn a_subtest_which_does_not_trigger_the_lint() {
        // The `expect` above is inherited as an `allow`. Inherited as an `expect` it would
        // be unfulfilled here, because we mask the unused-variables error for statements inherited
        // from the parent automatically in the subtest macro.
        assert_eq!(1 + 1, 2);
    }

    assert_eq!(1 + 1, 2);
}
