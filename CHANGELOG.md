# Changelog

## Next Release

First Production Release

* Fixed: A doc comment or a lint attribute (such as `#[allow]` or `#[cfg]`) on a nested `#[subtest]` no longer counts as an attribute override. Previously, those attributes suppressed the inherited `#[test]` attribute, preventing the subtest from running.

## v0.0.1 (2026-07-25)

Test Release on crates.io
