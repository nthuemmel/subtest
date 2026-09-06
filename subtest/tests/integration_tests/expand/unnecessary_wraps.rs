//! see `unnecessary_wraps_inherited` for the case where the lint is a false positive

use subtest::subtest;

// The parent never fails itself - only the subtest below does. The `Result` return type could be
// declared on the subtest instead of on the parent, so reporting the parent is right.
#[subtest]
#[test]
#[expect(
    clippy::unnecessary_wraps,
    reason = "only the subtest below fails, so the Result return type could be moved down to it \
              instead of being inherited from here"
)]
fn a_text_can_be_parsed() -> Result<(), String> {
    let text = "1";

    #[subtest]
    fn the_text_parses_to_one() {
        let parsed: i32 = text
            .parse()
            .map_err(|error: std::num::ParseIntError| error.to_string())?;
        assert_eq!(parsed, 1);
        Ok(())
    }

    assert_eq!(text, "1");
    Ok(())
}

#[subtest]
#[test]
fn a_number_is_one() {
    let number = 1;

    // The subtest overrides the parent's `()` return type with a `Result` of its own, but never
    // fails - so it can simply drop the return type again, and reporting it is right.
    #[subtest]
    #[expect(
        clippy::unnecessary_wraps,
        reason = "the subtest declares the Result return type itself and never fails, so it can \
                  drop it without touching the parent"
    )]
    fn the_number_is_one() -> Result<(), String> {
        assert_eq!(number, 1);
        Ok(())
    }

    assert_eq!(number, 1);
}
