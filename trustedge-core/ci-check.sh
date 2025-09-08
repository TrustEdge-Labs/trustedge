#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# Pre-commit script to run the same checks as GitHub CI
# This prevents the double work problem by catching issues locally

set -e

echo "● Running pre-commit CI checks..."
echo

echo "■ Step 1: Auto-formatting code..."
cargo fmt
echo "✔ Code formatted automatically"
echo

echo "■ Step 2: Running clippy with strict warnings..."
cargo clippy --all-targets --no-default-features -- -D warnings
echo "✔ Clippy check passed (no features)"

# Test with audio feature if available
if cargo check --features audio --quiet 2>/dev/null; then
    echo "■ Step 2b: Running clippy with audio feature..."
    cargo clippy --all-targets --features audio -- -D warnings
    echo "✔ Clippy check passed (audio feature)"
fi

# Test with yubikey feature if available
if cargo check --features yubikey --quiet 2>/dev/null; then
    echo "■ Step 2c: Running clippy with yubikey feature..."
    cargo clippy --all-targets --features yubikey -- -D warnings
    echo "✔ Clippy check passed (yubikey feature)"
fi
echo

echo "■ Step 3: Building all targets..."
cargo build --all-targets
echo "✔ Build check passed"
echo

echo "■ Step 4: Running all tests..."
cargo test
echo "✔ Test check passed"
echo

echo "♪ All CI checks passed! Safe to commit and push."
echo "   This should pass GitHub CI without issues."
