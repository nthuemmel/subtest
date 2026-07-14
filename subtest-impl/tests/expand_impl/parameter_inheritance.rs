#[subtest]
fn parent(arg: i32, arg2: bool) {
    #[subtest]
    fn child() {
        #[subtest]
        fn grandchild() {}
    }
}
