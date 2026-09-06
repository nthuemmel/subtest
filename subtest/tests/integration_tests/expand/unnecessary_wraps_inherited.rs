//! see `unnecessary_wraps` for the cases where the lint is reported rightfully

use subtest::subtest;

#[subtest]
#[test]
fn a_text_can_be_parsed() -> Result<(), String> {
    let text = "1";

    // The subtest inherits the parent's `Result` return type, but has no error path of its own:
    // the parent's `?` is written below the subtest, so it is not inherited. Dropping the return
    // type - as the lint suggests - is not something the author of this subtest can do, as the
    // return type belongs to the parent, which needs it for the `?` below.
    #[subtest]
    fn the_text_is_a_one() {
        assert_eq!(text, "1");
        Ok(())
    }

    let parsed: i32 = text
        .parse()
        .map_err(|error: std::num::ParseIntError| error.to_string())?;
    assert_eq!(parsed, 1);
    Ok(())
}
