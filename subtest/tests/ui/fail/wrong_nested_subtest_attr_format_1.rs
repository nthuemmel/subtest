fn main() {}

#[subtest::subtest]
fn parent() {
    #[subtest = "foo"]
    fn child() {}
}
