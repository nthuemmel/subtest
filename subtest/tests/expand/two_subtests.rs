#[subtest::subtest]
#[test]
fn two_subtests() {
    let i = 1;

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
