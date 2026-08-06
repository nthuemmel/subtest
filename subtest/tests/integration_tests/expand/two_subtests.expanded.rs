extern crate test;
#[rustc_test_marker = "two_subtests"]
#[doc(hidden)]
pub const two_subtests: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("two_subtests"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/two_subtests.rs",
        start_line: 3usize,
        start_col: 4usize,
        end_line: 3usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(two_subtests()),
    ),
};
fn two_subtests() {
    let i = 1;
    match (&i, &1) {
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
mod two_subtests_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "two_subtests_subtests::add"]
    #[doc(hidden)]
    pub const add: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("two_subtests_subtests::add"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/two_subtests.rs",
            start_line: 8usize,
            start_col: 8usize,
            end_line: 8usize,
            end_col: 11usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(add())),
    };
    fn add() {
        #[allow(unused_variables)]
        let i = 1;
        match (&i, &1) {
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
        let i = i + 1;
        match (&i, &2) {
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
    #[rustc_test_marker = "two_subtests_subtests::subtract"]
    #[doc(hidden)]
    pub const subtract: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("two_subtests_subtests::subtract"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/two_subtests.rs",
            start_line: 14usize,
            start_col: 8usize,
            end_line: 14usize,
            end_col: 16usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(subtract()),
        ),
    };
    fn subtract() {
        #[allow(unused_variables)]
        let i = 1;
        match (&i, &1) {
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
        let i = i - 1;
        match (&i, &0) {
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
    test::test_main_static(&[&two_subtests, &add, &subtract])
}
