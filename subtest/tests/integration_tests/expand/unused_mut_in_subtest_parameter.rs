use rstest::rstest;
use subtest::subtest;

#[subtest]
#[rstest]
#[case(1)]
#[case(2)]
fn value_can_be_incremented(#[case] mut value: u32) {
    #[subtest]
    fn value_is_positive() {
        // the subtest inherits `value`, but does not modify it - which must not be reported as an
        // unnecessary `mut`, because the parent test function below does modify it
        assert!(value > 0);
    }

    value += 1;
    assert!(value > 1);
}
