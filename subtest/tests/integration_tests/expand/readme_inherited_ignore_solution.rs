use subtest::subtest;

#[subtest]
#[test]
#[ignore]
fn parent() {
    let value = 1;
    assert_eq!(value, 1);

    #[subtest(inherit_attributes = false)]
    #[test] // <-- no longer inherited, so specify it explicitly
    fn child() {
        // <-- runs, even though the parent is ignored
        assert_eq!(value + 1, 2);
    }
}
