extern crate test;
#[rustc_test_marker = "parent"]
#[doc(hidden)]
pub const parent: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("parent"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/missing_test_attr_on_nested_subtest.rs",
        start_line: 3usize,
        start_col: 4usize,
        end_line: 3usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(parent())),
};
fn parent() {}
mod parent_subtests {
    use super::*;
    fn child() {}
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&parent])
}
