use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_incremented"]
#[doc(hidden)]
pub const value_can_be_incremented: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_incremented"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/readme_unused_mut_solution2.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(value_can_be_incremented()),
    ),
};
fn value_can_be_incremented() {
    let mut value = 1;
    match (&value, &1) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
    let _ = &mut value;
}
mod value_can_be_incremented_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_incremented_subtests::value_can_be_incremented_twice"]
    #[doc(hidden)]
    pub const value_can_be_incremented_twice: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_incremented_subtests::value_can_be_incremented_twice",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_unused_mut_solution2.rs",
            start_line: 9usize,
            start_col: 8usize,
            end_line: 9usize,
            end_col: 38usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_can_be_incremented_twice()),
        ),
    };
    fn value_can_be_incremented_twice() {
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut value = 1;
        value += 1;
        match (&value, &2) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    let kind = ::core::panicking::AssertKind::Eq;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&value_can_be_incremented, &value_can_be_incremented_twice])
}
