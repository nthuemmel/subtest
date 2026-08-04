fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest(SubTest)]
    fn child() {}
}
