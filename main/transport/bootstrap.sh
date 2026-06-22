#!/usr/bin/env sh
# bootstrap.sh — set up and verify the swe-edge-ingress-message-broker-transport workspace.
set -e

echo "==> swe-edge-ingress-message-broker-transport bootstrap"

# Verify Rust toolchain
rustc --version
cargo --version

# Build all features
echo "==> cargo build --all-features"
cargo build --all-features

# Run tests for the default feature set (in-memory)
echo "==> cargo test --features in-memory"
cargo test --features in-memory

# Lint
echo "==> cargo clippy"
cargo clippy -- -D warnings

echo "==> bootstrap complete"
