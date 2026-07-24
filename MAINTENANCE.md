# Maintenance

## Run Tests

```
cargo check --locked --all-targets
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
```

### Fix Issues

```
cargo fmt --all
cargo clippy --locked --all-targets --fix --allow-dirty --allow-staged
```
