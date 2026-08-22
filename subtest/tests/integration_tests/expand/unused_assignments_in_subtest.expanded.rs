use subtest::subtest;
extern crate test;
#[rustc_test_marker = "values_are_assigned"]
#[doc(hidden)]
pub const values_are_assigned: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("values_are_assigned"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/unused_assignments_in_subtest.rs",
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
        || test::assert_test_result(values_are_assigned()),
    ),
};
fn values_are_assigned() {
    let deferred;
    deferred = "a";
    let nested;
    {
        nested = "b";
    }
    let branched;
    if deferred == "a" {
        branched = "c";
    } else {
        branched = "d";
    }
    let mut counter = 0;
    counter += 1;
    match (&(deferred, nested, branched, counter), &("a", "b", "c", 1)) {
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
mod values_are_assigned_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "values_are_assigned_subtests::subtest_reads_none_of_them"]
    #[doc(hidden)]
    pub const subtest_reads_none_of_them: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "values_are_assigned_subtests::subtest_reads_none_of_them",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_assignments_in_subtest.rs",
            start_line: 25usize,
            start_col: 8usize,
            end_line: 25usize,
            end_col: 34usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(subtest_reads_none_of_them()),
        ),
    };
    fn subtest_reads_none_of_them() {
        #[allow(unused_variables)]
        let deferred;
        #[allow(unused_assignments)]
        {
            deferred = "a";
        }
        #[allow(unused_variables)]
        let nested;
        #[allow(unused_assignments)]
        {
            {
                nested = "b";
            };
        }
        #[allow(unused_variables)]
        let branched;
        #[allow(unused_assignments)]
        {
            if deferred == "a" {
                branched = "c";
            } else {
                branched = "d";
            };
        }
        #[allow(unused_variables)]
        #[allow(unused_mut)]
        let mut counter = 0;
        #[allow(unused_assignments)]
        {
            counter += 1;
        }
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&values_are_assigned, &subtest_reads_none_of_them])
}
