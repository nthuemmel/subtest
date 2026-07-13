fn main() {}

#[subtest::subtest]
fn parent() {
    missing_fn();

    #[subtest]
    fn missing_here() {
        let derived = missing_var + 1;
    }
}
