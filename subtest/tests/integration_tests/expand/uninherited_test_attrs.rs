use subtest::subtest;

#[subtest]
#[test]
#[ignore = "the test function is ignored on purpose, its subtest has to run regardless"]
fn an_ignored_test_function() {
    let value = 1;

    #[subtest]
    fn a_subtest_of_an_ignored_test_function() {
        assert_eq!(value, 1);
    }

    assert_eq!(value, 1);
    unimplemented!("never reached, as this test function is ignored")
}

#[subtest]
#[test]
#[should_panic(expected = "the test function panics")]
fn a_test_function_which_should_panic() {
    let value = 1;

    // Reported as "test did not panic as expected" if `#[should_panic]` were inherited, as the
    // panic below is not among the statements this subtest inherits.
    #[subtest]
    fn a_subtest_of_a_test_function_which_should_panic() {
        assert_eq!(value, 1);
    }

    assert_eq!(value, 1);
    panic!("the test function panics");
}
