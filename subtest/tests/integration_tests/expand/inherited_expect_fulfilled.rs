use subtest::subtest;

#[subtest]
#[test]
fn a_value_is_declared() {
    // Fulfilled in the parent test function, which never uses `value`. The subtest below does not
    // use it either, so the expectation would hold in the inherited copy as well - but the macro
    // masks inherited declarations with an `#[allow(unused_variables)]` of its own, which is
    // appended after this `#[expect]` and therefore wins. The lint can never fire in the subtest,
    // so the expectation can never be fulfilled there.
    #[expect(unused_variables, reason = "declared for a later commit")]
    let value = 1;

    #[subtest]
    fn a_subtest_does_not_use_it() {
        let other = 2;
        assert_eq!(other, 2);
    }
}
