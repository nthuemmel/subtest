fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest(inherit_attributes = false)]
    fn child() {}
}
