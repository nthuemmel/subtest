use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_sent"]
#[doc(hidden)]
pub const value_can_be_sent: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_via_macro.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 21usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(value_can_be_sent()),
    ),
};
fn value_can_be_sent() {
    let (sender, receiver) = std::sync::mpsc::channel();
    sender.send("Hello!").unwrap();
    let value = receiver.recv();
    let Ok(value) = value else {
        #[allow(unused)]
        use ::assert2::__assert2_impl::maybe_debug::{IsDebug, IsMaybeNotDebug};
        let value = (&&::assert2::__assert2_impl::maybe_debug::Wrap(&value))
            .__assert2_maybe_debug()
            .wrap(&value);
        ::assert2::__assert2_impl::print::FailedCheck {
            macro_name: "assert",
            file: "./tests/integration_tests/expand/unused_variables_in_subtest_via_macro.rs",
            line: 9u32,
            column: 5u32,
            custom_msg: ::core::option::Option::None,
            predicates: &[
                (
                    "",
                    ::assert2::__assert2_impl::print::Predicate::Let {
                        pattern: "Ok(value)",
                        expression: "receiver.recv()",
                    },
                ),
            ],
            multiline: false,
            failed: 0usize,
            expansion: ::assert2::__assert2_impl::print::Expansion::Let {
                expression: &value as &dyn ::core::fmt::Debug,
            },
            fragments: &[],
        }
            .print();
        {
            ::core::panicking::panic_fmt(format_args!("assertion failed"));
        };
    };
    match (&value, &"Hello!") {
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
mod value_can_be_sent_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_subtests::another_value_can_be_sent"]
    #[doc(hidden)]
    pub const another_value_can_be_sent: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_subtests::another_value_can_be_sent",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/unused_variables_in_subtest_via_macro.rs",
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
            || test::assert_test_result(another_value_can_be_sent()),
        ),
    };
    fn another_value_can_be_sent() {
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send("Hello!").unwrap();
        let value = receiver.recv();
        let Ok(value) = value else {
            #[allow(unused)]
            use ::assert2::__assert2_impl::maybe_debug::{IsDebug, IsMaybeNotDebug};
            let value = (&&::assert2::__assert2_impl::maybe_debug::Wrap(&value))
                .__assert2_maybe_debug()
                .wrap(&value);
            ::assert2::__assert2_impl::print::FailedCheck {
                macro_name: "assert",
                file: "./tests/integration_tests/expand/unused_variables_in_subtest_via_macro.rs",
                line: 3u32,
                column: 1u32,
                custom_msg: ::core::option::Option::None,
                predicates: &[
                    (
                        "",
                        ::assert2::__assert2_impl::print::Predicate::Let {
                            pattern: "Ok(value)",
                            expression: "receiver.recv()",
                        },
                    ),
                ],
                multiline: false,
                failed: 0usize,
                expansion: ::assert2::__assert2_impl::print::Expansion::Let {
                    expression: &value as &dyn ::core::fmt::Debug,
                },
                fragments: &[],
            }
                .print();
            {
                ::core::panicking::panic_fmt(format_args!("assertion failed"));
            };
        };
        sender.send("Hello again!").unwrap();
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&value_can_be_sent, &another_value_can_be_sent])
}
