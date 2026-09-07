use rstest::rstest;
use subtest::subtest;

#[subtest]
#[rstest]
#[case(1)]
#[case(2)]
fn a_number_is_positive(#[case] number: i32) {
    // the subtest inherits `number` without declaring it, and does not use it - which must not be
    // reported as an unused variable, because the parent test function below does use it
    #[subtest]
    fn a_subtest_ignores_the_parameter() {
        let unrelated = "no number in sight";
        assert_eq!(unrelated, "no number in sight");
    }

    assert!(number > 0);
}
