fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    let closure = || {
        #[subtest]
        fn in_closure_body() {}
    };
}
