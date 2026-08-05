use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_sent"]
#[doc(hidden)]
pub const value_can_be_sent: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/readme_unused_variables_solution.rs",
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
            source_file: "./tests/expand/readme_unused_variables_solution.rs",
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
        let value = receiver.recv().unwrap();
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
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&value_can_be_sent, &value_can_be_received])
}
