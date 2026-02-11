<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 4: Receipts Integration - Research

**Researched:** 2026-02-10
**Domain:** Rust workspace crate consolidation — migrating standalone receipts crate into monolithic core
**Confidence:** HIGH

## Summary

Phase 4 will merge the standalone trustedge-receipts crate (1,281 LOC, 23 tests) into trustedge-core at `applications/receipts/`. The receipts system implements cryptographic ownership chains using the Envelope API from core — it is a pure consumer of core functionality with no circular dependency risk. Current structure: receipts is a separate crate depending on trustedge-core for envelope operations. Post-migration: receipts becomes `core::applications::receipts` module, maintaining all 23 tests and preserving the same public API surface through re-exports.

The receipts crate is a textbook "thin application logic layer" — it provides three high-level functions (`create_receipt`, `assign_receipt`, `verify_receipt_chain`) and one struct (`Receipt`) that compose core's `Envelope` primitives. It uses `anyhow` for error handling (not a custom error type), making integration straightforward. The only external dependencies are ed25519-dalek, serde, serde_json, hex, rand — all already in core.

**Primary recommendation:** Move receipts code to `core/src/applications/receipts/`, preserve test structure, add top-level re-exports in core's lib.rs, verify no circular dependencies with `cargo modules dependencies --lib -p trustedge-core --acyclic`. Create backward-compatibility facade in receipts crate for 6-month deprecation window (deferred to Phase 7).

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| cargo-modules | 0.18+ | Module structure analysis | Official cargo plugin for detecting circular dependencies |
| thiserror | 1.0 | Library error handling | Workspace standard (established Phase 2) |
| anyhow | 1.0 | CLI error propagation | Workspace standard for application code |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| cargo-machete | 0.7+ | Unused dependency detection | Post-merge cleanup (Phase 8 validation) |
| tokei | 12+ | Line counting | Validate LOC preservation requirement |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| In-place migration | Create new code from scratch | Migration preserves git history, tests, and known-good implementation |
| Custom error type | Reuse anyhow | Receipts uses anyhow (application-level), no migration needed |
| Flat module | Submodules (lib.rs + chain.rs + policy.rs) | All receipts logic fits in single 1,281 LOC file — no need to split |

**Installation:**
```bash
# Already installed in Phase 1
cargo install cargo-modules
```

## Architecture Patterns

### Recommended Project Structure
```
crates/core/src/applications/
├── mod.rs                # Layer 4 documentation + receipts re-export
└── receipts/
    ├── mod.rs            # Main receipts implementation (1,281 LOC from lib.rs)
    └── README.md         # Optional: usage examples (from demo.rs)
```

**Rationale:** Single-file module initially (no need to split 1,281 LOC). Future phases can subdivide if needed (e.g., `chain.rs`, `policy.rs`), but defer until complexity demands it.

### Pattern 1: Application Logic Layering

**What:** Receipts as Layer 4 (applications) consuming Layer 3 (protocols) envelope operations
**When to use:** When business logic composes lower-layer primitives without adding new crypto

**Example:**
```rust
// Source: trustedge-receipts/src/lib.rs (lines 173-191)
pub fn create_receipt(
    issuer_key: &SigningKey,
    beneficiary_key: &VerifyingKey,
    amount: u64,
    description: Option<String>,
) -> Result<Envelope> {
    // Business logic: create Receipt struct
    let receipt = Receipt::new_origin(issuer_key, beneficiary_key, amount, description);
    receipt.validate()?;

    // Serialize to payload
    let payload = serde_json::to_vec(&receipt)?;

    // Delegate to core's envelope primitive (Layer 3)
    Envelope::seal(&payload, issuer_key, beneficiary_key)
        .context("Failed to seal receipt in envelope")
}
```

**Key insight:** Receipts never implements crypto — it only orchestrates core's Envelope API. This unidirectional dependency prevents cycles.

### Pattern 2: Test Preservation During Migration

**What:** Move tests with code, preserve module paths, verify exact count
**When to use:** Always during consolidation to prevent regression

**Example:**
```rust
// Before: crates/receipts/src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_receipt_creation() { /* ... */ }
}

// After: crates/core/src/applications/receipts/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_receipt_creation() { /* ... */ }
}
```

**Verification:**
```bash
# Baseline: 23 tests in receipts (from Phase 1 TEST-BASELINE.md)
cargo test -p trustedge-receipts 2>&1 | grep "test result:" | head -1

# After migration: 23 tests still pass in core
cargo test -p trustedge-core --lib applications::receipts 2>&1 | grep "test result:"
```

### Pattern 3: Circular Dependency Prevention

**What:** Use cargo-modules to verify acyclic module graph post-merge
**When to use:** After every crate consolidation phase

**Example:**
```bash
# Verify no cycles introduced
cargo modules dependencies --lib -p trustedge-core --acyclic

# Expected output: No cycles (success)
# Error output: Shows dependency chain forming cycle if detected
```

**Key insight:** Since receipts only imports from core (never exports to core), circular dependencies are structurally impossible. Verification is defense-in-depth.

### Anti-Patterns to Avoid

- **Moving tests to separate file:** Keep `#[cfg(test)] mod tests` co-located with implementation. Rust convention is inline tests for unit tests.
- **Creating ReceiptError type:** Receipts uses anyhow (application-level). Don't add unnecessary error enum just to match core's thiserror pattern (that's for libraries).
- **Splitting 1,281 LOC prematurely:** Single file is maintainable. Only split when module grows beyond ~2,000 LOC or distinct subdomains emerge.
- **Changing import paths in tests:** Tests use `use super::*` — no changes needed if code structure preserved.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Module dependency analysis | Custom grep/sed scripts | cargo-modules | Handles re-exports, feature flags, cfg attributes correctly |
| Circular dependency detection | Manual code review | cargo modules --acyclic | Catches transitive cycles humans miss |
| Test count validation | wc -l \| grep "fn test" | cargo test --list \| wc -l | Counts actual compiled tests (respects cfg) |
| Line counting | cat \| wc -l | tokei | Separates code/comments/blanks correctly |

**Key insight:** Rust's module system is complex (re-exports, glob imports, feature-gated modules). Use cargo tooling designed for it, not ad-hoc scripts.

## Common Pitfalls

### Pitfall 1: Forgetting to Re-export Public API

**What goes wrong:** Code moves to `core::applications::receipts::Receipt`, but consumers expect `trustedge_core::Receipt` at top level.

**Why it happens:** Module nesting changes import paths. Rust doesn't automatically hoist nested items to crate root.

**How to avoid:** Add explicit re-exports in `core/src/lib.rs` immediately after moving code:
```rust
pub use applications::receipts::{Receipt, create_receipt, assign_receipt, verify_receipt_chain};
```

**Warning signs:** Clippy warnings about unused imports in downstream crates. Consumer code fails to compile with "cannot find Receipt in trustedge_core".

### Pitfall 2: Test Module Path Confusion

**What goes wrong:** Tests fail after migration because internal test helpers can't find Receipt type.

**Why it happens:** Test module's `use super::*` now points to different parent after directory restructure.

**How to avoid:** Keep test module structure identical. If code is at `applications/receipts/mod.rs`, tests should be `#[cfg(test)] mod tests` inside same file. No separate tests/ directory needed for unit tests.

**Warning signs:** Compilation errors like "cannot find Receipt in this scope" inside test code that worked before migration.

### Pitfall 3: Dependency Inversion (Circular Dependency)

**What goes wrong:** After moving receipts into core, someone adds `use trustedge_core::receipt::Receipt` in core's envelope.rs — creating a cycle.

**Why it happens:** Developer forgets receipts is now part of core, treats it as external dependency.

**How to avoid:**
1. Run `cargo modules dependencies --lib -p trustedge-core --acyclic` after merge
2. Document in `applications/mod.rs` that Layer 4 CAN import primitives/backends/protocols but NEVER the reverse
3. Code review rule: core's lower layers (primitives, backends, protocols) cannot import from applications

**Warning signs:** Cargo compilation fails with "cyclic package dependency" error. cargo-modules --acyclic shows dependency chain.

### Pitfall 4: Losing Git History During Move

**What goes wrong:** `git mv` not used, so `git log --follow` can't track file history through the migration.

**Why it happens:** Manual copy-paste or scripted file creation loses git's rename detection.

**How to avoid:** Use `git mv crates/receipts/src/lib.rs crates/core/src/applications/receipts/mod.rs` to preserve history linkage.

**Warning signs:** `git log crates/core/src/applications/receipts/mod.rs` only shows "initial commit" after migration, missing prior history.

### Pitfall 5: Test Count Mismatch After Migration

**What goes wrong:** Phase 1 baseline shows 23 tests, but post-migration only 20 tests run.

**Why it happens:** Tests in `bin/demo.rs` not moved, or feature-gated tests not compiled.

**How to avoid:**
1. Check baseline: `cargo test -p trustedge-receipts 2>&1 | grep "test result:"`
2. After migration: `cargo test -p trustedge-core applications::receipts 2>&1 | grep "test result:"`
3. Verify exact match (23 tests in both)

**Warning signs:** Different test count in baseline vs. post-migration. Missing test names when compared to Phase 1 TEST-BASELINE.md.

## Code Examples

Verified patterns from receipts crate:

### Receipt Creation (Origin of Chain)

```rust
// Source: crates/receipts/src/lib.rs lines 173-191
use trustedge_core::Envelope;
use ed25519_dalek::{SigningKey, VerifyingKey};
use anyhow::{Context, Result};

pub fn create_receipt(
    issuer_key: &SigningKey,
    beneficiary_key: &VerifyingKey,
    amount: u64,
    description: Option<String>,
) -> Result<Envelope> {
    let receipt = Receipt::new_origin(issuer_key, beneficiary_key, amount, description);
    receipt.validate().context("Receipt validation failed")?;
    let payload = serde_json::to_vec(&receipt).context("Failed to serialize receipt")?;
    Envelope::seal(&payload, issuer_key, beneficiary_key)
        .context("Failed to seal receipt in envelope")
}
```

### Receipt Assignment (Chain Link)

```rust
// Source: crates/receipts/src/lib.rs lines 206-261
pub fn assign_receipt(
    previous_envelope: &Envelope,
    assigner_key: &SigningKey,
    new_beneficiary_key: &VerifyingKey,
    description: Option<String>,
) -> Result<Envelope> {
    // Verify previous envelope signature
    if !previous_envelope.verify() {
        return Err(anyhow::anyhow!("Previous envelope signature is invalid"));
    }

    // Verify assigner is current beneficiary
    let previous_beneficiary = previous_envelope.beneficiary();
    if previous_beneficiary != assigner_key.verifying_key() {
        return Err(anyhow::anyhow!("Assigner key does not match previous beneficiary"));
    }

    // Unseal to get actual amount
    let previous_payload = previous_envelope.unseal(assigner_key)?;
    let previous_receipt: Receipt = serde_json::from_slice(&previous_payload)?;

    // Create assignment receipt linking to previous
    let assignment_receipt = Receipt::new_assignment(
        assigner_key,
        new_beneficiary_key,
        previous_receipt.amount,
        previous_envelope.hash(),
        description,
    );
    assignment_receipt.validate()?;

    let payload = serde_json::to_vec(&assignment_receipt)?;
    Envelope::seal(&payload, assigner_key, new_beneficiary_key)
}
```

### Chain Verification

```rust
// Source: crates/receipts/src/lib.rs lines 301-329
pub fn verify_receipt_chain(envelopes: &[Envelope]) -> bool {
    if envelopes.is_empty() {
        return false;
    }

    // Verify each envelope individually
    for envelope in envelopes {
        if !envelope.verify() {
            return false;
        }
    }

    // Verify chain links (issuer of current == beneficiary of previous)
    for i in 1..envelopes.len() {
        let prev_envelope = &envelopes[i - 1];
        let current_envelope = &envelopes[i];

        if current_envelope.issuer() != prev_envelope.beneficiary() {
            return false;
        }
    }

    true
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Separate crate per feature | Monolithic core + thin shells | 2025-2026 (TrustEdge consolidation) | Eliminates duplicate error types, unified crypto ops, single test suite |
| Per-crate error types | Unified TrustEdgeError hierarchy | Phase 2 (Feb 2026) | Cross-boundary error context preserved |
| Duplicate ManifestError | ManifestFormatError in trst-protocols | Phase 3 (Feb 2026) | 454-line duplicate eliminated |
| Custom receipt error enum | anyhow for application logic | Current (receipts already uses anyhow) | No migration needed — application-level crate pattern |

**Deprecated/outdated:**
- **trustedge-receipts as standalone crate:** Post-Phase 4, receipts becomes `trustedge_core::applications::receipts`. Standalone crate deprecated in Phase 7 (backward-compatibility facade), removed in v0.3.0.

## Open Questions

1. **Should demo.rs become examples/receipts_demo.rs or docs-only?**
   - What we know: demo.rs is 158 LOC demonstrating 3-party ownership transfer (Alice -> Bob -> Charlie)
   - What's unclear: Whether to keep as runnable example or extract key snippets to rustdoc
   - Recommendation: Move to `examples/receipts_demo.rs` in Phase 4. Preserves runability. Can be invoked via `cargo run --example receipts_demo`.

2. **Should Receipt struct be re-exported at crate root or only in applications namespace?**
   - What we know: Current API is `use trustedge_receipts::Receipt`. Post-merge, natural path is `trustedge_core::applications::receipts::Receipt`.
   - What's unclear: Whether to hoist to `trustedge_core::Receipt` for ergonomics vs. keeping explicit layer structure
   - Recommendation: Hoist to crate root in Phase 4 (`pub use applications::receipts::Receipt`). Maintains backward compatibility. Explicit layer imports are for internal use.

3. **Will Phase 7 backward-compatibility facade use deprecated crate or re-export?**
   - What we know: Phase 7 creates "deprecated re-export facades for merged crates" with 6-month migration window
   - What's unclear: Whether trustedge-receipts v0.2.1 becomes a thin re-export wrapper or a deprecated separate crate
   - Recommendation: Defer to Phase 7 planning. Research indicates thin wrapper pattern (crate becomes `pub use trustedge_core::applications::receipts::*` with deprecation warnings).

## Sources

### Primary (HIGH confidence)
- trustedge-receipts source code (1,281 LOC, 23 tests) — Direct inspection of implementation
- Phase 1 baseline (TEST-BASELINE.md) — Official test count: 23 receipts tests
- Phase 2 RESEARCH.md — Error handling patterns (thiserror for libs, anyhow for apps)
- Phase 3 VERIFICATION.md — Manifest deduplication pattern (dependency + re-export)
- cargo-modules GitHub README — Circular dependency detection with --acyclic flag
- Rust Book Ch 14.03 "Cargo Workspaces" — Workspace structure best practices

### Secondary (MEDIUM confidence)
- [Cargo Workspaces - The Rust Programming Language](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Rust at scale: packages, crates, and modules](https://mmapped.blog/posts/03-rust-packages-crates-modules)
- [How to Use Cargo for Rust Project Management](https://oneuptime.com/blog/post/2026-01-26-rust-cargo-project-management/view)
- [Large Rust Workspaces](https://matklad.github.io/2021/08/22/large-rust-workspaces.html) — matklad on flat workspace layout
- [Resolving cyclic dependency at module level - Rust Forum](https://users.rust-lang.org/t/resolving-cyclic-dependency-at-module-level/116990)

### Tertiary (LOW confidence)
- None — all findings verified through primary sources (codebase inspection + official docs)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - cargo-modules is official, thiserror/anyhow established in Phase 2
- Architecture: HIGH - Pattern verified by inspecting receipts source (pure consumer of Envelope API)
- Pitfalls: HIGH - Derived from Phase 2/3 VERIFICATION.md anti-patterns + Rust forum cyclic dep threads

**Research date:** 2026-02-10
**Valid until:** 90 days (stable domain — Rust workspace conventions change slowly)
