use subtest::subtest;
extern crate test;
#[rustc_test_marker = "an_ignored_test_function"]
#[doc(hidden)]
pub const an_ignored_test_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("an_ignored_test_function"),
        ignore: true,
        ignore_message: ::core::option::Option::Some(
            "the test function is ignored on purpose, its subtest has to run regardless",
        ),
        source_file: "./tests/integration_tests/expand/uninherited_test_attrs.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(an_ignored_test_function()),
    ),
};
#[ignore = "the test function is ignored on purpose, its subtest has to run regardless"]
fn an_ignored_test_function() {
    let value = 1;
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
    {
        ::core::panicking::panic_fmt(
            format_args!(
                "not implemented: {0}",
                format_args!("never reached, as this test function is ignored"),
            ),
        );
    }
}
mod an_ignored_test_function_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "an_ignored_test_function_subtests::a_subtest_of_an_ignored_test_function"]
    #[doc(hidden)]
    pub const a_subtest_of_an_ignored_test_function: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "an_ignored_test_function_subtests::a_subtest_of_an_ignored_test_function",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/uninherited_test_attrs.rs",
            start_line: 10usize,
            start_col: 8usize,
            end_line: 10usize,
            end_col: 45usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_of_an_ignored_test_function()),
        ),
    };
    fn a_subtest_of_an_ignored_test_function() {
        #[allow(unused_variables)]
        let value = 1;
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
    }
}
extern crate test;
#[rustc_test_marker = "a_test_function_which_should_panic"]
#[doc(hidden)]
pub const a_test_function_which_should_panic: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_which_should_panic"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/uninherited_test_attrs.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 38usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::YesWithMessage("the test function panics"),
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_which_should_panic()),
    ),
};
#[should_panic(expected = "the test function panics")]
fn a_test_function_which_should_panic() {
    let value = 1;
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
    {
        ::core::panicking::panic_fmt(format_args!("the test function panics"));
    };
}
mod a_test_function_which_should_panic_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_which_should_panic_subtests::a_subtest_of_a_test_function_which_should_panic"]
    #[doc(hidden)]
    pub const a_subtest_of_a_test_function_which_should_panic: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_which_should_panic_subtests::a_subtest_of_a_test_function_which_should_panic",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/uninherited_test_attrs.rs",
            start_line: 27usize,
            start_col: 8usize,
            end_line: 27usize,
            end_col: 55usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(
                a_subtest_of_a_test_function_which_should_panic(),
            ),
        ),
    };
    fn a_subtest_of_a_test_function_which_should_panic() {
        #[allow(unused_variables)]
        let value = 1;
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
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &a_test_function_which_should_panic,
            &a_subtest_of_a_test_function_which_should_panic,
            &an_ignored_test_function,
            &a_subtest_of_an_ignored_test_function,
        ],
    )
}
