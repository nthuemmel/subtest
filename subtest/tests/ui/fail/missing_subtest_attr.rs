fn main() {}

#[subtest::subtest]
fn parent() {
    #[test]
    fn missing_here() {}

    #[subtest]
    fn correct_here() {}
}
