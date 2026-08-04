#[subtest]
#[test]
fn parent() {
    // Every attribute below must be additive to the inherited attributes: each subtest is expected
    // to keep the parent's `#[test]` attribute, otherwise it would silently never run.

    /// A doc comment
    #[subtest]
    fn doc_comment() {}

    #[subtest]
    #[doc = "An explicit doc attribute"]
    fn doc_attr() {}

    #[subtest]
    #[allow(clippy::eq_op)]
    fn allow_attr() {}

    #[subtest]
    #[expect(clippy::eq_op)]
    fn expect_attr() {}

    #[subtest]
    #[warn(clippy::eq_op)]
    fn warn_attr() {}

    #[subtest]
    #[deny(clippy::eq_op)]
    fn deny_attr() {}

    #[subtest]
    #[forbid(unsafe_code)]
    fn forbid_attr() {}

    #[subtest]
    #[cfg(test)]
    fn cfg_attr_() {}

    #[subtest]
    #[cfg_attr(test, inline)]
    fn cfg_attr_attr() {}

    #[subtest]
    #[rustfmt::skip]
    fn tool_attr() {}

    #[subtest]
    #[inline]
    fn inline_attr() {}

    #[subtest]
    #[must_use]
    fn must_use_attr() {}

    #[subtest]
    #[track_caller]
    fn track_caller_attr() {}

    #[subtest]
    #[cold]
    fn cold_attr() {}

    // combining all of them at once must not override either
    /// A doc comment
    #[subtest]
    #[allow(clippy::eq_op)]
    #[cfg(test)]
    #[rustfmt::skip]
    #[inline]
    #[track_caller]
    fn all_at_once() {}
}
