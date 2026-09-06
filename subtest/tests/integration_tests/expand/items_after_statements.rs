//! see `items_after_inherited_statements` for the case where the lint has to be masked

use subtest::subtest;

#[subtest]
#[test]
fn a_number_can_be_doubled() {
    let number = 2;

    #[subtest]
    fn the_number_doubles_to_four() {
        let doubled = number * 2;

        // written below a statement of the subtest itself rather than an inherited one, so the
        // macro must not mask it and the lint has to keep being reported
        fn quadruple(value: i32) -> i32 {
            value * 4
        }

        assert_eq!(doubled, 4);
        assert_eq!(quadruple(number), 8);
    }

    assert_eq!(number, 2);
}
