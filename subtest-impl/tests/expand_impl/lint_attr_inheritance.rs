#[subtest]
#[test]
fn parent() {
    #[subtest]
    #[allow(clippy::eq_op)]
    fn child_with_allow_attr() {}

    #[subtest]
    #[expect(clippy::eq_op)]
    fn child_with_expect_attr() {}

    #[subtest]
    #[warn(unused)]
    fn child_with_warn_attr() {}

    #[subtest]
    #[deny(unsafe_code)]
    fn child_with_deny_attr() {}

    #[subtest]
    #[forbid(unsafe_code)]
    fn child_with_forbid_attr() {}

    #[subtest]
    #[cfg(unix)]
    fn child_with_cfg_attr() {}

    #[subtest]
    #[cfg_attr(miri, ignore)]
    fn child_with_conditional_attr() {}

    #[subtest]
    #[rustfmt::skip]
    fn child_with_tool_attr() {}
}
