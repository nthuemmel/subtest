fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[test]
    fn missing_here() {}

    #[subtest]
    fn correct_here() {}
}
