#[subtest]
fn parent(arg: i32, arg2: bool) {
    #[subtest]
    fn child(arg: i32, arg2: bool, arg3: u64) {
        #[subtest]
        fn grandchild() {}
    }
}
