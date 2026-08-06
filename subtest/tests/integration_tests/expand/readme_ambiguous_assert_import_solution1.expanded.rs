use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_sent"]
#[doc(hidden)]
pub const value_can_be_sent: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/readme_ambiguous_assert_import_solution1.rs",
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
    drop(receiver);
}
mod value_can_be_sent_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_subtests::value_can_be_received"]
    #[doc(hidden)]
    pub const value_can_be_received: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_subtests::value_can_be_received",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_ambiguous_assert_import_solution1.rs",
            start_line: 10usize,
            start_col: 8usize,
            end_line: 10usize,
            end_col: 29usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_can_be_received()),
        ),
    };
    fn value_can_be_received() {
        #[allow(unused_variables)]
        let (sender, receiver) = std::sync::mpsc::channel();
        sender.send("Hello!").unwrap();
        use assert2::assert;
        let value = receiver.recv().unwrap();
        match match (&(value), &("Hello!")) {
            (left, right) if !(left == right) => {
                use ::assert2::__assert2_impl::maybe_debug::{IsDebug, IsMaybeNotDebug};
                let left = (&&::assert2::__assert2_impl::maybe_debug::Wrap(left))
                    .__assert2_maybe_debug()
                    .wrap(left);
                let right = (&&::assert2::__assert2_impl::maybe_debug::Wrap(right))
                    .__assert2_maybe_debug()
                    .wrap(right);
                ::assert2::__assert2_impl::print::FailedCheck {
                    macro_name: "assert",
                    file: "./tests/integration_tests/expand/readme_ambiguous_assert_import_solution1.rs",
                    line: 13u32,
                    column: 9u32,
                    predicates: &[
                        (
                            "",
                            ::assert2::__assert2_impl::print::Predicate::Binary {
                                left: "value",
                                operator: "==",
                                right: "\"Hello!\"",
                            },
                        ),
                    ],
                    multiline: false,
                    failed: 0usize,
                    expansion: ::assert2::__assert2_impl::print::Expansion::Binary {
                        left: (&left as &dyn ::core::fmt::Debug),
                        right: (&right as &dyn ::core::fmt::Debug),
                        operator: "==",
                    },
                    fragments: &[],
                    custom_msg: ::core::option::Option::None,
                }
                    .print();
                ::core::result::Result::Err(())
            }
            _ => ::core::result::Result::Ok::<(), ()>(()),
        } {
            ::core::result::Result::Ok(()) => {}
            ::core::result::Result::Err(()) => {
                ::core::panicking::panic_fmt(format_args!("assertion failed"));
            }
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&value_can_be_sent, &value_can_be_received])
}
