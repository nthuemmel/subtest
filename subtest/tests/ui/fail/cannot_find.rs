fn main() {}

#[subtest::subtest]
#[stubtest] // By default, successfully expanded test functions are removed because trybuild does not build test targets. Specify a stub macro to use instead of the default #[test]
fn parent() {
    missing_fn();

    #[subtest]
    fn missing_here() {
        let derived = missing_var + 1;
    }
}
