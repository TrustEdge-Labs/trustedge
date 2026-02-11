<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 7: Backward Compatibility - Research

**Researched:** 2026-02-10
**Domain:** Rust crate deprecation and API migration
**Confidence:** HIGH

## Summary

Backward compatibility in Rust workspace consolidation requires formal deprecation of moved crates using the `#[deprecated]` attribute on module/crate level, comprehensive migration documentation, and careful version management. The project has already created basic re-export facades for receipts, attestation, and trst-protocols during prior phases - this phase will formalize these deprecations with proper warnings, version bumps, README updates, and a migration guide.

**Primary recommendation:** Add `#[deprecated]` attributes to facade crate lib.rs files, bump facade versions to indicate deprecation (0.2.0 → 0.3.0), add README warnings with migration paths, create MIGRATION.md guide, update CHANGELOG.md, and verify all thin shells (CLI, WASM) compile successfully.

**Key insight:** The `#[deprecated]` attribute on module-level items (like the crate root module) inherits to all child items, making it possible to deprecate entire crates with a single annotation. However, deprecation on `pub use` re-exports does not reliably trigger warnings (known Rust limitation), so module-level deprecation is required.

## Standard Stack

### Core Tools
| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| `#[deprecated]` | Built-in | Mark items as deprecated | Rust standard mechanism, integrates with rustc and rustdoc |
| Semantic Versioning | 2.0.0 | Version management | Cargo ecosystem standard |
| `cargo-semver-checks` | Latest | Automated SemVer validation | Rust Project recommended tool |

### Supporting Documentation
| File | Purpose | When to Use |
|------|---------|-------------|
| MIGRATION.md | Step-by-step upgrade guide | API changes, import path changes |
| CHANGELOG.md | Version history | All releases with breaking changes |
| README.md | Deprecation notices | Crate-level warnings |
| Cargo.toml metadata | Package metadata | docs.rs configuration |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `#[deprecated]` | Yank versions | Yanking prevents new usage but doesn't provide migration guidance; deprecation is gentler |
| Minor version bump (0.2.0 → 0.2.1) | Major bump (0.2.0 → 1.0.0) | This is pre-1.0 software; minor bump signals deprecation without implying production stability |
| 6-month window | Immediate removal | Need to provide migration time for downstream consumers |

**Installation:**
```bash
# Standard Rust toolchain (already available)
rustc --version  # Built-in #[deprecated] support

# Optional validation tool
cargo install cargo-semver-checks
```

## Architecture Patterns

### Recommended Deprecation Structure

#### Pattern 1: Module-Level Deprecation
**What:** Apply `#[deprecated]` to the module/crate level so all items inherit the warning
**When to use:** Deprecating entire crates (facades) as part of consolidation

**Example:**
```rust
//! # TrustEdge Receipts
//!
//! **DEPRECATED:** This crate has been merged into `trustedge-core`.
//! Use `trustedge_core::applications::receipts` instead.
//!
//! This facade will be removed in version 0.4.0 (target: August 2026).
//!
//! ## Migration
//! ```rust
//! // Old (deprecated):
//! use trustedge_receipts::{Receipt, create_receipt};
//!
//! // New (recommended):
//! use trustedge_core::{Receipt, create_receipt};
//! ```

#![deprecated(
    since = "0.3.0",
    note = "Moved to trustedge_core::applications::receipts. \
            This crate will be removed in 0.4.0. \
            See https://github.com/TrustEdge-Labs/trustedge/blob/main/MIGRATION.md"
)]

pub use trustedge_core::{Receipt, create_receipt, assign_receipt, extract_receipt, verify_receipt_chain};
```

**Why this pattern:**
- Module-level `#[deprecated]` inherits to all re-exported items
- Clear `since` version for tracking
- Actionable `note` with migration path and removal timeline
- URL to detailed migration guide

#### Pattern 2: README Deprecation Notice
**What:** Add prominent deprecation warning to crate README
**When to use:** All deprecated facade crates

**Example structure:**
```markdown
# ⚠️ DEPRECATION NOTICE

**This crate has been deprecated and will be removed in version 0.4.0 (August 2026).**

All functionality has been consolidated into [`trustedge-core`](../core).

## Migration Path

Replace:
```rust
use trustedge_receipts::{Receipt, create_receipt};
```

With:
```rust
use trustedge_core::{Receipt, create_receipt};
```

See [MIGRATION.md](../../MIGRATION.md) for complete upgrade guide.
```

#### Pattern 3: Migration Guide Structure
**What:** Comprehensive guide covering all breaking changes
**When to use:** Major consolidation with multiple affected crates

**Structure:**
```markdown
# TrustEdge Migration Guide

## Version 0.3.0: Crate Consolidation

### Overview
Versions 0.3.0 of `trustedge-receipts`, `trustedge-attestation`, and
`trustedge-trst-protocols` are deprecated facades. All functionality
has moved to `trustedge-core`.

### Timeline
- **0.3.0** (Feb 2026): Facades deprecated with warnings
- **0.4.0** (Aug 2026): Facades removed from workspace

### Migration Steps

#### 1. Update Cargo.toml
[Before/after examples]

#### 2. Update Imports
[Old → New for each crate]

#### 3. Verify Compilation
[Test commands]

### Troubleshooting
[Common issues and solutions]
```

### Anti-Patterns to Avoid

- **Deprecating `pub use` re-exports only:** Known Rust limitation - warnings don't propagate reliably through re-exports. Use module-level deprecation instead.
- **Removing items without deprecation period:** RFC 1105 requires at least one minor release with deprecation before major removal
- **Vague deprecation notes:** Must include specific migration path and removal timeline
- **No version bump:** Deprecation requires at least minor version bump (0.2.0 → 0.3.0)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| SemVer validation | Custom version checker | `cargo-semver-checks` | Automated SemVer checking, catches API breaks |
| Deprecation timeline tracking | Manual calendar | CHANGELOG.md + GitHub milestones | Version control provides audit trail |
| Migration automation | Custom code rewriter | Deprecation warnings + manual migration | Rust ecosystem doesn't have reliable auto-migration (unlike `cargo fix` for editions) |

**Key insight:** Unlike Rust edition migrations (which have `cargo fix`), crate consolidations require manual migration by consumers. Focus on clear documentation and warnings rather than attempting automation.

## Common Pitfalls

### Pitfall 1: Re-export Deprecation Doesn't Work
**What goes wrong:** Adding `#[deprecated]` to individual `pub use` statements doesn't reliably trigger warnings for consumers

**Why it happens:** Rust limitation - deprecation on re-exports is not fully implemented (Issues #47236, #82123, #84584, #85388)

**How to avoid:** Use module-level `#![deprecated(...)]` in lib.rs instead:
```rust
// DON'T: Per-item deprecation
#[deprecated(since = "0.3.0", note = "use trustedge_core")]
pub use trustedge_core::Receipt;  // ⚠️ Warning may not appear!

// DO: Module-level deprecation
#![deprecated(since = "0.3.0", note = "use trustedge_core")]
pub use trustedge_core::Receipt;  // ✓ Entire crate deprecated
```

**Warning signs:** Running tests against deprecated crate shows no warnings; rustdoc doesn't show deprecation badge

### Pitfall 2: Cargo Silences Dependency Warnings
**What goes wrong:** Users don't see deprecation warnings from dependencies during normal builds

**Why it happens:** Cargo suppresses warnings from dependencies by default (not errors)

**How to avoid:**
- Document deprecation in README (always visible on crates.io)
- Use clear version bump (0.2.0 → 0.3.0) to signal change
- Add deprecation to CHANGELOG.md
- Consider GitHub issue/discussion for visibility

**Warning signs:** Internal tests show warnings, but external consumers report no warnings

### Pitfall 3: Insufficient Migration Period
**What goes wrong:** Removing deprecated crates too quickly breaks downstream consumers

**Why it happens:** Users need time to test, update, and redeploy

**How to avoid:**
- RFC 1105 recommends at least one minor release with deprecation
- Community norm: 3-6 months for pre-1.0, 6-12 months for post-1.0
- This project: 6 months (Feb 2026 → Aug 2026 for 0.4.0 removal)

**Warning signs:** GitHub issues from users reporting sudden breakage

### Pitfall 4: Forgetting Thin Shells
**What goes wrong:** Deprecating facade crates breaks thin shells (CLIs, WASM) that depend on facades

**Why it happens:** Shell crates may have been wired to facades during earlier phases

**How to avoid:**
- Verify all shell crates import from core, not facades
- Check: `trustedge-cli`, `trustedge-wasm`, `trst-cli`, `trst-wasm`
- Run full workspace build: `cargo check --workspace`

**Warning signs:** Workspace build fails with "deprecated" warnings-as-errors in shell crates

## Code Examples

Verified patterns from official sources:

### Module-Level Crate Deprecation
```rust
// Source: RFC 1270 + Rust Reference on diagnostics
// https://rust-lang.github.io/rfcs/1270-deprecation.html

//! Facade crate - DEPRECATED
//!
//! Use `trustedge_core` instead.

#![deprecated(
    since = "0.3.0",
    note = "This crate has been merged into trustedge-core. \
            Import from `trustedge_core::applications::receipts` instead. \
            This facade will be removed in 0.4.0 (August 2026). \
            See https://github.com/TrustEdge-Labs/trustedge/blob/main/MIGRATION.md"
)]

pub use trustedge_core::{
    Receipt,
    create_receipt,
    assign_receipt,
    extract_receipt,
    verify_receipt_chain
};
```

### Cargo.toml Metadata for Deprecated Crate
```toml
# Source: docs.rs metadata documentation
# https://docs.rs/about/metadata

[package]
name = "trustedge-receipts"
version = "0.3.0"  # Bumped from 0.2.0 to signal deprecation
edition = "2021"
description = "DEPRECATED: Use trustedge-core instead. Digital receipt system with cryptographically secure ownership chains"
authors = ["TrustEdge Labs LLC <contact@trustedgelabs.com>"]
license = "MPL-2.0"
repository = "https://github.com/TrustEdge-Labs/trustedge"
readme = "README.md"  # Contains deprecation notice
keywords = ["deprecated", "cryptography", "receipts"]
categories = ["deprecated"]

[dependencies]
trustedge-core = { path = "../core", version = "0.2.0" }

[package.metadata.docs.rs]
# Ensure docs.rs shows deprecation
all-features = false
```

### README Deprecation Notice
```markdown
# TrustEdge Receipts

## ⚠️ DEPRECATION NOTICE

**This crate has been deprecated as of version 0.3.0.**

All receipt functionality has been consolidated into [`trustedge-core`](https://docs.rs/trustedge-core).

### Timeline

- **0.3.0** (February 2026): Deprecated - warnings issued
- **0.4.0** (August 2026): Removal - crate will be deleted from workspace

### Migration

**Before (deprecated):**
```rust
use trustedge_receipts::{Receipt, create_receipt, assign_receipt};
```

**After (recommended):**
```rust
use trustedge_core::{Receipt, create_receipt, assign_receipt};
```

All APIs remain identical - only import paths change.

See [MIGRATION.md](../../MIGRATION.md) for detailed upgrade instructions.
```

### MIGRATION.md Structure
```markdown
# TrustEdge Migration Guide

## Migrating from 0.2.x to 0.3.x

### Summary

Version 0.3.0 deprecates three facade crates as part of workspace consolidation:

- `trustedge-receipts` → `trustedge-core`
- `trustedge-attestation` → `trustedge-core`
- `trustedge-trst-protocols` → `trustedge-core` (already consolidated in Phase 3)

### Timeline

| Version | Date | Status |
|---------|------|--------|
| 0.2.0 | Jan 2026 | Facades active, no warnings |
| 0.3.0 | Feb 2026 | Facades deprecated, warnings issued |
| 0.4.0 | Aug 2026 | Facades removed, breaking change |

### Migration Steps

#### 1. Update Cargo.toml Dependencies

**Replace:**
```toml
[dependencies]
trustedge-receipts = "0.2.0"
trustedge-attestation = "0.2.0"
```

**With:**
```toml
[dependencies]
trustedge-core = "0.2.0"
```

#### 2. Update Import Statements

**Receipts:**
```rust
// Old:
use trustedge_receipts::{Receipt, create_receipt, assign_receipt};

// New:
use trustedge_core::{Receipt, create_receipt, assign_receipt};
```

**Attestation:**
```rust
// Old:
use trustedge_attestation::{Attestation, create_signed_attestation};

// New:
use trustedge_core::{Attestation, create_signed_attestation};
```

#### 3. Verify Compilation

```bash
cargo clean
cargo check
cargo test
```

### API Compatibility

**No breaking changes to APIs.** All type signatures, function names, and
behaviors remain identical. Only import paths change.

### Troubleshooting

**"Cannot find type in this scope"**
- Ensure `trustedge-core` is in Cargo.toml dependencies
- Update import paths from old crate names to `trustedge_core`

**"Multiple versions of trustedge-core"**
- Run `cargo tree` to identify dependencies using old facades
- Update all dependencies to use `trustedge-core` directly

**Shell crates (CLIs, WASM) fail to build**
- Should not happen - shell crates already use `trustedge-core`
- Report as bug if encountered

### Need Help?

Open an issue: https://github.com/TrustEdge-Labs/trustedge/issues
```

### CHANGELOG.md Entry
```markdown
## [0.3.0] - 2026-02-XX

### ⚠️ Deprecation Notices

**Facade crates deprecated:** `trustedge-receipts`, `trustedge-attestation`,
and `trustedge-trst-protocols` are now deprecated facades. All functionality
has been consolidated into `trustedge-core`.

**Timeline:**
- 0.3.0 (Feb 2026): Deprecation warnings issued
- 0.4.0 (Aug 2026): Facades will be removed from workspace

**Migration:** See [MIGRATION.md](MIGRATION.md) for upgrade instructions.

### Changed
- `trustedge-receipts` 0.3.0: Now a deprecated facade re-exporting from core
- `trustedge-attestation` 0.3.0: Now a deprecated facade re-exporting from core
- `trustedge-trst-protocols`: Already consolidated in 0.2.0 (Phase 3)

### Deprecated
- Module-level deprecation on facade crates with 6-month migration window
- Import paths from `trustedge_receipts::*` - use `trustedge_core::*` instead
- Import paths from `trustedge_attestation::*` - use `trustedge_core::*` instead
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual deprecation notices | `#[deprecated]` attribute | RFC 1270 (2015) | Compiler integration, rustdoc display |
| Per-item `#[deprecated]` | Module-level `#![deprecated]` | Rust 1.0+ | Inherit to all items, cleaner for crate deprecation |
| Yanking crates | Deprecation with migration period | RFC 1105 (2015) | Gentler transition, provides guidance |
| No SemVer tooling | `cargo-semver-checks` | 2022+ | Automated breaking change detection |

**Deprecated/outdated:**
- `#[rustc_deprecated]`: Internal-only for rustc; use `#[deprecated]` for libraries
- Immediate removal: RFC 1105 requires deprecation period before removal
- No `readme` field in Cargo.toml: Now standard for crates.io display

## Open Questions

1. **Should we yank 0.2.0 versions of facades when 0.4.0 removes them?**
   - What we know: Yanking prevents new projects from using specific versions
   - What's unclear: Whether to yank or leave 0.2.0 available for existing Cargo.lock files
   - Recommendation: Don't yank 0.2.0 - let existing dependents continue working with older lockfiles

2. **Do we need to update docs.rs configuration for deprecated crates?**
   - What we know: docs.rs has `package.metadata.docs.rs` configuration
   - What's unclear: Whether deprecated crates need special config
   - Recommendation: Keep existing config; rustdoc will show deprecation automatically

3. **Should trst-protocols get same treatment as receipts/attestation?**
   - What we know: trst-protocols was renamed in Phase 3, not moved to core
   - What's unclear: Whether it counts as "facade" or is legitimate protocol definitions crate
   - Recommendation: No - trst-protocols is legitimate (WASM-safe protocol types), not a facade

## Sources

### Primary (HIGH confidence)
- [RFC 1270: Deprecation](https://rust-lang.github.io/rfcs/1270-deprecation.html) - Official deprecation attribute specification
- [Rust Reference: Diagnostics](https://doc.rust-lang.org/reference/attributes/diagnostics.html) - Official attribute documentation
- [RFC 1105: API Evolution](https://rust-lang.github.io/rfcs/1105-api-evolution.html) - SemVer and deprecation strategy
- [Cargo Book: SemVer Compatibility](https://doc.rust-lang.org/cargo/reference/semver.html) - Breaking vs non-breaking changes
- [docs.rs: Metadata](https://docs.rs/about/metadata) - Cargo.toml configuration for docs.rs

### Secondary (MEDIUM confidence)
- [Rust GitHub Issue #47236](https://github.com/rust-lang/rust/issues/47236) - Deprecation on re-exports limitation
- [Rust GitHub Issue #82123](https://github.com/rust-lang/rust/issues/82123) - Re-export deprecation behavior
- [Rust Project Goals: Yank with Reason](https://rust-lang.github.io/rust-project-goals/2024h2/yank-crates-with-a-reason.html) - Yanking vs deprecation
- [SemVer 2.0.0 Specification](https://semver.org/) - Semantic versioning rules

### Tertiary (LOW confidence)
- Project memory: Phase 4 and 5 summaries document existing facade pattern
- WebSearch: General Rust ecosystem migration practices (verified against official sources)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Official Rust documentation and RFCs
- Architecture: HIGH - Patterns verified in Rust Reference and community practice
- Pitfalls: HIGH - Documented in Rust GitHub issues and RFC discussions

**Research date:** 2026-02-10
**Valid until:** 30 days (stable Rust features, unlikely to change)

**Coverage assessment:**
- ✓ Deprecation mechanism (`#[deprecated]`)
- ✓ Version management strategy (SemVer)
- ✓ Migration documentation patterns
- ✓ Known limitations (re-export warnings)
- ✓ Timeline recommendations (6-month window)
- ✓ Integration with existing facades
- ⚠️ Tooling automation (limited - manual migration required)

**Research quality:** Comprehensive. All critical domains covered with official sources. Known limitations documented. Patterns verified against existing Phase 4/5 facade implementations.
