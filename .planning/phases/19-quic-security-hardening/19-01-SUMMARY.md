---
phase: 19-quic-security-hardening
plan: 01
subsystem: transport/quic
tags:
  - security
  - tls
  - quic
  - feature-flags
dependency_graph:
  requires:
    - rustls 0.23
    - quinn 0.11
  provides:
    - Secure QUIC TLS by default
    - Development-only insecure-tls feature flag
  affects:
    - QuicTransport::connect()
    - QuicTransport::create_client_endpoint()
tech_stack:
  added:
    - webpki-roots 0.26 (Mozilla root certificates)
  patterns:
    - Feature-gated conditional compilation for security bypass
    - Fail-secure by default with opt-in insecure path
key_files:
  created: []
  modified:
    - crates/core/Cargo.toml (webpki-roots dep, insecure-tls feature)
    - crates/core/src/transport/quic.rs (secure TLS default, feature-gated SkipServerVerification)
    - .github/workflows/ci.yml (insecure-tls CI steps)
    - scripts/ci-check.sh (insecure-tls local CI steps)
    - Cargo.lock (lockfile update for webpki-roots)
decisions:
  - Used webpki-roots instead of rustls-native-certs for consistent cross-platform behavior
  - Made insecure-tls a feature flag instead of environment variable for compile-time enforcement
  - Gated SkipServerVerification struct behind cfg(feature) so it literally doesn't exist in default builds
  - Added rustls crypto provider initialization in tests to fix runtime panics
metrics:
  duration_minutes: 6
  completed_date: 2026-02-14
  tasks_completed: 2
  files_modified: 5
  commits: 2
---

# Phase 19 Plan 01: Secure QUIC TLS by Default Summary

**One-liner:** QUIC transport now uses proper TLS certificate verification via Mozilla root store by default, with insecure certificate skipping gated behind development-only insecure-tls feature flag

## Overview

Closed critical security vulnerability where QUIC client unconditionally used `SkipServerVerification`, accepting any TLS certificate without validation. Refactored to use webpki_roots (Mozilla's trusted root certificates) by default and gated the insecure path behind `cfg(feature = "insecure-tls")` for development use only.

**Security impact:** MITM attacks against QUIC connections are now prevented by default. Developers must explicitly opt in to insecure mode via `--features insecure-tls`.

## Tasks Completed

### Task 1: Add insecure-tls feature flag and refactor QUIC TLS to secure-by-default
**Commit:** d5019f2

**Changes:**
- Added `webpki-roots = "0.26"` dependency for Mozilla root certificate store
- Added `insecure-tls = []` feature flag under "Development: Security bypass flags" section
- Refactored `create_client_endpoint()` to call new `build_client_tls_config()` helper
- Implemented `build_client_tls_config()` with two paths:
  - **Default (`#[cfg(not(feature = "insecure-tls"))]`):** Uses `RootCertStore` populated with `webpki_roots::TLS_SERVER_ROOTS` for proper certificate verification
  - **Feature-gated (`#[cfg(feature = "insecure-tls")]`):** Uses `SkipServerVerification` for local development
- Gated `SkipServerVerification` struct and impl behind `#[cfg(feature = "insecure-tls")]`
- Added comprehensive doc comments warning about insecure-tls being development-only
- Made `rustls::RootCertStore` import conditional to avoid unused warning with feature enabled

**Verification:**
- `cargo build -p trustedge-core --no-default-features` ✔ (secure default)
- `cargo build -p trustedge-core --features insecure-tls` ✔ (insecure dev path)
- `cargo clippy` passes with zero warnings on both paths
- `cargo test -p trustedge-core --no-default-features` passes (all existing tests)

**Files modified:**
- `crates/core/Cargo.toml`
- `crates/core/src/transport/quic.rs`
- `Cargo.lock`

### Task 2: Add CI validation for insecure-tls feature and add compile-time test
**Commit:** 109fd67

**Changes:**
- Added `test_default_build_uses_secure_tls()` to verify endpoint creation succeeds with secure TLS (no feature flag)
- Added `test_insecure_tls_feature_available()` gated behind `#[cfg(feature = "insecure-tls")]` to verify insecure path when opted in
- Both tests initialize rustls crypto provider (`rustls::crypto::aws_lc_rs::default_provider().install_default()`) to prevent runtime panics
- Added GitHub Actions CI steps (after keyring):
  - `clippy (trustedge-core with insecure-tls)`
  - `tests (trustedge-core with insecure-tls)`
- Added local CI script steps:
  - Step 9: Clippy (insecure-tls)
  - Step 16: Tests (insecure-tls)
- Renumbered subsequent local CI steps from 10-21 to maintain sequence

**Verification:**
- `cargo test -p trustedge-core test_default_build_uses_secure_tls --no-default-features` ✔
- `cargo test -p trustedge-core test_insecure_tls_feature_available --features insecure-tls` ✔
- `.github/workflows/ci.yml` contains 4 references to insecure-tls (clippy name, clippy command, test name, test command)
- `scripts/ci-check.sh` contains 10 references to insecure-tls (steps, comments, commands)

**Files modified:**
- `crates/core/src/transport/quic.rs`
- `.github/workflows/ci.yml`
- `scripts/ci-check.sh`

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed rustls crypto provider initialization in tests**
- **Found during:** Task 2 test execution
- **Issue:** Tests panicked with "Could not automatically determine the process-level CryptoProvider from Rustls crate features. Call CryptoProvider::install_default() before this point."
- **Fix:** Added `rustls::crypto::aws_lc_rs::default_provider().install_default()` call in both test functions before creating QUIC endpoint
- **Files modified:** `crates/core/src/transport/quic.rs` (tests module)
- **Commit:** 109fd67
- **Rationale:** Blocking issue preventing test execution. Rustls 0.23 requires explicit crypto provider initialization. This is standard boilerplate for rustls tests.

**2. [Rule 3 - Blocking] Made RootCertStore import conditional**
- **Found during:** Task 1 compilation with insecure-tls feature
- **Issue:** Unused import warning when insecure-tls feature enabled (RootCertStore only used in default path)
- **Fix:** Moved `use rustls::RootCertStore;` behind `#[cfg(not(feature = "insecure-tls"))]`
- **Files modified:** `crates/core/src/transport/quic.rs`
- **Commit:** d5019f2
- **Rationale:** Clippy warnings are CI-blocking. Conditional imports match conditional usage pattern.

## Success Criteria Met

- ✔ **QUIC-01:** QUIC client uses proper TLS certificate verification by default (webpki_roots trust store)
- ✔ **QUIC-02:** Insecure TLS skip is gated behind insecure-tls feature flag
- ✔ CI validates both default and insecure-tls builds on every PR
- ✔ Developer documentation (doc comments) warns about insecure-tls being development-only
- ✔ All existing tests continue to pass
- ✔ cargo-hack feature powerset still passes (insecure-tls is a simple feature with no deps)

## Technical Notes

### Why webpki-roots over rustls-native-certs?

`webpki-roots` provides Mozilla's curated root certificate store, which is:
- Consistent across platforms (same trust anchors on Linux/macOS/Windows)
- Well-maintained by rustls project
- Smaller than native cert stores (fewer false positives)
- Deterministic (same behavior in CI and production)

`rustls-native-certs` uses OS cert stores, which vary by platform and configuration.

### HardwareBackedVerifier unchanged

The `HardwareBackedVerifier` struct (used by `connect_with_hardware_verification`) was left unchanged. It uses a custom trust model (explicitly provided trusted certificates) which is a legitimate verification approach, not insecure skipping. It's not the default connection path.

### Feature flag vs environment variable

Made `insecure-tls` a Cargo feature instead of runtime environment variable to enforce security at compile time. A binary built without the feature flag cannot skip TLS verification, even if compromised at runtime.

## Self-Check: PASSED

**Created files:** None (all modifications)

**Modified files exist:**
- ✔ `/home/john/vault/projects/github.com/trustedge/crates/core/Cargo.toml`
- ✔ `/home/john/vault/projects/github.com/trustedge/crates/core/src/transport/quic.rs`
- ✔ `/home/john/vault/projects/github.com/trustedge/Cargo.lock`
- ✔ `/home/john/vault/projects/github.com/trustedge/.github/workflows/ci.yml`
- ✔ `/home/john/vault/projects/github.com/trustedge/scripts/ci-check.sh`

**Commits exist:**
- ✔ d5019f2: feat(19-01): secure QUIC TLS by default with feature-gated insecure bypass
- ✔ 109fd67: test(19-01): add CI validation for insecure-tls feature

**Functional verification:**
```bash
# Default secure build
cargo build -p trustedge-core --no-default-features
# ✔ Compiles successfully, uses webpki_roots

# Insecure dev build
cargo build -p trustedge-core --features insecure-tls
# ✔ Compiles successfully, SkipServerVerification available

# Verify conditional compilation
grep 'cfg.*feature.*insecure.tls' crates/core/src/transport/quic.rs
# ✔ Shows 5 cfg gates (import, build_client_tls_config paths, struct, impl)

# Verify secure default
grep 'webpki_roots' crates/core/src/transport/quic.rs
# ✔ Shows usage in default TLS config path

# Tests pass
cargo test -p trustedge-core --no-default-features
cargo test -p trustedge-core --features insecure-tls
# ✔ All tests pass in both modes
```

All checks passed.
