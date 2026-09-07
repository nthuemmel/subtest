use subtest::subtest;

fn main() {}

#[subtest(allow_missing_test_attribute)]
fn a_number_can_be_parsed() -> anyhow::Result<()> {
    // some filler comment
    // some filler comment
    // some filler comment
    let number: i32 = "5".parse()?;

    // the error message is currently suboptimal, since it does not point at the subtest below
    // (the_number_is_five), only into the parent, but the subtest is the one that must be changed

    #[subtest]
    fn the_number_is_five() -> () {
        assert_eq!(number, 5);
    }

    assert!(number != 0);

    Ok(())
}
