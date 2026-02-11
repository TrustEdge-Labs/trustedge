<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 8: Validation - Research

**Researched:** 2026-02-10
**Domain:** Rust workspace consolidation validation, API compatibility verification, test preservation
**Confidence:** HIGH

## Summary

Phase 8 validates the entire consolidation effort (Phases 1-7) has preserved functionality, API compatibility, and test coverage. The workspace consolidated from 10 separate crates into a monolithic core with thin facades. This research identifies the standard tooling and validation patterns for verifying such consolidations.

The validation domain combines five complementary verification strategies: test count preservation (baseline comparison), API compatibility (semver-checks), feature interaction testing (all-features + powerset), WASM build verification (wasm32-unknown-unknown target), and unused dependency cleanup (cargo-machete with metadata mode).

**Primary recommendation:** Use multi-layered validation with automated baselines (348 test count from Phase 1), commit-to-commit semver verification (HEAD~1 baseline), conditional all-features testing (when platform libs available), WASM target verification (already integrated in CI), and cargo-machete with --with-metadata to eliminate false positives during cleanup.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cargo-semver-checks | 0.46.0 | API compatibility linting | Official Rust team adoption path, rustdoc JSON analysis, dozens of semver violation lints |
| cargo-hack | 0.6.42 | Feature powerset testing | De facto standard for feature combination validation, used by rust-lang projects |
| cargo test | 1.92.0 (built-in) | Test execution and counting | Standard Rust test harness, integrated workspace support |
| cargo-machete | 0.9.1 | Unused dependency detection | Fast ripgrep-based analysis, --with-metadata mode for accuracy |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| cargo-nextest | Latest | Alternative test runner | When needing JUnit XML output, parallel execution improvements, or CI integration |
| rustup target | Built-in | WASM target management | Already integrated - wasm32-unknown-unknown target verification |
| bencher | Latest | Continuous benchmarking | When tracking build time regression is critical (optional for this phase) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| cargo test count | cargo-nextest | Nextest provides better output formatting and JUnit XML, but adds dependency and test count requires different extraction |
| cargo-semver-checks | Manual rustdoc JSON diff | cargo-semver-checks has 40+ lints pre-configured vs custom scripting |
| Baseline comparison | git bisect | Bisect finds regressions, baseline comparison validates consolidation completeness |

**Installation:**
```bash
# Already installed in Phase 1 and CI
cargo install cargo-semver-checks --locked
cargo install cargo-hack --locked
cargo install cargo-machete --locked  # If not already present

# WASM target (already integrated in Phase 6)
rustup target add wasm32-unknown-unknown
```

## Architecture Patterns

### Recommended Validation Structure
```
.planning/phases/08-validation/
├── 08-RESEARCH.md           # This file
├── 08-01-PLAN.md            # Test count + semver validation
├── 08-02-PLAN.md            # Unused dependency cleanup + final verification
└── VALIDATION-REPORT.md     # Comprehensive validation results
```

### Pattern 1: Baseline Test Count Validation
**What:** Compare current test count against Phase 1 baseline to verify all tests preserved during consolidation.
**When to use:** After major consolidation (like Phases 4-5 receipt/attestation merges) to detect test loss.
**Example:**
```bash
# Source: Phase 1 TEST-BASELINE.md
# Baseline: 348 tests (trustedge-core: 258, attestation: 10, receipts: 23, others: 57)
# Expected after consolidation: 348+ (receipts/attestation tests moved to core)

# Extract current count
CURRENT_COUNT=$(cargo test --workspace --no-fail-fast 2>&1 | grep -E "test result:" | awk '{sum+=$4} END {print sum}')

# Compare against baseline
BASELINE=348
if [ "$CURRENT_COUNT" -ge "$BASELINE" ]; then
    echo "✓ Test count preserved: $CURRENT_COUNT (baseline: $BASELINE)"
else
    echo "✖ Test count regression: $CURRENT_COUNT < $BASELINE"
    exit 1
fi
```

### Pattern 2: Commit-to-Commit Semver Verification
**What:** Use cargo-semver-checks with HEAD~1 baseline to track API changes incrementally during consolidation.
**When to use:** When consolidating unpublished crates - published version baselines don't exist.
**Example:**
```bash
# Source: ci-check.sh Step 15 + Phase 1 decisions
# Baseline strategy: HEAD~1 (commit-to-commit) not published versions

cargo semver-checks --package trustedge-core --baseline-rev HEAD~1 || echo "No baseline (expected first run)"

# For published crates (future), would use:
# cargo semver-checks --package trustedge-core --baseline-version 0.1.0
```

### Pattern 3: All-Features Integration Testing
**What:** Test all features enabled simultaneously to catch feature interaction bugs (audio + yubikey).
**When to use:** When workspace has platform-dependent features requiring conditional CI execution.
**Example:**
```bash
# Source: Phase 6 ci-check.sh Step 12
# Only runs when both ALSA and PCSC libraries available

if pkg-config --exists alsa 2>/dev/null && pkg-config --exists libpcsclite 2>/dev/null; then
    cargo build -p trustedge-core --all-features
    cargo test -p trustedge-core --all-features --locked --verbose
else
    echo "⚠ Platform libraries unavailable - skipping all-features test"
fi
```

### Pattern 4: WASM Target Verification
**What:** Verify WASM-compatible crates build for wasm32-unknown-unknown without pulling platform dependencies.
**When to use:** When workspace includes WASM bindings and needs to verify no platform leakage (already integrated in Phase 6).
**Example:**
```bash
# Source: Phase 6 ci-check.sh Step 14
# Verifies trustedge-wasm and trustedge-trst-wasm don't accidentally enable audio/yubikey

if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    cargo check -p trustedge-wasm --target wasm32-unknown-unknown
    cargo check -p trustedge-trst-wasm --target wasm32-unknown-unknown
else
    echo "⚠ Install with: rustup target add wasm32-unknown-unknown"
fi
```

### Pattern 5: Unused Dependency Cleanup
**What:** Use cargo-machete with --with-metadata to verify and remove unused dependencies after consolidation.
**When to use:** Post-consolidation cleanup - deferred from Phase 1 to avoid mid-consolidation disruption.
**Example:**
```bash
# Source: Phase 1 MACHETE-REPORT.md
# Deferred cleanup: thiserror, serde_bytes, trustedge-trst-core, getrandom, others

# Step 1: Run with metadata for accuracy (reduces false positives)
cargo machete --with-metadata > machete-report.txt

# Step 2: Manual review of findings
# FALSE POSITIVES: derive macros (thiserror), serde attributes (serde_bytes)
# REAL ISSUES: Truly unused deps after consolidation

# Step 3: Remove confirmed unused deps from Cargo.toml
# Step 4: Verify workspace still builds/tests
cargo test --workspace --locked
```

### Anti-Patterns to Avoid
- **Removing dependencies without --with-metadata verification:** cargo-machete's default regex mode has high false positive rate for derive macros and attribute-based usage.
- **Testing individual features without all-features:** Feature interaction bugs (audio + yubikey) only surface when both enabled simultaneously.
- **Baseline comparison without accounting for test migration:** Consolidation moves tests between crates - expect core count to increase by receipts (23) + attestation (10).
- **Semver checks against published versions during consolidation:** Unpublished crates have no published baseline - use HEAD~1 commit-to-commit tracking instead.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| API compatibility verification | Custom rustdoc JSON diff scripts | cargo-semver-checks | 40+ pre-configured semver violation lints, official Rust adoption path, handles edge cases like variance, lifetimes, trait bounds |
| Feature combination testing | Manual feature flag matrix | cargo-hack --feature-powerset | Automatically generates all valid feature combinations, handles feature dependencies correctly |
| Test count extraction | Custom grep/awk pipeline | cargo test output parsing (proven pattern) | cargo test format is stable, regex extraction works across 355+ tests, no need for external tools |
| Unused dependency detection | Manual Cargo.toml audit | cargo-machete --with-metadata | Leverages ripgrep speed + cargo metadata accuracy, detects usage patterns grep misses |
| WASM compatibility | Manual cfg analysis | cargo check --target wasm32-unknown-unknown | Compiler enforces platform compatibility, catches transitive dependency issues |

**Key insight:** Validation tooling is mature in Rust ecosystem. cargo-semver-checks is on official adoption path into cargo itself. Consolidation validation should use proven tools rather than custom scripts - edge cases (derive macros, feature interactions, platform dependencies) are already handled.

## Common Pitfalls

### Pitfall 1: Test Count Mismatch from Test Migration
**What goes wrong:** Baseline shows 348 tests, but consolidation moved receipts (23) and attestation (10) tests from separate crates into core. Naive comparison fails because core test count increased while facade crates now have 0 tests.
**Why it happens:** Baseline captured pre-consolidation distribution - tests were in separate crates. Post-consolidation, tests centralized in core.
**How to avoid:** Compare workspace total (348+) not individual crate counts. Phase 1 baseline: trustedge-core 258 tests. Post-consolidation expectation: trustedge-core ~291 tests (258 + 23 receipts + 10 attestation), total workspace: 355+ tests.
**Warning signs:** Individual crate test count drops below baseline but workspace total increases - this is expected during consolidation.

### Pitfall 2: cargo-machete False Positives on Derive Macros
**What goes wrong:** cargo-machete reports thiserror and serde_bytes as unused, but removing them breaks build because they're used via derive macros and serde attributes.
**Why it happens:** Default regex-based analysis can't detect macro expansions or attribute usage. Phase 1 MACHETE-REPORT.md flagged these as likely false positives.
**How to avoid:** Always use `cargo machete --with-metadata` for final verification. Manually verify each flagged dependency by checking for derive macros (#[derive(Error)]), serde attributes (#[serde(with = "serde_bytes")]), and feature-gated usage.
**Warning signs:** Dependency shows zero grep matches but is in [dependencies] - likely used via macros. Phase 1 flagged: thiserror (attestation), serde_bytes (core), getrandom (WASM), serde-wasm-bindgen (WASM).

### Pitfall 3: All-Features Test Skipped Due to Missing Platform Libraries
**What goes wrong:** CI Step 12 (all-features test) silently skips when ALSA or PCSC libraries unavailable, leaving audio+yubikey interaction untested.
**Why it happens:** Platform dependencies (ALSA, PCSC) may not be available in all CI environments. Phase 6 added conditional guards that skip gracefully.
**How to avoid:** Validation must verify all-features test EXECUTED, not just passed. Check CI logs for "✓ All-features test passed" vs "⚠ Platform libraries unavailable". Consider adding explicit CI environment check.
**Warning signs:** CI passes but Step 12 shows warning instead of success. Local ci-check.sh skips all-features. No all-features coverage in test output.

### Pitfall 4: Semver Baseline Missing on First Run
**What goes wrong:** cargo-semver-checks fails on first run because HEAD~1 doesn't have rustdoc JSON baseline generated.
**Why it happens:** semver-checks requires comparing against a previous commit's API surface. First run has no prior baseline. ci-check.sh Step 15 handles this with `|| echo "No baseline yet"`.
**How to avoid:** Accept baseline generation on first run. Subsequent runs compare against HEAD~1. Phase 1 created API baseline JSON files but semver-checks generates its own rustdoc output per run.
**Warning signs:** Error "no such file" or "baseline not found" on first cargo-semver-checks run - expected behavior, not a failure.

### Pitfall 5: WASM Build Succeeds but Accidentally Enables Platform Features
**What goes wrong:** WASM build passes, but investigation reveals it pulled in platform-incompatible dependencies through feature transitivity.
**Why it happens:** Feature flags can propagate through dependency chains. A WASM crate depending on core with default-features=true might accidentally enable audio.
**How to avoid:** Verify WASM crates import core with `default-features = false`. Phase 6 verification confirms trustedge-wasm and trustedge-trst-wasm use no-default-features. Check Cargo.lock for unexpected platform dependencies in WASM build.
**Warning signs:** WASM Cargo.lock contains cpal, pkcs11, yubikey, or other platform crates. Build succeeds but wasm-pack warns about unavailable APIs.

### Pitfall 6: Workspace Test Count Drops Due to Feature-Gated Tests
**What goes wrong:** Some tests only compile when specific features enabled. Running `cargo test --workspace --no-default-features` shows lower count than baseline because feature-gated tests excluded.
**Why it happens:** Workspace has feature-gated tests (yubikey integration tests, audio tests). Default CI runs test without features, with audio, with yubikey separately. Total count spans all feature combinations.
**How to avoid:** Baseline comparison should use same feature configuration. Phase 1 baseline used default features. Validation should test: no-features count, audio count, yubikey count, and verify total >= baseline.
**Warning signs:** Test count varies based on feature flags. Yubikey tests (90+) only run with --features yubikey. Audio tests only run with --features audio.

## Code Examples

Verified patterns from official sources:

### Test Count Extraction and Validation
```bash
# Source: Cargo Book - continuous integration patterns
# Extract total test count from cargo test output

# Method 1: Sum all "test result: ok" lines
cargo test --workspace --no-fail-fast 2>&1 | \
    grep -E "test result:" | \
    awk '{sum+=$4} END {print sum}'

# Method 2: Count individual test lines (running N tests)
cargo test --workspace --no-fail-fast 2>&1 | \
    grep "running [0-9]* test" | \
    awk '{sum+=$2} END {print sum}'

# Validation script
BASELINE=348
CURRENT=$(cargo test --workspace --no-fail-fast 2>&1 | grep "test result:" | awk '{sum+=$4} END {print sum}')

if [ "$CURRENT" -ge "$BASELINE" ]; then
    echo "✓ Test preservation verified: $CURRENT tests (baseline: $BASELINE)"
else
    echo "✖ Test regression detected: $CURRENT < $BASELINE"
    exit 1
fi
```

### Cargo-Semver-Checks with HEAD~1 Baseline
```bash
# Source: cargo-semver-checks documentation
# https://github.com/obi1kenobi/cargo-semver-checks

# Commit-to-commit baseline (for unpublished crates)
cargo semver-checks --package trustedge-core --baseline-rev HEAD~1

# If first run (no baseline yet), allow graceful failure
cargo semver-checks --package trustedge-core --baseline-rev HEAD~1 || \
    echo "No baseline available (expected on first run)"

# Check all workspace crates
for crate in trustedge-core trustedge-receipts trustedge-attestation; do
    echo "Checking $crate..."
    cargo semver-checks --package $crate --baseline-rev HEAD~1 || true
done
```

### Cargo-Machete with Metadata Mode
```bash
# Source: cargo-machete documentation
# https://github.com/bnjbvr/cargo-machete

# Default mode (regex-based, fast but false positives)
cargo machete

# Metadata mode (uses cargo metadata, more accurate)
cargo machete --with-metadata

# Workspace-wide with metadata (recommended for validation)
cargo machete --with-metadata > MACHETE-REPORT.txt

# Example output interpretation:
# trustedge-core -- ./crates/core/Cargo.toml:
#     serde_bytes  ← FALSE POSITIVE: used via #[serde(with = "serde_bytes")]
#
# trustedge-trst-wasm -- ./crates/trst-wasm/Cargo.toml:
#     getrandom    ← FALSE POSITIVE: required for WASM RNG
```

### All-Features Conditional Testing
```bash
# Source: Phase 6 ci-check.sh integration
# Conditional all-features test based on platform library availability

echo "■ Step 12: Build and test all features together..."
if pkg-config --exists alsa 2>/dev/null && pkg-config --exists libpcsclite 2>/dev/null; then
    cargo build -p trustedge-core --all-features
    cargo test -p trustedge-core --all-features --locked --verbose
    echo "✔ All-features test passed"
else
    echo "⚠ Not all platform libraries available - skipping all-features test"
    echo "  Missing: $(pkg-config --exists alsa || echo 'ALSA') $(pkg-config --exists libpcsclite || echo 'PCSC')"
fi
```

### WASM Build Verification with Target Check
```bash
# Source: Phase 6 ci-check.sh Step 14
# Verify WASM target installed and builds succeed

echo "■ WASM build verification..."
if rustup target list --installed | grep -q wasm32-unknown-unknown; then
    cargo check -p trustedge-wasm --target wasm32-unknown-unknown
    cargo check -p trustedge-trst-wasm --target wasm32-unknown-unknown
    echo "✔ WASM build check passed"
else
    echo "⚠ wasm32-unknown-unknown target not installed - skipping WASM check"
    echo "  Install with: rustup target add wasm32-unknown-unknown"
    exit 1  # Validation phase should fail if WASM verification impossible
fi

# Verify no platform features leaked into WASM
if cargo tree -p trustedge-wasm --target wasm32-unknown-unknown | grep -E "cpal|pkcs11|yubikey"; then
    echo "✖ WASM build includes platform dependencies!"
    exit 1
fi
```

### Build Time Baseline (Optional)
```bash
# Source: Bencher continuous benchmarking
# https://bencher.dev/learn/track-in-ci/rust/compile-time/

# Capture build time baseline
time cargo build --workspace --release > build-time.txt 2>&1

# Extract compilation time
BUILD_TIME=$(grep "Finished" build-time.txt | awk '{print $3}')

# For validation, ensure <2x baseline (success criterion)
# Baseline from Phase 1 would need to be captured
# This is optional - not critical for validation pass/fail
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual rustdoc diff | cargo-semver-checks | 2023-2024 | Automated semver verification with 40+ lints, official Rust adoption path |
| cargo test only | cargo-hack feature-powerset | 2020+ | Feature interaction bugs caught before merging |
| Manual Cargo.toml audit | cargo-machete --with-metadata | 2023+ | Fast ripgrep + accurate metadata mode reduces false positives |
| Published version baselines | HEAD~1 commit tracking | Ongoing | Works for unpublished crates during consolidation |
| Manual test counting | Automated baseline comparison | Standard practice | CI can enforce test preservation automatically |

**Deprecated/outdated:**
- **cargo-semver**: Replaced by cargo-semver-checks (different tool, better linting)
- **Manual feature matrix testing**: Replaced by cargo-hack powerset automation
- **cargo-machete regex-only mode for CI**: Metadata mode reduces false positive churn

## Open Questions

1. **YubiKey Hardware Integration Documentation**
   - What we know: Phase 1-6 have YubiKey tests that skip when hardware unavailable (yubikey_hardware_detection module). CI conditionally runs yubikey tests based on PCSC library availability.
   - What's unclear: Success criterion #4 requires "YubiKey hardware integration documented (manual test protocol if hardware unavailable)". Does this mean document the existing skip behavior, or create a manual protocol for testing WITH hardware?
   - Recommendation: Document the current approach - CI tests YubiKey backend initialization and configuration without hardware (simulation tests), manual protocol documents steps for testing WITH hardware when available. Phase 1 baseline shows 90+ yubikey tests that run in simulation mode.

2. **Build Time Acceptable Bounds**
   - What we know: Success criterion #5 requires "Build time measured and within acceptable bounds (<2x baseline)". Phase 1 did not capture build time baseline.
   - What's unclear: What is the baseline? Should we capture current build time and declare it acceptable, or measure against pre-consolidation time?
   - Recommendation: Measure current workspace build time, document as post-consolidation baseline. Verify clean build completes in reasonable time (<5 minutes typical for workspace this size). Future builds should not exceed 2x this baseline.

3. **Exact Test Count Target**
   - What we know: Phase 1 baseline: 348 tests. Phase 4 moved receipts (23 tests), Phase 5 moved attestation (10 tests) into core. Expected workspace total: ~355+ tests.
   - What's unclear: Should validation enforce exact count (355) or allow range (355+)? What if new tests were added during consolidation?
   - Recommendation: Enforce minimum (348 baseline preserved) and document current count as new baseline for Phase 8. Exceeding baseline is acceptable - regression is <348.

## Sources

### Primary (HIGH confidence)
- [cargo-semver-checks GitHub repository](https://github.com/obi1kenobi/cargo-semver-checks) - Official tool documentation, baseline-rev usage
- [cargo-machete GitHub repository](https://github.com/bnjbvr/cargo-machete) - --with-metadata flag documentation
- [Rust Cargo Book - Continuous Integration](https://doc.rust-lang.org/cargo/guide/continuous-integration.html) - Standard CI patterns for Rust
- [rustdoc JSON RFC 2963](https://rust-lang.github.io/rfcs/2963-rustdoc-json.html) - Foundation for cargo-semver-checks
- Phase 1 TEST-BASELINE.md - 348 test baseline
- Phase 6 ci-check.sh - All-features and WASM verification patterns

### Secondary (MEDIUM confidence)
- [Bencher - How to track Rust compile times in CI](https://bencher.dev/learn/track-in-ci/rust/compile-time/) - Build time benchmarking (optional)
- [cargo-nextest documentation](https://nexte.st/) - Alternative test runner (not required)
- [Rust Performance Book - Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html) - Build time measurement context
- [Rust Workspace RFC 1525](https://rust-lang.github.io/rfcs/1525-cargo-workspace.html) - Workspace validation requirements

### Tertiary (LOW confidence)
- WebSearch results for YubiKey CI testing - Limited information on hardware-less testing protocols (manual protocol required)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All tools installed in Phase 1 and integrated in Phase 6 CI, versions verified
- Architecture: HIGH - Patterns extracted from current ci-check.sh and Phase 1-6 execution
- Pitfalls: HIGH - Derived from Phase 1 MACHETE-REPORT.md findings and Phase 6 integration experience
- YubiKey manual protocol: MEDIUM - Existing simulation tests documented, manual protocol needs creation
- Build time baseline: MEDIUM - Measurement approach clear, Phase 1 baseline gap acknowledged

**Research date:** 2026-02-10
**Valid until:** 60 days (stable tooling - cargo-semver-checks, cargo-hack, cargo-machete APIs stable)
