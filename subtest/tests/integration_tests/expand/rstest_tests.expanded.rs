use rstest::rstest;
use subtest::subtest;
fn fibonacci_test(input: u32, expected: u32) {
    {
        let number = fibonacci(input);
        match (&expected, &number) {
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
mod fibonacci_test {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_1"]
    #[doc(hidden)]
    pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_1"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_1()),
        ),
    };
    fn case_1() {
        let input = 0;
        let expected = 0;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_2"]
    #[doc(hidden)]
    pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_2"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_2()),
        ),
    };
    fn case_2() {
        let input = 1;
        let expected = 1;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_3"]
    #[doc(hidden)]
    pub const case_3: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_3"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_3()),
        ),
    };
    fn case_3() {
        let input = 2;
        let expected = 1;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_4"]
    #[doc(hidden)]
    pub const case_4: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_4"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_4()),
        ),
    };
    fn case_4() {
        let input = 3;
        let expected = 2;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_5"]
    #[doc(hidden)]
    pub const case_5: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_5"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_5()),
        ),
    };
    fn case_5() {
        let input = 4;
        let expected = 3;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_6"]
    #[doc(hidden)]
    pub const case_6: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_6"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_6()),
        ),
    };
    fn case_6() {
        let input = 5;
        let expected = 5;
        fibonacci_test(input, expected)
    }
    extern crate test;
    #[rustc_test_marker = "fibonacci_test::case_7"]
    #[doc(hidden)]
    pub const case_7: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("fibonacci_test::case_7"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/integration_tests/expand/rstest_tests.rs",
            start_line: 13usize,
            start_col: 4usize,
            end_line: 13usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(case_7()),
        ),
    };
    fn case_7() {
        let input = 6;
        let expected = 8;
        fibonacci_test(input, expected)
    }
}
mod fibonacci_test_subtests {
    use super::*;
    fn next(input: u32, expected: u32) {
        {
            #[allow(unused_variables)]
            let number = fibonacci(input);
            match (&expected, &number) {
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
            let next = number + 1;
            match (&next, &(fibonacci(input) + 1)) {
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
    mod next {
        use super::*;
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_1"]
        #[doc(hidden)]
        pub const case_1: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_1"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_1()),
            ),
        };
        fn case_1() {
            let input = 0;
            let expected = 0;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_2"]
        #[doc(hidden)]
        pub const case_2: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_2"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_2()),
            ),
        };
        fn case_2() {
            let input = 1;
            let expected = 1;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_3"]
        #[doc(hidden)]
        pub const case_3: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_3"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_3()),
            ),
        };
        fn case_3() {
            let input = 2;
            let expected = 1;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_4"]
        #[doc(hidden)]
        pub const case_4: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_4"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_4()),
            ),
        };
        fn case_4() {
            let input = 3;
            let expected = 2;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_5"]
        #[doc(hidden)]
        pub const case_5: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_5"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_5()),
            ),
        };
        fn case_5() {
            let input = 4;
            let expected = 3;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_6"]
        #[doc(hidden)]
        pub const case_6: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_6"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_6()),
            ),
        };
        fn case_6() {
            let input = 5;
            let expected = 5;
            next(input, expected)
        }
        extern crate test;
        #[rustc_test_marker = "fibonacci_test_subtests::next::case_7"]
        #[doc(hidden)]
        pub const case_7: test::TestDescAndFn = test::TestDescAndFn {
            desc: test::TestDesc {
                name: test::StaticTestName("fibonacci_test_subtests::next::case_7"),
                ignore: false,
                ignore_message: ::core::option::Option::None,
                source_file: "./tests/integration_tests/expand/rstest_tests.rs",
                start_line: 19usize,
                start_col: 8usize,
                end_line: 19usize,
                end_col: 12usize,
                compile_fail: false,
                no_run: false,
                should_panic: test::ShouldPanic::No,
                test_type: test::TestType::Unknown,
            },
            testfn: test::StaticTestFn(
                #[coverage(off)]
                || test::assert_test_result(case_7()),
            ),
        };
        fn case_7() {
            let input = 6;
            let expected = 8;
            next(input, expected)
        }
    }
}
fn fibonacci(input: u32) -> u32 {
    match input {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 2) + fibonacci(n - 1),
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &case_1,
            &case_2,
            &case_3,
            &case_4,
            &case_5,
            &case_6,
            &case_7,
            &case_1,
            &case_2,
            &case_3,
            &case_4,
            &case_5,
            &case_6,
            &case_7,
        ],
    )
}
