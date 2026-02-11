<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 5: Attestation Integration - Research

**Researched:** 2026-02-10
**Domain:** Rust workspace crate consolidation — migrating standalone attestation crate into monolithic core
**Confidence:** HIGH

## Summary

Phase 5 will merge the standalone trustedge-attestation crate (~826 LOC library, 11 tests, 2 CLI binaries, 1 example) into trustedge-core at `applications/attestation/`. The attestation system creates cryptographically signed "birth certificates" for software artifacts, capturing provenance information (Git commit, builder ID, timestamp, SHA-256 hash). Current structure: attestation is a separate crate depending on trustedge-core for optional envelope operations (feature-gated). Post-migration: attestation becomes `core::applications::attestation` module, maintaining all 11 tests and preserving the same public API surface through re-exports.

The attestation crate follows the exact same pattern as receipts (Phase 4): it's a pure consumer of core's Envelope API with no circular dependency risk. Key difference: attestation has an optional `envelope` feature flag that gates envelope functionality. The crate has two modes: JSON-only attestations (no crypto) and sealed envelope attestations (Ed25519 signatures + AES-256-GCM). The library uses `anyhow` for error handling (application-level), making integration identical to receipts migration.

**Primary recommendation:** Follow the Phase 4 receipts pattern exactly: move attestation code to `core/src/applications/attestation/`, preserve test structure, add top-level re-exports in core's lib.rs, convert CLI binaries to cargo examples, create backward-compatibility facade in attestation crate (deferred to Phase 7). The envelope feature integration requires careful attention to maintain feature-gating semantics in the unified core.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cargo-modules | 0.18+ | Module structure analysis | Official cargo plugin for detecting circular dependencies |
| git2 | 0.18 | Git integration | Already in workspace, used for source commit hash capture |
| sha2 | 0.10 | SHA-256 hashing | Already in workspace, used for artifact fingerprinting |
| chrono | 0.4 | Timestamp generation | Already in workspace, RFC3339 format for ISO 8601 compliance |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| tempfile | 3.8 | Test artifact creation | Already in dev-dependencies for tests |
| bincode | 1.3 | Binary serialization | Used for sealed envelope format (not JSON) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| In-place migration | Create new code from scratch | Migration preserves git history, tests, and known-good implementation |
| Custom error type | Reuse anyhow | Attestation uses anyhow (application-level), matches receipts pattern |
| Flat module | Submodules (lib.rs + provenance.rs + verify.rs) | All attestation logic fits in single 826 LOC file — no need to split |
| SLSA/in-toto format | Custom attestation format | Current format is simpler, sufficient for internal use; SLSA compliance could be future enhancement |

**Installation:**
```bash
# Already installed in Phase 1
cargo install cargo-modules
```

## Architecture Patterns

### Recommended Project Structure
```
crates/core/src/applications/
├── mod.rs                     # Layer 4 documentation + attestation re-export
├── receipts/
│   └── mod.rs                 # Receipts implementation (migrated Phase 4)
└── attestation/
    ├── mod.rs                 # Main attestation implementation (~826 LOC from lib.rs)
    └── README.md              # Optional: CLI usage examples (from README.md)
```

**Rationale:** Single-file module initially (no need to split 826 LOC). Binary CLIs convert to cargo examples (following Phase 4 pattern). Future phases can subdivide if needed (e.g., `provenance.rs`, `verify.rs`), but defer until complexity demands it.

### Pattern 1: Application Logic Layering with Feature-Gated Envelope Integration

**What:** Attestation as Layer 4 (applications) consuming Layer 3 (protocols) envelope operations — with feature flag for optional envelope sealing
**When to use:** When business logic composes lower-layer primitives conditionally based on feature flags

**Example:**
```rust
// Source: trustedge-attestation/src/lib.rs (lines 176-232)
#[cfg(feature = "envelope")]
fn create_sealed_attestation(
    attestation: Attestation,
    key_source: KeySource,
) -> Result<AttestationResult> {
    use trustedge_core::Envelope;  // <- BECOMES: use crate::Envelope after migration

    let signing_key = match key_source {
        KeySource::Generate => {
            let mut csprng = rand::rngs::OsRng;
            ed25519_dalek::SigningKey::generate(&mut csprng)
        }
        KeySource::Provided { signing_key } => *signing_key,
    };

    let payload = serde_json::to_vec(&attestation)?;
    let beneficiary_key = signing_key.verifying_key();

    // Delegate to core's envelope primitive (Layer 3)
    let envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key)?;

    // ... serialize with bincode
}

#[cfg(not(feature = "envelope"))]
fn create_sealed_attestation(
    attestation: Attestation,
    _key_source: KeySource,
) -> Result<AttestationResult> {
    // Fallback to JSON when envelope feature is not available
    let serialized_output = serde_json::to_vec_pretty(&attestation)?;
    Ok(AttestationResult { attestation, serialized_output, verification_info: None })
}
```

**Key insight:** Attestation uses conditional compilation (`#[cfg(feature = "envelope")]`) to optionally depend on core's Envelope. After migration, this becomes a self-dependency within core, which is fine since it's just import paths within the same crate. The feature flag still controls whether envelope functionality is compiled.

### Pattern 2: Test Preservation with Feature-Gated Tests

**What:** Move tests with code, preserve feature flags, verify exact count
**When to use:** Always during consolidation to prevent regression, especially with feature-gated functionality

**Example:**
```rust
// Before: crates/attestation/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centralized_json_attestation() { /* ... */ }

    #[test]
    #[cfg(feature = "envelope")]
    fn test_centralized_envelope_attestation() { /* ... */ }

    #[test]
    #[cfg(not(feature = "envelope"))]
    fn test_sealed_envelope_fallback_without_feature() { /* ... */ }
}

// After: crates/core/src/applications/attestation/mod.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centralized_json_attestation() { /* ... */ }

    #[test]
    #[cfg(feature = "envelope")]
    fn test_centralized_envelope_attestation() { /* ... */ }

    #[test]
    #[cfg(not(feature = "envelope"))]
    fn test_sealed_envelope_fallback_without_feature() { /* ... */ }
}
```

**Verification:**
```bash
# Baseline: 11 tests in attestation (10 tests + 1 integration test)
cargo test -p trustedge-attestation 2>&1 | grep "test result:" | head -1

# After migration: 11 tests still pass in core
cargo test -p trustedge-core --lib applications::attestation 2>&1 | grep "test result:"

# Test with envelope feature
cargo test -p trustedge-core --lib applications::attestation --features envelope
```

### Pattern 3: CLI Binary to Cargo Example Conversion

**What:** Convert standalone binaries to cargo examples for better discoverability
**When to use:** When binaries are demonstrations/tools rather than primary deliverables

**Example:**
```bash
# Before: crates/attestation/src/bin/attest.rs + verify.rs
cargo run -p trustedge-attestation --bin trustedge-attest -- --file artifact.bin

# After: crates/core/examples/attest.rs + verify.rs
cargo run -p trustedge-core --example attest -- --file artifact.bin

# Demo example (from attestation_demo.rs)
cargo run -p trustedge-core --example attestation_demo
```

**Migration pattern:**
1. Move `src/bin/attest.rs` → `examples/attest.rs`
2. Move `src/bin/verify.rs` → `examples/verify.rs`
3. Move `examples/attestation_demo.rs` → `examples/attestation_demo.rs`
4. Update imports from `trustedge_attestation` to `trustedge_core`
5. Remove `[[bin]]` sections from attestation Cargo.toml

**Key insight:** Examples can still use clap CLI parsing and be fully functional tools. They're just accessed via `cargo run --example` instead of `cargo run --bin`.

### Pattern 4: Circular Dependency Prevention

**What:** Use cargo-modules to verify acyclic module graph post-merge
**When to use:** After every crate consolidation phase

**Example:**
```bash
# Verify no cycles introduced
cargo modules dependencies --lib -p trustedge-core --acyclic

# Expected output: No cycles (success)
# Error output: Shows dependency chain forming cycle if detected
```

**Key insight:** Since attestation only imports from core (never exports to core), circular dependencies are structurally impossible. Verification is defense-in-depth.

### Anti-Patterns to Avoid

- **Moving tests to separate file:** Keep `#[cfg(test)] mod tests` co-located with implementation. Rust convention is inline tests for unit tests.
- **Creating AttestationError type:** Attestation uses anyhow (application-level). Don't add unnecessary error enum just to match core's thiserror pattern (that's for libraries).
- **Splitting 826 LOC prematurely:** Single file is maintainable. Only split when module grows beyond ~2,000 LOC or distinct subdomains emerge.
- **Changing import paths in tests:** Tests use `use super::*` — no changes needed if code structure preserved.
- **Breaking envelope feature flag semantics:** Preserve `#[cfg(feature = "envelope")]` blocks exactly as-is. Feature flag consolidation happens in Phase 6, not Phase 5.
- **Converting examples to standalone binaries:** Follow Phase 4 pattern of cargo examples for consistency and discoverability.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Module dependency analysis | Custom grep/sed scripts | cargo-modules | Handles re-exports, feature flags, cfg attributes correctly |
| Circular dependency detection | Manual code review | cargo modules --acyclic | Catches transitive cycles humans miss |
| Test count validation | wc -l \| grep "fn test" | cargo test --list \| wc -l | Counts actual compiled tests (respects cfg) |
| Line counting | cat \| wc -l | tokei | Separates code/comments/blanks correctly |
| Git commit hash capture | Manual shell scripting | git2 crate | Already in workspace, used by attestation |
| SHA-256 artifact hashing | Custom implementation | sha2 crate | Already in workspace, standard library |

**Key insight:** Rust's module system is complex (re-exports, glob imports, feature-gated modules). Use cargo tooling designed for it, not ad-hoc scripts.

## Common Pitfalls

### Pitfall 1: Feature Flag Scope Confusion
**What goes wrong:** Assuming `#[cfg(feature = "envelope")]` in attestation refers to attestation's feature, when after migration it refers to core's feature
**Why it happens:** Feature flags are crate-scoped, and migration changes the crate context
**How to avoid:**
- Preserve exact `#[cfg(feature = "envelope")]` syntax
- Document that core will need an `envelope` feature if it doesn't already have one
- Test both with and without the feature flag after migration
**Warning signs:** Tests fail when run without `--features envelope`, envelope functionality always available even when it shouldn't be

### Pitfall 2: Bincode Dependency for Sealed Format
**What goes wrong:** Forgetting that sealed envelope format uses bincode serialization (not JSON)
**Why it happens:** JSON is more visible in the code; bincode is only used in sealed path
**How to avoid:**
- Check that bincode is in core's dependencies (it is — already in workspace)
- Verify sealed envelope tests pass (they exercise bincode path)
- Document that attestation file format is bincode-serialized when using SealedEnvelope
**Warning signs:** Sealed envelope tests fail with serialization errors, envelope files are unreadable

### Pitfall 3: Git Integration Optional Behavior
**What goes wrong:** Expecting git commit hash to always be present, when it gracefully falls back to "unknown"
**Why it happens:** Tests don't always run in a git repository context
**How to avoid:**
- Preserve the `Repository::discover(".").unwrap_or("unknown")` pattern
- Document that commit hash is best-effort (not guaranteed)
- Test in both git and non-git contexts
**Warning signs:** Tests fail when not run from git repository root, commit hash is always "unknown"

### Pitfall 4: Import Path Changes Breaking CLI Binaries
**What goes wrong:** Updating library import from `trustedge_core::Envelope` to `crate::Envelope` but not updating binary imports
**Why it happens:** Binaries are separate compilation units with different import paths
**How to avoid:**
- When converting binaries to examples, update imports from `trustedge_attestation` to `trustedge_core`
- Examples import from the crate they belong to (trustedge_core), not from `crate::`
- Test all examples after migration
**Warning signs:** Examples fail to compile with "unresolved import" errors

## Code Examples

Verified patterns from official sources:

### Attestation Creation (JSON and Sealed Envelope)
```rust
// Source: trustedge-attestation/src/lib.rs (lines 118-173)
pub fn create_signed_attestation(config: AttestationConfig) -> Result<AttestationResult> {
    use sha2::{Digest, Sha256};

    // Step 1: Read and hash the artifact
    let artifact_data = std::fs::read(&config.artifact_path)?;
    let artifact_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // Step 2: Get Git commit hash (use placeholder if not in git repo)
    let source_commit_hash = {
        use git2::Repository;
        match Repository::discover(".") {
            Ok(repo) => repo.head()
                .and_then(|head| head.peel_to_commit())
                .map(|commit| commit.id().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
            Err(_) => "unknown".to_string(),
        }
    };

    // Step 3: Create the attestation data structure
    let attestation = Attestation {
        artifact_hash,
        artifact_name: config.artifact_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_commit_hash,
        builder_id: config.builder_id,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // Step 4: Handle output format
    match config.output_format {
        OutputFormat::JsonOnly => {
            let serialized_output = serde_json::to_vec_pretty(&attestation)?;
            Ok(AttestationResult {
                attestation,
                serialized_output,
                verification_info: None,
            })
        }
        OutputFormat::SealedEnvelope => create_sealed_attestation(attestation, config.key_source),
    }
}
```

### Attestation Verification
```rust
// Source: trustedge-attestation/src/lib.rs (lines 296-342)
pub fn verify_attestation(config: VerificationConfig) -> Result<VerificationResult> {
    // Read and parse the attestation (auto-detect format)
    let attestation = if config.force_json {
        read_json_attestation(&config.attestation_path)?
    } else {
        #[cfg(feature = "envelope")]
        {
            match read_envelope_attestation(&config.attestation_path) {
                Ok(att) => att,
                Err(_) => read_json_attestation(&config.attestation_path)?,
            }
        }
        #[cfg(not(feature = "envelope"))]
        {
            read_json_attestation(&config.attestation_path)?
        }
    };

    // Compute artifact hash
    let artifact_data = std::fs::read(&config.artifact_path)?;
    use sha2::{Digest, Sha256};
    let computed_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // Check if hashes match
    let is_valid = computed_hash == attestation.artifact_hash;

    let verification_details = VerificationDetails {
        computed_hash: computed_hash.clone(),
        expected_hash: attestation.artifact_hash.clone(),
        artifact_size: artifact_data.len() as u64,
        envelope_verified: None, // Set by envelope reading if applicable
    };

    Ok(VerificationResult {
        attestation,
        is_valid,
        verification_details,
    })
}
```

## State of the Art

### Industry Standards for Software Attestation

| Standard | Purpose | Adoption | Relevance to TrustEdge |
|----------|---------|----------|------------------------|
| SLSA (Supply-chain Levels for Software Artifacts) | Incremental security framework with provenance attestations | Industry consensus, updated Feb 2026 | Future enhancement — current format is simpler |
| in-toto attestations | JSON-based attestation format with DSSE envelope | Docker, Kubernetes ecosystems | DSSE envelope similar to TrustEdge Envelope concept |
| Sigstore (Cosign/Fulcio) | Keyless signing with OIDC identity binding | Kubernetes-native deployments | Different trust model — TrustEdge uses explicit keys |

**Current TrustEdge approach:**
- Custom attestation format (simpler than SLSA)
- Ed25519 signatures (not ECDSA P-256)
- Optional sealed envelope (AES-256-GCM + Ed25519)
- Explicit key management (not keyless)

**Future evolution possibilities:**
- SLSA compliance as optional output format
- in-toto envelope compatibility layer
- Hardware attestation integration (TPM, secure enclaves)

### Attestation Format Comparison

| Aspect | TrustEdge Current | SLSA Provenance | in-toto ITE-6 |
|--------|-------------------|-----------------|---------------|
| **Envelope** | Bincode-serialized or JSON | DSSE (Dead Simple Signing Envelope) | DSSE |
| **Signature** | Ed25519 | ECDSA P-256 or RSA-2048 | Algorithm-agnostic |
| **Metadata** | artifact_hash, commit, builder, timestamp | Build config, materials, byproducts | Predicate-based (extensible) |
| **Serialization** | JSON or bincode | JSON | JSON |
| **Complexity** | Simple (5 fields) | Complex (nested objects) | Moderate (header + predicate) |

**Key insight:** TrustEdge attestation is intentionally simpler than industry standards. This is appropriate for internal use cases. SLSA compliance could be added as an optional export format without replacing the core attestation structure.

## Open Questions

1. **Should attestation binaries remain as binaries or convert to examples?**
   - What we know: Phase 4 converted receipts demo to example; attestation has 2 CLI binaries (attest, verify) that are functional tools
   - What's unclear: Are the CLI binaries primary deliverables or demonstrations?
   - Recommendation: Convert to examples following Phase 4 pattern for consistency. If later determined to be primary tools, they can be restored as binaries in Phase 6 feature consolidation.

2. **Should envelope feature be a core feature or attestation-specific sub-feature?**
   - What we know: Core already uses Envelope everywhere; attestation makes it optional via feature flag
   - What's unclear: After migration, should core have an `attestation-envelope` feature or just reuse existing envelope functionality?
   - Recommendation: Phase 5 preserves feature flag as-is (`#[cfg(feature = "envelope")]`). Phase 6 (Feature Flags) is where feature architecture is consolidated. For now, ensure core compiles with and without attestation envelope functionality.

3. **How to handle the bincode dependency scope?**
   - What we know: Bincode is used only for sealed envelope format; already in workspace dependencies
   - What's unclear: Should bincode be a required dependency or optional based on envelope feature?
   - Recommendation: Keep bincode as required dependency (it's small, 1.3 crate). Feature flag gates the code path that uses it, not the dependency itself. This simplifies feature matrix.

4. **Should SLSA compliance be researched now or deferred?**
   - What we know: SLSA is industry standard; TrustEdge format is simpler
   - What's unclear: Is SLSA compliance a Phase 5 concern or a future feature?
   - Recommendation: Defer to post-consolidation. Phase 5 focus is migration, not feature enhancement. SLSA export format could be added later without changing core attestation structure.

## Sources

### Primary (HIGH confidence)
- `/home/john/vault/projects/github.com/trustedge/crates/attestation/src/lib.rs` - Attestation implementation (826 LOC)
- `/home/john/vault/projects/github.com/trustedge/crates/attestation/Cargo.toml` - Dependency configuration and feature flags
- `/home/john/vault/projects/github.com/trustedge/.planning/phases/04-receipts-integration/04-01-SUMMARY.md` - Phase 4 migration pattern reference

### Secondary (MEDIUM confidence)
- [SLSA Software Attestations](https://slsa.dev/attestation-model) - Industry standard attestation format
- [in-toto Attestation Framework](https://github.com/in-toto/attestation) - JSON schema and DSSE envelope specification
- [Software Attestation in Build Security](https://xygeni.io/blog/an-introduction-to-software-attestation-in-build-security/) - Best practices overview
- [2026 Software Supply Chain Report](https://www.sonatype.com/state-of-the-software-supply-chain/2026/software-compliance) - Compliance trends

### Tertiary (LOW confidence)
- [JFrog Software Provenance](https://jfrog.com/learn/grc/software-provenance/) - General provenance concepts
- [Deep Dive Into SLSA Provenance](https://www.legitsecurity.com/blog/slsa-provenance-blog-series-part-2-deeper-dive-into-slsa-provenance) - SLSA technical details

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All dependencies verified in workspace Cargo.toml
- Architecture: HIGH - Pattern follows proven Phase 4 receipts migration exactly
- Pitfalls: HIGH - Based on actual attestation code analysis and Phase 4 learnings
- State of the art: MEDIUM - Industry standards verified via official specs, but TrustEdge intentionally diverges

**Research date:** 2026-02-10
**Valid until:** ~30 days (stable domain - attestation format unlikely to change rapidly)

**Key findings:**
1. Attestation crate is 826 LOC library + 2 CLI binaries + 1 example + 11 tests
2. Feature flag pattern (`#[cfg(feature = "envelope")]`) must be preserved carefully
3. Migration follows Phase 4 receipts pattern exactly (application layer consumer of envelope)
4. Industry standards (SLSA, in-toto) are more complex than needed for current use case
5. CLI binaries should convert to cargo examples for consistency with Phase 4

**Ready for planning:** Yes. Research complete. Planner can now create PLAN.md files following the Phase 4 receipts pattern as a template.
