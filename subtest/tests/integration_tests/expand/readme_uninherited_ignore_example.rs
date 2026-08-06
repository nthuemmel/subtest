use subtest::subtest;

#[subtest]
#[test]
#[ignore]
fn parent() {
    let value = 1;
    assert_eq!(value, 1);

    #[subtest]
    fn child() {
        // <-- runs, even though the parent is ignored
        assert_eq!(value + 1, 2);
    }
}
