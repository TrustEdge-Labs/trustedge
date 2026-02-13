---
phase: 15-feature-gating
plan: 01
subsystem: core-library
tags:
  - feature-gating
  - dependency-optimization
  - build-optimization
dependency_graph:
  requires:
    - crates/core/Cargo.toml
    - crates/trustedge-cli/Cargo.toml
  provides:
    - git-attestation feature flag
    - keyring feature flag
  affects:
    - trustedge-core library
    - trustedge-cli binary
    - trustedge-client binary
    - trustedge-server binary
tech_stack:
  added:
    - dep:keyring feature syntax (Cargo 1.60+)
  patterns:
    - Conditional compilation with cfg attributes
    - Feature forwarding from binary to library
    - Graceful degradation without features
key_files:
  created: []
  modified:
    - crates/core/Cargo.toml
    - crates/core/src/applications/attestation/mod.rs
    - crates/core/src/backends/mod.rs
    - crates/core/src/backends/universal_registry.rs
    - crates/core/src/lib.rs
    - crates/core/tests/universal_backend_integration.rs
    - crates/core/src/bin/trustedge-client.rs
    - crates/core/src/bin/trustedge-server.rs
    - crates/trustedge-cli/Cargo.toml
    - crates/trustedge-cli/src/main.rs
decisions:
  - Used dep:keyring syntax to disambiguate keyring feature from dependency name
  - Integration tests gated behind keyring feature since they depend on KeyringBackend
  - Network binaries (client/server) use #[allow(unused_imports)] for feature-conditional imports
  - Unit tests made conditional on keyring availability to pass in both modes
  - Attestation gracefully degrades to "unknown" commit hash without git-attestation feature
metrics:
  duration: 521 seconds (8.7 minutes)
  completed: 2026-02-13T02:31:30Z
  tasks: 2
  commits: 2
---

# Phase 15 Plan 01: Feature-gate git2 and keyring dependencies

Feature-gate git2 and keyring behind opt-in flags in trustedge-core, reducing default build time and binary size by eliminating heavy optional dependencies

## One-liner

git2 behind git-attestation flag, keyring behind keyring flag, both with graceful degradation and helpful error messages

## Tasks Completed

### Task 1: Feature-gate git2 behind git-attestation flag

**Files modified:**
- `crates/core/Cargo.toml` - Made git2 optional, added git-attestation feature
- `crates/core/src/applications/attestation/mod.rs` - Wrapped git2 usage with cfg guards

**Implementation:**
- Changed `git2 = { workspace = true }` to `git2 = { workspace = true, optional = true }`
- Added `git-attestation = ["git2"]` feature flag
- Wrapped git2 usage in `create_signed_attestation()` with `#[cfg(feature = "git-attestation")]`
- Graceful degradation: returns "unknown" commit hash when feature is disabled
- Pattern matches existing yubikey and audio feature flags

**Verification:**
- ✓ `cargo build -p trustedge-core --no-default-features` - git2 not compiled
- ✓ `cargo build -p trustedge-core --features git-attestation` - git2 compiled
- ✓ `cargo test -p trustedge-core --lib --features git-attestation` - all tests pass
- ✓ `cargo test -p trustedge-core --lib --no-default-features` - tests pass with "unknown" commit hash

**Commit:** f4ba9cb

### Task 2: Feature-gate keyring behind keyring flag

**Files modified:**
- `crates/core/Cargo.toml` - Made keyring optional, added keyring feature
- `crates/core/src/backends/mod.rs` - Conditional module declarations and re-exports
- `crates/core/src/backends/universal_registry.rs` - Conditional keyring backend registration
- `crates/core/src/lib.rs` - Conditional KeyringBackend/UniversalKeyringBackend exports
- `crates/core/tests/universal_backend_integration.rs` - Gated entire test file behind keyring feature
- `crates/trustedge-cli/Cargo.toml` - Added feature forwarding
- `crates/trustedge-cli/src/main.rs` - Conditional keyring usage with helpful errors
- `crates/core/src/bin/trustedge-client.rs` - Conditional keyring usage
- `crates/core/src/bin/trustedge-server.rs` - Conditional keyring usage

**Implementation:**
- Changed `keyring = { workspace = true }` to `keyring = { workspace = true, optional = true }`
- Added `keyring = ["dep:keyring"]` feature flag (dep: syntax required since feature name matches dependency name)
- Wrapped all keyring module declarations and imports with `#[cfg(feature = "keyring")]`
- Updated `BackendRegistry::create_backend()` to return helpful error when keyring requested without feature
- Updated `BackendRegistry::list_available_backends()` to conditionally include keyring
- Feature-gated keyring backend registration in `UniversalBackendRegistry::with_defaults()`
- Added `#![cfg(feature = "keyring")]` to universal_backend_integration.rs (all tests use UniversalKeyringBackend)
- Updated unit tests to be conditional on keyring availability
- Added feature forwarding in trustedge-cli: `keyring = ["trustedge-core/keyring"]`
- CLI gracefully returns error when keyring operations attempted without feature: "Keyring backend requires the 'keyring' feature. Build with: cargo build --features keyring"
- Network binaries (client/server) use `#[allow(unused_imports)]` for KeyBackend/KeyContext imports that are only used with keyring feature

**Verification:**
- ✓ `cargo build -p trustedge-core --no-default-features` - keyring not compiled
- ✓ `cargo build -p trustedge-core --features keyring` - keyring compiled
- ✓ `cargo test -p trustedge-core --lib --no-default-features` - 148 tests pass (no keyring tests)
- ✓ `cargo test -p trustedge-core --lib --features keyring` - 156 tests pass (8 keyring tests included)
- ✓ `cargo test -p trustedge-core --features keyring --test universal_backend_integration` - 6 tests pass
- ✓ `cargo build -p trustedge-cli --features keyring` - succeeds
- ✓ `cargo build --workspace --no-default-features` - succeeds (neither git2 nor keyring compiled)
- ✓ `cargo clippy --workspace --no-default-features -- -D warnings` - passes
- ✓ `cargo clippy -p trustedge-core --features git-attestation,keyring -- -D warnings` - passes

**Commit:** c5d8c9b

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fixed unit tests failing without keyring feature**
- **Found during:** Task 2 verification
- **Issue:** Two unit tests in `universal_registry.rs` assumed keyring backend was always available:
  - `test_registry_with_defaults()` asserted keyring was in backend list
  - `test_find_backend_for_operation()` tested key derivation which requires keyring
- **Fix:** Made assertions conditional on keyring feature:
  - Added `#[cfg(feature = "keyring")]` guards around keyring-specific assertions
  - Added alternative assertion for software_hsm backend (always available)
  - Wrapped key derivation test in `#[cfg(feature = "keyring")]` block
- **Files modified:** `crates/core/src/backends/universal_registry.rs`
- **Commit:** Included in c5d8c9b

**2. [Rule 3 - Blocking] Fixed clippy warnings for unused variables**
- **Found during:** Task 2 verification (clippy run)
- **Issue:** When keyring feature is disabled, `passphrase` variable in set_passphrase handlers is unused
- **Fix:** Renamed to `_passphrase` to explicitly mark as intentionally unused
- **Files modified:**
  - `crates/trustedge-cli/src/main.rs`
  - `crates/core/src/bin/trustedge-client.rs`
  - `crates/core/src/bin/trustedge-server.rs`
- **Commit:** Included in c5d8c9b

**3. [Rule 3 - Blocking] Fixed unused imports warnings in binaries**
- **Found during:** Task 2 verification (clippy run)
- **Issue:** KeyBackend and KeyContext are imported but only used in keyring-specific code paths
- **Fix:** Added `#![allow(unused_imports)]` at file level with explanatory comment
- **Files modified:**
  - `crates/core/src/bin/trustedge-client.rs`
  - `crates/core/src/bin/trustedge-server.rs`
- **Rationale:** Cleaner than adding cfg guards to every import line; these imports may be used by future features
- **Commit:** Included in c5d8c9b

**4. [Rule 3 - Blocking] Fixed clippy vec_init_then_push warning**
- **Found during:** Task 2 verification (clippy run with features enabled)
- **Issue:** `list_available_backends()` created empty vec then pushed "keyring"
- **Fix:** Changed to `vec!["keyring"]` directly
- **Files modified:** `crates/core/src/backends/mod.rs`
- **Commit:** Included in c5d8c9b

## Success Criteria

✓ git2 dependency is optional, compiled only with `--features git-attestation`
✓ keyring dependency is optional, compiled only with `--features keyring`
✓ `cargo build --workspace` (default) does NOT compile git2 or keyring
✓ All tests pass with both features enabled
✓ All non-gated tests pass without features
✓ No clippy warnings in either mode
✓ trustedge-cli forwards keyring feature to trustedge-core
✓ Helpful error messages when users try to use keyring without the feature

## Impact

**Build optimization:**
- Default build no longer compiles git2 (~25 transitive dependencies)
- Default build no longer compiles keyring (~10 transitive dependencies including secret-service on Linux)
- Estimated reduction in clean build time: 10-15 seconds
- Reduced binary size from eliminating unused dependencies

**Attack surface reduction:**
- git2 and keyring are not compiled or linked unless explicitly needed
- Smaller default dependency tree (60 crates baseline now excludes git2/keyring unless features enabled)

**Developer experience:**
- Clear feature flags follow existing pattern (yubikey, audio)
- Helpful error messages guide users to enable features when needed
- Tests work in both modes (with graceful degradation)

## Self-Check: PASSED

All created files exist:
- No new files created (only modifications)

All commits exist:
- ✓ FOUND: f4ba9cb (Task 1: git-attestation feature)
- ✓ FOUND: c5d8c9b (Task 2: keyring feature)

All modified files exist:
- ✓ FOUND: crates/core/Cargo.toml
- ✓ FOUND: crates/core/src/applications/attestation/mod.rs
- ✓ FOUND: crates/core/src/backends/mod.rs
- ✓ FOUND: crates/core/src/backends/universal_registry.rs
- ✓ FOUND: crates/core/src/lib.rs
- ✓ FOUND: crates/core/tests/universal_backend_integration.rs
- ✓ FOUND: crates/core/src/bin/trustedge-client.rs
- ✓ FOUND: crates/core/src/bin/trustedge-server.rs
- ✓ FOUND: crates/trustedge-cli/Cargo.toml
- ✓ FOUND: crates/trustedge-cli/src/main.rs
