fn main() {}

#[subtest::subtest(allow_missing_test_attribute)]
fn parent() {
    missing_fn();

    #[subtest]
    fn missing_here() {
        let derived = missing_var + 1;
    }
}
