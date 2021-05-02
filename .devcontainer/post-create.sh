#!/bin/sh

set -e

RUST_TOOLCHAIN=$(cat rust-toolchain.toml | grep channel | cut -d'"' -f2)

echo "==> Installing toolchain \`$RUST_TOOLCHAIN\`..."
rustup show

echo '==> Installed tools:'
rustc --version
cargo --version
cargo fmt --version
cargo clippy --version

echo '==> Fetching dependencies (`cargo fetch`)...'
cargo fetch --locked

echo '==> Building package (`cargo build`)...'
cargo build --frozen --all-targets

echo '==> Installing pre-commit hook...'
ln -sf "$PWD/.devcontainer/pre-commit.sh" .git/hooks/pre-commit

echo '>>> Post-create steps complete!'
