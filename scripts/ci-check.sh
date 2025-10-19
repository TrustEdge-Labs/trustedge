#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# Pre-commit script to run the EXACT SAME checks as GitHub CI
# This prevents the double work problem by catching issues locally
#
# IMPORTANT: This script mirrors .github/workflows/ci.yml exactly
# If you change one, update the other!

set -e

echo "● Running pre-commit CI checks (matching GitHub CI exactly)..."
echo

# Change to the project root directory
cd "$(dirname "$0")/.."

echo "■ Step 1: Checking code formatting..."
cargo fmt --all -- --check
echo "✔ Formatting check passed"
echo

echo "■ Step 2: Clippy (workspace - no features)..."
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
echo "✔ Clippy check passed (no features)"
echo

echo "■ Step 3: Clippy (trustedge-core with audio)..."
if pkg-config --exists alsa 2>/dev/null; then
    cargo clippy --package trustedge-core --all-targets --features audio -- -D warnings
    echo "✔ Clippy check passed (audio feature)"
else
    echo "⚠ ALSA not available - skipping audio feature clippy"
fi
echo

echo "■ Step 4: Clippy (trustedge-core with yubikey)..."
if pkg-config --exists libpcsclite 2>/dev/null; then
    cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings
    echo "✔ Clippy check passed (yubikey feature)"
else
    echo "⚠ PCSC not available - skipping yubikey feature clippy"
fi
echo

echo "■ Step 5: Build binaries (workspace - no features)..."
cargo build --workspace --bins --no-default-features
echo "✔ Build check passed (no features)"
echo

echo "■ Step 6: Tests (workspace - no features)..."
cargo test --workspace --no-default-features --locked --verbose
echo "✔ Test check passed (no features)"
echo

echo "■ Step 7: Build binaries (trustedge-core with audio)..."
if pkg-config --exists alsa 2>/dev/null; then
    cargo build --package trustedge-core --bins --features audio
    echo "✔ Build check passed (audio feature)"
else
    echo "⚠ ALSA not available - skipping audio feature build"
fi
echo

echo "■ Step 8: Tests (trustedge-core with audio)..."
if pkg-config --exists alsa 2>/dev/null; then
    cargo test --package trustedge-core --features audio --locked --verbose
    echo "✔ Test check passed (audio feature)"
else
    echo "⚠ ALSA not available - skipping audio feature tests"
fi
echo

echo "■ Step 9: Build binaries (trustedge-core with yubikey)..."
if pkg-config --exists libpcsclite 2>/dev/null; then
    cargo build --package trustedge-core --bins --features yubikey
    echo "✔ Build check passed (yubikey feature)"
else
    echo "⚠ PCSC not available - skipping yubikey feature build"
fi
echo

echo "■ Step 10: Tests (trustedge-core with yubikey)..."
if pkg-config --exists libpcsclite 2>/dev/null; then
    cargo test --package trustedge-core --features yubikey --locked --verbose
    echo "✔ Test check passed (yubikey feature)"
else
    echo "⚠ PCSC not available - skipping yubikey feature tests"
fi
echo

echo "♪ All CI checks passed! Safe to commit and push."
echo "   This script matches GitHub CI workflow exactly."
