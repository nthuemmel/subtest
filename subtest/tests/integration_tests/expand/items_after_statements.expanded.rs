//! see `items_after_inherited_statements` for the case where the lint has to be masked
use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_number_can_be_doubled"]
#[doc(hidden)]
pub const a_number_can_be_doubled: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_number_can_be_doubled"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/items_after_statements.rs",
        start_line: 7usize,
        start_col: 4usize,
        end_line: 7usize,
        end_col: 27usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_number_can_be_doubled()),
    ),
};
fn a_number_can_be_doubled() {
    let number = 2;
    match (&number, &2) {
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
mod a_number_can_be_doubled_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_number_can_be_doubled_subtests::the_number_doubles_to_four"]
    #[doc(hidden)]
    pub const the_number_doubles_to_four: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_number_can_be_doubled_subtests::the_number_doubles_to_four",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/items_after_statements.rs",
            start_line: 11usize,
            start_col: 8usize,
            end_line: 11usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_number_doubles_to_four()),
        ),
    };
    fn the_number_doubles_to_four() {
        #[allow(unused_variables)]
        let number = 2;
        let doubled = number * 2;
        fn quadruple(value: i32) -> i32 {
            value * 4
        }
        match (&doubled, &4) {
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
        match (&quadruple(number), &8) {
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
    test::test_main_static(&[&a_number_can_be_doubled, &the_number_doubles_to_four])
}
