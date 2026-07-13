mod async_tests;
mod rstest_tests;

#[cfg(test)]
mod tests {
    use subtest::subtest;

    #[subtest]
    #[test]
    fn no_subtests() {}

    #[subtest]
    #[test]
    fn two_subtests() {
        let i = 1;
        assert_eq!(i, 1);

        #[subtest]
        fn add() {
            let i = i + 1;
            assert_eq!(i, 2);
        }

        #[subtest]
        fn subtract() {
            let i = i - 1;
            assert_eq!(i, 0);
        }
    }
}

#[expect(
    dead_code,
    reason = "since #[test] is missing, the function should lead to a dead code warning,\
              like normal test functions which you forgot to annotate with #[test]"
)]
mod missing_test_attr {
    #[subtest::subtest]
    fn parent() {
        #[subtest]
        fn child() {}
    }
}
