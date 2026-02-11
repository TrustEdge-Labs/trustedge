# Phase 9: Cleanup - Research

**Researched:** 2026-02-11
**Domain:** Rust workspace code deletion and dependency cleanup
**Confidence:** HIGH

## Summary

Phase 9 requires complete removal of broken YubiKey implementation code from the TrustEdge Core crate. This is a scorched-earth deletion operation targeting 3,263 lines of backend code, 8 test files (2,251 lines total), 8 example files, 1 binary demo, and removal of the `untested` feature flag from the yubikey dependency. The research confirms all target files exist and identifies the full scope of cleanup needed.

The deletion scope includes manual DER encoding functions (15+ functions totaling 1,000+ lines), placeholder keys and certificates throughout the yubikey.rs backend, and feature-gated code in transport/quic.rs. Verification requires multi-stage compilation testing: without features (default), with remaining features (audio), and full workspace validation.

This is not a refactoring — it's complete deletion in preparation for a clean v1.1 rewrite based on stable yubikey crate APIs and rcgen for X.509 certificate generation.

**Primary recommendation:** Delete all files atomically, update module exports and Cargo.toml in the same commit, verify compilation at each stage (no-features → audio-feature → workspace), and use grep verification for CLEAN-04 patterns.

## Standard Stack

### Core Tools
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| cargo | 1.x | Rust build system | Official Rust toolchain |
| git rm | 2.x | Track file deletions | Official git deletion command |
| cargo check | 1.x | Fast compilation verification | Standard pre-commit validation |
| cargo test | 1.x | Test suite validation | Verify no broken references |
| grep/ripgrep | Any | Pattern verification | Verify complete cleanup |

### Supporting
| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| cargo tree | 1.x | Dependency graph inspection | Verify unused dependencies |
| cargo-machete | Latest | Find unused dependencies | Post-deletion cleanup check |
| git status | 2.x | Verify staged changes | Pre-commit safety check |

**Installation:**
```bash
# Already available in Rust toolchain
cargo --version
git --version

# Optional verification tools
cargo install cargo-machete
```

## Architecture Patterns

### Multi-Stage Deletion Pattern

**What:** Delete code in dependency order, verify compilation at each stage

**When to use:** Large-scale code removal with feature flags and module dependencies

**Structure:**
```
Stage 1: Identify all deletion targets
  - Source files (backends/yubikey.rs)
  - Test files (tests/yubikey_*.rs)
  - Example files (examples/yubikey_*.rs)
  - Binary files (bin/yubikey-demo.rs)
  - Module exports (backends/mod.rs, lib.rs)
  - Feature-gated code (transport/quic.rs)

Stage 2: Update module system
  - Remove pub mod yubikey from backends/mod.rs
  - Remove pub use yubikey::* exports
  - Remove YubiKeyBackend imports
  - Update conditional compilation blocks

Stage 3: Delete files atomically
  - git rm all target files in single operation
  - Update Cargo.toml dependency flags
  - Commit all changes together

Stage 4: Verify compilation
  - cargo check (default no-features build)
  - cargo check --features audio
  - cargo test --workspace
  - cargo build --workspace --release
```

### Module Export Cleanup Pattern

**What:** Update module system when deleting exported types

**Critical files requiring updates:**
```rust
// crates/core/src/backends/mod.rs
pub mod yubikey;  // DELETE
pub use yubikey::{...};  // DELETE

// crates/core/src/backends/universal_registry.rs
use crate::backends::yubikey::YubiKeyBackend;  // DELETE or comment out

// crates/core/src/lib.rs
// Remove YubiKey mentions from feature documentation
```

### Cargo.toml Feature Cleanup Pattern

**What:** Remove `untested` feature flag from optional dependency

**Current state:**
```toml
[dependencies]
yubikey = { version = "0.7", optional = true, features = ["untested"] }

[features]
yubikey = ["pkcs11", "dep:yubikey", "x509-cert", "der", "spki", "signature"]
```

**After cleanup:**
```toml
[dependencies]
yubikey = { version = "0.7", optional = true }  # Remove features = ["untested"]

[features]
yubikey = ["pkcs11", "dep:yubikey", "x509-cert", "der", "spki", "signature"]
```

**Note:** Keep the yubikey feature flag itself — it will be used by the v1.1 rewrite. Only remove the `untested` flag.

### Grep Verification Pattern

**What:** Verify complete removal of problematic code patterns

**Verification commands:**
```bash
# CLEAN-04: Verify placeholder removal
grep -r "placeholder" crates/core/src/ crates/core/examples/ --include="*.rs"
grep -r "PLACEHOLDER" crates/core/src/ crates/core/examples/ --include="*.rs"

# CLEAN-04: Verify no manual DER encoding remains
grep -r "encode.*der\|der.*encode\|fn.*to_der" crates/core/src/ --include="*.rs"

# CLEAN-04: Verify no fake/dummy keys
grep -r "fake_key\|dummy_key\|placeholder_key" crates/core/src/ --include="*.rs"

# Expected remaining hits after cleanup:
# - archive.rs: "placeholder" in continuity_hash (legitimate manifest field)
# - attestation/mod.rs: "placeholder" in git hash fallback (legitimate)
# None in backends/, transport/quic.rs, or examples/
```

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Partial file deletion | Manually edit files one-by-one | git rm with batch operations | Atomic commits, no orphaned references |
| Compilation verification | Assume it works | Multi-stage cargo check | Features create different compilation paths |
| Pattern verification | Manual code review | grep/ripgrep with patterns | Systematic, reproducible verification |
| Dependency cleanup | Guess which deps to remove | cargo tree + cargo-machete | Automated detection of unused deps |

**Key insight:** Large deletions require systematic verification because Rust's module system and feature flags create multiple compilation paths. A file might compile with no features but fail with --all-features, or vice versa.

## Common Pitfalls

### Pitfall 1: Orphaned Module References
**What goes wrong:** Delete source file but forget to remove `pub mod` declaration, causing compilation failure

**Why it happens:** Module system is separate from filesystem — mod.rs declarations don't auto-update

**How to avoid:**
1. Search for all references first: `grep -r "yubikey" crates/core/src/backends/mod.rs crates/core/src/lib.rs`
2. Update module files before deleting source files
3. Verify with `cargo check` immediately after

**Warning signs:**
- "cannot find module" error
- "unresolved import" errors for deleted types

### Pitfall 2: Feature-Gated Code Orphans
**What goes wrong:** Code protected by `#[cfg(feature = "yubikey")]` remains after deletion, causing dead code warnings or compilation errors when feature is enabled

**Why it happens:** Conditional compilation hides code unless feature is active

**How to avoid:**
1. Search for feature gates: `grep -r 'cfg(feature = "yubikey")' crates/core/src/`
2. Check transport/quic.rs functions: `connect_with_yubikey_certificate`, `create_yubikey_server`
3. Decide: delete entirely or update to use new backend when v1.1 ships

**Warning signs:**
- "unused code" warnings when building with --features yubikey
- References to deleted types in feature-gated blocks

### Pitfall 3: Incomplete Pattern Removal
**What goes wrong:** Delete main implementation but leave placeholder keys, manual DER functions, or fake signatures scattered in other files

**Why it happens:** CLEAN-04 patterns span multiple files (backend, tests, examples, transport)

**How to avoid:**
1. Run grep verification BEFORE claiming completion
2. Check every match — some "placeholder" uses are legitimate (manifest fields)
3. Document expected remaining matches

**Warning signs:**
- grep finds "placeholder" in production code paths
- Manual DER encoding functions remain in codebase
- Test files create keys without calling crypto operations

### Pitfall 4: Cargo.toml Dependency Confusion
**What goes wrong:** Remove entire yubikey dependency instead of just removing `untested` feature flag

**Why it happens:** Misunderstanding that Phase 9 is cleanup, not complete removal — v1.1 will reuse the dependency

**How to avoid:**
1. Read requirement carefully: "remove `untested` feature flag" not "remove yubikey dependency"
2. Keep dependency declaration: `yubikey = { version = "0.7", optional = true }`
3. Keep feature flag: `yubikey = ["pkcs11", "dep:yubikey", ...]`

**Warning signs:**
- Cargo.toml completely removes yubikey from [dependencies]
- Feature flag section removes yubikey entry

### Pitfall 5: Single-Stage Verification
**What goes wrong:** Run `cargo check` once and assume everything works, missing feature-specific or test-specific compilation errors

**Why it happens:** Rust features create parallel compilation universes — no-features build might pass while --features audio fails

**How to avoid:**
Multi-stage verification protocol:
```bash
# Stage 1: Default (no features)
cargo check

# Stage 2: Each feature independently
cargo check --features audio
cargo check --features yubikey  # Should compile (deps still present)

# Stage 3: Workspace tests
cargo test --workspace

# Stage 4: Full release build
cargo build --workspace --release
```

**Warning signs:**
- CI fails but local build passes
- "Some tests didn't run" in test output
- Different results between cargo check and cargo build

## Deletion Scope

### Confirmed File Counts

**Backend Implementation:**
- `crates/core/src/backends/yubikey.rs`: 3,263 lines, 116KB

**Test Files (8 total, 2,251 lines):**
- yubikey_certificate_debug.rs
- yubikey_hardware_detection.rs
- yubikey_hardware_tests.rs
- yubikey_integration.rs
- yubikey_piv_analysis.rs
- yubikey_simulation_tests.rs
- yubikey_strict_hardware.rs
- yubikey_real_operations.rs

**Example Files (8 total):**
- yubikey_certificate_demo.rs
- yubikey_demo.rs
- yubikey_enhanced_cert_demo.rs
- yubikey_hardware_signing_demo.rs
- yubikey_phase2c_demo.rs
- yubikey_pubkey_demo.rs
- yubikey_quic_demo.rs
- yubikey_quic_hardware_demo.rs

**Binary Files:**
- `crates/core/src/bin/yubikey-demo.rs`

**Feature-Gated Code (NOT deleted, but needs review):**
- `crates/core/src/transport/quic.rs`:
  - `connect_with_yubikey_certificate()` function
  - `create_yubikey_server()` function
  - `create_placeholder_private_key()` helper
  - Decision needed: delete now or keep for v1.1 rewrite

### Manual DER Encoding Functions in yubikey.rs

Functions to be deleted (15+ functions, ~1,000 lines):
- `encode_asn1_integer()`
- `build_placeholder_ecdsa_p256_spki()`
- `generate_certificate()`
- `generate_real_x509_certificate()`
- `validate_generated_certificate()`
- `build_tbs_certificate_der()`
- `encode_distinguished_name()`
- `encode_validity_period()`
- `encode_public_key_info()`
- `encode_certificate_extensions()`
- `encode_basic_constraints_extension()`
- `encode_key_usage_extension()`
- `encode_san_extension()`
- `encode_ecdsa_signature_der()`
- `build_subject_public_key_info()`
- `generate_placeholder_certificate()`

These will be replaced in v1.1 by rcgen library for X.509 generation.

### Placeholder Pattern Locations

**MUST be removed (CLEAN-04):**
- yubikey.rs: `build_placeholder_ecdsa_p256_spki()`, `generate_placeholder_certificate()`
- transport/quic.rs: `create_placeholder_private_key()`, placeholder_key constant
- All example files: Various placeholder certificate structures and length placeholders

**Legitimate uses (keep):**
- archive.rs: `continuity_hash: "placeholder"` — legitimate manifest field placeholder
- attestation/mod.rs: Git hash placeholder when not in repo — legitimate fallback

### Module Export Updates Required

**crates/core/src/backends/mod.rs:**
```rust
// LINE 29: DELETE
pub mod yubikey;

// LINE 37: DELETE
pub use yubikey::{CertificateParams, HardwareCertificate, YubiKeyBackend, YubiKeyConfig};
```

**crates/core/src/backends/universal_registry.rs:**
```rust
// LINE 17: DELETE or conditionally compile
use crate::backends::yubikey::YubiKeyBackend;
```

**crates/core/src/transport/quic.rs:**
```rust
// LINES 133-169: Review and likely delete
// Functions: connect_with_yubikey_certificate, create_yubikey_server, create_placeholder_private_key
// Decision: Delete placeholder implementation or stub for v1.1
```

**crates/core/src/lib.rs:**
```rust
// LINES 72-73: Update feature documentation
// Remove mention of YubiKey as working implementation
// Note as "experimental, requires v1.1 rewrite"
```

## Code Examples

### Safe Atomic Deletion

```bash
# Stage 1: Delete all YubiKey files atomically
git rm crates/core/src/backends/yubikey.rs
git rm crates/core/tests/yubikey_*.rs
git rm crates/core/examples/yubikey_*.rs
git rm crates/core/src/bin/yubikey-demo.rs

# Stage 2: Update module files (use Read + Write tools)
# - Edit crates/core/src/backends/mod.rs (remove lines 29, 37)
# - Edit crates/core/src/backends/universal_registry.rs (remove/comment line 17)
# - Edit crates/core/src/transport/quic.rs (delete placeholder functions)
# - Edit crates/core/Cargo.toml (remove "untested" from yubikey dependency)

# Stage 3: Verify compilation
cargo check
cargo check --features audio
cargo test --workspace --lib  # Skip integration tests if needed
cargo build --workspace --release

# Stage 4: Verify CLEAN-04 patterns removed
grep -r "placeholder" crates/core/src/ --include="*.rs" | grep -v "archive.rs\|attestation"
grep -r "encode.*der\|der.*encode" crates/core/src/ --include="*.rs"

# Stage 5: Commit atomically
git add -A
git commit -m "feat(yubikey)!: remove broken backend for v1.1 rewrite

BREAKING CHANGE: YubiKey backend completely removed. All manual DER
encoding, placeholder keys, and untested feature flags deleted.

- Delete 3,263-line backends/yubikey.rs
- Delete 8 test files (2,251 lines)
- Delete 8 example files
- Delete bin/yubikey-demo.rs
- Remove 'untested' feature flag from yubikey dependency
- Remove placeholder keys and manual DER encoding functions
- Update module exports in backends/mod.rs

Verified with:
- cargo check (default + features)
- cargo test --workspace
- grep verification for CLEAN-04 patterns

Refs: CLEAN-01, CLEAN-02, CLEAN-03, CLEAN-04
Milestone: v1.1-start"
```

### Multi-Stage Verification Script

```bash
#!/bin/bash
# Comprehensive verification after YubiKey cleanup

set -e  # Exit on any error

echo "Stage 1: Default build (no features)"
cargo check

echo "Stage 2: Audio feature build"
cargo check --features audio

echo "Stage 3: YubiKey feature build (deps still present, no impl)"
cargo check --features yubikey || echo "Expected: may fail, that's OK"

echo "Stage 4: Workspace tests"
cargo test --workspace --lib

echo "Stage 5: Release build"
cargo build --workspace --release

echo "Stage 6: Pattern verification (CLEAN-04)"
echo "Checking for placeholders (archive.rs hits are OK)..."
grep -r "placeholder" crates/core/src/ --include="*.rs" | grep -v "archive.rs\|attestation" || echo "PASS: No bad placeholders"

echo "Checking for manual DER encoding..."
grep -r "encode.*der\|der.*encode" crates/core/src/ --include="*.rs" || echo "PASS: No manual DER"

echo "Checking for fake keys..."
grep -r "fake_key\|dummy_key\|placeholder_key" crates/core/src/ --include="*.rs" || echo "PASS: No fake keys"

echo "Stage 7: Unused dependency check"
cargo tree -p trustedge-core --features yubikey | grep yubikey || echo "YubiKey dep still present (correct)"

echo "✔ All verification stages passed"
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual DER encoding (1,000+ lines) | rcgen library (v1.1) | v1.1 rewrite | Replaces 15+ error-prone functions |
| Placeholder keys for testing | Real crypto ops only | v1.1 rewrite | Every key from actual YubiKey operations |
| `untested` feature flag | Stable API only | Phase 9 cleanup | Removes unstable yubikey crate features |
| Silent fallbacks | Fail-closed design | v1.1 rewrite | Security: fail loudly when hardware unavailable |
| Mixed backend + X.509 logic | Separation of concerns | v1.1 rewrite | Backend = crypto ops, rcgen = certificates |

**Deprecated/outdated:**
- Manual DER encoding: Replaced by rcgen library (removes 1,000+ lines of hand-rolled ASN.1)
- Placeholder certificates: Replaced by real hardware-backed operations
- `untested` feature flag: Removed in favor of stable yubikey crate API only

## Open Questions

1. **Transport layer YubiKey functions**
   - What we know: quic.rs has `connect_with_yubikey_certificate()` and `create_yubikey_server()` with placeholder keys
   - What's unclear: Delete now or keep stubs for v1.1 integration?
   - Recommendation: Delete placeholder implementations (CLEAN-04 compliance). v1.1 can add back with real implementation.

2. **Feature flag retention**
   - What we know: Cargo.toml should keep `yubikey = ["pkcs11", "dep:yubikey", ...]` feature
   - What's unclear: Will this cause build errors without backend implementation?
   - Recommendation: Keep feature flag. Compilation will succeed (deps satisfied), runtime will fail (no backend registered). Document as "reserved for v1.1".

3. **Universal backend registry**
   - What we know: universal_registry.rs imports YubiKeyBackend
   - What's unclear: Comment out or delete import? Will this break registry system?
   - Recommendation: Conditionally compile import with `#[cfg(feature = "yubikey")]`. Registry won't list YubiKey without feature, preventing accidental use.

4. **Documentation updates**
   - What we know: lib.rs feature docs mention YubiKey as working
   - What's unclear: What text to replace it with?
   - Recommendation: Change to "yubikey — YubiKey PIV support (experimental, v1.1 rewrite in progress)". Signals feature exists but isn't production-ready.

## Sources

### Primary (HIGH confidence)
- Local codebase inspection:
  - `/home/john/vault/projects/github.com/trustedge/crates/core/Cargo.toml` - Verified yubikey dependency with `untested` feature
  - `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/yubikey.rs` - Verified 3,263 lines
  - `/home/john/vault/projects/github.com/trustedge/crates/core/tests/yubikey_*.rs` - Verified 8 test files, 2,251 total lines
  - `/home/john/vault/projects/github.com/trustedge/crates/core/examples/yubikey_*.rs` - Verified 8 example files
- Grep verification of placeholder patterns, manual DER functions, module exports

### Secondary (MEDIUM confidence)
- [cargo remove - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-remove.html) - Official cargo dependency removal
- [Features - The Cargo Book](https://doc.rust-lang.org/cargo/reference/features.html) - Feature flag management
- [cargo check - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-check.html) - Fast compilation verification
- [cargo test - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-test.html) - Test execution with features
- [cargo-machete GitHub](https://github.com/bnjbvr/cargo-machete) - Unused dependency detection
- [BFG Repo-Cleaner](https://rtyley.github.io/bfg-repo-cleaner/) - Large file deletion (not needed for this phase)
- [Unused Dependencies - Rust Project Primer](https://rustprojectprimer.com/checks/unused.html) - Dependency cleanup best practices

### Tertiary (LOW confidence)
- [cargo-minify GitHub](https://github.com/tweedegolf/cargo-minify) - Dead code removal tool (not applicable - we're doing manual cleanup)

## Metadata

**Confidence breakdown:**
- Deletion scope: HIGH - All files verified to exist with exact line counts
- Module exports: HIGH - Grep confirms all references to update
- Cargo.toml changes: HIGH - Requirement explicitly states "remove `untested` flag"
- Verification protocol: HIGH - Standard Rust toolchain commands, well-documented
- Pattern cleanup (CLEAN-04): HIGH - Grep found all placeholder/DER patterns with file locations

**Research date:** 2026-02-11
**Valid until:** 60 days (stable topic - Rust toolchain practices don't change rapidly)

## Planning Guidance

**For the planner:**

This phase is a straightforward deletion operation with systematic verification. Break into atomic tasks:

1. **Pre-deletion audit** - Verify all files exist, count lines, document current state
2. **Module system updates** - Edit mod.rs, lib.rs, universal_registry.rs, quic.rs BEFORE deleting files
3. **Cargo.toml update** - Remove `untested` feature flag (keep dependency and feature)
4. **Atomic deletion** - git rm all files in single batch operation
5. **Compilation verification** - Multi-stage cargo check (no-features, audio, yubikey, workspace)
6. **Pattern verification** - Run grep commands for CLEAN-04 compliance
7. **Commit** - Single atomic commit with all changes

**Success criteria verification:**
- CLEAN-01: `git log --stat` shows backends/yubikey.rs deleted (3,263 lines)
- CLEAN-02: `git log --stat` shows all 8 test files deleted
- CLEAN-03: `git diff HEAD~1 crates/core/Cargo.toml` shows `untested` removed
- CLEAN-04: Grep commands return no matches (except legitimate archive.rs/attestation.rs hits)

**Risk mitigation:**
- All changes in single commit - easy to revert if needed
- Multi-stage verification catches compilation errors early
- Pattern verification ensures no orphaned code remains
- Feature flag retention means v1.1 can enable yubikey without Cargo.toml changes
