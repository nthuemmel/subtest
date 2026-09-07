use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_helper_is_declared"]
#[doc(hidden)]
pub const a_helper_is_declared: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_helper_is_declared"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/inherited_expect_unfulfilled.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 24usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_helper_is_declared()),
    ),
};
fn a_helper_is_declared() {
    #[expect(dead_code, reason = "called only by the subtest below")]
    fn helper() {}
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
mod a_helper_is_declared_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_helper_is_declared_subtests::a_subtest_calls_the_helper"]
    #[doc(hidden)]
    pub const a_subtest_calls_the_helper: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_helper_is_declared_subtests::a_subtest_calls_the_helper",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/inherited_expect_unfulfilled.rs",
            start_line: 15usize,
            start_col: 8usize,
            end_line: 15usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_calls_the_helper()),
        ),
    };
    fn a_subtest_calls_the_helper() {
        #[expect(dead_code, reason = "called only by the subtest below")]
        fn helper() {}
        #[allow(unused_variables)]
        let value = 1;
        helper();
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&a_helper_is_declared, &a_subtest_calls_the_helper])
}
