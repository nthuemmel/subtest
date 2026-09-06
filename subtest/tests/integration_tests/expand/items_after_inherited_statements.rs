use subtest::subtest;

#[subtest]
#[test]
fn a_number_can_be_rendered() {
    let number = 1;

    // The three items below are the first thing in the subtest's body. They only end up after
    // statements because the macro prepends the parent's `let number` above them, so the
    // items_after_statements lint must not be reported for them.
    #[subtest]
    fn the_number_renders_as_one() {
        use std::fmt::Write as _;
        const SUFFIX: &str = "!";
        fn render(value: i32, suffix: &str) -> String {
            let mut rendered = String::new();
            write!(rendered, "{value}{suffix}").unwrap();
            rendered
        }

        assert_eq!(render(number, SUFFIX), "1!");

        // a nested subtest inherits the three items above, where they must stay unreported too
        #[subtest]
        fn the_number_still_renders_as_one() {
            assert_eq!(render(number, SUFFIX), "1!");
        }
    }

    assert_eq!(number, 1);
}
