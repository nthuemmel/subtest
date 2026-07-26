# Maintenance

## Run Tests

You will need [cargo-expand](https://github.com/dtolnay/cargo-expand) (`cargo install cargo-expand`).

```
cargo check --locked --workspace --all-targets --all-features
cargo test --locked --workspace --all-features 
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
```

### Fix Issues

```
cargo fmt --all
cargo clippy --locked --all-targets --fix --allow-dirty --allow-staged
```

## MSRV

Use [cargo-msrv](https://github.com/foresterre/cargo-msrv) (`cargo install cargo-msrv`).

* Find MSRV: `cargo msrv find --min 2024 -- cargo test`
* When changed, update the `rust-version` field in [`subtest/Cargo.toml`](subtest/Cargo.toml) and [`subtest-impl/Cargo.toml`](subtest-impl/Cargo.toml), and the versions in [CHANGELOG.md](CHANGELOG.md#next-release) and [`env.MSRV` in the CI workflow](.github/workflows/ci.yml)

## Pinned Toolchain

A toolchain for development is pinned in [`rust-toolchain.toml`](rust-toolchain.toml), because tests use `cargo-expand`, which produces slightly different output depending on compiler versions.

When necessary, just update the toolchain there. CI tests will pick it up automatically. `rustfmt` and `clippy` also run against the pinned toolchain.

The pinned toolchain is only relevant for tests. The macro itself should compile and be usable on any compiler version >= [MSRV](#msrv).

## Release

1. Update version in [`subtest/Cargo.toml`](subtest/Cargo.toml), [`subtest-impl/Cargo.toml`](subtest-impl/Cargo.toml) and [`workspace.dependencies` of `Cargo.toml`](Cargo.toml), update `Cargo.lock`
2. Update version & release date in [`CHANGELOG.md`](CHANGELOG.md)
3. Commit changes
4. Tag commit with version
5. Push
6. Publish
   1. `cargo publish -p subtest-impl --dry-run`
   2. `cargo publish -p subtest-impl`
   3. `cargo publish -p subtest --dry-run`
   4. `cargo publish -p subtest`
7. Pull in new version in a test project, make sure everything works
