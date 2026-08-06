use subtest::subtest;
extern crate test;
#[rustc_test_marker = "simple"]
#[doc(hidden)]
pub const simple: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("simple"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/async_tests.rs",
        start_line: 5usize,
        start_col: 10usize,
        end_line: 5usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(#[coverage(off)] || test::assert_test_result(simple())),
};
fn simple() {
    let body = async {};
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
mod simple_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "simple_subtests::nested"]
    #[doc(hidden)]
    pub const nested: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("simple_subtests::nested"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/async_tests.rs",
            start_line: 7usize,
            start_col: 14usize,
            end_line: 7usize,
            end_col: 20usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(nested()),
        ),
    };
    fn nested() {
        let body = async {};
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
    test::test_main_static(&[&simple, &nested])
}
