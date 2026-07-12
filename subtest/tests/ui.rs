/// Test that all fixtures in the directory 'ui/fail' fail to expand with an expected error message.
/// Uses trybuild for error message snapshot management and assertions.
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}
