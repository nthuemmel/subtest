# subtest

[![crates.io][crate-image]][crate-link]
[![docs.rs][docs-image]][docs-link]
[![Build Status][ci-image]][ci-link]
[![Apache 2.0 Licensed][license-apache-image]][license-apache-link]
[![MIT Licensed][license-mit-image]][license-mit-link]

`#[subtest]` is a Rust macro to easily share test setup code between related test functions.

## Example

## How it works

statement copy

## What you can do

* arbitrarily nest
* use async
* use rstest
* upgrade sync -> async
* omit attr, params, return type

## What you can't do

* omit top-level test attr
* make async non-async again
* ambiguous assert macro import
* unused variables

## Changelog

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

[//]: # (links)

[crate-image]: https://img.shields.io/crates/v/subtest.svg

[crate-link]: https://crates.io/crates/subtest

[docs-image]: https://docs.rs/subtest/badge.svg

[docs-link]: https://docs.rs/subtest/

[ci-image]: https://github.com/nthuemmel/subtest/actions/workflows/ci.yml/badge.svg?branch=master

[ci-link]: https://github.com/nthuemmel/subtest/actions/workflows/ci.yml?query=branch%3Amaster

[license-apache-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg

[license-apache-link]: LICENSE-APACHE

[license-mit-image]: https://img.shields.io/badge/license-MIT-blue.svg

[license-MIT-link]: LICENSE-MIT
