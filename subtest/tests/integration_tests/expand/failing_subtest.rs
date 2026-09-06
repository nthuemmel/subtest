use subtest::subtest;

#[subtest]
#[test]
fn a_test_function_with_a_panicking_subtest() {
    let value = 1;

    // Ignored so that a normal test run stays green - `a_panicking_subtest_is_reported_as_a_failure`
    // in tests/integration_tests/run.rs runs it in a child process and asserts that it is reported.
    #[subtest]
    #[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
    fn a_subtest_panicking() {
        assert_eq!(value, 2);
    }

    assert_eq!(value, 1);
}

#[subtest]
#[test]
#[expect(
    clippy::unnecessary_wraps,
    reason = "the return type exists for the subtest below"
)]
fn a_test_function_with_a_subtest_returning_an_error() -> Result<(), String> {
    let list: Vec<u32> = Vec::new();

    // A subtest inherits the `-> Result` return type, so it can fail without panicking - which
    // `#[should_panic]` cannot express, as functions using it must return `()`.
    #[subtest]
    #[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
    fn a_subtest_returning_an_error() {
        let first = list.first().ok_or("the list is empty")?;
        assert_eq!(*first, 1);
        Ok(())
    }

    assert!(list.is_empty());
    Ok(())
}

#[subtest]
#[test]
#[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
fn a_panicking_test_function() {
    // Declared before the panic below, so it inherits none of it and would pass on its own - the
    // failure has to be reported for the test function, not for the subtest.
    #[subtest]
    fn a_subtest_of_a_panicking_test_function() {
        assert_eq!(1 + 1, 2);
    }

    panic!("the test function itself fails");
}
