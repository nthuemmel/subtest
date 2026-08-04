#[subtest::subtest(allow_missing_test_attribute)]
fn parent() {
    #[subtest]
    fn child() {}
}
