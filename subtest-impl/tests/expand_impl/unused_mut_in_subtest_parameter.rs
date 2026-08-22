#[subtest]
#[rstest]
#[case(1)]
fn value_can_be_incremented(#[case] mut value: u32) {
    #[subtest]
    fn value_is_positive() {
        assert!(value > 0);
    }

    value += 1;
    assert!(value > 1);
}
