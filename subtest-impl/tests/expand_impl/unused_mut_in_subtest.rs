#[subtest]
#[test]
fn value_can_be_sent() {
    let mut value = 1;

    #[subtest]
    fn another_value_can_be_sent() {
        assert_eq!(value, 1);
    }

    value += 1;
    assert_eq!(value, 2);
}
