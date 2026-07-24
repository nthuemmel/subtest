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
