use rstest::rstest;
use subtest::subtest;
fn a_number_is_positive(number: i32) {
    {
        if !(number > 0) {
            ::core::panicking::panic("assertion failed: number > 0")
        }
    }
}
mod a_number_is_positive {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_number_is_positive::case_1"]
    #[doc(hidden)]
    pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_number_is_positive::case_1"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_parameters.rs",
            start_line: 8usize,
            start_col: 4usize,
            end_line: 8usize,
            end_col: 24usize,
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
        let number = 1;
        a_number_is_positive(number)
    }
    extern crate test;
    #[rustc_test_marker = "a_number_is_positive::case_2"]
    #[doc(hidden)]
    pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_number_is_positive::case_2"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_parameters.rs",
            start_line: 8usize,
            start_col: 4usize,
            end_line: 8usize,
            end_col: 24usize,
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
        let number = 2;
        a_number_is_positive(number)
    }
}
mod a_number_is_positive_subtests {
    use super::*;
    fn a_subtest_ignores_the_parameter(#[allow(unused_variables)] number: i32) {
        {
            let unrelated = "no number in sight";
            match (&unrelated, &"no number in sight") {
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
    mod a_subtest_ignores_the_parameter {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "a_number_is_positive_subtests::a_subtest_ignores_the_parameter::case_1"]
        #[doc(hidden)]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "a_number_is_positive_subtests::a_subtest_ignores_the_parameter::case_1",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_parameters.rs",
                start_line: 12usize,
                start_col: 8usize,
                end_line: 12usize,
                end_col: 39usize,
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
            let number = 1;
            a_subtest_ignores_the_parameter(number)
        }
        extern crate test;
        #[rustc_test_marker = "a_number_is_positive_subtests::a_subtest_ignores_the_parameter::case_2"]
        #[doc(hidden)]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "a_number_is_positive_subtests::a_subtest_ignores_the_parameter::case_2",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_parameters.rs",
                start_line: 12usize,
                start_col: 8usize,
                end_line: 12usize,
                end_col: 39usize,
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
            let number = 2;
            a_subtest_ignores_the_parameter(number)
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
