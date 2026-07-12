mod async_tests;

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
