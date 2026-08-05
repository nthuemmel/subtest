use subtest::subtest;
extern crate test;
#[rustc_test_marker = "parent"]
#[doc(hidden)]
pub const parent: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("parent"),
        ignore: true,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/readme_inherited_ignore_solution.rs",
        start_line: 6usize,
        start_col: 4usize,
        end_line: 6usize,
        end_col: 10usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(parent())),
};
#[ignore]
fn parent() {
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
mod parent_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "parent_subtests::child"]
    #[doc(hidden)]
    pub const child: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("parent_subtests::child"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/readme_inherited_ignore_solution.rs",
            start_line: 12usize,
            start_col: 8usize,
            end_line: 12usize,
            end_col: 13usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(child())),
    };
    fn child() {
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
        match (&(value + 1), &2) {
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
    test::test_main_static(&[&parent, &child])
}
