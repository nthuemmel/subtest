use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_test_function_with_an_unused_variable"]
#[doc(hidden)]
pub const a_test_function_with_an_unused_variable: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_with_an_unused_variable"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/inherited_lint_expectation.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 43usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_with_an_unused_variable()),
    ),
};
#[expect(unused_variables, reason = "kept for a later commit")]
fn a_test_function_with_an_unused_variable() {
    let unused = 1;
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
mod a_test_function_with_an_unused_variable_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_with_an_unused_variable_subtests::a_subtest_which_does_not_trigger_the_lint"]
    #[doc(hidden)]
    pub const a_subtest_which_does_not_trigger_the_lint: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_with_an_unused_variable_subtests::a_subtest_which_does_not_trigger_the_lint",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/inherited_lint_expectation.rs",
            start_line: 10usize,
            start_col: 8usize,
            end_line: 10usize,
            end_col: 49usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_which_does_not_trigger_the_lint()),
        ),
    };
    #[allow(unused_variables, reason = "kept for a later commit")]
    fn a_subtest_which_does_not_trigger_the_lint() {
        #[allow(unused_variables)]
        let unused = 1;
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
extern crate test;
#[rustc_test_marker = "a_test_function_triggering_the_lint_after_the_subtest"]
#[doc(hidden)]
pub const a_test_function_triggering_the_lint_after_the_subtest: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName(
            "a_test_function_triggering_the_lint_after_the_subtest",
        ),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/inherited_lint_expectation.rs",
        start_line: 26usize,
        start_col: 4usize,
        end_line: 26usize,
        end_col: 57usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(
            a_test_function_triggering_the_lint_after_the_subtest(),
        ),
    ),
};
#[expect(
    unused_variables,
    reason = "the lint fires on the statement following the subtest"
)]
fn a_test_function_triggering_the_lint_after_the_subtest() {
    let unused = 1;
}
mod a_test_function_triggering_the_lint_after_the_subtest_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_triggering_the_lint_after_the_subtest_subtests::a_subtest_inheriting_nothing"]
    #[doc(hidden)]
    pub const a_subtest_inheriting_nothing: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_triggering_the_lint_after_the_subtest_subtests::a_subtest_inheriting_nothing",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/inherited_lint_expectation.rs",
            start_line: 28usize,
            start_col: 8usize,
            end_line: 28usize,
            end_col: 36usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_inheriting_nothing()),
        ),
    };
    #[allow(
        unused_variables,
        reason = "the lint fires on the statement following the subtest"
    )]
    fn a_subtest_inheriting_nothing() {
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
            &a_test_function_triggering_the_lint_after_the_subtest,
            &a_subtest_inheriting_nothing,
            &a_test_function_with_an_unused_variable,
            &a_subtest_which_does_not_trigger_the_lint,
        ],
    )
}
