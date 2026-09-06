use subtest::subtest;
extern crate test;
#[rustc_test_marker = "a_number_can_be_rendered"]
#[doc(hidden)]
pub const a_number_can_be_rendered: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("a_number_can_be_rendered"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/items_after_inherited_statements.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 28usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(a_number_can_be_rendered()),
    ),
};
fn a_number_can_be_rendered() {
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
mod a_number_can_be_rendered_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "a_number_can_be_rendered_subtests::the_number_renders_as_one"]
    #[doc(hidden)]
    pub const the_number_renders_as_one: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "a_number_can_be_rendered_subtests::the_number_renders_as_one",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/items_after_inherited_statements.rs",
            start_line: 12usize,
            start_col: 8usize,
            end_line: 12usize,
            end_col: 33usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(the_number_renders_as_one()),
        ),
    };
    fn the_number_renders_as_one() {
        #[allow(unused_variables)]
        let number = 1;
        #[allow(clippy::items_after_statements)]
        use std::fmt::Write as _;
        #[allow(clippy::items_after_statements)]
        const SUFFIX: &str = "!";
        #[allow(clippy::items_after_statements)]
        fn render(value: i32, suffix: &str) -> String {
            let mut rendered = String::new();
            rendered.write_fmt(format_args!("{0}{1}", value, suffix)).unwrap();
            rendered
        }
        match (&render(number, SUFFIX), &"1!") {
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
    mod the_number_renders_as_one_subtests {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "a_number_can_be_rendered_subtests::the_number_renders_as_one_subtests::the_number_still_renders_as_one"]
        #[doc(hidden)]
        pub const the_number_still_renders_as_one: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName(
                    "a_number_can_be_rendered_subtests::the_number_renders_as_one_subtests::the_number_still_renders_as_one",
                ),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/items_after_inherited_statements.rs",
                start_line: 25usize,
                start_col: 12usize,
                end_line: 25usize,
                end_col: 43usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(the_number_still_renders_as_one()),
            ),
        };
        fn the_number_still_renders_as_one() {
            #[allow(unused_variables)]
            let number = 1;
            #[allow(clippy::items_after_statements)]
            use std::fmt::Write as _;
            #[allow(clippy::items_after_statements)]
            const SUFFIX: &str = "!";
            #[allow(clippy::items_after_statements)]
            fn render(value: i32, suffix: &str) -> String {
                let mut rendered = String::new();
                rendered.write_fmt(format_args!("{0}{1}", value, suffix)).unwrap();
                rendered
            }
            match (&render(number, SUFFIX), &"1!") {
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
            match (&render(number, SUFFIX), &"1!") {
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &a_number_can_be_rendered,
            &the_number_renders_as_one,
            &the_number_still_renders_as_one,
        ],
    )
}
