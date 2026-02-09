# Pitfalls Research: Rust Workspace Consolidation

**Research Date:** 2026-02-09
**Project:** TrustEdge workspace consolidation (10 crates → monolith core + thin shells)
**Context:** Subsequent milestone, crypto library with hardware integration, 150+ tests, WASM targets

## Overview

This document catalogs common mistakes when consolidating Rust workspaces, specifically when merging multiple library crates into a monolithic core. Each pitfall includes warning signs, prevention strategies, and phase mapping for the consolidation roadmap.

---

## 1. Feature Flag Explosion and Combinatorial Complexity

### Description

When merging crates with different optional features, teams often create too many feature flags without considering the combinatorial explosion. With `n` feature flags, you have `2^n` possible configurations, most of which are never tested.

**TrustEdge Risk:** The project has `yubikey` (hardware), `audio` (platform-specific), and will inherit features from receipts/attestation/WASM. Merging blindly could create 16+ feature combinations.

### Warning Signs

- New features like `receipt-ops`, `attestation-ops`, `archive-ops` created per-subsystem
- Features that are always used together but remain separate
- CI only tests `--all-features` and `--no-default-features`, not individual feature combinations
- Confusion about which features are required for which functionality
- Documentation drift between feature descriptions and actual functionality

### Prevention Strategy

1. **Feature Categorization:** Group features into orthogonal dimensions:
   - **Backend:** `software-hsm` (default), `keyring`, `yubikey` (mutually exclusive capabilities)
   - **Platform:** `audio` (opt-in), `wasm` (target-specific)
   - **Validation:** `strict-validation` (opt-in for extra checks)

2. **Feature Unification:** Merge always-together features:
   - Don't create `receipts` + `receipt-signatures` + `receipt-chains` — just `receipts`
   - Attestation and receipts likely share envelope crypto — don't duplicate feature gates

3. **Test Matrix:** In CI, test critical combinations:
   ```bash
   cargo test --no-default-features
   cargo test --features yubikey
   cargo test --features audio
   cargo test --all-features
   ```

4. **Feature Documentation:** In `Cargo.toml`, document each feature's purpose:
   ```toml
   [features]
   default = ["software-hsm"]

   # Backend capabilities (choose one or more)
   software-hsm = []     # Pure Rust crypto (always available)
   keyring = ["dep:keyring"]  # OS keychain integration
   yubikey = ["pkcs11", "dep:yubikey", "x509-cert"]  # Hardware security keys

   # Platform features (opt-in)
   audio = ["cpal"]      # Live audio capture (requires ALSA/CoreAudio/WASAPI)
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Audit all existing features across all crates
- **Phase 2 (Design):** Design unified feature set before merging code
- **Phase 3 (Implementation):** Merge features incrementally, testing each combination

---

## 2. Dependency Constraint Conflicts (WASM vs Native)

### Description

WASM targets have strict dependency restrictions (no `std::fs`, no threading, no native IO). Merging WASM-compatible crates with native-only crates often results in either breaking WASM or creating "WASM stub" implementations that silently fail.

**TrustEdge Risk:** `trst-core` is intentionally minimal for WASM. Merging into `trustedge-core` (which has `tokio`, `quinn`, `cpal`, `pkcs11`) will break WASM unless carefully gated.

### Warning Signs

- Build errors when targeting `wasm32-unknown-unknown`
- Dependencies like `tokio`, `quinn`, `cpal`, `pkcs11` without `target` gates
- WASM tests passing but browser integration failing
- Conditional compilation becoming dominant (`#[cfg(not(target_arch = "wasm32"))]` everywhere)
- Different API surfaces for WASM vs native (same function name, different signatures)

### Prevention Strategy

1. **Dependency Auditing:** Before merge, categorize every dependency:
   ```
   WASM-safe: serde, serde_json, ed25519-dalek, aes-gcm, blake3
   Native-only: tokio, quinn, cpal, pkcs11, keyring
   Needs-gating: std::fs, std::net, std::thread
   ```

2. **Target-Specific Dependencies:**
   ```toml
   [target.'cfg(not(target_arch = "wasm32"))'.dependencies]
   tokio = { version = "1.0", features = ["full"] }
   quinn = "0.11"
   cpal = { version = "0.15", optional = true }
   pkcs11 = { version = "0.5", optional = true }

   [target.'cfg(target_arch = "wasm32")'.dependencies]
   wasm-bindgen = "0.2"
   getrandom = { version = "0.2", features = ["js"] }
   ```

3. **Core Abstraction Layer:** Keep a WASM-safe core module:
   ```
   trustedge-core/src/
   ├── core/           # WASM-safe: crypto, envelope, signing
   ├── platform/       # Native-only: network, audio, backends
   └── lib.rs          # Conditional exports
   ```

4. **Compile Tests:** Add to CI:
   ```bash
   cargo check --target wasm32-unknown-unknown -p trustedge-core
   cargo build -p trustedge-core --no-default-features  # Must work for WASM
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Audit dependencies for WASM compatibility
- **Phase 2 (Design):** Design module structure with WASM-safe core
- **Phase 3 (Implementation):** Move WASM-safe code first, then add platform gates

---

## 3. Test Namespace Collisions and Lost Coverage

### Description

When merging crates, integration tests with the same names overwrite each other. Teams also often delete "duplicate" tests without verifying they test different scenarios, leading to silent test coverage loss.

**TrustEdge Risk:** 150+ tests across 10 crates. High probability of naming collisions (e.g., `integration_tests.rs` in multiple crates). YubiKey tests require hardware, WASM tests require browser — easy to lose during merge.

### Warning Signs

- Test count after merge is less than sum of original tests
- Tests like `tests/integration_tests.rs` in multiple crates (collision risk)
- Tests passing locally but failing in crate-specific test runs
- Coverage dropping after merge
- Hardware-specific tests (YubiKey) silently disabled
- WASM-specific tests not running in CI

### Prevention Strategy

1. **Pre-Merge Test Inventory:**
   ```bash
   # Count tests before merge
   cargo test -p trustedge-core --lib --bins --tests -- --list | wc -l
   cargo test -p trustedge-receipts -- --list | wc -l
   cargo test -p trustedge-attestation -- --list | wc -l
   # Document: 101 + 23 + X = 150+ total
   ```

2. **Namespace Preservation:** When moving tests, preserve crate context:
   ```
   Before: crates/receipts/tests/integration_tests.rs
   After:  crates/core/tests/receipts_integration.rs
   ```

3. **Test Categorization:** Group by requirement:
   ```
   tests/
   ├── unit/               # No external deps
   ├── integration/        # Crate integration
   ├── hardware/           # YubiKey (manual)
   ├── wasm/              # Browser (wasm-bindgen-test)
   └── acceptance/        # End-to-end
   ```

4. **Test Validation Script:**
   ```bash
   #!/bin/bash
   # Ensure no test loss during merge
   BEFORE_COUNT=150
   AFTER_COUNT=$(cargo test --workspace -- --list | grep -c "test")
   if [ $AFTER_COUNT -lt $BEFORE_COUNT ]; then
     echo "ERROR: Lost tests! Before: $BEFORE_COUNT, After: $AFTER_COUNT"
     exit 1
   fi
   ```

5. **Hardware Test Gates:** Preserve hardware tests with clear markers:
   ```rust
   #[test]
   #[cfg(feature = "yubikey")]
   #[ignore] // Requires actual YubiKey hardware
   fn yubikey_real_signing_operations() { ... }
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Inventory all tests, document counts
- **Phase 2 (Design):** Plan test directory structure, namespace strategy
- **Phase 3 (Implementation):** Move tests with validation, run before/after comparison
- **Phase 4 (Validation):** Verify total test count, coverage, hardware/WASM tests preserved

---

## 4. API Surface Regression and Breaking Changes

### Description

Consolidation often inadvertently breaks public APIs. Common causes: changing module paths, making public items private, renaming types, or removing convenience re-exports. Downstream consumers (including your own CLI crates) break silently.

**TrustEdge Risk:** `trustedge-cli`, `trst-cli`, `trustedge-wasm`, `trst-wasm` all consume library crates. Merging could break these thin shells if API surface changes.

### Warning Signs

- Compilation errors in CLI crates after merging library code
- Need to add `pub use` re-exports everywhere to fix imports
- Module paths changing: `use trustedge_receipts::Receipt` → `use trustedge_core::receipts::Receipt`
- Types moving from root to nested modules without re-export
- Documentation examples breaking due to path changes
- Semver major version bump required after "just a reorganization"

### Prevention Strategy

1. **API Compatibility Layer:** Preserve old paths with re-exports:
   ```rust
   // trustedge-core/src/lib.rs

   // Core exports (new canonical location)
   pub mod envelope;
   pub mod receipts;
   pub mod attestation;

   // Backward compatibility re-exports (deprecated)
   #[deprecated(since = "0.3.0", note = "use trustedge_core::receipts instead")]
   pub use crate::receipts as receipt_system;
   ```

2. **Consumer Testing:** Test all downstream crates against the merged library:
   ```bash
   # After merging receipts into core
   cargo test -p trustedge-cli        # Should still compile
   cargo test -p trst-cli             # Should still compile
   cargo test -p trustedge-wasm       # Should still compile
   cargo test -p trst-wasm            # Should still compile
   ```

3. **API Inventory:** Before merge, document public API:
   ```bash
   cargo doc --no-deps -p trustedge-receipts
   cargo doc --no-deps -p trustedge-attestation
   # Review what's pub vs pub(crate)
   ```

4. **Staged Deprecation:**
   - Phase 1: Merge code, preserve old paths with re-exports
   - Phase 2: Add deprecation warnings
   - Phase 3: Update downstream consumers
   - Phase 4: Remove old paths in next major version

5. **Semver Checking:**
   ```bash
   cargo install cargo-semver-checks
   cargo semver-checks check-release
   # Flags breaking changes
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Document current public API surface
- **Phase 2 (Design):** Plan re-export strategy for backward compatibility
- **Phase 3 (Implementation):** Merge with re-exports, test all consumers
- **Phase 4 (Validation):** Verify no semver breakage, all consumers compile

---

## 5. Circular Dependency Creation

### Description

Merging crates that previously depended on each other can create circular module dependencies. Example: `receipts` uses `core::envelope`, but `core::manifest` uses `receipts::ReceiptChain` for metadata — deadlock.

**TrustEdge Risk:** `attestation` and `receipts` both use `core::envelope`. If attestation types are used in core, you create a cycle.

### Warning Signs

- Compilation errors: "cyclic dependency detected"
- Need to move types to a separate `types` module to break cycles
- Traits in one module, impls in another, neither can import the other
- Cfg-gated features creating cycles (feature A needs B, B needs A when enabled)
- Module hierarchy becomes deeply nested to avoid cycles

### Prevention Strategy

1. **Dependency Graphing:** Before merge, map all inter-crate dependencies:
   ```
   core ← receipts ← attestation
   core ← trst-core
   core ← wasm
   ```
   No arrows should form cycles after merge.

2. **Extract Common Types:** Create a `types` or `primitives` module:
   ```
   core/src/
   ├── primitives/    # Shared types, no dependencies on other modules
   │   ├── envelope.rs
   │   ├── receipt.rs
   │   └── attestation.rs
   ├── operations/    # Operations using primitives
   │   ├── receipt_ops.rs
   │   └── attestation_ops.rs
   └── lib.rs
   ```

3. **Trait-Based Abstraction:** Use traits to break concrete dependencies:
   ```rust
   // Instead of: pub fn process(r: Receipt) -> Attestation
   // Use:
   pub trait Verifiable {
       fn verify(&self) -> Result<bool>;
   }

   pub fn process(v: &impl Verifiable) -> Result<()>
   ```

4. **Dependency Layering:** Enforce strict module hierarchy:
   ```
   Layer 3: operations (uses types + crypto)
   Layer 2: crypto (uses types)
   Layer 1: types (no internal dependencies)
   ```

5. **Incremental Merging:** Merge bottom-up:
   - Step 1: Merge `trst-core` (types only, no dependencies)
   - Step 2: Merge `receipts` (depends on core envelope)
   - Step 3: Merge `attestation` (depends on receipts)

### Phase Mapping

- **Phase 1 (Discovery):** Map current dependency graph
- **Phase 2 (Design):** Design layered module structure
- **Phase 3 (Implementation):** Merge in dependency order (bottom-up)

---

## 6. Build Time Explosion from Monolithic Compilation

### Description

Consolidating crates can dramatically increase build times. Small changes to core types force recompilation of the entire monolith, whereas separate crates only recompile affected parts.

**TrustEdge Risk:** `trustedge-core` already has 101 tests. Adding receipts (23 tests), attestation, trst-core will create a large compilation unit. CI time may double.

### Warning Signs

- `cargo build` time increasing 2x-5x after merge
- Incremental compilation not helping (touching `envelope.rs` rebuilds everything)
- CI timeouts on test runs
- Developer frustration with long edit-compile-test cycles
- Parallel compilation not effective (single large crate bottleneck)

### Prevention Strategy

1. **Module Isolation:** Keep modules loosely coupled:
   ```rust
   // BAD: Everything depends on everything
   pub mod receipts {
       use crate::attestation::*;
       use crate::envelope::*;
       use crate::transport::*;  // Why does receipts need network transport?
   }

   // GOOD: Minimal cross-module dependencies
   pub mod receipts {
       use crate::primitives::{Envelope, Signature};
       // Only import what's needed
   }
   ```

2. **Conditional Compilation:** Use features to reduce default compile scope:
   ```toml
   [features]
   default = ["receipts", "attestation"]  # Common features
   full = ["default", "yubikey", "audio", "network"]  # Everything
   minimal = []  # Bare crypto primitives only
   ```

3. **Parallel Test Execution:**
   ```bash
   cargo test --workspace --jobs 8  # Parallel test execution
   ```

4. **Build Caching:** In CI, use `sccache` or `cargo-chef`:
   ```dockerfile
   # Cache dependencies separately from code
   COPY Cargo.toml Cargo.lock ./
   RUN cargo chef prepare
   RUN cargo chef cook --release
   COPY . .
   RUN cargo build --release
   ```

5. **Benchmarking:** Measure before/after:
   ```bash
   # Before merge
   time cargo build --workspace

   # After merge
   time cargo build -p trustedge-core

   # If >2x slower, reconsider granularity
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Measure current build times
- **Phase 3 (Implementation):** Monitor build time after each merge
- **Phase 4 (Validation):** Ensure CI time < 2x baseline, or optimize

---

## 7. Error Type Consolidation Gone Wrong

### Description

Each crate typically has its own error type. Consolidating these into a single error enum often creates either a massive enum with 100+ variants or overly generic errors that lose context.

**TrustEdge Risk:** `core` has `thiserror`, `receipts` likely has receipt-specific errors, `attestation` has attestation errors. Merging into one giant error enum loses clarity.

### Warning Signs

- Error enum with 50+ variants: `TrustEdgeError::{EnvelopeError, ReceiptError, AttestationError, ...}`
- Error messages losing specificity: "Operation failed" instead of "Receipt signature invalid"
- Downstream users doing excessive pattern matching: `match err { Variant1 | Variant2 | ... => }`
- Error conversions everywhere: `ReceiptError → CoreError → Result`
- Documentation for errors becoming confusing (which variants apply to which operations?)

### Prevention Strategy

1. **Error Hierarchy:** Keep domain-specific error types, use trait objects for propagation:
   ```rust
   // Domain-specific errors
   pub mod receipts {
       #[derive(thiserror::Error, Debug)]
       pub enum ReceiptError {
           #[error("Invalid signature: {0}")]
           InvalidSignature(String),
           #[error("Chain continuity broken at index {0}")]
           BrokenChain(usize),
       }
   }

   pub mod attestation {
       #[derive(thiserror::Error, Debug)]
       pub enum AttestationError {
           #[error("Provenance verification failed: {0}")]
           ProvenanceFailed(String),
       }
   }

   // Top-level uses anyhow for flexibility
   pub type Result<T> = anyhow::Result<T>;
   ```

2. **Context Preservation:** Use `anyhow::Context`:
   ```rust
   use anyhow::Context;

   pub fn verify_receipt(r: &Receipt) -> Result<()> {
       r.verify()
           .context("Receipt verification failed")
           .with_context(|| format!("Receipt ID: {}", r.id()))?;
       Ok(())
   }
   ```

3. **Public vs Internal Errors:**
   ```rust
   // Public API: Simple error type
   #[derive(thiserror::Error, Debug)]
   pub enum TrustEdgeError {
       #[error("Cryptographic operation failed")]
       CryptoError,
       #[error("Invalid input: {0}")]
       InvalidInput(String),
   }

   // Internal: Rich error types (not exposed)
   mod internal {
       #[derive(thiserror::Error, Debug)]
       enum DetailedError {
           #[error("AES-GCM decryption failed: {0}")]
           AesGcmError(#[from] aes_gcm::Error),
           // ... 50 more variants
       }
   }
   ```

4. **Error Testing:** Verify error context is preserved:
   ```rust
   #[test]
   fn error_messages_are_specific() {
       let result = verify_invalid_receipt();
       let err = result.unwrap_err();
       assert!(err.to_string().contains("signature"));  // Not generic "failed"
   }
   ```

### Phase Mapping

- **Phase 2 (Design):** Design error hierarchy before merging code
- **Phase 3 (Implementation):** Keep domain-specific errors, use `anyhow` for propagation
- **Phase 4 (Validation):** Test error messages for specificity

---

## 8. Benchmark and Performance Regression

### Description

Merging crates can accidentally introduce performance regressions. Common causes: additional trait bounds adding overhead, increased dependency chains slowing compilation, or optimizations disabled due to feature interactions.

**TrustEdge Risk:** `core` has benchmarks for crypto operations. Merging receipts/attestation could slow down envelope operations if not careful about trait abstractions.

### Warning Signs

- Benchmark results degrading after merge (even by 5-10%)
- New trait bounds with runtime dispatch where static dispatch existed before
- Feature flags affecting optimization (e.g., `--features receipts` slows down core crypto)
- Release builds slower than before merge
- Unexpected heap allocations in hot paths

### Prevention Strategy

1. **Benchmark Baseline:** Run benchmarks before merge:
   ```bash
   cargo bench --bench crypto_benchmarks > baseline.txt
   ```

2. **Post-Merge Validation:**
   ```bash
   cargo bench --bench crypto_benchmarks > post-merge.txt
   diff baseline.txt post-merge.txt
   # Flag any regression >5%
   ```

3. **Zero-Cost Abstractions:** Verify traits compile to static dispatch:
   ```rust
   // BAD: Runtime dispatch
   pub fn process(backend: &dyn Backend) -> Result<()> {
       backend.operation()  // Virtual call
   }

   // GOOD: Static dispatch
   pub fn process<B: Backend>(backend: &B) -> Result<()> {
       backend.operation()  // Inlined
   }
   ```

4. **Inline Annotations:** Preserve performance-critical inlining:
   ```rust
   #[inline]
   pub fn hot_path_function() { ... }

   #[inline(always)]  // Force inline even in debug builds
   pub fn critical_crypto_primitive() { ... }
   ```

5. **Feature-Gated Benchmarks:**
   ```toml
   [[bench]]
   name = "core_crypto"
   harness = false
   required-features = []  # Always run

   [[bench]]
   name = "yubikey_ops"
   harness = false
   required-features = ["yubikey"]  # Only with hardware
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Run baseline benchmarks
- **Phase 3 (Implementation):** Re-run benchmarks after each major merge
- **Phase 4 (Validation):** Verify no regression >5%

---

## 9. Documentation Fragmentation and Stale Examples

### Description

After consolidation, documentation becomes fragmented. README examples reference old crate names, API docs point to removed modules, and examples in `examples/` break due to path changes.

**TrustEdge Risk:** `examples/cam.video/` likely references multiple crates. CLAUDE.md documents testing commands for individual crates. All needs updating.

### Warning Signs

- Examples not compiling: `cargo run --example receipt_demo` fails
- README showing `use trustedge_receipts::Receipt` (old path)
- API documentation showing "404 Not Found" for linked modules
- Integration guides referring to deleted crates
- `CLAUDE.md` build commands failing (e.g., `cargo test -p trustedge-receipts`)

### Prevention Strategy

1. **Example Validation:** Add to CI:
   ```bash
   # Ensure all examples compile
   for example in examples/*/; do
     cargo check --manifest-path "$example/Cargo.toml" || exit 1
   done
   ```

2. **Documentation Update Checklist:**
   - [ ] Update CLAUDE.md build/test commands
   - [ ] Update README.md import examples
   - [ ] Update API doc links
   - [ ] Update examples/ crate references
   - [ ] Update inline doc examples (`cargo test --doc`)

3. **Doc Tests:** Ensure documentation examples compile:
   ```rust
   /// # Example
   /// ```
   /// use trustedge_core::receipts::Receipt;
   /// let r = Receipt::new();
   /// ```
   pub struct Receipt { ... }
   ```
   Run with: `cargo test --doc`

4. **Path Aliases:** In documentation, use modern paths but mention old ones:
   ```rust
   /// # Migrating from `trustedge-receipts`
   ///
   /// If you previously used:
   /// ```ignore
   /// use trustedge_receipts::Receipt;
   /// ```
   ///
   /// Update to:
   /// ```
   /// use trustedge_core::receipts::Receipt;
   /// ```
   pub struct Receipt { ... }
   ```

### Phase Mapping

- **Phase 3 (Implementation):** Update docs incrementally with each merge
- **Phase 4 (Validation):** Run `cargo test --doc`, validate all examples

---

## 10. Version Number Confusion and Semver Violations

### Description

After merging, teams struggle with version numbering. Should the merged crate use the highest version? Should it bump major version due to path changes? Mishandling leads to semver violations.

**TrustEdge Risk:** `core` is 0.2.0, `receipts` is 0.2.0, `attestation` is 0.2.0. After merge, is core still 0.2.0? 0.3.0? 1.0.0?

### Warning Signs

- Version number decreasing (merged crate 0.2.0 but dependency was 0.3.0)
- Breaking changes but only minor version bump
- Downstream consumers getting surprise breakage on "patch" update
- Confusion about what version to publish
- Changelog not reflecting actual changes

### Prevention Strategy

1. **Semver Decision Matrix:**
   ```
   API backward compatible? → Patch bump (0.2.0 → 0.2.1)
   New features, no breaks?  → Minor bump (0.2.0 → 0.3.0)
   Breaking changes?         → Major bump (0.2.0 → 1.0.0) or (1.2.0 → 2.0.0)
   ```

2. **Pre-1.0 Rules:** While 0.x.y, breaking changes allowed in minor bumps:
   ```
   Current: 0.2.0
   Consolidation with breaking path changes: 0.3.0
   Reasoning: Pre-1.0, minor bump signals potential breakage
   ```

3. **Changelog Discipline:**
   ```markdown
   # Changelog

   ## [0.3.0] - 2026-02-XX

   ### Changed (BREAKING)
   - Consolidated `trustedge-receipts` into `trustedge-core`
   - Module paths changed: `trustedge_receipts::*` → `trustedge_core::receipts::*`

   ### Added
   - Re-exports for backward compatibility (deprecated)

   ### Migration Guide
   - Update imports: `use trustedge_core::receipts::Receipt;`
   ```

4. **Deprecation Period:** For pre-1.0, allow one release with deprecations:
   ```
   Version 0.3.0: Add re-exports with deprecation warnings
   Version 0.4.0: Remove re-exports (breaking)
   ```

5. **Consumer Notification:**
   ```toml
   # For downstream consumers, be explicit
   [dependencies]
   trustedge-core = "0.3"  # Includes receipts, attestation
   # trustedge-receipts = "0.2"  # REMOVED - now part of trustedge-core
   ```

### Phase Mapping

- **Phase 2 (Design):** Decide on versioning strategy
- **Phase 4 (Validation):** Write changelog, tag release, notify consumers

---

## 11. Hardware Integration Regression (YubiKey)

### Description

Hardware integration is fragile. Consolidation can break hardware backends due to: dependency version conflicts, feature flag misconfigurations, or accidental removal of hardware-specific initialization code.

**TrustEdge Risk:** YubiKey backend (3236 lines) is already complex with PKCS#11, PIV, X.509. Merging other crypto code could introduce conflicts in crypto library versions or initialization order.

### Warning Signs

- YubiKey tests failing after merge (if you have hardware to test)
- PKCS#11 library loading failures
- Version conflicts: `pkcs11 0.5` vs newer version pulled by merged crate
- YubiKey feature flag not enabling all necessary dependencies
- Hardware initialization order changed (PKCS#11 must init before use)
- X.509 certificate generation broken due to `der` crate version mismatch

### Prevention Strategy

1. **Dependency Pinning:** Lock hardware-critical dependencies:
   ```toml
   [dependencies]
   pkcs11 = "=0.5.0"  # Exact version, not "0.5"
   yubikey = "=0.7.0"
   x509-cert = "=0.2.0"
   ```

2. **Hardware Test Preservation:**
   ```bash
   # Before merge, document YubiKey test count
   cargo test -p trustedge-core --features yubikey --test yubikey_integration -- --list

   # After merge, verify same tests exist
   cargo test -p trustedge-core --features yubikey --test yubikey_integration -- --list
   ```

3. **Feature Dependency Completeness:**
   ```toml
   [features]
   yubikey = [
     "pkcs11",
     "dep:yubikey",
     "x509-cert",
     "der",
     "spki",
     "signature",
     # Don't forget any dependency!
   ]
   ```

4. **Initialization Order Documentation:**
   ```rust
   /// # YubiKey Backend Initialization
   ///
   /// CRITICAL: Must be called before any PKCS#11 operations.
   ///
   /// 1. Load PKCS#11 library (OpenSC or YubiKey Manager)
   /// 2. Initialize PKCS#11 context
   /// 3. Open session to slot 0
   /// 4. Login with PIN
   ///
   /// Incorrect order will cause undefined behavior.
   pub fn initialize_yubikey() -> Result<YubiKeyBackend> { ... }
   ```

5. **Manual Hardware Test Protocol:**
   ```markdown
   # YubiKey Test Protocol (Manual)

   Before merge:
   1. Insert YubiKey
   2. Run: cargo test --features yubikey --test yubikey_real_operations
   3. Document pass/fail

   After merge:
   1. Insert same YubiKey
   2. Run: cargo test --features yubikey --test yubikey_real_operations
   3. Verify same results
   ```

### Phase Mapping

- **Phase 1 (Discovery):** Document current YubiKey test status
- **Phase 2 (Design):** Pin hardware dependencies, plan preservation
- **Phase 3 (Implementation):** Merge carefully, test after each change
- **Phase 4 (Validation):** Manual hardware test verification

---

## 12. Macro and Derive Propagation Failures

### Description

Procedural macros and derive macros often break during consolidation. Causes: macro crate dependencies not propagated, derive features not enabled, or macro-generated code referencing old module paths.

**TrustEdge Risk:** `serde` derives are everywhere. If any crate uses custom derives or conditional serialization, merging could break serialization.

### Warning Signs

- Compilation errors: "cannot find derive macro `Serialize`"
- Features like `serde/derive` not enabled in merged crate
- Macro-generated code referencing wrong module paths
- Custom derives (e.g., `#[derive(CustomTrait)]`) not working
- Serialization format changing unexpectedly after merge

### Prevention Strategy

1. **Feature Propagation:**
   ```toml
   [dependencies]
   serde = { workspace = true, features = ["derive"] }  # Don't forget "derive"!
   ```

2. **Serialization Testing:**
   ```rust
   #[test]
   fn serialization_format_unchanged() {
       let receipt = Receipt::new();
       let json = serde_json::to_string(&receipt).unwrap();

       // Ensure format didn't change after merge
       assert!(json.contains("\"version\":"));
       assert!(json.contains("\"signature\":"));
   }
   ```

3. **Custom Derive Auditing:**
   ```bash
   # Find all custom derives
   rg '#\[derive\(' | grep -v 'Serialize\|Deserialize\|Debug\|Clone'
   # Ensure each custom derive still works after merge
   ```

4. **Serde Compatibility:**
   ```toml
   [dependencies]
   serde = { workspace = true, features = ["derive"] }
   serde_json = { workspace = true }
   serde_bytes = { workspace = true }  # If using binary serialization
   bincode = { workspace = true }      # If using bincode
   ```

### Phase Mapping

- **Phase 3 (Implementation):** Verify derives compile after each module move
- **Phase 4 (Validation):** Run serialization tests

---

## Summary: Critical Phases for Pitfall Prevention

| Phase | Key Pitfalls to Address |
|-------|------------------------|
| **Phase 1: Discovery** | Feature inventory (#1), dependency audit (#2), test counting (#3), API documentation (#4), benchmark baseline (#8), hardware test status (#11) |
| **Phase 2: Design** | Feature unification (#1), WASM-safe module design (#2), test namespace strategy (#3), re-export plan (#4), error hierarchy (#7), versioning strategy (#10) |
| **Phase 3: Implementation** | Feature testing (#1), target-specific deps (#2), test preservation (#3), consumer testing (#4), incremental merging (#5), doc updates (#9), derive verification (#12) |
| **Phase 4: Validation** | CI matrix (#1), WASM compilation (#2), test count verification (#3), semver check (#4), build time measurement (#6), benchmark regression (#8), changelog (#10), hardware tests (#11) |

---

## TrustEdge-Specific Recommendations

Based on the project context, prioritize these pitfalls:

1. **High Priority:**
   - #2 (WASM vs Native) — `trst-core` must remain WASM-compatible
   - #3 (Test Loss) — 150+ tests must be preserved
   - #11 (YubiKey) — Hardware integration is the differentiator

2. **Medium Priority:**
   - #1 (Feature Explosion) — Already have `yubikey`, `audio` features
   - #4 (API Breaking) — Thin shells depend on core API
   - #5 (Circular Deps) — Receipts/attestation both use core

3. **Monitor:**
   - #6 (Build Time) — 10 crates is not huge, but monitor
   - #7 (Errors) — Each subsystem likely has its own error types
   - #8 (Performance) — Crypto benchmarks exist, preserve them

---

**Next Steps:**
1. Use this document during Phase 1 (Discovery) to audit for warning signs
2. Reference prevention strategies during Phase 2 (Design)
3. Create validation checklists from Phase Mapping sections
4. Add pitfall checks to CI/test scripts

---

*Research completed: 2026-02-09*
*Sources: Rust workspace consolidation patterns, semver.org, cargo book, WASM targets guide, hardware integration best practices*
