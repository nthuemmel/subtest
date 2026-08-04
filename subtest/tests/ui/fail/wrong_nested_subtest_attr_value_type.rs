fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest(inherit_attributes = "value")]
    fn child() {}
}
