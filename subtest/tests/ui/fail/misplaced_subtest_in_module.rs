fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    mod inner {
        #[subtest]
        fn in_nested_module() {}
    }
}
