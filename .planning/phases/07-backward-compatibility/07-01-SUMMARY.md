---
phase: 07-backward-compatibility
plan: 01
subsystem: workspace-architecture
tags: [deprecation, semver, backward-compatibility, facades, migration]

# Dependency graph
requires:
  - phase: 04-receipts-integration
    provides: Re-export facade for receipts crate
  - phase: 05-attestation-integration
    provides: Re-export facade for attestation crate
provides:
  - Formal #[deprecated] attributes on facade crates
  - Version bump to 0.3.0 signals deprecation
  - README deprecation notices with migration guidance
  - 6-month migration timeline (Feb 2026 → Aug 2026)
affects: [07-02-migration-guide, 08-cleanup]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Module-level #![deprecated] attribute for crate deprecation"
    - "Version bump strategy (0.2.0 → 0.3.0) to signal breaking changes"
    - "README-first deprecation notices for crates.io visibility"
    - "Deprecated keyword in Cargo.toml metadata"

key-files:
  created:
    - crates/receipts/README.md
    - crates/attestation/README.md
  modified:
    - crates/receipts/src/lib.rs
    - crates/receipts/Cargo.toml
    - crates/attestation/src/lib.rs
    - crates/attestation/Cargo.toml

key-decisions:
  - "Module-level #![deprecated] chosen over per-item deprecation (Rust limitation: re-export warnings don't propagate reliably)"
  - "Version 0.3.0 signals deprecation with 6-month removal timeline (0.4.0 in Aug 2026)"
  - "README replaced entirely with deprecation notice (not appended) for maximum visibility on crates.io"
  - "Deprecated keyword added as first keyword in Cargo.toml for searchability"

patterns-established:
  - "Deprecation notice pattern: Timeline + Before/After code examples + MIGRATION.md link"
  - "DEPRECATED prefix in Cargo.toml description field"
  - "readme field added to Cargo.toml to ensure deprecation notice appears on crates.io"

# Metrics
duration: 3m 52s
completed: 2026-02-11
---

# Phase 7 Plan 01: Facade Deprecation Summary

**Formal Rust #[deprecated] attributes added to receipts and attestation facades with 0.3.0 version bump and README migration guidance**

## Performance

- **Duration:** 3 min 52 sec
- **Started:** 2026-02-11T01:21:08Z
- **Completed:** 2026-02-11T01:25:00Z
- **Tasks:** 3
- **Files modified:** 6

## Accomplishments

- Both facade crates (receipts, attestation) formally deprecated with module-level `#![deprecated]` attributes
- Version bumped from 0.2.0 to 0.3.0 to signal breaking change on semver timeline
- README files replaced with prominent deprecation notices including migration timeline and code examples
- Workspace compiles successfully with all tests passing (325+ tests)
- Thin shells (CLIs, WASM) compile clean without deprecation warnings (they import from core, not facades)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add formal deprecation to receipts facade crate** - `e49dcf0` (feat)
2. **Task 2: Add formal deprecation to attestation facade crate** - `ab09680` (feat)
3. **Task 3: Verify workspace compiles with deprecation warnings** - `88fefe8` (chore)

## Files Created/Modified

- `crates/receipts/src/lib.rs` - Added #![deprecated] attribute with removal timeline and enhanced module docs
- `crates/receipts/Cargo.toml` - Version 0.3.0, deprecated keyword, DEPRECATED description prefix, readme field
- `crates/receipts/README.md` - Replaced with deprecation notice (from 480 lines to 62 lines)
- `crates/attestation/src/lib.rs` - Added #![deprecated] attribute with removal timeline and enhanced module docs
- `crates/attestation/Cargo.toml` - Version 0.3.0, deprecated keyword, DEPRECATED description prefix, readme field
- `crates/attestation/README.md` - Replaced with deprecation notice (from 296 lines to 65 lines)

## Decisions Made

**Module-level deprecation chosen over per-item:**
- Research revealed that `#[deprecated]` on `pub use` re-exports doesn't reliably trigger warnings (known Rust limitation from issues #47236, #82123)
- Module-level `#![deprecated]` inherits to all items and works reliably

**Version 0.3.0 with 6-month timeline:**
- 0.2.0 → 0.3.0 bump signals deprecation (minor bump appropriate for pre-1.0)
- Timeline: 0.3.0 (Feb 2026) deprecation warnings, 0.4.0 (Aug 2026) removal
- Follows RFC 1105 guidance for at least one minor release with deprecation

**README replacement strategy:**
- Replaced entire README with deprecation notice instead of prepending
- Ensures maximum visibility on crates.io and docs.rs
- Original detailed docs still available in git history
- README now focuses solely on migration path

**Cargo.toml metadata:**
- "deprecated" as first keyword for crates.io search visibility
- DEPRECATED prefix in description for package manager display
- readme field ensures custom README appears on crates.io

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None - all tasks completed without issues.

## Self-Check

**Files verified:**
```
✔ crates/receipts/src/lib.rs - Contains #![deprecated] attribute
✔ crates/receipts/Cargo.toml - Version 0.3.0 with deprecated keyword
✔ crates/receipts/README.md - Deprecation notice present
✔ crates/attestation/src/lib.rs - Contains #![deprecated] attribute
✔ crates/attestation/Cargo.toml - Version 0.3.0 with deprecated keyword
✔ crates/attestation/README.md - Deprecation notice present
```

**Commits verified:**
```
✔ e49dcf0 - receipts facade deprecation
✔ ab09680 - attestation facade deprecation
✔ 88fefe8 - workspace verification
```

**Workspace verification:**
```
✔ cargo check --workspace - Passed
✔ cargo clippy --workspace -- -D warnings - Passed
✔ cargo test --workspace - Passed (325+ tests)
✔ Thin shells compile clean (no facade imports)
```

## Self-Check: PASSED

All claimed files exist, all commits present, workspace builds successfully.

## Next Phase Readiness

Ready for 07-02 (Migration Guide):
- Facade deprecation formalized with compiler warnings
- Version bumps signal breaking changes
- README deprecation notices provide initial migration guidance
- Need comprehensive MIGRATION.md with full upgrade instructions

**Blockers:** None

**Note for next plan:**
- MIGRATION.md should be created at workspace root (referenced in deprecation notices)
- Should cover all three facade deprecations (receipts, attestation, and trst-protocols from Phase 3)
- Include troubleshooting section for common migration issues

---
*Phase: 07-backward-compatibility*
*Completed: 2026-02-11*
