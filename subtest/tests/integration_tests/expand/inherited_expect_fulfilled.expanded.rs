use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_value_is_declared"]
#[doc(hidden)]
pub const a_value_is_declared: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_value_is_declared"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/inherited_expect_fulfilled.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 23usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_value_is_declared()),
    ),
};
fn a_value_is_declared() {
    #[expect(unused_variables, reason = "declared for a later commit")]
    let value = 1;
}
mod a_value_is_declared_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_value_is_declared_subtests::a_subtest_does_not_use_it"]
    #[doc(hidden)]
    pub const a_subtest_does_not_use_it: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_value_is_declared_subtests::a_subtest_does_not_use_it",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/inherited_expect_fulfilled.rs",
            start_line: 15usize,
            start_col: 8usize,
            end_line: 15usize,
            end_col: 33usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(a_subtest_does_not_use_it()),
        ),
    };
    fn a_subtest_does_not_use_it() {
        #[expect(unused_variables, reason = "declared for a later commit")]
        #[allow(unused_variables)]
        let value = 1;
        let other = 2;
        match (&other, &2) {
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
    test::test_main_static(&[&a_value_is_declared, &a_subtest_does_not_use_it])
}
