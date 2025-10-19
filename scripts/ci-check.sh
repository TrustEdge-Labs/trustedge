#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# Pre-commit script to run the same checks as GitHub CI
# This prevents the double work problem by catching issues locally

set -e

echo "● Running pre-commit CI checks..."
echo

# Change to the project root directory
cd "$(dirname "$0")/.."

echo "■ Step 1: Checking code formatting for all crates..."
cargo fmt --check --all
echo "✔ Formatting check passed"
echo

echo "■ Step 2: Running clippy with strict warnings for all crates..."
cargo clippy --workspace --all-targets --all-features -- -D warnings
echo "✔ Clippy check passed"
echo

echo "■ Step 3: Running clippy on WASM targets..."
if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    cargo clippy -p trustedge-wasm -p trustedge-trst-wasm --target wasm32-unknown-unknown -- -D warnings
    echo "✔ WASM clippy check passed"
else
    echo "⚠ WASM target not installed - skipping WASM clippy check"
    echo "  Install with: rustup target add wasm32-unknown-unknown"
fi
echo

echo "■ Step 4: Building all targets for all crates..."
cargo build --workspace --all-targets --all-features
echo "✔ Build check passed"
echo

echo "■ Step 5: Running all tests for all crates..."
cargo test --workspace --all-features
echo "✔ Test check passed"
echo

echo "♪ All CI checks passed! Safe to commit and push."
echo "   This should pass GitHub CI without issues."
