# Changelog

## Next Release

First Production Release

* Fixed: A doc comment, a lint attribute (such as `#[allow]` or `#[cfg]`) or a function modifier (`#[inline]`, `#[must_use]`, `#[track_caller]`, `#[cold]`) on a nested `#[subtest]` no longer counts as an attribute override. Previously, those attributes suppressed the inherited `#[test]` attribute, preventing the subtest from running.
* Fixed: A `#[subtest]` function nested inside a block, an expression or another item is now rejected with a clear error message. Previously, it was either silently ignored or led to a confusing `can't capture dynamic environment in a fn item` error.
* Added: Helper functions (without `#[subtest]`) may now be declared in a test function's body

## v0.0.1 (2026-07-25)

Test Release on crates.io
