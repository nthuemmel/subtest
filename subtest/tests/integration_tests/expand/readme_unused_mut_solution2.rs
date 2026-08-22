use subtest::subtest;

#[subtest]
#[test]
fn value_can_be_incremented() {
    let mut value = 1;

    #[subtest]
    fn value_can_be_incremented_twice() {
        value += 1;
        assert_eq!(value, 2);
    }

    assert_eq!(value, 1);

    let _ = &mut value;
}
