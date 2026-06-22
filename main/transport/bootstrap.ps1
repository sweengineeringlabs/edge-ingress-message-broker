# bootstrap.ps1 — set up and verify the swe-edge-ingress-message-broker-transport workspace.
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

Write-Host "==> swe-edge-ingress-message-broker-transport bootstrap"

# Verify Rust toolchain
rustc --version
cargo --version

# Build all features
Write-Host "==> cargo build --all-features"
cargo build --all-features

# Run tests for the default feature set (in-memory)
Write-Host "==> cargo test --features in-memory"
cargo test --features in-memory

# Lint
Write-Host "==> cargo clippy"
cargo clippy -- -D warnings

Write-Host "==> bootstrap complete"
