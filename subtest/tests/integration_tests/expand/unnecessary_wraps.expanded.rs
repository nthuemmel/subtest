//! see `unnecessary_wraps_inherited` for the case where the lint is a false positive
use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_text_can_be_parsed"]
#[doc(hidden)]
pub const a_text_can_be_parsed: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_text_can_be_parsed"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/unnecessary_wraps.rs",
        start_line: 14usize,
        start_col: 4usize,
        end_line: 14usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_text_can_be_parsed()),
    ),
};
#[expect(
    clippy::unnecessary_wraps,
    reason = "only the subtest below fails, so the Result return type could be moved down to it \
              instead of being inherited from here"
)]
fn a_text_can_be_parsed() -> Result<(), String> {
    let text = "1";
    match (&text, &"1") {
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
mod a_text_can_be_parsed_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_text_can_be_parsed_subtests::the_text_parses_to_one"]
    #[doc(hidden)]
    pub const the_text_parses_to_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_text_can_be_parsed_subtests::the_text_parses_to_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unnecessary_wraps.rs",
            start_line: 18usize,
            start_col: 8usize,
            end_line: 18usize,
            end_col: 30usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_text_parses_to_one()),
        ),
    };
    #[allow(
        clippy::unnecessary_wraps,
        reason = "only the subtest below fails, so the Result return type could be moved down to it \
              instead of being inherited from here"
    )]
    #[allow(clippy::unnecessary_wraps)]
    fn the_text_parses_to_one() -> Result<(), String> {
        #[allow(unused_variables)]
        let text = "1";
        let parsed: i32 = text
            .parse()
            .map_err(|error: std::num::ParseIntError| error.to_string())?;
        match (&parsed, &1) {
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
#[rustc_test_marker = "a_number_is_one"]
#[doc(hidden)]
pub const a_number_is_one: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_number_is_one"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/unnecessary_wraps.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_number_is_one()),
    ),
};
fn a_number_is_one() {
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
mod a_number_is_one_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_number_is_one_subtests::the_number_is_one"]
    #[doc(hidden)]
    pub const the_number_is_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("a_number_is_one_subtests::the_number_is_one"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unnecessary_wraps.rs",
            start_line: 43usize,
            start_col: 8usize,
            end_line: 43usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_number_is_one()),
        ),
    };
    #[expect(
        clippy::unnecessary_wraps,
        reason = "the subtest declares the Result return type itself and never fails, so it can \
                  drop it without touching the parent"
    )]
    fn the_number_is_one() -> Result<(), String> {
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
        Ok(())
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &a_number_is_one,
            &the_number_is_one,
            &a_text_can_be_parsed,
            &the_text_parses_to_one,
        ],
    )
}
