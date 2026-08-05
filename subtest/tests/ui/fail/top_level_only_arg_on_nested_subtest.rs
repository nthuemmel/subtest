fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest(allow_missing_test_attribute)]
    fn child() {}
}
