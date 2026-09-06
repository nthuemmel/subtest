//! see `unnecessary_wraps` for the cases where the lint is reported rightfully
use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_text_can_be_parsed"]
#[doc(hidden)]
pub const a_text_can_be_parsed: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_text_can_be_parsed"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/unnecessary_wraps_inherited.rs",
        start_line: 7usize,
        start_col: 4usize,
        end_line: 7usize,
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
fn a_text_can_be_parsed() -> Result<(), String> {
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
mod a_text_can_be_parsed_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_text_can_be_parsed_subtests::the_text_is_a_one"]
    #[doc(hidden)]
    pub const the_text_is_a_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_text_can_be_parsed_subtests::the_text_is_a_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unnecessary_wraps_inherited.rs",
            start_line: 15usize,
            start_col: 8usize,
            end_line: 15usize,
            end_col: 25usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_text_is_a_one()),
        ),
    };
    #[allow(clippy::unnecessary_wraps)]
    fn the_text_is_a_one() -> Result<(), String> {
        #[allow(unused_variables)]
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&a_text_can_be_parsed, &the_text_is_a_one])
}
