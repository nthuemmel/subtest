fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest = "foo"]
    fn child() {}
}
