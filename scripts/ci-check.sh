#!/bin/bash
# Copyright (c) 2025 TRUSTEDGE LABS LLC
# MPL-2.0: https://mozilla.org/MPL/2.0/
# Project: trustedge — Privacy and trust at the edge.
#
# Local CI check — mirrors .github/workflows/ci.yml
# Run before pushing to catch issues without burning GH Actions minutes.
#
# Usage:
#   ./scripts/ci-check.sh          # Fast incremental (default)
#   ./scripts/ci-check.sh --clean  # Full clean build (matches CI exactly)

set -e

cd "$(dirname "$0")/.."

CLEAN=false
if [ "$1" = "--clean" ]; then
    CLEAN=true
fi

PASS=0
FAIL=0
SKIP=0

step() {
    echo
    echo "■ $1"
}

pass() {
    echo "  ✔ $1"
    PASS=$((PASS + 1))
}

fail() {
    echo "  ✖ $1"
    FAIL=$((FAIL + 1))
}

skip() {
    echo "  ⚠ $1 (skipped)"
    SKIP=$((SKIP + 1))
}

echo "● TrustEdge local CI check"
if $CLEAN; then
    echo "  Mode: clean build"
else
    echo "  Mode: incremental (use --clean for fresh build)"
fi
echo

# ── Step 0: Optional clean ──────────────────────────────────────────
if $CLEAN; then
    step "Step 0: Clean build cache"
    cargo clean
    pass "Cache cleared"
fi

# ── Step 1: Copyright headers ───────────────────────────────────────
step "Step 1: Copyright headers"
missing=0
while IFS= read -r file; do
    if ! head -10 "$file" | grep -q "Copyright (c) 2025 TRUSTEDGE LABS LLC"; then
        echo "  Missing: $file"
        missing=$((missing + 1))
    fi
done < <(find . -type f \( -name "*.rs" -o -name "*.yml" -o -name "*.yaml" -o -name "*.toml" \) \
    -not -path "./target/*" -not -path "./.git/*" -not -path "./.planning/*")
if [ $missing -gt 0 ]; then
    fail "$missing files missing copyright headers — run: ./scripts/fix-copyright.sh"
else
    pass "All source files have copyright headers"
fi

# ── Step 2: Format ──────────────────────────────────────────────────
step "Step 2: Format check"
if cargo fmt --all -- --check; then
    pass "cargo fmt"
else
    fail "cargo fmt — run: cargo fmt --all"
fi

# ── Step 3: Clippy (workspace) ──────────────────────────────────────
step "Step 3: Clippy (workspace - no features)"
if cargo clippy --workspace --all-targets --no-default-features -- -D warnings; then
    pass "clippy workspace"
else
    fail "clippy workspace"
fi

# ── Step 4: Clippy (audio) ──────────────────────────────────────────
step "Step 4: Clippy (trustedge-core with audio)"
if pkg-config --exists alsa 2>/dev/null; then
    if cargo clippy --package trustedge-core --all-targets --features audio -- -D warnings; then
        pass "clippy audio"
    else
        fail "clippy audio"
    fi
else
    skip "ALSA not available"
fi

# ── Step 5: Clippy (yubikey) ────────────────────────────────────────
step "Step 5: Clippy (trustedge-core with yubikey)"
if pkg-config --exists libpcsclite 2>/dev/null; then
    if cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings; then
        pass "clippy yubikey"
    else
        fail "clippy yubikey"
    fi
else
    skip "PCSC not available"
fi

# ── Step 6: Feature powerset (cargo-hack) ───────────────────────────
step "Step 6: Feature compatibility (cargo-hack)"
if command -v cargo-hack &> /dev/null; then
    if cargo hack check --feature-powerset --no-dev-deps --package trustedge-core; then
        pass "cargo-hack feature powerset"
    else
        fail "cargo-hack feature powerset"
    fi
else
    skip "cargo-hack not installed"
fi

# ── Step 7: Build + test workspace ──────────────────────────────────
step "Step 7: Build and test workspace (no features)"
cargo build --workspace --bins --no-default-features
if cargo test --workspace --no-default-features --locked; then
    pass "workspace tests"
else
    fail "workspace tests"
fi

# ── Step 8: Audio tests ─────────────────────────────────────────────
step "Step 8: Tests (trustedge-core with audio)"
if pkg-config --exists alsa 2>/dev/null; then
    cargo build --package trustedge-core --bins --features audio
    if cargo test --package trustedge-core --features audio --locked; then
        pass "audio tests"
    else
        fail "audio tests"
    fi
else
    skip "ALSA not available"
fi

# ── Step 9: YubiKey tests ───────────────────────────────────────────
step "Step 9: Tests (trustedge-core with yubikey)"
if pkg-config --exists libpcsclite 2>/dev/null; then
    cargo build --package trustedge-core --bins --features yubikey
    if cargo test --package trustedge-core --features yubikey --locked; then
        pass "yubikey tests"
    else
        fail "yubikey tests"
    fi
else
    skip "PCSC not available"
fi

# ── Step 10: All features (clean first to avoid disk exhaustion) ────
step "Step 10: All features combined"
if pkg-config --exists alsa 2>/dev/null && pkg-config --exists libpcsclite 2>/dev/null; then
    cargo clean
    cargo build --workspace --bins --all-features
    if cargo test -p trustedge-core --all-features --locked; then
        pass "all-features tests"
    else
        fail "all-features tests"
    fi
else
    skip "Not all platform libraries available"
fi

# ── Step 11: Downstream feature check ──────────────────────────────
step "Step 11: Downstream crate feature check (trustedge-cli)"
if command -v cargo-hack &> /dev/null; then
    if cargo hack check --feature-powerset --no-dev-deps --package trustedge-cli; then
        pass "downstream feature check"
    else
        fail "downstream feature check"
    fi
else
    skip "cargo-hack not installed"
fi

# ── Step 12: WASM ───────────────────────────────────────────────────
step "Step 12: WASM build verification"
if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    if cargo check -p trustedge-wasm --target wasm32-unknown-unknown && \
       cargo check -p trustedge-trst-wasm --target wasm32-unknown-unknown; then
        pass "WASM build"
    else
        fail "WASM build"
    fi
else
    skip "wasm32-unknown-unknown target not installed"
fi

# ── Step 13: Semver ─────────────────────────────────────────────────
step "Step 13: API compatibility (cargo-semver-checks)"
if command -v cargo-semver-checks &> /dev/null; then
    if cargo semver-checks --package trustedge-core --baseline-rev HEAD~1 2>/dev/null; then
        pass "semver check"
    else
        echo "  ⚠ semver check failed (non-blocking)"
    fi
else
    skip "cargo-semver-checks not installed"
fi

# ── Summary ─────────────────────────────────────────────────────────
echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Results: $PASS passed, $FAIL failed, $SKIP skipped"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $FAIL -gt 0 ]; then
    echo "  ✖ Fix failures before pushing."
    exit 1
else
    echo "  ✔ All checks passed. Safe to push."
fi
