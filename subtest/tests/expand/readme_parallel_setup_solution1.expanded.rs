use subtest::subtest;
extern crate test;
#[rustc_test_marker = "server_starts"]
#[doc(hidden)]
pub const server_starts: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("server_starts"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/readme_parallel_setup_solution1.rs",
        start_line: 5usize,
        start_col: 4usize,
        end_line: 5usize,
        end_col: 17usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(server_starts()),
    ),
};
fn server_starts() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    match (&port, &0) {
        (left_val, right_val) => {
            if *left_val == *right_val {
                let kind = ::core::panicking::AssertKind::Ne;
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
mod server_starts_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "server_starts_subtests::server_accepts_a_connection"]
    #[doc(hidden)]
    pub const server_accepts_a_connection: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "server_starts_subtests::server_accepts_a_connection",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/readme_parallel_setup_solution1.rs",
            start_line: 11usize,
            start_col: 8usize,
            end_line: 11usize,
            end_col: 35usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(server_accepts_a_connection()),
        ),
    };
    fn server_accepts_a_connection() {
        #[allow(unused_variables)]
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        #[allow(unused_variables)]
        let port = listener.local_addr().unwrap().port();
        match (&port, &0) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    let kind = ::core::panicking::AssertKind::Ne;
                    ::core::panicking::assert_failed(
                        kind,
                        &*left_val,
                        &*right_val,
                        ::core::option::Option::None,
                    );
                }
            }
        };
        std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&server_starts, &server_accepts_a_connection])
}
