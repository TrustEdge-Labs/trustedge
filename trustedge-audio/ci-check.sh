#!/bin/bash
# Copyright (c) 2025 John Turner
# Pre-commit script to run the same checks as GitHub CI
# This prevents the double work problem by catching issues locally

set -e

echo "● Running pre-commit CI checks..."
echo

echo "■ Step 1: Checking code formatting..."
cargo fmt --check
echo "✔ Formatting check passed"
echo

echo "■ Step 2: Running clippy with strict warnings..."
cargo clippy --all-targets --no-default-features -- -D warnings
echo "✔ Clippy check passed"
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
