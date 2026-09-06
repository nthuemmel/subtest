use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_number_can_be_parsed"]
#[doc(hidden)]
pub const a_number_can_be_parsed: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_number_can_be_parsed"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/return_type_reset.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_number_can_be_parsed()),
    ),
};
fn a_number_can_be_parsed() -> anyhow::Result<()> {
    let number = 1;
    let parsed: i32 = "1".parse()?;
    match (&parsed, &number) {
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
mod a_number_can_be_parsed_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_number_can_be_parsed_subtests::the_number_is_one"]
    #[doc(hidden)]
    pub const the_number_is_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_number_can_be_parsed_subtests::the_number_is_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/return_type_reset.rs",
            start_line: 13usize,
            start_col: 8usize,
            end_line: 13usize,
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
    #[allow(clippy::unused_unit)]
    fn the_number_is_one() -> () {
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
    mod the_number_is_one_subtests {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "a_number_can_be_parsed_subtests::the_number_is_one_subtests::the_number_is_positive"]
        #[doc(hidden)]
        pub const the_number_is_positive: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "a_number_can_be_parsed_subtests::the_number_is_one_subtests::the_number_is_positive",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/return_type_reset.rs",
                start_line: 18usize,
                start_col: 12usize,
                end_line: 18usize,
                end_col: 34usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(the_number_is_positive()),
            ),
        };
        fn the_number_is_positive() {
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
            if !(number > 0) {
                ::core::panicking::panic("assertion failed: number > 0")
            }
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&a_number_can_be_parsed, &the_number_is_one, &the_number_is_positive],
    )
}
