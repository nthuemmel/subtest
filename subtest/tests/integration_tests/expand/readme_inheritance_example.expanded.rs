use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_sent_async"]
#[doc(hidden)]
pub const value_can_be_sent_async: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent_async"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
        start_line: 5usize,
        start_col: 10usize,
        end_line: 5usize,
        end_col: 33usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(value_can_be_sent_async()),
    ),
};
fn value_can_be_sent_async() -> anyhow::Result<()> {
    let body = async {
        let (sender, receiver) = tokio::sync::mpsc::channel(5);
        sender.send("Hello!").await?;
        drop(receiver);
        Ok(())
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<
        &mut dyn ::core::future::Future<Output = anyhow::Result<()>>,
    > = body;
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        use tokio::runtime::Builder;
        return Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
mod value_can_be_sent_async_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_async_subtests::value_can_be_received_inherit"]
    #[doc(hidden)]
    pub const value_can_be_received_inherit: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_async_subtests::value_can_be_received_inherit",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
            start_line: 10usize,
            start_col: 14usize,
            end_line: 10usize,
            end_col: 43usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_can_be_received_inherit()),
        ),
    };
    fn value_can_be_received_inherit() -> anyhow::Result<()> {
        let body = async {
            #[allow(unused_variables)]
            let (sender, receiver) = tokio::sync::mpsc::channel(5);
            sender.send("Hello!").await?;
            let mut receiver = receiver;
            let value = receiver.try_recv()?;
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
            Ok(())
        };
        let mut body = body;
        #[allow(unused_mut)]
        let mut body = unsafe {
            ::tokio::macros::support::Pin::new_unchecked(&mut body)
        };
        let body: ::core::pin::Pin<
            &mut dyn ::core::future::Future<Output = anyhow::Result<()>>,
        > = body;
        #[allow(
            clippy::expect_used,
            clippy::diverging_sub_expression,
            clippy::needless_return,
            clippy::unwrap_in_result
        )]
        {
            use tokio::runtime::Builder;
            return Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body);
        }
    }
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_async_subtests::value_can_be_received_repeat"]
    #[doc(hidden)]
    pub const value_can_be_received_repeat: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_async_subtests::value_can_be_received_repeat",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
            start_line: 19usize,
            start_col: 14usize,
            end_line: 19usize,
            end_col: 42usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_can_be_received_repeat()),
        ),
    };
    fn value_can_be_received_repeat() -> anyhow::Result<()> {
        let body = async {
            #[allow(unused_variables)]
            let (sender, receiver) = tokio::sync::mpsc::channel(5);
            sender.send("Hello!").await?;
            let mut receiver = receiver;
            let value = receiver.try_recv()?;
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
            Ok(())
        };
        let mut body = body;
        #[allow(unused_mut)]
        let mut body = unsafe {
            ::tokio::macros::support::Pin::new_unchecked(&mut body)
        };
        let body: ::core::pin::Pin<
            &mut dyn ::core::future::Future<Output = anyhow::Result<()>>,
        > = body;
        #[allow(
            clippy::expect_used,
            clippy::diverging_sub_expression,
            clippy::needless_return,
            clippy::unwrap_in_result
        )]
        {
            use tokio::runtime::Builder;
            return Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(body);
        }
    }
}
extern crate test;
#[rustc_test_marker = "value_can_be_sent_and_received"]
#[doc(hidden)]
pub const value_can_be_sent_and_received: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent_and_received"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
        start_line: 32usize,
        start_col: 4usize,
        end_line: 32usize,
        end_col: 34usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(value_can_be_sent_and_received()),
    ),
};
fn value_can_be_sent_and_received() {
    let (sender, mut receiver) = tokio::sync::mpsc::channel(5);
    sender.try_send("Hello!").unwrap();
    receiver.try_recv().unwrap();
    drop(receiver);
}
mod value_can_be_sent_and_received_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_and_received_subtests::value_cannot_be_received_a_second_time"]
    #[doc(hidden)]
    pub const value_cannot_be_received_a_second_time: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_and_received_subtests::value_cannot_be_received_a_second_time",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
            start_line: 39usize,
            start_col: 8usize,
            end_line: 39usize,
            end_col: 46usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage(
                "called `Result::unwrap()` on an `Err` value: Empty",
            ),
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_cannot_be_received_a_second_time()),
        ),
    };
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: Empty")]
    fn value_cannot_be_received_a_second_time() {
        #[allow(unused_variables)]
        let (sender, mut receiver) = tokio::sync::mpsc::channel(5);
        sender.try_send("Hello!").unwrap();
        receiver.try_recv().unwrap();
        receiver.try_recv().unwrap();
    }
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_and_received_subtests::value_can_be_sent_a_second_time"]
    #[doc(hidden)]
    pub const value_can_be_sent_a_second_time: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_and_received_subtests::value_can_be_sent_a_second_time",
            ),
            ignore: true,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/readme_inheritance_example.rs",
            start_line: 45usize,
            start_col: 8usize,
            end_line: 45usize,
            end_col: 39usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(value_can_be_sent_a_second_time()),
        ),
    };
    #[ignore]
    fn value_can_be_sent_a_second_time() {
        #[allow(unused_variables)]
        let (sender, mut receiver) = tokio::sync::mpsc::channel(5);
        sender.try_send("Hello!").unwrap();
        receiver.try_recv().unwrap();
        ::core::panicking::panic("not implemented")
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &value_can_be_sent_and_received,
            &value_can_be_sent_a_second_time,
            &value_cannot_be_received_a_second_time,
            &value_can_be_sent_async,
            &value_can_be_received_inherit,
            &value_can_be_received_repeat,
        ],
    )
}
