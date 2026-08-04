fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest(foo = false)]
    fn child() {}
}
