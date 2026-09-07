use subtest::subtest;

#[subtest]
#[test]
fn a_helper_is_declared() {
    // Fulfilled in the parent test function, which never calls the helper. The subtest below does
    // call it, so `dead_code` cannot fire in the copy inherited by the subtest - which leaves the
    // expectation unfulfilled there, on a line the author wrote for the parent.
    #[expect(dead_code, reason = "called only by the subtest below")]
    fn helper() {}

    let value = 1;

    #[subtest]
    fn a_subtest_calls_the_helper() {
        helper();
        assert_eq!(value, 1);
    }

    assert_eq!(value, 1);
}
