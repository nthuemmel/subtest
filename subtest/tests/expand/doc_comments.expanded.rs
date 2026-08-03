extern crate test;
#[rustc_test_marker = "doc_comments"]
#[doc(hidden)]
pub const doc_comments: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("doc_comments"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/doc_comments.rs",
        start_line: 3usize,
        start_col: 4usize,
        end_line: 3usize,
        end_col: 16usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(doc_comments()),
    ),
};
fn doc_comments() {
    let i = 1;
    match (&i, &1) {
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
mod doc_comments_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "doc_comments_subtests::documented"]
    #[doc(hidden)]
    pub const documented: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName("doc_comments_subtests::documented"),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/doc_comments.rs",
            start_line: 10usize,
            start_col: 8usize,
            end_line: 10usize,
            end_col: 18usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(documented()),
        ),
    };
    /// A doc comment must not override the inherited `#[test]` attribute - otherwise this subtest
    /// would silently never run.
    fn documented() {
        let i = 1;
        match (&i, &1) {
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
        let i = i + 1;
        match (&i, &2) {
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
    extern crate test;
    #[rustc_test_marker = "doc_comments_subtests::documented_with_lint_attr"]
    #[doc(hidden)]
    pub const documented_with_lint_attr: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "doc_comments_subtests::documented_with_lint_attr",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/doc_comments.rs",
            start_line: 18usize,
            start_col: 8usize,
            end_line: 18usize,
            end_col: 33usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(documented_with_lint_attr()),
        ),
    };
    /// Neither must a lint attribute.
    #[allow(clippy::eq_op)]
    fn documented_with_lint_attr() {
        let i = 1;
        match (&i, &1) {
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
        match (&i, &i) {
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
    extern crate test;
    #[rustc_test_marker = "doc_comments_subtests::documented_with_override"]
    #[doc(hidden)]
    pub const documented_with_override: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "doc_comments_subtests::documented_with_override",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/doc_comments.rs",
            start_line: 26usize,
            start_col: 8usize,
            end_line: 26usize,
            end_col: 32usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::YesWithMessage("my failure"),
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(documented_with_override()),
        ),
    };
    /// Explicit test attributes still override.
    #[should_panic(expected = "my failure")]
    fn documented_with_override() {
        let i = 1;
        match (&i, &1) {
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
        {
            ::core::panicking::panic_fmt(format_args!("my failure"));
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &doc_comments,
            &documented,
            &documented_with_lint_attr,
            &documented_with_override,
        ],
    )
}
