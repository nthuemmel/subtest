fn main() {}

#[subtest::subtest]
fn parent() {
    #[subtest(SubTest)]
    fn child() {}
}
