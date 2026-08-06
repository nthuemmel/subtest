use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_long_test_function"]
#[doc(hidden)]
pub const a_long_test_function: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_long_test_function"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/too_many_lines_expectation.rs",
        start_line: 9usize,
        start_col: 4usize,
        end_line: 9usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_long_test_function()),
    ),
};
#[expect(clippy::too_many_lines, reason = "this test function is long on purpose")]
fn a_long_test_function() {
    let mut total = 0;
    total += 1;
    total += 2;
    total += 3;
    total += 4;
    total += 5;
    total += 6;
    total += 7;
    total += 8;
    total += 9;
    total += 10;
    match (&total, &55) {
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
mod a_long_test_function_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_long_test_function_subtests::a_short_subtest"]
    #[doc(hidden)]
    pub const a_short_subtest: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_long_test_function_subtests::a_short_subtest"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/too_many_lines_expectation.rs",
            start_line: 11usize,
            start_col: 8usize,
            end_line: 11usize,
            end_col: 23usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_short_subtest()),
        ),
    };
    fn a_short_subtest() {
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
    extern crate test;
    #[rustc_test_marker = "a_long_test_function_subtests::a_subtest_inheriting_more_lines_than_the_threshold"]
    #[doc(hidden)]
    pub const a_subtest_inheriting_more_lines_than_the_threshold: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_long_test_function_subtests::a_subtest_inheriting_more_lines_than_the_threshold",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/too_many_lines_expectation.rs",
            start_line: 31usize,
            start_col: 8usize,
            end_line: 31usize,
            end_col: 58usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(
                a_subtest_inheriting_more_lines_than_the_threshold(),
            ),
        ),
    };
    fn a_subtest_inheriting_more_lines_than_the_threshold() {
        #[allow(unused_variables)]
        let mut total = 0;
        #[allow(unused_assignments)]
        {
            total += 1;
        }
        #[allow(unused_assignments)]
        {
            total += 2;
        }
        #[allow(unused_assignments)]
        {
            total += 3;
        }
        #[allow(unused_assignments)]
        {
            total += 4;
        }
        #[allow(unused_assignments)]
        {
            total += 5;
        }
        #[allow(unused_assignments)]
        {
            total += 6;
        }
        #[allow(unused_assignments)]
        {
            total += 7;
        }
        #[allow(unused_assignments)]
        {
            total += 8;
        }
        #[allow(unused_assignments)]
        {
            total += 9;
        }
        #[allow(unused_assignments)]
        {
            total += 10;
        }
        match (&total, &55) {
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
        match (&total, &55) {
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
#[rustc_test_marker = "a_test_function_with_a_long_subtest"]
#[doc(hidden)]
pub const a_test_function_with_a_long_subtest: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_with_a_long_subtest"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/too_many_lines_expectation.rs",
        start_line: 44usize,
        start_col: 4usize,
        end_line: 44usize,
        end_col: 39usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_with_a_long_subtest()),
    ),
};
#[expect(
    clippy::too_many_lines,
    reason = "this test function is long because of the subtest written in its body"
)]
fn a_test_function_with_a_long_subtest() {}
mod a_test_function_with_a_long_subtest_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_with_a_long_subtest_subtests::a_long_subtest"]
    #[doc(hidden)]
    pub const a_long_subtest: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_with_a_long_subtest_subtests::a_long_subtest",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/too_many_lines_expectation.rs",
            start_line: 47usize,
            start_col: 8usize,
            end_line: 47usize,
            end_col: 22usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_long_subtest()),
        ),
    };
    #[expect(clippy::too_many_lines, reason = "this subtest is long on purpose")]
    fn a_long_subtest() {
        let mut total = 0;
        total += 1;
        total += 2;
        total += 3;
        total += 4;
        total += 5;
        total += 6;
        total += 7;
        total += 8;
        total += 9;
        total += 10;
        match (&total, &55) {
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
            &a_long_test_function,
            &a_short_subtest,
            &a_subtest_inheriting_more_lines_than_the_threshold,
            &a_test_function_with_a_long_subtest,
            &a_long_subtest,
        ],
    )
}
