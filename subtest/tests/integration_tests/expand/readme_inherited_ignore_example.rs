use subtest::subtest;

#[subtest]
#[test]
#[ignore]
fn parent() {
    let value = 1;
    assert_eq!(value, 1);

    #[subtest]
    fn child() {
        // <-- inherits #[ignore], so it does not run either
        assert_eq!(value + 1, 2);
    }
}
