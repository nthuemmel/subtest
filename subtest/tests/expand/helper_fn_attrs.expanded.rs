extern crate test;
#[rustc_test_marker = "helper_fn_attrs"]
#[doc(hidden)]
pub const helper_fn_attrs: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("helper_fn_attrs"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "./tests/expand/helper_fn_attrs.rs",
        start_line: 3usize,
        start_col: 4usize,
        end_line: 3usize,
        end_col: 19usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::Unknown,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(helper_fn_attrs()),
    ),
};
fn helper_fn_attrs() {
    /// A doc comment
    #[allow(dead_code)]
    fn documented_and_allowed() {}
    #[warn(clippy::eq_op)]
    fn warns() -> u32 {
        1
    }
    #[deny(unsafe_code)]
    fn denies() -> u32 {
        2
    }
    #[forbid(unsafe_code)]
    fn forbids() -> u32 {
        3
    }
    fn always_compiled() -> u32 {
        4
    }
    #[inline]
    fn conditionally_inlined() -> u32 {
        5
    }
    #[rustfmt::skip]
    fn not_formatted() -> u32 {
        6
    }
    #[inline]
    fn inlined() -> u32 {
        7
    }
    #[must_use]
    fn must_be_used() -> u32 {
        8
    }
    #[cold]
    fn cold() -> u32 {
        9
    }
    #[track_caller]
    fn assert_positive(value: u32) {
        if !(value > 0) {
            ::core::panicking::panic("assertion failed: value > 0")
        }
    }
    let sum = warns() + denies() + forbids() + always_compiled()
        + conditionally_inlined() + not_formatted() + inlined() + must_be_used()
        + cold();
    assert_positive(sum);
    #[expect(dead_code)]
    fn expects_to_be_dead() {}
}
mod helper_fn_attrs_subtests {
    use super::*;
    extern crate test;
    #[rustc_test_marker = "helper_fn_attrs_subtests::attributed_helpers_are_copied_into_subtests"]
    #[doc(hidden)]
    pub const attributed_helpers_are_copied_into_subtests: test::TestDescAndFn = test::TestDescAndFn {
        desc: test::TestDesc {
            name: test::StaticTestName(
                "helper_fn_attrs_subtests::attributed_helpers_are_copied_into_subtests",
            ),
            ignore: false,
            ignore_message: ::core::option::Option::None,
            source_file: "./tests/expand/helper_fn_attrs.rs",
            start_line: 69usize,
            start_col: 8usize,
            end_line: 69usize,
            end_col: 51usize,
            compile_fail: false,
            no_run: false,
            should_panic: test::ShouldPanic::No,
            test_type: test::TestType::Unknown,
        },
        testfn: test::StaticTestFn(
            #[coverage(off)]
            || test::assert_test_result(attributed_helpers_are_copied_into_subtests()),
        ),
    };
    fn attributed_helpers_are_copied_into_subtests() {
        /// A doc comment
        #[allow(dead_code)]
        fn documented_and_allowed() {}
        #[warn(clippy::eq_op)]
        fn warns() -> u32 {
            1
        }
        #[deny(unsafe_code)]
        fn denies() -> u32 {
            2
        }
        #[forbid(unsafe_code)]
        fn forbids() -> u32 {
            3
        }
        fn always_compiled() -> u32 {
            4
        }
        #[inline]
        fn conditionally_inlined() -> u32 {
            5
        }
        #[rustfmt::skip]
        fn not_formatted() -> u32 {
            6
        }
        #[inline]
        fn inlined() -> u32 {
            7
        }
        #[must_use]
        fn must_be_used() -> u32 {
            8
        }
        #[cold]
        fn cold() -> u32 {
            9
        }
        #[track_caller]
        fn assert_positive(value: u32) {
            if !(value > 0) {
                ::core::panicking::panic("assertion failed: value > 0")
            }
        }
        let sum = warns() + denies() + forbids() + always_compiled()
            + conditionally_inlined() + not_formatted() + inlined() + must_be_used()
            + cold();
        assert_positive(sum);
        match (&sum, &45) {
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
        assert_positive(inlined());
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[&helper_fn_attrs, &attributed_helpers_are_copied_into_subtests],
    )
}
