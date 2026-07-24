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
