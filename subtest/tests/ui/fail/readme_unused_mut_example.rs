#![deny(unused_mut)]

fn main() {}

use subtest::subtest;

#[subtest(allow_missing_test_attribute)]
// #[test] // if marked as test, the function is removed before the diagnostic shows, because trybuild doesn't compile with `--test`
fn value_can_be_incremented() {
    let mut value = 1;

    #[subtest]
    fn value_can_be_incremented_twice() {
        value += 1;
        assert_eq!(value, 2);
    }

    assert_eq!(value, 1);
}
