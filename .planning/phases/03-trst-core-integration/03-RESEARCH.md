<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 3: trst-core-integration - Research

**Researched:** 2026-02-10
**Domain:** Rust crate refactoring, WASM compatibility, module organization
**Confidence:** HIGH

## Summary

Phase 3 renames `trst-core` to `trst-protocols` and consolidates manifest/archive types as the single source of truth for protocol definitions. This phase resolves the duplicate manifest code identified in Phase 1 audit (8 exact type duplicates, 9 exact function duplicates totaling ~1,200 LOC). The renamed crate becomes a WASM-safe protocol definition layer that `trustedge-core` depends on for archive types, while `trst-wasm` imports directly to stay lightweight.

The research validates that the user's architectural decisions align with Rust ecosystem best practices: cargo workspace dependency reordering, `pub use` re-export patterns, nested module organization (`protocols::archive::manifest`), and scoped error types. WASM compatibility is achieved through careful dependency selection (serde, thiserror only) without needing feature flags.

**Primary recommendation:** Execute as designed. User decisions are technically sound and follow standard Rust patterns. The three-bucket manifest.rs triage (types → trst-protocols, operations → core, duplicates → delete) cleanly separates concerns.

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Module placement:**
- Archive/manifest types live under `protocols/` layer — they are protocol/format definitions, not application features
- Full archive format moves together: manifest types, chunk structure, signature format — it's one format spec
- Nested submodules: `protocols::archive::manifest`, `protocols::archive::chunks`, `protocols::archive::signatures`
- Core's existing `manifest.rs` (cam.video profile serialization) stays SEPARATE from trst-protocols archive manifest types — different concerns

**WASM boundary:**
- Feature-gated WASM: a `wasm` feature flag is NOT needed on trst-protocols — it's WASM-safe by design (no non-WASM deps)
- Subtractive approach for trustedge-core: non-WASM things (transport, YubiKey) use `cfg(not(target_arch = "wasm32"))` — but that's Phase 6 scope
- This phase only ensures archive code compiles to WASM — broader dependency cfg-gating is Phase 6
- WASM verification: `cargo check -p trst-protocols --target wasm32-unknown-unknown` (not whole workspace)

**Migration strategy:**
- Rename `trst-core` → `trst-protocols`, directory `crates/trst-core/` → `crates/trst-protocols/`
- trustedge-core DEPENDS ON trst-protocols for manifest/archive types (single source of truth, core re-exports)
- trst-cli depends on trustedge-core only (gets trst-protocols types via core re-exports)
- trst-wasm depends on trst-protocols directly (stays lightweight, no unnecessary core dependency)
- Delete old trst-core crate immediately — no deprecation facade

**Deduplication scope — manifest.rs triage:**
- Three-bucket triage for every item in core's manifest.rs:
  1. **Type definitions** (structs, enums, format constants, serde derives, pure validation) → move to trst-protocols (`archive::manifest` or `capture::profile` depending on domain)
  2. **Operational logic** (I/O, crypto, orchestration) → keep in trustedge-core, rewrite imports to point at trst-protocols types
  3. **Duplicated code** identical to trst-core → delete (this is the duplication Phase 1 audit flagged)
- After triage, delete core's manifest.rs entirely — replace with targeted imports from trst-protocols

**trst-protocols crate structure:**
- Two top-level domains: `archive` + `capture` — extensible so additional domains slot in without reorganizing
- `archive::manifest` — manifest types, format constants, validation
- `archive::chunks` — chunk header/structure
- `archive::signatures` — signature envelope types
- `capture::profile` — cam.video capture profile types

**Error ownership — split by operation location:**
- trst-protocols defines scoped errors per sub-module (no top-level ArchiveError):
  - `archive::manifest::ManifestFormatError` — manifest parsing/validation only
  - `archive::chunks::ChunkFormatError` — chunk header/sequence validation only
  - `archive::signatures::SignatureFormatError` — signature envelope parsing only
  - `capture::profile::ProfileFormatError` — capture profile validation only
- Core's `TrustEdgeError` wraps these via `From` impls
- Errors live where the operations that produce them live

### Claude's Discretion

- Exact re-export surface from trustedge-core (which trst-protocols types to re-export)
- Internal module organization within each sub-module
- Test file placement (unit tests in trst-protocols vs integration tests in consumers)

### Deferred Ideas (OUT OF SCOPE)

- Full WASM cfg-gating of trustedge-core dependencies — Phase 6 (Feature Flags)
- cargo-machete unused dependency cleanup — Phase 8 (Validation)

</user_constraints>

## Standard Stack

### Core Dependencies

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| serde | 1.0 | Serialization framework | De facto standard for Rust serialization, WASM-compatible |
| serde_json | 1.0 | JSON serialization | Official JSON backend for serde, WASM-safe |
| thiserror | 1.0 | Error type derivation | Standard library error pattern, zero runtime cost |

### Verification Tools

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| cargo check | built-in | Fast compilation check | Verify WASM target compatibility without full build |
| cargo test | built-in | Run test suite | Validate no functionality lost during migration |
| cargo tree | built-in | Dependency graph visualization | Verify dependency direction after rename |

**Installation:**
```bash
# All tools built-in to cargo
# WASM target must be installed:
rustup target add wasm32-unknown-unknown
```

## Architecture Patterns

### Recommended Project Structure

```
crates/trst-protocols/
├── Cargo.toml              # Minimal deps: serde, serde_json, thiserror
├── src/
│   ├── lib.rs             # Crate root with re-exports
│   ├── archive/           # Archive format domain
│   │   ├── mod.rs         # Archive domain root
│   │   ├── manifest.rs    # ManifestV1, DeviceInfo, CaptureInfo, etc.
│   │   ├── chunks.rs      # ChunkHeader, chunk validation
│   │   └── signatures.rs  # SignatureEnvelope types
│   └── capture/           # Capture profile domain
│       ├── mod.rs         # Capture domain root
│       └── profile.rs     # CaptureProfile, cam.video types
```

### Pattern 1: Cargo Workspace Dependency Reordering

**What:** Rename a crate and reverse dependency direction (was: core provides types, trst-core duplicates them; becomes: trst-protocols defines types, core imports them).

**When to use:** When consolidating duplicate types and establishing single source of truth.

**Example:**
```toml
# crates/trst-protocols/Cargo.toml (renamed from trst-core)
[package]
name = "trustedge-trst-protocols"
version = "0.2.0"

[dependencies]
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
thiserror = { workspace = true }

# crates/core/Cargo.toml (now depends on protocols)
[dependencies]
trustedge-trst-protocols = { path = "../trst-protocols" }

# crates/trst-cli/Cargo.toml (now only depends on core, gets protocols transitively)
[dependencies]
trustedge-core = { path = "../core" }
# REMOVED: trustedge-trst-core = { path = "../trst-core" }

# crates/trst-wasm/Cargo.toml (lightweight, depends on protocols directly)
[dependencies]
trustedge-trst-protocols = { path = "../trst-protocols" }
# Does NOT depend on trustedge-core (stays minimal for WASM)
```

**Source:** [Cargo Workspaces - The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), [Monorepos with Cargo Workspace and Crates](https://earthly.dev/blog/cargo-workspace-crates/)

### Pattern 2: Re-export Surface Design

**What:** Use `pub use` to control public API surface, hiding internal module structure while maintaining backward compatibility.

**When to use:** When migrating types between crates but preserving existing import paths for consumers.

**Example:**
```rust
// crates/trst-protocols/src/lib.rs
pub mod archive;
pub mod capture;

// Flatten common types at crate root for convenience
pub use archive::manifest::{ManifestV1, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo};
pub use archive::chunks::ChunkHeader;
pub use archive::signatures::SignatureEnvelope;

// crates/core/src/lib.rs
pub use trustedge_trst_protocols as protocols;

// Selective re-exports at core root for backward compatibility
pub use protocols::archive::manifest::{ManifestV1 as CamVideoManifest, DeviceInfo, CaptureInfo};
// Users can still do: use trustedge_core::{CamVideoManifest, DeviceInfo};
```

**Rationale:** Re-exports allow internal reorganization without breaking downstream code. Users don't need to know types moved from core to protocols.

**Source:** [Re-exports - The rustdoc book](https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html), [Item 24: Re-export dependencies whose types appear in your API](https://effective-rust.com/re-export.html), [Why we re-export symbols from other libraries in Rust](https://brokenco.de/2023/07/26/rust-re-export.html)

### Pattern 3: Nested Submodule Organization

**What:** Use nested modules (`protocols::archive::manifest`) to organize related types by domain and sub-domain.

**When to use:** When building extensible systems where new domains can be added without restructuring existing code.

**Example:**
```rust
// crates/trst-protocols/src/lib.rs
pub mod archive {
    pub mod manifest;
    pub mod chunks;
    pub mod signatures;
}

pub mod capture {
    pub mod profile;
}

// Future extension (Phase 4+):
// pub mod receipts {
//     pub mod ownership;
//     pub mod chain;
// }

// crates/trst-protocols/src/archive/mod.rs
pub mod manifest;
pub mod chunks;
pub mod signatures;

// Re-export commonly used items at domain level
pub use manifest::ManifestV1;
pub use chunks::ChunkHeader;

// crates/trst-protocols/src/archive/manifest.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestV1 {
    pub trst_version: String,
    pub profile: String,
    // ... fields
}

impl ManifestV1 {
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, ManifestFormatError> {
        // Validation and serialization logic
    }
}

// Error scoped to this sub-module
#[derive(thiserror::Error, Debug)]
pub enum ManifestFormatError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid field value: {0}")]
    InvalidField(String),
}
```

**Rationale:** Nested structure maps to conceptual hierarchy (protocols → archive → manifest). Future domains (receipts, attestation) slot in as siblings without reorganizing existing code.

**Source:** [Control Scope and Privacy with Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html), [The Module System](https://highassurance.rs/chp3/modules.html), [Rust Project Structure and Best Practices](https://www.djamware.com/post/68b2c7c451ce620c6f5efc56/rust-project-structure-and-best-practices-for-clean-scalable-code)

### Pattern 4: WASM-Safe Crate Design (Subtractive Approach)

**What:** Design crates to be WASM-compatible by default through careful dependency selection, not feature flags. Non-WASM code uses `cfg(not(target_arch = "wasm32"))`.

**When to use:** When building protocol/format crates that both native and WASM consumers will use.

**Example:**
```toml
# crates/trst-protocols/Cargo.toml
# WASM-safe by design - no feature flags needed
[dependencies]
serde = { workspace = true, features = ["derive"] }
serde_json = { workspace = true }
thiserror = { workspace = true }
# No std I/O, no OS-specific deps, no threading

# Verify WASM compatibility in CI:
# cargo check -p trustedge-trst-protocols --target wasm32-unknown-unknown
```

```rust
// crates/core/src/transport.rs (future Phase 6 work)
// Non-WASM code is cfg-gated out, not the other way around
#[cfg(not(target_arch = "wasm32"))]
pub mod tcp;

#[cfg(not(target_arch = "wasm32"))]
pub mod quic;
```

**Rationale:** Subtractive approach is cleaner than additive. Protocol definitions are naturally WASM-safe (just types and validation). Transport/YubiKey are naturally non-WASM (need OS).

**Source:** [How to Add WebAssembly Support to a General-Purpose Crate](https://rustwasm.github.io/book/reference/add-wasm-support-to-crate.html), [Check if a rust crate is compatible with wasm](https://gist.github.com/matthewjberger/e5d6b5c7c6437f55073dd6949aa51943), [Arch-dependant dependencies](https://users.rust-lang.org/t/arch-dependant-dependencies/78258)

### Pattern 5: Scoped Error Types (Not Top-Level)

**What:** Define errors in the module where the operations that produce them live, not at crate root.

**When to use:** When building protocol libraries with multiple independent sub-domains.

**Example:**
```rust
// crates/trst-protocols/src/archive/manifest.rs
#[derive(thiserror::Error, Debug)]
pub enum ManifestFormatError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid field value: {0}")]
    InvalidField(String),
}

// crates/trst-protocols/src/archive/chunks.rs
#[derive(thiserror::Error, Debug)]
pub enum ChunkFormatError {
    #[error("Invalid chunk header: {0}")]
    InvalidHeader(String),
    #[error("Chunk sequence gap at index {0}")]
    SequenceGap(usize),
}

// crates/core/src/error.rs (wraps protocols errors)
#[derive(thiserror::Error, Debug)]
pub enum TrustEdgeError {
    #[error("Manifest format error")]
    ManifestFormat(#[from] trst_protocols::archive::manifest::ManifestFormatError),

    #[error("Chunk format error")]
    ChunkFormat(#[from] trst_protocols::archive::chunks::ChunkFormatError),

    // ... other variants
}
```

**Rationale:** Errors are scoped to their domain. Manifest parsing errors don't belong in a top-level ArchiveError — they belong in `archive::manifest`. Core wraps them for operational context.

**Source:** [How to Design Error Types with thiserror and anyhow in Rust](https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view), [Modular Errors with Rust's thiserror](https://gist.github.com/quad/a8a7cc87d1401004c6a8973947f20365), [Modular Errors in Rust](https://sabrinajewson.org/blog/errors)

### Anti-Patterns to Avoid

- **Top-level ArchiveError in protocols crate:** Archives have multiple sub-concerns (manifest parsing, chunk validation, signature verification). Don't flatten them into one error type — scope errors to operations.
- **Feature flags for WASM in protocol crate:** Protocol definitions (types, validation) are naturally WASM-safe. Don't add a `wasm` feature — build subtractive cfg gates into consumers (trustedge-core) instead.
- **Deprecation facades when renaming crates:** No users depend on `trst-core` crate name externally (internal workspace only). Rename cleanly, update workspace members, no transition period needed.
- **Bidirectional dependencies:** After rename, `trst-protocols` must have zero workspace dependencies. Only serde/thiserror from crates.io. Core depends on protocols, not vice versa.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Canonical JSON serialization | Manual string building with key ordering (currently in manifest.rs) | `serde_json_canonicalizer` crate (RFC 8785 JSON Canonicalization Scheme) | Cryptographic operations require byte-level reproduction. RFC 8785 handles edge cases: number normalization, Unicode escaping, whitespace stripping. Current manual approach in `serialize_with_ordered_keys()` is error-prone and misses subtle edge cases. |
| Cargo workspace management | Manual Cargo.toml editing, git mv, workspace member updates | `cargo-workspaces` CLI tool (`cargo workspaces rename`) | Automates crate renaming across workspace: updates Cargo.toml member lists, dependency paths, and package names. Reduces human error. |
| Module re-exports at scale | Manual pub use for every type | Group re-exports with `pub use module::{Type1, Type2, ...}` | Reduces boilerplate. Single import line for related types. |
| Error conversion boilerplate | Manual From impls | `#[from]` attribute in thiserror | Zero-cost abstraction. Compiler generates From impl. Already used in Phase 2. |

**Key insight:** This phase involves significant file movement and import rewiring. Automation tools (cargo-workspaces) reduce risk. For canonical JSON, the current manual implementation works but should be flagged for future replacement with RFC-compliant library.

**Note on canonical JSON:** The existing `serialize_with_ordered_keys()` implementation in manifest.rs is functional for current use but not RFC 8785 compliant. For production use with cryptographic signatures, consider migrating to `serde_json_canonicalizer` in a future phase.

**Sources:**
- [serde_json_canonicalizer - crates.io](https://crates.io/crates/serde_json_canonicalizer)
- [JSON Canonicalization Scheme (JCS) implementation](https://github.com/evik42/serde-json-canonicalizer)
- [cargo-workspaces - crates.io](https://crates.io/crates/cargo-workspaces/0.2.38)

## Common Pitfalls

### Pitfall 1: Circular Dependencies After Rename

**What goes wrong:** Renaming `trst-core` to `trst-protocols` but forgetting to remove all traces of old crate name creates workspace compilation errors or circular dependency loops.

**Why it happens:** Cargo.toml files in multiple locations reference the crate. Workspace members list, dependency declarations, and potentially documentation examples all need updating.

**How to avoid:**
1. Update workspace members in root Cargo.toml FIRST: change `"crates/trst-core"` to `"crates/trst-protocols"`
2. Rename directory: `mv crates/trst-core crates/trst-protocols`
3. Update package name in crate's Cargo.toml: `name = "trustedge-trst-protocols"`
4. Update all consumers (core, trst-cli, trst-wasm) to use new path and name
5. Verify no stale references: `cargo tree -p trustedge-trst-protocols` should show correct name throughout

**Warning signs:**
- `cargo build` fails with "crate not found" errors
- Workspace shows duplicate crates in `cargo tree`
- IDE shows unresolved imports after rename

### Pitfall 2: Broken Re-exports After Type Movement

**What goes wrong:** Moving types from `core::manifest` to `trst_protocols::archive::manifest` breaks downstream code that imports from core.

**Why it happens:** Types move but re-exports aren't added, or re-exports use wrong paths.

**How to avoid:**
1. Before deleting core's manifest.rs, document all types exported from core::manifest
2. After moving types to protocols, add re-exports in core/src/lib.rs:
   ```rust
   pub use trustedge_trst_protocols::archive::manifest::{
       ManifestV1 as CamVideoManifest,
       DeviceInfo,
       CaptureInfo,
       // ... all types previously at core::manifest
   };
   ```
3. Test that old import paths still work:
   ```rust
   use trustedge_core::{CamVideoManifest, DeviceInfo}; // Should still compile
   ```

**Warning signs:**
- trst-cli or trst-wasm fail to compile after core changes
- "trait not in scope" errors when methods previously worked
- Clippy warns about unused imports of protocols crate

### Pitfall 3: Test Loss During Module Split

**What goes wrong:** Tests embedded in manifest.rs get deleted instead of moved, or moved to wrong location causing loss of test coverage.

**Why it happens:** Large refactors involve moving code between files. Tests at bottom of files can be overlooked.

**How to avoid:**
1. BEFORE moving code: Run `cargo test --lib -p trustedge-core -- --list | grep manifest` to capture all manifest-related tests
2. Document test names and their purpose (canonicalization tests, validation tests, etc.)
3. After moving types to protocols: Add equivalent tests in protocols crate OR keep as integration tests in core that import from protocols
4. Run full test suite: `cargo test --workspace` should show same test count (or more if adding new coverage)
5. Compare before/after test counts per-module

**Warning signs:**
- Test count decreases after refactor: `cargo test --lib -p trustedge-core -- manifest` shows fewer tests
- Coverage report shows uncovered code that was previously tested
- Tests pass but specific validation logic isn't exercised

### Pitfall 4: WASM Build Breaks Due to New Dependencies

**What goes wrong:** Moving code to trst-protocols accidentally pulls in non-WASM-compatible dependencies (std::fs, tokio, etc.).

**Why it happens:** Code movement brings transitive dependencies. A function that seems pure might import a module with I/O operations.

**How to avoid:**
1. Keep protocols crate dependency list minimal: only serde, serde_json, thiserror
2. BEFORE moving functions to protocols: Check their imports. If they use std::fs, std::io, or non-WASM deps, they're operational logic (stay in core), not protocol definitions
3. After every change: `cargo check -p trustedge-trst-protocols --target wasm32-unknown-unknown`
4. Set up CI check: Add to CI pipeline to catch WASM breakage immediately

**Warning signs:**
- `cargo check --target wasm32-unknown-unknown` fails after previously passing
- Error messages mention std::fs, std::net, or platform-specific code
- WASM build works but native build has unused imports

### Pitfall 5: Error Conversion Gaps

**What goes wrong:** Core's TrustEdgeError can't wrap protocols errors because `#[from]` impls are missing.

**Why it happens:** New error types in protocols (ManifestFormatError, ChunkFormatError) need explicit From impls in core's error.rs.

**How to avoid:**
1. For each new error type in protocols, add From impl in core/src/error.rs:
   ```rust
   #[derive(thiserror::Error, Debug)]
   pub enum TrustEdgeError {
       #[error("Manifest format error")]
       ManifestFormat(#[from] trst_protocols::archive::manifest::ManifestFormatError),
       // ... other variants
   }
   ```
2. Test error propagation: Write a test that triggers protocols error and verify it converts to TrustEdgeError
3. Check for clippy warnings: `clippy::result_large_err` might fire if error types balloon

**Warning signs:**
- Compilation errors about "trait not satisfied" for error conversions
- Need to manually unwrap/rewrap errors instead of using `?` operator
- Error chains lose context (can't see underlying cause)

## Code Examples

Verified patterns from official sources and existing codebase:

### Canonical JSON Serialization (Current Implementation)

```rust
// Source: crates/trst-core/src/manifest.rs (existing implementation)
// NOTE: This implementation works but is not RFC 8785 compliant
// Consider migrating to serde_json_canonicalizer for production crypto use
impl ManifestV1 {
    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, ManifestFormatError> {
        let mut canonical_manifest = self.clone();
        canonical_manifest.signature = None;

        // Manual key ordering (ensures deterministic serialization)
        let json_string = self.serialize_with_ordered_keys(&canonical_manifest)?;
        Ok(json_string.into_bytes())
    }

    fn serialize_with_ordered_keys(&self, manifest: &ManifestV1) -> Result<String, ManifestFormatError> {
        let mut result = String::from("{");
        result.push_str(&format!("\"trst_version\":{}", serde_json::to_string(&manifest.trst_version)?));
        result.push_str(&format!(",\"profile\":{}", serde_json::to_string(&manifest.profile)?));
        // ... ordered field serialization
        result.push('}');
        Ok(result)
    }
}
```

### Workspace Crate Rename Pattern

```bash
# Source: Cargo workspaces documentation
# Step 1: Update workspace root
# Edit root Cargo.toml:
# members = ["crates/trst-core"] → members = ["crates/trst-protocols"]

# Step 2: Rename directory
mv crates/trst-core crates/trst-protocols

# Step 3: Update crate package name
# Edit crates/trst-protocols/Cargo.toml:
# [package]
# name = "trustedge-trst-protocols"  # Was: trustedge-trst-core

# Step 4: Update all consumers
# crates/core/Cargo.toml:
# trustedge-trst-protocols = { path = "../trst-protocols" }

# crates/trst-cli/Cargo.toml:
# Remove: trustedge-trst-core = { path = "../trst-core" }
# Already has: trustedge-core = { path = "../core" }

# crates/trst-wasm/Cargo.toml:
# trustedge-trst-protocols = { path = "../trst-protocols" }

# Step 5: Verify
cargo tree -p trustedge-trst-protocols  # Check dependency graph
cargo check --workspace                  # Ensure everything compiles
```

### Error Scoping in Protocol Crate

```rust
// Source: thiserror documentation + user decisions
// crates/trst-protocols/src/archive/manifest.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestFormatError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid field value: {0}")]
    InvalidField(String),
}

// crates/trst-protocols/src/archive/chunks.rs
#[derive(Error, Debug)]
pub enum ChunkFormatError {
    #[error("Invalid chunk header: {0}")]
    InvalidHeader(String),

    #[error("Chunk sequence gap at index {0}")]
    SequenceGap(usize),
}

// crates/core/src/error.rs (wraps protocols errors)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrustEdgeError {
    #[error("Manifest format error")]
    ManifestFormat(#[from] trustedge_trst_protocols::archive::manifest::ManifestFormatError),

    #[error("Chunk format error")]
    ChunkFormat(#[from] trustedge_trst_protocols::archive::chunks::ChunkFormatError),

    #[error("Archive operation failed")]
    Archive(#[from] ArchiveError),

    // ... other variants
}

// ArchiveError stays in core (for I/O operations like file reading)
#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Manifest error: {0}")]
    Manifest(#[from] trustedge_trst_protocols::archive::manifest::ManifestFormatError),

    // ... operational errors
}
```

### Three-Bucket Manifest Triage

```rust
// Source: User decisions from CONTEXT.md + existing codebase

// BUCKET 1: Type definitions → Move to trst-protocols
// crates/trst-protocols/src/archive/manifest.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestV1 { /* ... fields ... */ }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo { /* ... fields ... */ }

// Pure validation method → goes with types
impl ManifestV1 {
    pub fn validate(&self) -> Result<(), ManifestFormatError> {
        if self.trst_version.is_empty() {
            return Err(ManifestFormatError::InvalidField("trst_version cannot be empty".into()));
        }
        // ... pure validation logic
        Ok(())
    }
}

// BUCKET 2: Operational logic → Keep in trustedge-core
// crates/core/src/archive.rs
use trustedge_trst_protocols::archive::manifest::ManifestV1;

pub fn write_archive<P: AsRef<Path>>(
    base_dir: P,
    manifest: &ManifestV1,
    chunk_ciphertexts: Vec<Vec<u8>>,
) -> Result<(), ArchiveError> {
    // I/O operations stay in core
    fs::create_dir_all(base_path)?;
    // ... file writing logic
}

// BUCKET 3: Duplicated code → Delete from core
// The types CamVideoManifest, DeviceInfo, etc. that exist in both
// core/src/manifest.rs and trst-core/src/manifest.rs are identical.
// After moving to protocols, delete core's version entirely.
```

### Re-export for Backward Compatibility

```rust
// Source: Rust re-export patterns + effective-rust.com guidance
// crates/core/src/lib.rs
pub use trustedge_trst_protocols as protocols;

// Re-export types at core root for backward compatibility
pub use protocols::archive::manifest::{
    ManifestV1 as CamVideoManifest,  // Keep old name alias
    DeviceInfo,
    CaptureInfo,
    ChunkInfo,
    SegmentInfo,
};

// Downstream code continues to work unchanged:
// use trustedge_core::{CamVideoManifest, DeviceInfo};
// ✓ No breaking changes

// New code can use more explicit paths:
// use trustedge_core::protocols::archive::manifest::ManifestV1;
// ✓ Also works
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Duplicate manifest types across crates | Single source of truth in protocols crate | 2026 (this phase) | Eliminates 8 type duplicates, ~1,200 LOC duplication |
| Local error enums in each module | Unified hierarchy with scoped sub-module errors | Phase 2 (complete) | Consistent error handling, automatic conversions via `#[from]` |
| Manual JSON key ordering | Manual but structured (future: RFC 8785 compliance) | 2026 (current) | Deterministic serialization for signatures, but not RFC-compliant |
| Feature flags for WASM (`#[cfg(feature = "wasm")]`) | Subtractive cfg gates (`#[cfg(not(target_arch = "wasm32"))]`) | 2024+ | Cleaner WASM-first design, non-WASM is exception |
| Top-level crate errors | Domain-scoped errors in sub-modules | 2026 (this phase) | Better error locality, easier to understand failure context |

**Deprecated/outdated:**
- **Feature-gated WASM in protocol crates:** Modern approach is WASM-safe by design (careful deps), not feature flags. Only consumers (core) need cfg gates for non-WASM features (transport, hardware).
- **Flat module structure (`mod manifest`):** Deep systems benefit from nested modules (`protocols::archive::manifest`) for extensibility and namespace clarity.
- **`anyhow::Error` in library trait signatures:** Transitioned to typed errors (thiserror) in Phase 2. anyhow is for binaries only.

## Open Questions

1. **Canonical JSON implementation**
   - What we know: Current `serialize_with_ordered_keys()` works for the use case but isn't RFC 8785 compliant
   - What's unclear: Production readiness for cryptographic operations with diverse clients (browsers, other tools)
   - Recommendation: Flag for Phase 7 (Backward Compatibility) or Phase 8 (Validation). Research migration path to `serde_json_canonicalizer` crate. Current implementation is functional for immediate needs.

2. **Test file placement after migration**
   - What we know: trst-core currently has 5 tests for manifest logic. core has 7 archive tests that use manifest types.
   - What's unclear: Should manifest type tests move to protocols? Or stay in core as integration tests?
   - Recommendation: Move pure type tests (validation, serialization) to protocols. Keep I/O-dependent tests (archive read/write) in core. Both approaches are valid per Claude's Discretion.

3. **Re-export surface from core**
   - What we know: Core must re-export protocols types for backward compatibility
   - What's unclear: Should core re-export ALL protocols types or only commonly used ones?
   - Recommendation: Re-export types currently in core's public API (CamVideoManifest, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo). New protocols types added in future phases can use explicit paths (`protocols::*`). This is Claude's Discretion area.

## Sources

### Primary (HIGH confidence)

- [Cargo Workspaces - The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) - Official cargo workspace documentation
- [Re-exports - The rustdoc book](https://doc.rust-lang.org/rustdoc/write-documentation/re-exports.html) - Official re-export patterns
- [Control Scope and Privacy with Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html) - Official module organization
- [How to Add WebAssembly Support to a General-Purpose Crate](https://rustwasm.github.io/book/reference/add-wasm-support-to-crate.html) - Official WASM compatibility guide
- Codebase audit: `.planning/phases/01-foundation/AUDIT.md` - Documents exact duplication (8 types, 9 functions, ~1,200 LOC)
- Codebase state: Phase 2 complete with unified error hierarchy (verified 2026-02-10)

### Secondary (MEDIUM confidence)

- [Item 24: Re-export dependencies whose types appear in your API](https://effective-rust.com/re-export.html) - Best practices for re-exports
- [How to Design Error Types with thiserror and anyhow in Rust](https://oneuptime.com/blog/post/2026-01-25-error-types-thiserror-anyhow-rust/view) - Current error handling patterns (2026)
- [Modular Errors with Rust's thiserror](https://gist.github.com/quad/a8a7cc87d1401004c6a8973947f20365) - Error organization patterns
- [Monorepos with Cargo Workspace and Crates](https://earthly.dev/blog/cargo-workspace-crates/) - Workspace management patterns
- [Why we re-export symbols from other libraries in Rust](https://brokenco.de/2023/07/26/rust-re-export.html) - Re-export rationale

### Tertiary (LOW confidence - informational only)

- [cargo-workspaces - crates.io](https://crates.io/crates/cargo-workspaces/0.2.38) - Automation tool for workspace operations
- [serde_json_canonicalizer - crates.io](https://crates.io/crates/serde_json_canonicalizer) - RFC 8785 implementation (future consideration)
- [Check if a rust crate is compatible with wasm](https://gist.github.com/matthewjberger/e5d6b5c7c6437f55073dd6949aa51943) - WASM compatibility checklist

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - All dependencies (serde, serde_json, thiserror) are workspace-defined and WASM-verified
- Architecture: HIGH - User decisions align with official Rust patterns, verified against Rust book and rustwasm guide
- Pitfalls: HIGH - Derived from common Cargo workspace refactoring issues documented in Rust forums and official guides

**Research date:** 2026-02-10
**Valid until:** 2026-03-10 (30 days - stable Rust ecosystem)

**Assumptions validated:**
- Rust edition 2021 is stable and current (verified in Cargo.toml)
- Workspace dependencies feature is available (used throughout existing workspace)
- WASM target `wasm32-unknown-unknown` is standard target (verified in documentation)
- thiserror 1.0 is stable and won't break error patterns (verified via crates.io)
