fn main() {}

#[subtest::subtest]
#[test]
fn parent() {
    let i = 1;

    if i == 1 {
        #[subtest::subtest]
        fn in_if_block() {}
    }
}
