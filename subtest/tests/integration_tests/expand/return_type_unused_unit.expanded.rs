//! see `return_type_reset` for cases where the `unused_unit` lint should be suppressed
use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_top_level_test_function_returning_unit"]
#[doc(hidden)]
pub const a_top_level_test_function_returning_unit: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_top_level_test_function_returning_unit"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/return_type_unused_unit.rs",
        start_line: 11usize,
        start_col: 4usize,
        end_line: 11usize,
        end_col: 44usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_top_level_test_function_returning_unit()),
    ),
};
#[expect(
    clippy::unused_unit,
    reason = "a top-level test function inherits no return type, so this `-> ()` resets nothing \
              and has to keep being reported as unneeded"
)]
fn a_top_level_test_function_returning_unit() -> () {
    let number = 1;
    match (&number, &1) {
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
mod a_top_level_test_function_returning_unit_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_top_level_test_function_returning_unit_subtests::the_inherited_number_is_one"]
    #[doc(hidden)]
    pub const the_inherited_number_is_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_top_level_test_function_returning_unit_subtests::the_inherited_number_is_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/return_type_unused_unit.rs",
            start_line: 17usize,
            start_col: 8usize,
            end_line: 17usize,
            end_col: 35usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_inherited_number_is_one()),
        ),
    };
    #[allow(
        clippy::unused_unit,
        reason = "a top-level test function inherits no return type, so this `-> ()` resets nothing \
              and has to keep being reported as unneeded"
    )]
    fn the_inherited_number_is_one() {
        #[allow(unused_variables)]
        let number = 1;
        match (&number, &1) {
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
#[rustc_test_marker = "a_test_function_with_a_subtest_returning_unit"]
#[doc(hidden)]
pub const a_test_function_with_a_subtest_returning_unit: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_test_function_with_a_subtest_returning_unit"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/return_type_unused_unit.rs",
        start_line: 26usize,
        start_col: 4usize,
        end_line: 26usize,
        end_col: 49usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_test_function_with_a_subtest_returning_unit()),
    ),
};
fn a_test_function_with_a_subtest_returning_unit() {
    let number = 1;
    match (&number, &1) {
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
mod a_test_function_with_a_subtest_returning_unit_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_test_function_with_a_subtest_returning_unit_subtests::the_declared_number_is_one"]
    #[doc(hidden)]
    pub const the_declared_number_is_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_test_function_with_a_subtest_returning_unit_subtests::the_declared_number_is_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/return_type_unused_unit.rs",
            start_line: 35usize,
            start_col: 8usize,
            end_line: 35usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_declared_number_is_one()),
        ),
    };
    #[expect(
        clippy::unused_unit,
        reason = "this `-> ()` resets nothing, as the parent test function returns () anyway"
    )]
    fn the_declared_number_is_one() -> () {
        #[allow(unused_variables)]
        let number = 1;
        match (&number, &1) {
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
            &a_test_function_with_a_subtest_returning_unit,
            &the_declared_number_is_one,
            &a_top_level_test_function_returning_unit,
            &the_inherited_number_is_one,
        ],
    )
}
