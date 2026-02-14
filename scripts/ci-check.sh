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
WARN=0

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

warn() {
    echo "  ⚠ $1 (warning)"
    WARN=$((WARN + 1))
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

# ── Step 2: Security audit ──────────────────────────────────────────
step "Step 2: Security audit (cargo-audit)"
if command -v cargo-audit &> /dev/null; then
    if cargo audit; then
        pass "cargo audit"
    else
        fail "cargo audit — run: cargo audit to see details"
    fi
else
    skip "cargo-audit not installed (install: cargo install cargo-audit)"
fi

# ── Step 3: Format ──────────────────────────────────────────────────
step "Step 3: Format check"
if cargo fmt --all -- --check; then
    pass "cargo fmt"
else
    fail "cargo fmt — run: cargo fmt --all"
fi

# ── Step 4: Clippy (tiered) ────────────────────────────────────────
step "Step 4: Clippy (tiered - core blocking, experimental non-blocking)"

# Core crates (blocking)
if cargo clippy \
    -p trustedge-core \
    -p trustedge-cli \
    -p trustedge-trst-protocols \
    -p trustedge-trst-cli \
    -p trustedge-trst-wasm \
    -p trustedge-cam-video-examples \
    --all-targets --no-default-features -- -D warnings; then
    pass "clippy core crates"
else
    fail "clippy core crates"
fi

# Experimental crates (non-blocking)
if cargo clippy \
    -p trustedge-wasm \
    -p trustedge-pubky \
    -p trustedge-pubky-advanced \
    -p trustedge-receipts \
    -p trustedge-attestation \
    --all-targets --no-default-features -- -D warnings 2>/dev/null; then
    pass "clippy experimental crates"
else
    warn "clippy experimental crates failed (non-blocking)"
fi

# ── Step 5: Clippy (audio) ──────────────────────────────────────────
step "Step 5: Clippy (trustedge-core with audio)"
if pkg-config --exists alsa 2>/dev/null; then
    if cargo clippy --package trustedge-core --all-targets --features audio -- -D warnings; then
        pass "clippy audio"
    else
        fail "clippy audio"
    fi
else
    skip "ALSA not available"
fi

# ── Step 6: Clippy (yubikey) ────────────────────────────────────────
step "Step 6: Clippy (trustedge-core with yubikey)"
if pkg-config --exists libpcsclite 2>/dev/null; then
    if cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings; then
        pass "clippy yubikey"
    else
        fail "clippy yubikey"
    fi
else
    skip "PCSC not available"
fi

# ── Step 7: Clippy (git-attestation) ────────────────────────────────
step "Step 7: Clippy (trustedge-core with git-attestation)"
if cargo clippy --package trustedge-core --all-targets --features git-attestation -- -D warnings; then
    pass "clippy git-attestation"
else
    fail "clippy git-attestation"
fi

# ── Step 8: Clippy (keyring) ────────────────────────────────────────
step "Step 8: Clippy (trustedge-core with keyring)"
if cargo clippy --package trustedge-core --all-targets --features keyring -- -D warnings; then
    pass "clippy keyring"
else
    fail "clippy keyring"
fi

# ── Step 9: Clippy (insecure-tls) ───────────────────────────────────
step "Step 9: Clippy (trustedge-core with insecure-tls)"
if cargo clippy --package trustedge-core --all-targets --features insecure-tls -- -D warnings; then
    pass "clippy insecure-tls"
else
    fail "clippy insecure-tls"
fi

# ── Step 10: Feature powerset (cargo-hack) ──────────────────────────
step "Step 10: Feature compatibility (cargo-hack)"
if command -v cargo-hack &> /dev/null; then
    if cargo hack check --feature-powerset --no-dev-deps --package trustedge-core; then
        pass "cargo-hack feature powerset"
    else
        fail "cargo-hack feature powerset"
    fi
else
    skip "cargo-hack not installed"
fi

# ── Step 11: Build + test (tiered) ──────────────────────────────────
step "Step 11: Build and test (tiered - core blocking, experimental non-blocking)"

# Build remains workspace-wide
cargo build --workspace --bins --no-default-features

# Core crate tests (blocking)
if cargo test \
    -p trustedge-core \
    -p trustedge-cli \
    -p trustedge-trst-protocols \
    -p trustedge-trst-cli \
    -p trustedge-trst-wasm \
    -p trustedge-cam-video-examples \
    --no-default-features --locked; then
    pass "core crate tests"
else
    fail "core crate tests"
fi

# Experimental crate tests (non-blocking)
if cargo test \
    -p trustedge-wasm \
    -p trustedge-pubky \
    -p trustedge-pubky-advanced \
    -p trustedge-receipts \
    -p trustedge-attestation \
    --no-default-features --locked 2>/dev/null; then
    pass "experimental crate tests"
else
    warn "experimental crate tests failed (non-blocking)"
fi

# ── Step 12: Audio tests ────────────────────────────────────────────
step "Step 12: Tests (trustedge-core with audio)"
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

# ── Step 13: YubiKey tests (simulation) ────────────────────────────
step "Step 13: Tests (trustedge-core with yubikey simulation)"
if pkg-config --exists libpcsclite 2>/dev/null; then
    cargo build --package trustedge-core --bins --features yubikey
    if cargo test --package trustedge-core --features yubikey --lib --locked; then
        pass "yubikey tests"
    else
        fail "yubikey tests"
    fi
else
    skip "PCSC not available"
fi

# ── Step 14: Tests (git-attestation) ───────────────────────────────
step "Step 14: Tests (trustedge-core with git-attestation)"
if cargo test --package trustedge-core --features git-attestation --locked; then
    pass "git-attestation tests"
else
    fail "git-attestation tests"
fi

# ── Step 15: Tests (keyring) ───────────────────────────────────────
step "Step 15: Tests (trustedge-core with keyring)"
if cargo test --package trustedge-core --features keyring --locked; then
    pass "keyring tests"
else
    fail "keyring tests"
fi

# ── Step 16: Tests (insecure-tls) ──────────────────────────────────
step "Step 16: Tests (trustedge-core with insecure-tls)"
if cargo test --package trustedge-core --features insecure-tls --locked; then
    pass "insecure-tls tests"
else
    fail "insecure-tls tests"
fi

# ── Step 17: All features (clean first to avoid disk exhaustion) ────
step "Step 17: All features combined"
if pkg-config --exists alsa 2>/dev/null && pkg-config --exists libpcsclite 2>/dev/null; then
    cargo clean
    cargo build --workspace --bins --all-features
    if cargo test -p trustedge-core --all-features --lib --locked; then
        pass "all-features tests"
    else
        fail "all-features tests"
    fi
else
    skip "Not all platform libraries available"
fi

# ── Step 18: Downstream feature check ──────────────────────────────
step "Step 18: Downstream crate feature check (trustedge-cli)"
if command -v cargo-hack &> /dev/null; then
    if cargo hack check --feature-powerset --no-dev-deps --package trustedge-cli; then
        pass "downstream feature check"
    else
        fail "downstream feature check"
    fi
else
    skip "cargo-hack not installed"
fi

# ── Step 19: WASM ───────────────────────────────────────────────────
step "Step 19: WASM build verification"
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

# ── Step 20: Semver ─────────────────────────────────────────────────
step "Step 20: API compatibility (cargo-semver-checks)"
if command -v cargo-semver-checks &> /dev/null; then
    if cargo semver-checks --package trustedge-core --baseline-rev HEAD~1 2>/dev/null; then
        pass "semver check"
    else
        echo "  ⚠ semver check failed (non-blocking)"
    fi
else
    skip "cargo-semver-checks not installed"
fi

# ── Step 21: Dependency tree size ────────────────────────────────────
step "Step 21: Dependency tree size check"
dep_count=$(cargo tree --workspace --depth 1 --prefix none --no-dedupe 2>/dev/null | sort -u | wc -l)
baseline=60
threshold=$((baseline + 10))
echo "  Dependency tree: $dep_count unique crates (baseline: $baseline)"
if [ "$dep_count" -gt "$threshold" ]; then
    echo "  ⚠ Dependency tree grew beyond threshold ($dep_count > $threshold)"
    WARN=$((WARN + 1))
else
    pass "dependency tree within baseline"
fi

# ── Step 22: TODO hygiene ──────────────────────────────────────────
step "Step 22: TODO hygiene (no unimplemented markers)"
# Scan for TODO/FIXME/HACK/XXX comments that indicate unimplemented functionality
# Excludes: test fixtures, planning docs, target dir, .git dir
todo_count=0
while IFS= read -r match; do
    # Skip test-only placeholder data (e.g., continuity_hash in test fixtures)
    case "$match" in
        *"#[cfg(test)]"*) continue ;;
        *"_test_"*|*"test_"*) continue ;;
    esac
    echo "  Found: $match"
    todo_count=$((todo_count + 1))
done < <(grep -rn '// TODO\|// FIXME\|// HACK\|// XXX\|todo!()\|unimplemented!()' \
    --include="*.rs" crates/ \
    2>/dev/null || true)
if [ "$todo_count" -gt 0 ]; then
    fail "$todo_count unimplemented TODO/FIXME markers found"
else
    pass "No unimplemented TODO/FIXME markers"
fi

# ── Summary ─────────────────────────────────────────────────────────
echo
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  Results: $PASS passed, $FAIL failed, $WARN warnings, $SKIP skipped"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

if [ $FAIL -gt 0 ]; then
    echo "  ✖ Fix failures before pushing."
    exit 1
else
    echo "  ✔ All checks passed. Safe to push."
fi
