# Phase 12: CI Integration - Research

**Researched:** 2026-02-11
**Domain:** GitHub Actions CI/CD for Rust feature-gated builds with optional system dependencies
**Confidence:** HIGH

## Summary

Phase 12 requires making YubiKey feature compilation and simulation testing unconditional in CI, while keeping hardware tests gated with `#[ignore]`. The core challenge is that the current CI workflow (`.github/workflows/ci.yml`) uses conditional steps (`if: steps.yubikey-deps.outputs.yubikey-available == 'true'`) that skip YubiKey builds when `libpcsclite-dev` installation fails or is unavailable.

Rust's feature system allows compilation with `--features yubikey` even when system dependencies (libpcsclite) are missing at CI time, as long as the code is properly guarded with `#[cfg(feature = "yubikey")]` and tests don't require runtime access to PCSC libraries. Phase 11 created 18 simulation tests that run without hardware and validate all non-hardware logic (slot parsing, capability reporting, config validation, error mapping). These tests are in `crates/core/src/backends/yubikey.rs` under `#[cfg(test)]` and must run on every PR.

GitHub Actions conditional execution uses `if` expressions with step outputs. The current workflow installs dependencies with fallback (`|| echo "..."`) and sets outputs that later steps check. To make steps unconditional, simply remove the `if` condition — steps without `if` always run as part of normal workflow execution.

**Primary recommendation:** Remove conditional `if` checks from YubiKey CI steps (clippy, build, test). Install `libpcsclite-dev` and `pkgconf` without fallback — let installation fail loudly if packages are unavailable (they're in Ubuntu's main repository and should always be present on `ubuntu-latest` runners). Run simulation tests unconditionally with `cargo test --features yubikey --lib`. Hardware tests remain `#[ignore]` and are NOT run in CI.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| GitHub Actions | N/A | CI/CD platform | GitHub-hosted runners, free for public repos, integrated with repo |
| ubuntu-latest | 24.04 LTS | Runner image | Stable, long-term support, well-tested for Rust projects |
| dtolnay/rust-toolchain@stable | N/A | Rust toolchain installer | Maintained by Rust core team member, standard in Rust CI |
| Swatinem/rust-cache@v2 | 2.x | Cargo build cache | Industry standard, reduces CI time from 8-10 min to 2-4 min |
| cargo-hack | 0.6+ | Feature powerset testing | Detects feature interaction bugs, validates all feature combinations |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| cargo-semver-checks | 0.36+ | API compatibility | Already used (non-blocking), validates no breaking changes |
| libpcsclite-dev | Ubuntu main repo | PCSC-Lite development headers | Required for YubiKey feature compilation (pkcs11 crate dependency) |
| pkgconf | Ubuntu main repo | Package configuration tool | Required by Cargo build scripts to find libpcsclite |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Unconditional install | Conditional with fallback | Conditional hides real failures; libpcsclite is in Ubuntu main and should always work |
| Remove `if` condition | Use `if: always()` | `always()` runs even after failures and breaks cancellation; omitting `if` is cleaner |
| `--lib` flag for tests | `--test yubikey_integration --include-ignored` | Hardware tests need physical YubiKey; CI should only run simulation tests |
| `ubuntu-latest` | `ubuntu-22.04` (pinned) | `latest` gets security patches automatically; pinning reduces maintenance but risks stale environment |

**Installation:**
```bash
# GitHub Actions runner (already has these)
sudo apt-get update
sudo apt-get install -y libpcsclite-dev pkgconf

# Developer local setup (manual)
# Debian/Ubuntu:
sudo apt-get install -y libpcsclite-dev pkg-config

# Fedora:
sudo dnf install -y pcsc-lite-devel pkgconf

# macOS:
brew install pcsc-lite pkg-config
```

## Architecture Patterns

### Recommended CI Workflow Structure
```yaml
# .github/workflows/ci.yml
jobs:
  rust:
    runs-on: ubuntu-latest
    steps:
      - name: Install YubiKey dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y libpcsclite-dev pkgconf

      - name: clippy (trustedge-core with yubikey)
        run: cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings

      - name: build binaries with yubikey
        run: cargo build --package trustedge-core --bins --features yubikey

      - name: tests (trustedge-core with yubikey simulation only)
        run: cargo test --package trustedge-core --features yubikey --lib --locked --verbose
```

**Key changes from current workflow:**
1. Remove `id: yubikey-deps` output capture
2. Remove `if: steps.yubikey-deps.outputs.yubikey-available == 'true'` conditions
3. Remove `|| echo "..."` fallback from apt-get install (fail loudly)
4. Change test command from generic `cargo test` to `--lib` flag (runs only unit/simulation tests, skips integration tests with `#[ignore]`)

### Pattern 1: Unconditional Feature Compilation
**What:** Always compile with feature flags regardless of runtime dependency availability
**When to use:** When code is properly guarded with `#[cfg(feature = "...")]` and tests don't require runtime access to system libraries
**Example:**
```yaml
# Source: Cargo Features documentation + Clippy CI docs
# Unconditional build with optional feature

- name: clippy (trustedge-core with yubikey)
  run: cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings

# No `if` condition = always runs
# Works because:
# 1. Code is guarded with #[cfg(feature = "yubikey")]
# 2. libpcsclite-dev is installed (compile-time only)
# 3. Simulation tests don't require physical hardware or PCSC daemon
```

### Pattern 2: Separating Simulation Tests from Hardware Tests
**What:** Use `--lib` flag to run only unit tests (simulation), skip integration tests with `#[ignore]`
**When to use:** When you have hardware-dependent integration tests that can't run in CI
**Example:**
```bash
# Source: Phase 11 testing strategy + Rust Book #[ignore] patterns

# Run simulation tests only (in yubikey.rs #[cfg(test)] module)
cargo test --package trustedge-core --features yubikey --lib

# Output:
# test backends::yubikey::tests::test_parse_slot_valid_authentication ... ok
# test backends::yubikey::tests::test_capabilities_reports_hardware_backed ... ok
# ... (18 total simulation tests, 0 ignored)

# Integration tests NOT run:
# tests/yubikey_integration.rs - all marked with #[ignore = "requires physical YubiKey"]
```

### Pattern 3: Clippy Warnings as Errors
**What:** Use `-- -D warnings` flag to treat all warnings (rustc + clippy) as errors
**When to use:** CI enforcement of code quality (always in CI, optional locally)
**Example:**
```bash
# Source: Clippy documentation - CI Integration
cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings

# --all-targets: Check lib, bins, tests, examples, benches
# -- -D warnings: Pass rustc flag to deny all warnings
# Fails CI if ANY warning exists (clippy lints + compiler warnings)
```

### Anti-Patterns to Avoid
- **Conditional feature compilation in CI:** If `if` conditions skip feature builds, you can merge broken code. Always compile all features unconditionally.
- **Using `|| true` or `|| echo` in apt-get install:** Silences real failures. Let apt-get fail loudly if packages are missing.
- **Running hardware tests in CI:** Integration tests with `#[ignore]` should NOT be run with `--ignored` or `--include-ignored` in CI. Hardware tests require physical devices.
- **Using `if: always()`:** Breaks workflow cancellation. If you want unconditional execution, omit the `if` condition entirely.
- **Generic `cargo test` for feature-gated code:** Runs all tests including integration tests. Use `--lib` to run only unit/simulation tests.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Conditional CI steps based on dependency availability | Custom shell scripts with exit code checks | Remove `if` conditions, install dependencies unconditionally | libpcsclite-dev is in Ubuntu main repo, should always be available; conditional logic hides real failures |
| Feature powerset testing | Manual matrix of feature combinations | cargo-hack with `--feature-powerset` | Already in CI (Step 4), validates no feature interaction bugs |
| Test selection (simulation vs hardware) | Custom test runner or tags | `--lib` flag + `#[ignore]` attribute | Rust built-in pattern: `--lib` runs unit tests, `#[ignore]` gates hardware tests |
| Clippy configuration files | `.clippy.toml` with allowed warnings | Command-line `-- -D warnings` flag | Centralized in CI workflow, easier to audit, no local config drift |

**Key insight:** Cargo's feature system already handles conditional compilation correctly. The CI workflow should trust Cargo and Rust's `#[cfg]` attributes rather than adding external conditional logic with GitHub Actions `if` statements.

## Common Pitfalls

### Pitfall 1: Assuming Compilation Requires Runtime Dependencies
**What goes wrong:** Developers think `--features yubikey` won't compile if PCSC daemon (pcscd) isn't running
**Why it happens:** Confusion between compile-time dependencies (headers/libraries for linking) and runtime dependencies (daemons, hardware)
**How to avoid:** Understand that `libpcsclite-dev` provides compile-time headers. Compilation succeeds without pcscd running or YubiKey inserted. Only actual hardware operations fail at runtime.
**Warning signs:** CI logs show "PCSC daemon not running" during compilation (this is fine), but builds still succeed

### Pitfall 2: Running Hardware Tests in CI
**What goes wrong:** CI hangs or fails with "no readers available" when running tests marked `#[ignore]`
**Why it happens:** Using `cargo test --ignored` or `--include-ignored` runs hardware integration tests that require physical YubiKey
**How to avoid:** Always use `cargo test --lib` for YubiKey feature in CI. Integration tests (`tests/yubikey_integration.rs`) should only be run manually by developers with hardware.
**Warning signs:** CI timeout after 10+ minutes on test step, logs show "Failed to create YubiKey backend. Is a YubiKey inserted?"

### Pitfall 3: Conditional Install with Silent Fallback
**What goes wrong:** YubiKey CI steps are skipped silently when `apt-get install` fails, and broken code gets merged
**Why it happens:** Current workflow uses `|| echo "..."` fallback and `if` conditions based on install success
**How to avoid:** Remove fallback from apt-get install. Let it fail loudly if package is missing (indicates real infrastructure problem).
**Warning signs:** CI shows "yubikey-available=false" in logs, but package should be in Ubuntu's main repository

### Pitfall 4: Clippy Passing Locally, Failing in CI
**What goes wrong:** Developer runs `cargo clippy --features yubikey` locally without warnings, but CI fails
**Why it happens:** Different clippy versions, different feature combinations, or different Rust versions
**How to avoid:** Use `dtolnay/rust-toolchain@stable` to match CI's Rust version locally. Run `./scripts/ci-check.sh` before pushing.
**Warning signs:** CI logs show clippy warnings that don't appear locally (check `rustc --version` and `clippy-driver --version`)

### Pitfall 5: Testing Non-Hardware Code with #[ignore]
**What goes wrong:** Simulation tests (slot parsing, capability reporting) are marked `#[ignore]`, so they never run in CI
**Why it happens:** Misunderstanding of what requires hardware vs. what can be simulated
**How to avoid:** Phase 11 established the pattern: simulation tests in `yubikey.rs` under `#[cfg(test)]` (no `#[ignore]`), hardware tests in `tests/yubikey_integration.rs` (with `#[ignore]`).
**Warning signs:** Test coverage reports show 0% coverage for YubiKey backend logic that doesn't need hardware

## Code Examples

Verified patterns from official sources:

### Unconditional YubiKey CI Steps (Official Pattern)
```yaml
# Source: GitHub Actions Conditional Execution docs + Rust CI best practices
# File: .github/workflows/ci.yml

- name: Install YubiKey dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y libpcsclite-dev pkgconf

# No if condition = always runs
# No || fallback = fails loudly if packages missing

- name: clippy (trustedge-core with yubikey)
  run: cargo clippy --package trustedge-core --all-targets --features yubikey -- -D warnings

# No if condition = always runs
# Compilation succeeds even if pcscd daemon not running

- name: tests (trustedge-core with yubikey simulation only)
  run: cargo test --package trustedge-core --features yubikey --lib --locked --verbose

# --lib flag = run unit tests only (simulation tests in yubikey.rs)
# Does NOT run tests/yubikey_integration.rs (those are integration tests with #[ignore])
```

### Running Simulation Tests Locally
```bash
# Source: Phase 11-01 SUMMARY.md verification results
# Run YubiKey simulation tests (18 tests, no hardware required)

cargo test -p trustedge-core --features yubikey --lib -- yubikey::tests

# Output:
# running 18 tests
# test backends::yubikey::tests::test_parse_slot_valid_authentication ... ok
# test backends::yubikey::tests::test_capabilities_reports_hardware_backed ... ok
# ... (16 more tests)
# test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured
```

### Running Hardware Tests Locally (Manual Only)
```bash
# Source: tests/yubikey_integration.rs header comment
# Run hardware integration tests (requires physical YubiKey)

# Check hardware availability first
ykman piv info

# Run hardware tests only
cargo test --features yubikey --test yubikey_integration -- --ignored

# CAUTION: Some tests consume PIN retry attempts
# Set custom PIN if not using default (123456)
YUBIKEY_TEST_PIN=654321 cargo test --features yubikey --test yubikey_integration -- --ignored
```

### Validating CI Changes Locally
```bash
# Source: scripts/ci-check.sh
# Local CI validation before pushing

./scripts/ci-check.sh --clean

# Runs all CI checks including:
# - Copyright headers
# - cargo fmt
# - cargo clippy (workspace, audio, yubikey)
# - cargo test (workspace, audio, yubikey simulation)
# - cargo-hack feature powerset
# - WASM build

# Step 5 output (YubiKey clippy):
# ■ Step 5: Clippy (trustedge-core with yubikey)
#   ✔ clippy yubikey

# Step 9 output (YubiKey tests):
# ■ Step 9: Tests (trustedge-core with yubikey)
#   ✔ yubikey tests
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Conditional YubiKey CI with apt-get fallback | Unconditional compilation and testing (Phase 12) | 2026-02-11 | Prevents merging broken YubiKey code; libpcsclite is stable in Ubuntu main |
| Run all tests with `--features yubikey` (including hardware tests) | Separate simulation (`--lib`) from hardware (`--ignored`) tests | Phase 11 (2026-02-11) | CI runs 18 simulation tests without hardware; developers run 9 hardware tests manually |
| Install system deps with `|| echo "..."` fallback | Fail loudly if apt-get install fails | Phase 12 | Surface real infrastructure problems instead of silent skips |
| Generic `cargo test` for feature testing | `cargo test --lib` for simulation, `--test name -- --ignored` for hardware | Phase 11 | Precise test selection, no CI timeouts from hardware tests |
| RUSTFLAGS env var for warnings | `-- -D warnings` flag on clippy command | Established pattern | Avoids cache invalidation issues, cleaner command-line |

**Deprecated/outdated:**
- **Conditional `if: steps.yubikey-deps.outputs.yubikey-available == 'true'`:** Phase 12 removes this. YubiKey steps always run unconditionally.
- **`apt-get install ... || echo "yubikey-available=false"`:** Phase 12 removes fallback. Let installation fail loudly.
- **Running hardware tests in CI:** Phase 11 established that hardware tests are `#[ignore]` and NOT run in CI. Only simulation tests run.

## Open Questions

1. **Should CI run `cargo-hack` on YubiKey feature combinations?**
   - What we know: cargo-hack is already in CI (Step 6), runs feature powerset on trustedge-core
   - What's unclear: Whether this already covers `--features yubikey` in powerset testing
   - Recommendation: Verify cargo-hack output includes yubikey feature. If not, it's already covered by existing `--feature-powerset` step.

2. **Should local CI script (scripts/ci-check.sh) also become unconditional?**
   - What we know: Script uses `pkg-config --exists libpcsclite` to conditionally run YubiKey steps
   - What's unclear: Whether developers should be forced to install libpcsclite-dev locally
   - Recommendation: Keep local script conditional (developer convenience), make GitHub Actions unconditional (enforcement). Developers without PCSC can skip YubiKey steps locally, but CI always validates.

3. **Should we add a CI job that explicitly tests compilation WITHOUT yubikey feature?**
   - What we know: Workspace tests run with `--no-default-features` (Step 7), which excludes yubikey
   - What's unclear: Whether this already validates no accidental yubikey dependencies leak
   - Recommendation: Current coverage is sufficient. Workspace tests already validate no-feature builds.

## Sources

### Primary (HIGH confidence)
- [Cargo Features Documentation](https://doc.rust-lang.org/cargo/reference/features.html) - Optional dependencies, feature flags, cfg attributes
- [Rust Book: Controlling How Tests Are Run](https://doc.rust-lang.org/book/ch11-02-running-tests.html) - `#[ignore]` attribute, `--ignored` flag, test execution patterns
- [Clippy Usage Documentation](https://doc.rust-lang.org/clippy/usage.html) - `--all-targets` flag, `-D warnings` flag, CI integration
- Phase 11-01 SUMMARY.md - 18 simulation tests in yubikey.rs, verification results
- Phase 11-02 SUMMARY.md - 9 hardware tests in yubikey_integration.rs, all marked `#[ignore]`
- `.github/workflows/ci.yml` (current) - Lines 63-106: Conditional YubiKey steps with `if` checks
- `scripts/ci-check.sh` (current) - Lines 105-115: Conditional YubiKey clippy with pkg-config check

### Secondary (MEDIUM confidence)
- [GitHub Actions: Using Conditions to Control Job Execution](https://docs.github.com/en/actions/writing-workflows/choosing-when-your-workflow-runs/using-conditions-to-control-job-execution) - Conditional execution, when to use `if`, when to omit
- [RunsOn: Conditions for GitHub Actions](https://runs-on.com/github-actions/conditions/) - `if` expressions, step outputs, `always()` function behavior
- [Clippy GitHub Actions Documentation](https://doc.rust-lang.org/nightly/clippy/continuous_integration/github_actions.html) - dtolnay/rust-toolchain, clippy pre-installed on GitHub runners
- [GitHub Issue #3499: installdependencies.sh returns success when there are errors](https://github.com/actions/runner/issues/3499) - apt-get can succeed even when packages aren't installed (why fallback is dangerous)

### Tertiary (LOW confidence)
- [Sling Academy: Controlling Test Execution with #[ignore] in Rust](https://www.slingacademy.com/article/controlling-test-execution-with-ignore-in-rust/) - Basic #[ignore] usage patterns
- [Rust Testing Mastery LinkedIn Article](https://www.linkedin.com/pulse/rust-testing-mastery-from-basics-best-practices-luis-soares-m-sc-) - General testing best practices (not YubiKey-specific)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - GitHub Actions, dtolnay/rust-toolchain, cargo-hack all verified in current CI workflow
- Architecture: HIGH - Patterns verified from official Cargo/Clippy docs + Phase 11 established test structure
- Pitfalls: HIGH - Directly observed in current CI workflow and Phase 11 implementation

**Research date:** 2026-02-11
**Valid until:** 2026-03-11 (30 days - stable domain: GitHub Actions syntax and Cargo features are mature and change slowly)
