extern crate test;
#[rustc_test_marker = "helper_fn"]
#[doc(hidden)]
pub const helper_fn: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("helper_fn"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/helper_fn.rs",
        start_line: 3usize,
        start_col: 4usize,
        end_line: 3usize,
        end_col: 13usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(helper_fn())),
};
fn helper_fn() {
    fn double(value: u32) -> u32 {
        value * 2
    }
    let local_var = double(1);
    /// Helper functions may carry doc comments as well as lint & configuration attributes
    #[allow(dead_code)]
    fn noop() {}
    fn triple(value: u32) -> u32 {
        value * 3
    }
    match (&triple(local_var), &6) {
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
mod helper_fn_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "helper_fn_subtests::sees_preceding_helper"]
    #[doc(hidden)]
    pub const sees_preceding_helper: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("helper_fn_subtests::sees_preceding_helper"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/helper_fn.rs",
            start_line: 11usize,
            start_col: 8usize,
            end_line: 11usize,
            end_col: 29usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(sees_preceding_helper()),
        ),
    };
    fn sees_preceding_helper() {
        fn double(value: u32) -> u32 {
            value * 2
        }
        #[allow(unused_variables)]
        let local_var = double(1);
        match (&double(local_var), &4) {
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
    #[rustc_test_marker = "helper_fn_subtests::sees_all_preceding_helpers"]
    #[doc(hidden)]
    pub const sees_all_preceding_helpers: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("helper_fn_subtests::sees_all_preceding_helpers"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/helper_fn.rs",
            start_line: 20usize,
            start_col: 8usize,
            end_line: 20usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(sees_all_preceding_helpers()),
        ),
    };
    fn sees_all_preceding_helpers() {
        fn double(value: u32) -> u32 {
            value * 2
        }
        #[allow(unused_variables)]
        let local_var = double(1);
        /// Helper functions may carry doc comments as well as lint & configuration attributes
        #[allow(dead_code)]
        fn noop() {}
        noop();
        match (&double(local_var), &4) {
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
        &[&helper_fn, &sees_all_preceding_helpers, &sees_preceding_helper],
    )
}
