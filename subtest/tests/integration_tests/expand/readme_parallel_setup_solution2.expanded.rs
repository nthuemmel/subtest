use serial_test::serial;
use subtest::subtest;
extern crate test;
#[rustc_test_marker = "server_starts"]
#[doc(hidden)]
pub const server_starts: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("server_starts"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/integration_tests/expand/readme_parallel_setup_solution2.rs",
        start_line: 7usize,
        start_col: 4usize,
        end_line: 7usize,
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
    serial_test::local_serial_core(
        ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [""],
            ),
        ),
        ::std::option::Option::None,
        || {
            let listener = std::net::TcpListener::bind("127.0.0.1:39118").unwrap();
            let port = listener.local_addr().unwrap().port();
            match (&port, &39118) {
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
        },
    );
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
            source_file: "./tests/integration_tests/expand/readme_parallel_setup_solution2.rs",
            start_line: 13usize,
            start_col: 8usize,
            end_line: 13usize,
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
        serial_test::local_serial_core(
            ::alloc::boxed::box_assume_init_into_vec_unsafe(
                ::alloc::intrinsics::write_box_via_move(
                    ::alloc::boxed::Box::new_uninit(),
                    [""],
                ),
            ),
            ::std::option::Option::None,
            || {
                #[allow(unused_variables)]
                let listener = std::net::TcpListener::bind("127.0.0.1:39118").unwrap();
                #[allow(unused_variables)]
                let port = listener.local_addr().unwrap().port();
                match (&port, &39118) {
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
                std::net::TcpStream::connect(("127.0.0.1", port)).unwrap();
            },
        );
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&server_starts, &server_accepts_a_connection])
}
