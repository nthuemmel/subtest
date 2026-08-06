#[subtest::subtest]
#[test]
fn helper_fn() {
    fn double(value: u32) -> u32 {
        value * 2
    }

    let local_var = double(1);

    #[subtest]
    fn sees_preceding_helper() {
        assert_eq!(double(local_var), 4);
    }

    /// Helper functions may carry doc comments as well as lint & configuration attributes
    #[allow(dead_code)]
    fn noop() {}

    #[subtest]
    fn sees_all_preceding_helpers() {
        noop();
        assert_eq!(double(local_var), 4);
    }

    // helper functions declared after a subtest are not copied into it
    fn triple(value: u32) -> u32 {
        value * 3
    }

    assert_eq!(triple(local_var), 6);
}
