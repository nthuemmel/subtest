use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_test_function_with_a_panicking_subtest"]
#[doc(hidden)]
pub const a_test_function_with_a_panicking_subtest: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_with_a_panicking_subtest"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/failing_subtest.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 44usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_with_a_panicking_subtest()),
    ),
};
fn a_test_function_with_a_panicking_subtest() {
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
mod a_test_function_with_a_panicking_subtest_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_with_a_panicking_subtest_subtests::a_subtest_panicking"]
    #[doc(hidden)]
    pub const a_subtest_panicking: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_with_a_panicking_subtest_subtests::a_subtest_panicking",
            ),
            ignore: true,
            ignore_message: ::core::option::Option::Some(
                "fails on purpose, run in a child process by tests/integration_tests/run.rs",
            ),
            source_file: "./tests/integration_tests/expand/failing_subtest.rs",
            start_line: 12usize,
            start_col: 8usize,
            end_line: 12usize,
            end_col: 27usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_panicking()),
        ),
    };
    #[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
    fn a_subtest_panicking() {
        #[allow(unused_variables)]
        let value = 1;
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
extern crate test;
#[rustc_test_marker = "a_test_function_with_a_subtest_returning_an_error"]
#[doc(hidden)]
pub const a_test_function_with_a_subtest_returning_an_error: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_with_a_subtest_returning_an_error"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/failing_subtest.rs",
        start_line: 25usize,
        start_col: 4usize,
        end_line: 25usize,
        end_col: 53usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_with_a_subtest_returning_an_error()),
    ),
};
#[expect(
    clippy::unnecessary_wraps,
    reason = "the return type exists for the subtest below"
)]
fn a_test_function_with_a_subtest_returning_an_error() -> Result<(), String> {
    let list: Vec<u32> = Vec::new();
    if !list.is_empty() {
        ::core::panicking::panic("assertion failed: list.is_empty()")
    }
    Ok(())
}
mod a_test_function_with_a_subtest_returning_an_error_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_with_a_subtest_returning_an_error_subtests::a_subtest_returning_an_error"]
    #[doc(hidden)]
    pub const a_subtest_returning_an_error: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_with_a_subtest_returning_an_error_subtests::a_subtest_returning_an_error",
            ),
            ignore: true,
            ignore_message: ::core::option::Option::Some(
                "fails on purpose, run in a child process by tests/integration_tests/run.rs",
            ),
            source_file: "./tests/integration_tests/expand/failing_subtest.rs",
            start_line: 32usize,
            start_col: 8usize,
            end_line: 32usize,
            end_col: 36usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_returning_an_error()),
        ),
    };
    #[allow(
        clippy::unnecessary_wraps,
        reason = "the return type exists for the subtest below"
    )]
    #[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
    fn a_subtest_returning_an_error() -> Result<(), String> {
        #[allow(unused_variables)]
        let list: Vec<u32> = Vec::new();
        let first = list.first().ok_or("the list is empty")?;
        match (&*first, &1) {
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
        Ok(())
    }
}
extern crate test;
#[rustc_test_marker = "a_panicking_test_function"]
#[doc(hidden)]
pub const a_panicking_test_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_panicking_test_function"),
        ignore: true,
        ignore_message: ::core::option::Option::Some(
            "fails on purpose, run in a child process by tests/integration_tests/run.rs",
        ),
        source_file: "./tests/integration_tests/expand/failing_subtest.rs",
        start_line: 45usize,
        start_col: 4usize,
        end_line: 45usize,
        end_col: 29usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_panicking_test_function()),
    ),
};
#[ignore = "fails on purpose, run in a child process by tests/integration_tests/run.rs"]
fn a_panicking_test_function() {
    {
        ::core::panicking::panic_fmt(format_args!("the test function itself fails"));
    };
}
mod a_panicking_test_function_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_panicking_test_function_subtests::a_subtest_of_a_panicking_test_function"]
    #[doc(hidden)]
    pub const a_subtest_of_a_panicking_test_function: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_panicking_test_function_subtests::a_subtest_of_a_panicking_test_function",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/failing_subtest.rs",
            start_line: 49usize,
            start_col: 8usize,
            end_line: 49usize,
            end_col: 46usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_of_a_panicking_test_function()),
        ),
    };
    fn a_subtest_of_a_panicking_test_function() {
        match (&(1 + 1), &2) {
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
            &a_panicking_test_function,
            &a_subtest_of_a_panicking_test_function,
            &a_test_function_with_a_panicking_subtest,
            &a_subtest_panicking,
            &a_test_function_with_a_subtest_returning_an_error,
            &a_subtest_returning_an_error,
        ],
    )
}
