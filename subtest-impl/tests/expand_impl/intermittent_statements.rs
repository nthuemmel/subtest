#[subtest]
#[test]
fn parent() {
    let local_var = 1;

    #[subtest]
    fn first() {
        assert_eq!(local_var, 1);
    }

    let local_var = local_var + 1;

    #[subtest]
    fn second() {
        assert_eq!(local_var, 2);
    }

    let local_var = 24;
}
