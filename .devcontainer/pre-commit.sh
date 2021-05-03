
#!/bin/sh

set -e

echo '==> Running `cargo clean`...'
cargo clean --frozen

echo '==> Running `cargo check`...'
cargo check --all-targets --frozen

echo '==> Running `cargo fmt`...'
cargo fmt -- --check

echo '==> Running `cargo clippy`...'
cargo clippy --frozen

echo '>>> Pre-commit hook complete!'
