# Phase 2: Error Handling - Research

**Researched:** 2026-02-09
**Domain:** Rust error handling consolidation for workspace-scale projects
**Confidence:** HIGH

## Summary

Phase 2 will unify TrustEdge's 10+ duplicate error types into a single hierarchical `TrustEdgeError` enum with subsystem variants. Current state: 9 independent error enums scattered across crates (CryptoError, ManifestError, ArchiveError, ChainError, AsymmetricError, PubkyAdapterError, PubkyError, plus TrustEdgeError already exists in hybrid.rs but only covers Pubky integration). This fragmentation causes context loss at crate boundaries, prevents unified error handling, and creates maintenance burden.

The standard approach for Rust workspace consolidation (2026) is:
- **thiserror** for library code (structured, matchable errors with automatic Display/From implementations)
- **anyhow** restricted to CLI binaries only (opaque error propagation for applications)
- Subsystem variants in a unified enum (e.g., `TrustEdgeError::Crypto(CryptoError)`)
- `#[from]` attribute for automatic conversion paths (preserves error chains)
- `#[source]` to maintain full error context for debugging tools

**Primary recommendation:** Create unified `TrustEdgeError` enum in `trustedge-core/src/error.rs` with subsystem variants. Keep detailed domain errors as nested enums. Use `#[error(transparent)]` for external dependencies. Preserve all existing error context through proper `#[source]` attribution.

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| thiserror | 1.0 | Structured library errors | Industry standard, minimal boilerplate, automatic trait impls |
| anyhow | 1.0 | CLI error propagation | Standard for application code, context-rich error chains |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| snafu | 0.8+ | Advanced context preservation | Large projects with complex error chains (alternative to thiserror) |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| thiserror | snafu | Better context preservation via context-selectors, but more verbose. GreptimeDB pattern shows value for large systems. |
| Nested enums | Flat enum | Flat loses call-graph context, requires all leaf errors bubble up (verbose, rigid). Nested enables modular refactoring. |
| Single workspace error | Per-crate errors | Current state. Loses context at boundaries, duplicate implementations, no unified handling. |

**Installation:**
```bash
# Already in workspace dependencies (Cargo.toml lines 54, 59)
# No additional dependencies needed
```

## Architecture Patterns

### Recommended Project Structure
```
crates/core/src/
├── error.rs              # Unified TrustEdgeError enum + subsystem errors
├── crypto.rs             # CryptoError moves to error.rs
├── manifest.rs           # ManifestError moves to error.rs
├── archive.rs            # ArchiveError moves to error.rs
├── chain.rs              # ChainError moves to error.rs
└── asymmetric.rs         # AsymmetricError moves to error.rs
```

**Migration path:** Create `error.rs` with all subsystem enums, then gradually migrate individual modules to use it.

### Pattern 1: Unified Enum with Subsystem Variants

**What:** Top-level enum wrapping domain-specific error enums
**When to use:** Workspace consolidation where multiple crates share errors

**Example:**
```rust
// Source: OneUptime blog 2026-01-25 + TrustEdge context
#[derive(Debug, thiserror::Error)]
pub enum TrustEdgeError {
    #[error("Cryptographic operation failed")]
    Crypto(#[from] CryptoError),

    #[error("Backend operation failed")]
    Backend(#[from] BackendError),

    #[error("Transport layer error")]
    Transport(#[from] TransportError),

    #[error("Archive operation failed")]
    Archive(#[from] ArchiveError),

    #[error("Manifest processing error")]
    Manifest(#[from] ManifestError),

    // External dependencies with transparent forwarding
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

// Subsystem error remains detailed
#[derive(Debug, thiserror::Error)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,
}
```

### Pattern 2: Context-Preserving Error Chains

**What:** Using `#[from]` and `#[source]` to maintain full error chains
**When to use:** Always, to support debugging tools that walk error chains

**Example:**
```rust
// Source: Error Handling Best Practices 2026 (Medium @Murtza)
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),  // #[from] implies #[source]

    #[error("Manifest error: {0}")]
    Manifest(#[from] ManifestError),  // Preserves error chain

    #[error("Missing chunk file: {0}")]
    MissingChunk(String),
}
```

**Key insight:** `#[from]` automatically implements `From<T>` and marks field as `#[source]`, enabling `?` operator and preserving error chains for tools like `std::error::Error::source()`.

### Pattern 3: Avoiding #[from] Anti-Pattern

**What:** Don't blindly wrap every error with `#[from]` - add context when converting
**When to use:** When the source error type appears in multiple code paths

**Example:**
```rust
// Anti-pattern (blurs context)
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error(transparent)]
    Io(#[from] std::io::Error),  // Was this from read or write?
}

// Better pattern (preserves context)
#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("Failed to read chunk file: {0}")]
    ChunkReadFailed(#[source] std::io::Error),

    #[error("Failed to write manifest: {0}")]
    ManifestWriteFailed(#[source] std::io::Error),
}

// Manual conversion with .map_err() to add context
std::fs::read(path).map_err(ArchiveError::ChunkReadFailed)?;
```

**Source:** Error Handling In Rust - A Deep Dive (Luca Palmieri)

### Pattern 4: Application vs Library Error Handling

**What:** Private workspace crates count as "application code", not public libraries
**When to use:** Deciding between per-function vs crate-level errors

**Key distinction:**
- **Libraries (published crates):** Need stable API, anticipate diverse callers, use crate-level Error enum
- **Applications (private workspace crates):** Can refactor freely, use per-function or per-module errors, nest for modularity

**TrustEdge context:** All 10 crates are private (not published to crates.io), but `trustedge-core` serves as workspace hub (FanIn=6). Treat core as library-style with unified enum, but preserve module-level detail through nesting.

**Source:** Designing Error Types in Rust Applications (Dmitrii Aleksandrov)

### Anti-Patterns to Avoid

- **Over-nesting with #[from]:** Don't create variants solely for `From` conversion without adding value. Add context when wrapping.
- **Losing context with multiple source types:** When `std::io::Error` appears in read AND write paths, don't use single `#[from]` variant - create distinct variants with context.
- **Flat enum explosion:** Don't put every leaf error at top level. Use nested enums to preserve call-graph context and enable modular refactoring.
- **Mixing anyhow in library APIs:** Keep anyhow in CLI binaries only. Library code (`trustedge-core`) must use thiserror for structured errors.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Error Display impl | Manual `impl Display` | `#[error("...")]` attribute | Automatic, supports format args, maintains consistency |
| Error conversion | Manual `impl From<SourceErr>` | `#[from]` attribute | Automatic, preserves source chain, enables `?` operator |
| Error source chain | Manual `impl Error::source()` | `#[source]` attribute | Automatic, supports debugging tools that walk chains |
| Context-rich errors | Custom wrapper types | `anyhow::Context` trait | Standard, composable, works with `?` operator |

**Key insight:** thiserror eliminates ~80% of error boilerplate. GreptimeDB case study shows even large projects (multi-subsystem) achieve clean error handling with derive macros + disciplined enum design.

## Common Pitfalls

### Pitfall 1: Multiple Error Enums Named "Error"

**What goes wrong:** TrustEdge already has `TrustEdgeError` in `hybrid.rs` (lines 16-35) covering only Pubky integration. Creating another `TrustEdgeError` in `error.rs` causes namespace collision.

**Why it happens:** Incremental evolution - Pubky integration added its own error type without workspace-wide design.

**How to avoid:**
1. Rename existing `hybrid::TrustEdgeError` to `hybrid::HybridEncryptionError`
2. Create new unified `error::TrustEdgeError` as workspace-wide type
3. Pubky crate can convert: `impl From<hybrid::HybridEncryptionError> for TrustEdgeError`

**Warning signs:** Compiler errors about "multiple definitions of TrustEdgeError" during consolidation.

### Pitfall 2: Breaking Error Conversion Paths

**What goes wrong:** Code using `?` operator stops compiling when error types change, especially at crate boundaries.

**Why it happens:** `?` operator requires `From<SourceError> for TargetError` impl. Consolidation changes the target type.

**How to avoid:**
1. Add conversion impls: `impl From<OldError> for TrustEdgeError` during transition
2. Use `#[deprecated]` on old error types
3. Migrate call sites gradually
4. Run full test suite after each conversion

**Warning signs:** Many "the `?` operator can only be used on `Result` values" errors during migration.

### Pitfall 3: Duplicate ManifestError in core vs trst-core

**What goes wrong:** AUDIT.md documents exact duplicate (lines 14, 10). Both define identical enum. During Phase 2, must decide which survives.

**Why it happens:** Phase 1 documented duplication, Phase 3 merges trst-core, but Phase 2 (errors) runs first.

**How to avoid:**
1. Phase 2 creates unified error hierarchy but keeps both ManifestError enums temporarily
2. Both convert to `TrustEdgeError::Manifest`
3. Phase 3 removes core's copy, keeps trst-core's (per AUDIT recommendation line 14)

**Warning signs:** Confusion about which ManifestError to import during Phase 2 work.

### Pitfall 4: Context Loss When Using #[error(transparent)]

**What goes wrong:** `#[error(transparent)]` forwards Display and source directly, but loses the variant context in error messages.

**Why it happens:** Transparent errors don't add a layer - they "disappear" into the underlying error.

**How to avoid:** Only use `#[error(transparent)]` for true pass-through cases (e.g., wrapping anyhow::Error). For domain errors, use explicit messages even when forwarding source.

**Example:**
```rust
// Loses context
#[error(transparent)]
Io(#[from] std::io::Error),

// Preserves context
#[error("IO operation failed: {0}")]
Io(#[from] std::io::Error),
```

**Warning signs:** Error messages that don't indicate which subsystem failed.

### Pitfall 5: Not Planning for CLI Migration

**What goes wrong:** CLI binaries (`trustedge`, `trst`, `trustedge-server`) currently use mix of anyhow and Result<T, SpecificError>. They need migration too.

**Why it happens:** Requirements focus on library consolidation (ERR-03), easy to forget application binary code.

**How to avoid:**
1. Identify all binary entry points: `find crates -name main.rs -o -path "*/bin/*.rs"`
2. Audit their error handling (do they use anyhow already?)
3. If not, add anyhow migration as explicit sub-task
4. CLIs should use `anyhow::Result<T>` and `.context()` for user-facing messages

**Warning signs:** CI failures in binary builds after library error refactor.

## Code Examples

Verified patterns from official sources and TrustEdge codebase:

### Current State Example

```rust
// crates/core/src/crypto.rs (lines 17-33)
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
    #[error("Invalid nonce format: {0}")]
    InvalidNonceFormat(String),
}
```

### Target State Example

```rust
// crates/core/src/error.rs (NEW FILE)
use thiserror::Error;

/// Unified error type for the TrustEdge workspace
#[derive(Debug, Error)]
pub enum TrustEdgeError {
    #[error("Cryptographic operation failed")]
    Crypto(#[from] CryptoError),

    #[error("Backend operation failed")]
    Backend(#[from] BackendError),

    #[error("Transport layer error")]
    Transport(#[from] TransportError),

    #[error("Archive operation failed")]
    Archive(#[from] ArchiveError),

    #[error("Manifest processing error")]
    Manifest(#[from] ManifestError),

    #[error("Chain validation error")]
    Chain(#[from] ChainError),

    #[error("Asymmetric crypto error")]
    Asymmetric(#[from] AsymmetricError),

    // External dependencies
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Cryptographic operation errors
#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),
}

/// Backend operation errors
#[derive(Debug, Error)]
pub enum BackendError {
    #[error("Backend operation not supported: {0}")]
    UnsupportedOperation(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Backend initialization failed: {0}")]
    InitializationFailed(String),
}

// ... other subsystem errors
```

### Migration Example: Function Signature Changes

```rust
// BEFORE (current state)
// crates/core/src/archive.rs
use crate::manifest::ManifestError;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Manifest error: {0}")]
    Manifest(#[from] ManifestError),
    // ...
}

pub fn write_archive(...) -> Result<(), ArchiveError> { ... }

// AFTER (target state)
// crates/core/src/archive.rs
use crate::error::{TrustEdgeError, ArchiveError, ManifestError};

pub fn write_archive(...) -> Result<(), TrustEdgeError> {
    // Function body unchanged - error conversion happens via #[from]
    // ...
}

// Alternative: Keep detailed error for precise caller handling
pub fn write_archive(...) -> Result<(), ArchiveError> {
    // Caller converts: write_archive().map_err(TrustEdgeError::Archive)?
}
```

### CLI Binary Example

```rust
// crates/trustedge-cli/src/main.rs (AFTER)
use anyhow::{Context, Result};
use trustedge_core::{TrustEdgeError, Envelope};

fn main() -> Result<()> {
    let envelope = Envelope::seal(data, &signing_key, &beneficiary_key)
        .context("Failed to seal envelope")?;  // Converts TrustEdgeError -> anyhow::Error

    std::fs::write("output.bin", envelope.serialize())
        .context("Failed to write output file")?;  // Adds context layer

    Ok(())
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual From impls | `#[from]` attribute | thiserror 1.0 (2019) | Reduced boilerplate by ~70% |
| Custom Display impls | `#[error("...")]` with format args | thiserror 1.0 (2019) | Consistent error messages |
| String-based errors | Structured enums | Community consensus 2020+ | Matchable errors, better tooling |
| Per-crate error types | Workspace-level unified enum | Large project pattern 2024+ | Better cross-crate handling |
| Global Result type aliases | `anyhow::Result` for apps | anyhow 1.0 (2020) | Standard application pattern |

**Deprecated/outdated:**
- `failure` crate: Replaced by thiserror + anyhow (deprecated 2019)
- `error-chain` crate: Replaced by thiserror (superseded 2020)
- Flat workspace error enums: Current 2026 guidance is nested enums for modularity

## Open Questions

### Question 1: Should Pubky crates use TrustEdgeError or remain independent?

**What we know:**
- Pubky crates (`trustedge-pubky`, `trustedge-pubky-advanced`) are community contributions (not core product)
- They already have distinct error types: `PubkyAdapterError`, `PubkyError`
- AUDIT.md line 100 notes they're "out of scope per v1 requirements"
- They do depend on `trustedge-core` (AUDIT lines 56-57, 72)

**What's unclear:**
- Should they convert `trustedge_core` errors to their own types, or propagate `TrustEdgeError` directly?
- Does unified error hierarchy extend to community crates?

**Recommendation:**
- Keep Pubky errors independent (they're community-maintained)
- Add conversion: `impl From<TrustEdgeError> for PubkyAdapterError` to preserve compatibility
- Document in Phase 2 plan: "Pubky crates out of scope, maintain conversion layer"

**Confidence:** MEDIUM (project structure suggests independence, but boundary is fuzzy)

### Question 2: Timing conflict with Phase 3 manifest merge

**What we know:**
- Phase 2 (errors) runs before Phase 3 (trst-core merge)
- ManifestError duplicated in `core/manifest.rs` and `trst-core/manifest.rs` (AUDIT line 14)
- AUDIT recommends keeping trst-core version (WASM-compatible)

**What's unclear:**
- Should Phase 2 consolidate both ManifestErrors immediately?
- Or keep both temporarily and let Phase 3 handle the deduplication?

**Recommendation:**
- Phase 2: Create `error::ManifestError` that matches trst-core's version
- Both `core::manifest` and `trst-core::manifest` convert to it
- Phase 3: Remove core's ManifestError entirely, trst-core becomes canonical
- This preserves AUDIT recommendation while unblocking error consolidation

**Confidence:** HIGH (aligns with ROADMAP phase sequence)

### Question 3: How to handle Backend errors?

**What we know:**
- Universal Backend system uses capability-based dispatch (CLAUDE.md pattern)
- Backend operations return `CryptoResult` enum, not `Result<T, E>` (core/backends/universal.rs:139)
- Backend trait functions use `anyhow::Result` (core/backends/traits.rs:13, 19-31)

**What's unclear:**
- Should backends continue using `anyhow::Result` (they're library code, not CLI)?
- Or should they use structured `TrustEdgeError::Backend`?
- What about the `CryptoResult` enum pattern - is that a success case, not error?

**Recommendation:**
- `CryptoResult` is fine (it's a success value, different data types)
- Backend trait functions should return `Result<T, BackendError>` not `anyhow::Result`
- Create `BackendError` enum with variants: `UnsupportedOperation`, `KeyNotFound`, `HardwareError`, etc.
- This maintains structured errors for library code (per ERR-03 requirement)

**Confidence:** HIGH (aligns with library vs application error handling standards)

### Question 4: Error migration order within Phase 2

**What we know:**
- 9 distinct error types to consolidate
- Some have cross-dependencies (ArchiveError includes ManifestError)
- 42 files use `.map_err()` or `.context()` (Grep found files_with_matches)

**What's unclear:**
- What order minimizes compilation breakage?
- Which errors should be migrated first?

**Recommendation:**
- **Wave 1:** Create `error.rs` module with all enums but no usage
- **Wave 2:** Migrate leaf errors (no dependencies): `CryptoError`, `ChainError`, `AsymmetricError`
- **Wave 3:** Migrate dependent errors: `ArchiveError` (depends on ManifestError), `ManifestError`
- **Wave 4:** Update function signatures across workspace
- **Wave 5:** Migrate CLI binaries to use anyhow
- This bottom-up approach keeps codebase compiling at each step

**Confidence:** HIGH (standard refactoring pattern for dependency graphs)

## Sources

### Primary (HIGH confidence)
- [thiserror official docs](https://docs.rs/thiserror/latest/thiserror/) - Attribute reference and examples
- [anyhow official docs](https://github.com/dtolnay/anyhow) - Application error handling patterns
- [How to Design Error Types with thiserror and anyhow in Rust](https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view) - 2026 best practices
- TrustEdge AUDIT.md (.planning/phases/01-foundation/AUDIT.md) - Current error duplication state
- TrustEdge CLAUDE.md - Architecture patterns and backend design

### Secondary (MEDIUM confidence)
- [Error Handling for Large Rust Projects - GreptimeDB](https://greptime.com/blogs/2024-05-07-error-rust) - Multi-subsystem error handling with snafu
- [Designing Error Types in Rust Applications](https://home.expurple.me/posts/designing-error-types-in-rust-applications/) - Application vs library patterns
- [Error Handling Best Practices in Rust](https://medium.com/@Murtza/error-handling-best-practices-in-rust-a-comprehensive-guide-to-building-resilient-applications-46bdf6fa6d9d) - Context preservation strategies
- [Rust Error Handling Compared: anyhow vs thiserror vs snafu](https://dev.to/leapcell/rust-error-handling-compared-anyhow-vs-thiserror-vs-snafu-2003) - Library comparison
- [Rust's Error Handling Trio](https://www.oreateai.com/blog/rusts-error-handling-trio-navigating-the-nuances-of-anyhow-thiserror-and-snafu/b9843489b5bc598ffeb83b48076f03ba) - When to use each tool

### Tertiary (LOW confidence, marked for validation)
- [Error Handling In Rust - A Deep Dive](https://lpalmieri.com/posts/error-handling-rust/) - Comprehensive but older (pre-2026), verify patterns
- [Rust API Guidelines - Naming](https://rust-lang.github.io/api-guidelines/naming.html) - General conventions, not error-specific
- [Error enum variant naming conventions](https://users.rust-lang.org/t/error-enum-variant-naming-conventions/53257) - Forum discussion, no authoritative answer

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - thiserror + anyhow are industry consensus in 2026
- Architecture: HIGH - Unified enum pattern verified in recent blog posts and GreptimeDB case study
- Pitfalls: MEDIUM-HIGH - Derived from project state (AUDIT.md) and migration experience patterns
- Open Questions: MEDIUM - Workspace-specific decisions that need validation during planning

**Research date:** 2026-02-09
**Valid until:** 2026-03-09 (30 days - error handling patterns are stable, but check for thiserror updates)

## Key Findings for Planner

### Must Address in Plans

1. **Name collision:** Existing `TrustEdgeError` in `hybrid.rs` must be renamed before creating unified version
2. **ManifestError timing:** Phase 2 creates unified version, Phase 3 removes duplication
3. **Backend error type mismatch:** Backend traits currently use `anyhow::Result`, need migration to structured errors
4. **42+ files use .map_err/.context:** Extensive call site migration needed
5. **CLI binaries need migration:** Not just library code, application entry points need anyhow conversion

### Recommended Task Structure

1. **Prepare:** Audit all error types and usages, identify dependencies
2. **Create:** New `error.rs` with full hierarchy (non-breaking)
3. **Migrate (Wave 1):** Leaf errors (Crypto, Chain, Asymmetric)
4. **Migrate (Wave 2):** Dependent errors (Archive, Manifest)
5. **Migrate (Wave 3):** Update function signatures workspace-wide
6. **Migrate (Wave 4):** CLI binaries to anyhow
7. **Cleanup:** Remove old error modules, deprecation warnings
8. **Verify:** Full test suite + error message validation

### Success Criteria Validation

Requirements state:
- ERR-01: Unified TrustEdgeError enum with subsystem variants ✓ (Crypto, Backend, Transport, Archive, Manifest pattern documented)
- ERR-02: All 10+ duplicate error types consolidated ✓ (AUDIT.md identified 9 error types, consolidation path defined)
- ERR-03: thiserror for library code, anyhow for CLI ✓ (Standard pattern documented with examples)

All requirements are achievable with documented patterns. No blockers identified.
