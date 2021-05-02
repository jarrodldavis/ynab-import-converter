
#!/bin/sh

set -e

echo '==> Running `cargo clean`...'
cargo clean --frozen

echo '==> Running `cargo check`...'
cargo check --all-targets --frozen

echo '==> Running `cargo fmt`...'
cargo fmt --all -- --check

echo '==> Running `cargo clippy`...'
cargo clippy

echo '>>> Pre-commit hook complete!'
