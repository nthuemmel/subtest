fn main() {}

#[subtest::subtest]
fn parent() {
    #[subtest]
    fn child() {}
}
