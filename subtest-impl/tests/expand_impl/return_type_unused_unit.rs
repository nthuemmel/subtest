#[subtest]
#[test]
fn parent() -> () {
    // resets nothing, as the parent returns `()` anyway - so it must not be exempted from the
    // clippy::unused_unit lint
    #[subtest]
    fn child() -> () {
        #[subtest]
        fn grandchild() -> () {}
    }
}
