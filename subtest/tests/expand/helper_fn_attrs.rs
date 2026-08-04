#[subtest::subtest]
#[test]
fn helper_fn_attrs() {
    /// A doc comment
    #[allow(dead_code)]
    fn documented_and_allowed() {}

    #[warn(clippy::eq_op)]
    fn warns() -> u32 {
        1
    }

    #[deny(unsafe_code)]
    fn denies() -> u32 {
        2
    }

    #[forbid(unsafe_code)]
    fn forbids() -> u32 {
        3
    }

    #[cfg(test)]
    fn always_compiled() -> u32 {
        4
    }

    #[cfg_attr(test, inline)]
    fn conditionally_inlined() -> u32 {
        5
    }

    #[rustfmt::skip]
    fn not_formatted() -> u32 { 6 }

    #[inline]
    fn inlined() -> u32 {
        7
    }

    #[must_use]
    fn must_be_used() -> u32 {
        8
    }

    #[cold]
    fn cold() -> u32 {
        9
    }

    #[track_caller]
    fn assert_positive(value: u32) {
        assert!(value > 0);
    }

    let sum = warns()
        + denies()
        + forbids()
        + always_compiled()
        + conditionally_inlined()
        + not_formatted()
        + inlined()
        + must_be_used()
        + cold();

    assert_positive(sum);

    #[subtest]
    fn attributed_helpers_are_copied_into_subtests() {
        assert_eq!(sum, 45);
        assert_positive(inlined());
    }

    // declared after the subtest, so it is never copied - the dead code expectation below is
    // fulfilled exactly once
    #[expect(dead_code)]
    fn expects_to_be_dead() {}
}
