# Changelog

## Next Release

First stable release!

### Added

* Helper functions (without `#[subtest]`) may now be declared in a test function's body

### Changed

* `#[ignore]` and `#[should_panic]` are no longer inherited by nested subtests
* It is no longer necessary to repeat the test attribute (such as `#[test]`, `#[tokio::test]`, `#[rstest]` etc.) when adding attributes like `#[ignore]` or `#[should_panic]` to nested subtests
* When changing the test attribute, you have to opt-out of attribute inheritance using `#[subtest(inherit_attributes = false)]`
* Top-level test functions missing a test attribute (such as `#[test]`, `#[tokio::test]`, `#[rstest]` etc.) now cause a compiler error. To opt out of the check, use `#[subtest(allow_missing_test_attribute)]`

### Fixed

* A doc comment, a lint attribute (such as `#[allow]` or `#[cfg]`) or a function modifier (`#[inline]`, `#[must_use]`, `#[track_caller]`, `#[cold]`) on a nested `#[subtest]` no longer discard the inherited `#[test]` attribute, which prevented the subtest from running
* A `#[subtest]` function nested inside a block, an expression or another item is now rejected with a clear error message. Previously, it was either silently ignored or led to a confusing `can't capture dynamic environment in a fn item` error
* Variables and parameters are no longer reported as unused if they are unused in a nested subtest, but used in the parent test function. The same goes for assigned values which are never read in a nested subtest. Limitation: Does not work in a crate which sets `#![forbid(unused_variables)]` or `#![forbid(unused_assignments)]`, as `forbid` rejects the generated `#[allow]` attributes
* `#[expect]` on parent test functions no longer misfires on nested subtests (it is turned into an `#[allow]` by the macro now in inherited attributes)

## v0.0.1 (2026-07-25)

Test Release on crates.io
