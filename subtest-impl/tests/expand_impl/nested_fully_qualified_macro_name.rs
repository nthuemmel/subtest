#[subtest]
#[test]
fn parent() {
    let i = 1;

    #[subtest::subtest]
    fn child() {
        let i = i + 1;
        assert_eq!(i, 2);
    }
}
