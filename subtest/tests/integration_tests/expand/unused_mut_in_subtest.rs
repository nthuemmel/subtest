use subtest::subtest;

#[subtest]
#[test]
fn value_can_be_sent() {
    let mut value = 1;

    #[subtest]
    fn another_value_can_be_sent() {
        // the subtest inherits `receiver`, but does not use it - which must not be reported as an
        // unused variable, because the parent test function below does use it
        assert_eq!(value, 1);
    }

    value += 1;
    assert_eq!(value, 2);
}
