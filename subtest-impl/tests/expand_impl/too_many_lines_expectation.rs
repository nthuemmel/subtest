#[subtest]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "this test function is long on purpose"
)]
fn a_long_test_function() {
    #[subtest]
    fn a_short_subtest() {
        // this subtest inherits none of the statements below, so it stays short - which means the
        // expectation above must not be inherited either, as it could never be fulfilled here
        assert_eq!(1 + 1, 2);
    }

    let mut total = 0;
    total += 1;
    total += 2;
    total += 3;
    total += 4;
    total += 5;
    total += 6;
    total += 7;
    total += 8;
    total += 9;
    total += 10;
    assert_eq!(total, 55);

    // This subtest inherits every statement above, but clippy doesn't count inherited lines,
    // so `too_many_lines` does not fire. We have to turn inheritance off, otherwise the parent's
    // `expect` would be turned into an `allow` on the nested subtest
    #[subtest(inherit_attributes = false)]
    #[test]
    fn a_subtest_inheriting_more_lines_than_the_threshold() {
        assert_eq!(total, 55);
    }
}
