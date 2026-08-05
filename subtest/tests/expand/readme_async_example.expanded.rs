use subtest::subtest;
extern crate test;
#[rustc_test_marker = "value_can_be_sent_async"]
#[doc(hidden)]
pub const value_can_be_sent_async: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent_async"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/readme_async_example.rs",
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
fn value_can_be_sent_async() {
    let body = async {
        let (sender, receiver) = tokio::sync::mpsc::channel(5);
        sender.send("Hello!").await.unwrap();
        drop(receiver);
    };
    let mut body = body;
    #[allow(unused_mut)]
    let mut body = unsafe { ::tokio::macros::support::Pin::new_unchecked(&mut body) };
    let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
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
    #[rustc_test_marker = "value_can_be_sent_async_subtests::value_can_be_received"]
    #[doc(hidden)]
    pub const value_can_be_received: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_async_subtests::value_can_be_received",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/readme_async_example.rs",
            start_line: 10usize,
            start_col: 14usize,
            end_line: 10usize,
            end_col: 35usize,
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
        let body = async {
            #[allow(unused_variables)]
            let (sender, receiver) = tokio::sync::mpsc::channel(5);
            sender.send("Hello!").await.unwrap();
            let mut receiver = receiver;
            let value = receiver.recv().await.unwrap();
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
        };
        let mut body = body;
        #[allow(unused_mut)]
        let mut body = unsafe {
            ::tokio::macros::support::Pin::new_unchecked(&mut body)
        };
        let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
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
#[rustc_test_marker = "value_can_be_sent_sync"]
#[doc(hidden)]
pub const value_can_be_sent_sync: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("value_can_be_sent_sync"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/readme_async_example.rs",
        start_line: 21usize,
        start_col: 4usize,
        end_line: 21usize,
        end_col: 26usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(value_can_be_sent_sync()),
    ),
};
fn value_can_be_sent_sync() {
    let (sender, receiver) = tokio::sync::mpsc::channel(5);
    sender.try_send("Hello!").unwrap();
    drop(receiver);
}
mod value_can_be_sent_sync_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "value_can_be_sent_sync_subtests::value_can_be_received"]
    #[doc(hidden)]
    pub const value_can_be_received: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "value_can_be_sent_sync_subtests::value_can_be_received",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/readme_async_example.rs",
            start_line: 27usize,
            start_col: 14usize,
            end_line: 27usize,
            end_col: 35usize,
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
        let body = async {
            #[allow(unused_variables)]
            let (sender, receiver) = tokio::sync::mpsc::channel(5);
            sender.try_send("Hello!").unwrap();
            let mut receiver = receiver;
            let value = receiver.recv().await.unwrap();
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
        };
        let mut body = body;
        #[allow(unused_mut)]
        let mut body = unsafe {
            ::tokio::macros::support::Pin::new_unchecked(&mut body)
        };
        let body: ::core::pin::Pin<&mut dyn ::core::future::Future<Output = ()>> = body;
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
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &value_can_be_sent_async,
            &value_can_be_received,
            &value_can_be_sent_sync,
            &value_can_be_received,
        ],
    )
}
