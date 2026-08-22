use rstest::rstest;
use subtest::subtest;
fn value_can_be_incremented(mut value: u32) {
    {
        value += 1;
        if !(value > 1) {
            ::core::panicking::panic("assertion failed: value > 1")
        }
    }
}
mod value_can_be_incremented {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_incremented::case_1"]
    #[doc(hidden)]
    pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("value_can_be_incremented::case_1"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_mut_in_subtest_parameter.rs",
            start_line: 8usize,
            start_col: 4usize,
            end_line: 8usize,
            end_col: 28usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_1()),
        ),
    };
    fn case_1() {
        #[allow(unused_mut)]
        let mut value = 1;
        value_can_be_incremented(value)
    }
    extern crate test;
    #[rustc_test_marker = "value_can_be_incremented::case_2"]
    #[doc(hidden)]
    pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("value_can_be_incremented::case_2"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_mut_in_subtest_parameter.rs",
            start_line: 8usize,
            start_col: 4usize,
            end_line: 8usize,
            end_col: 28usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_2()),
        ),
    };
    fn case_2() {
        #[allow(unused_mut)]
        let mut value = 2;
        value_can_be_incremented(value)
    }
}
mod value_can_be_incremented_subtests {
    use super::*;
    fn value_is_positive(
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        mut value: u32,
    ) {
        {
            if !(value > 0) {
                ::core::panicking::panic("assertion failed: value > 0")
            }
        }
    }
    mod value_is_positive {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "value_can_be_incremented_subtests::value_is_positive::case_1"]
        #[doc(hidden)]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "value_can_be_incremented_subtests::value_is_positive::case_1",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/unused_mut_in_subtest_parameter.rs",
                start_line: 10usize,
                start_col: 8usize,
                end_line: 10usize,
                end_col: 25usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_1()),
            ),
        };
        fn case_1() {
            #[allow(unused_mut)]
            let mut value = 1;
            value_is_positive(value)
        }
        extern crate test;
        #[rustc_test_marker = "value_can_be_incremented_subtests::value_is_positive::case_2"]
        #[doc(hidden)]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "value_can_be_incremented_subtests::value_is_positive::case_2",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/unused_mut_in_subtest_parameter.rs",
                start_line: 10usize,
                start_col: 8usize,
                end_line: 10usize,
                end_col: 25usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_2()),
            ),
        };
        fn case_2() {
            #[allow(unused_mut)]
            let mut value = 2;
            value_is_positive(value)
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&case_1, &case_2, &case_1, &case_2])
}
