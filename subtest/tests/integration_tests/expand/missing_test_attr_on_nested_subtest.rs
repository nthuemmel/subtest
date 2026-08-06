#[subtest::subtest(allow_missing_test_attribute)]
#[test]
fn parent() {
    #[subtest(inherit_attributes = false)]
    fn child() {}
}
