use subtest::subtest;

#[subtest]
#[test]
fn a_number_can_be_parsed() -> anyhow::Result<()> {
    let number = 1;

    // the subtest resets the return type to `()`, because the parent's `anyhow::Result<()>` would
    // be inherited otherwise - which must not be reported as an unneeded unit return type, as
    // removing the `-> ()` does not leave the subtest returning `()`, but re-inherits the parent's
    // return type
    #[subtest]
    fn the_number_is_one() -> () {
        assert_eq!(number, 1);

        // inherits the `-> ()` of the subtest above, which must not be reported either
        #[subtest]
        fn the_number_is_positive() {
            assert!(number > 0);
        }
    }

    // the `?` is what makes the parent return a `Result` in the first place - it is written below
    // the subtests, so they do not inherit it (they could not propagate an error while returning
    // `()`)
    let parsed: i32 = "1".parse()?;
    assert_eq!(parsed, number);
    Ok(())
}
