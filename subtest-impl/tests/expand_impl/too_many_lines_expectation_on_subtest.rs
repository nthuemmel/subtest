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
        // expectation above - were it not reported, that expectation would be unfulfilled.
        // Expectations written on the subtest itself are kept, only inherited ones are removed.
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
