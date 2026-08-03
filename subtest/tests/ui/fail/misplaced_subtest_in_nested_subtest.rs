fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    #[subtest]
    fn child() {
        loop {
            #[subtest]
            fn in_loop_body_of_subtest() {}
        }
    }
}
