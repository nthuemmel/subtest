//! see `return_type_reset` for cases where the `unused_unit` lint should be suppressed
use subtest::subtest;

#[subtest]
#[test]
#[expect(
    clippy::unused_unit,
    reason = "a top-level test function inherits no return type, so this `-> ()` resets nothing \
              and has to keep being reported as unneeded"
)]
fn a_top_level_test_function_returning_unit() -> () {
    let number = 1;

    // inherits the `-> ()` above, which is the same as declaring no return type at all - so there
    // is nothing here for the lint to report
    #[subtest]
    fn the_inherited_number_is_one() {
        assert_eq!(number, 1);
    }

    assert_eq!(number, 1);
}

#[subtest]
#[test]
fn a_test_function_with_a_subtest_returning_unit() {
    let number = 1;

    // the parent returns `()` anyway, so this `-> ()` resets nothing and stays reported
    #[subtest]
    #[expect(
        clippy::unused_unit,
        reason = "this `-> ()` resets nothing, as the parent test function returns () anyway"
    )]
    fn the_declared_number_is_one() -> () {
        assert_eq!(number, 1);
    }

    assert_eq!(number, 1);
}
