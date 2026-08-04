#[subtest]
#[test]
fn parent() {
    // A function carrying only non-overriding attributes is a helper function, not a subtest whose
    // `#[subtest]` attribute was forgotten. Each of them is expected to be kept in the test fn and
    // copied into the subtest below, instead of being rejected.

    /// A doc comment
    fn doc_comment() {}

    #[doc = "An explicit doc attribute"]
    fn doc_attr() {}

    #[allow(clippy::eq_op)]
    fn allow_attr() {}

    #[expect(clippy::eq_op)]
    fn expect_attr() {}

    #[warn(clippy::eq_op)]
    fn warn_attr() {}

    #[deny(clippy::eq_op)]
    fn deny_attr() {}

    #[forbid(unsafe_code)]
    fn forbid_attr() {}

    #[cfg(test)]
    fn cfg_attr_() {}

    #[cfg_attr(test, inline)]
    fn cfg_attr_attr() {}

    #[rustfmt::skip]
    fn tool_attr() {}

    #[inline]
    fn inline_attr() {}

    #[must_use]
    fn must_use_attr() -> u32 {
        1
    }

    #[track_caller]
    fn track_caller_attr() {}

    #[cold]
    fn cold_attr() {}

    /// A doc comment
    #[allow(clippy::eq_op)]
    #[cfg(test)]
    #[rustfmt::skip]
    #[inline]
    #[track_caller]
    fn all_at_once() {}

    #[subtest]
    fn child() {}
}
