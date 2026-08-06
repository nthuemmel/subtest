use subtest::subtest;

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

    #[subtest]
    fn a_subtest_inheriting_more_lines_than_the_threshold() {
        // this subtest inherits every statement above, but the lint counts the lines a subtest is
        // written on - so it is not reported either, and needs no expectation of its own
        assert_eq!(total, 55);
    }
}

#[subtest]
#[test]
#[expect(
    clippy::too_many_lines,
    reason = "this test function is long because of the subtest written in its body"
)]
fn a_test_function_with_a_long_subtest() {
    #[subtest]
    #[expect(clippy::too_many_lines, reason = "this subtest is long on purpose")]
    fn a_long_subtest() {
        // A subtest which is long in its own right *is* reported, which is what fulfills the
        // expectation above
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
    }
}
