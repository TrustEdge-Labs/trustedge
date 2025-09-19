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
cargo clippy --all-targets --all-features -- -D warnings
echo "✔ Clippy check passed"
echo

echo "■ Step 3: Building all targets for all crates..."
cargo build --all-targets --all-features
echo "✔ Build check passed"
echo

echo "■ Step 4: Running all tests for all crates..."
cargo test --all-features
echo "✔ Test check passed"
echo

echo "♪ All CI checks passed! Safe to commit and push."
echo "   This should pass GitHub CI without issues."
