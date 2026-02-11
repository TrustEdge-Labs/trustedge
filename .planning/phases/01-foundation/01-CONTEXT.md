<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 1: Foundation - Context

**Gathered:** 2026-02-09
**Status:** Ready for planning

<domain>
## Phase Boundary

Establish baseline metrics and create the layered module hierarchy skeleton in trustedge-core — before any code merges begin. Deliverables: dependency graph visualization, test inventory baseline, module hierarchy skeleton, API surface snapshot. No code is moved or merged in this phase.

</domain>

<decisions>
## Implementation Decisions

### Module hierarchy
- Create 6-layer directory structure in trustedge-core/src/: primitives/, protocols/, applications/, io/ (backends/ and transport/ already exist)
- Directories + mod.rs files only — no code moves, no re-exports, just the scaffolding
- Existing flat modules (crypto.rs, envelope.rs, chain.rs, etc.) stay in place — moves happen in later phases
- New directories sit alongside existing backends/ and transport/ (flat layout, no src/layers/ parent)
- Each mod.rs gets layer contract documentation: what belongs here, dependency rules ("never imports from protocols or above"), and what will live here after consolidation

### Duplication audit
- Map both exact duplicates AND near-duplicates (functions/types that do the same thing with minor differences)
- Table summary format: one line per finding with module paths, crates affected, duplicate type (exact/near), and best-implementation recommendation
- Also map cross-crate dependency usage (who imports what from whom) to verify merge order
- Output: .planning/phases/01-foundation/AUDIT.md

### Test baseline
- Per-crate AND per-module granularity (e.g., "core::backends: 15, core::envelope: 12")
- Include full test names per crate/module, not just counts — pinpoints exactly what's missing if a count drops
- Create scripts/test-inventory.sh that outputs current counts in the same format as the baseline
- Baseline snapshot saved to .planning/phases/01-foundation/TEST-BASELINE.md
- Script can be re-run after each subsequent phase to compare against baseline

### Tooling
- Install all four recommended tools: cargo-semver-checks, cargo-modules, cargo-hack, cargo-machete
- Add cargo-semver-checks and cargo-hack to CI (GitHub Actions) immediately in Phase 1 — catches regressions from the start
- Also update scripts/ci-check.sh with the new tool checks
- Save API surface snapshot as a baseline artifact in the phase directory (not just pass/fail gate)

### Claude's Discretion
- Exact cargo-modules visualization format and output
- How to structure the dependency graph output (text, mermaid, DOT, etc.)
- cargo-machete findings format and whether to act on unused deps in Phase 1 or defer

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches. The key principle is: Phase 1 produces analysis artifacts and scaffolding, not code changes. Everything should be a reference point for future phases.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 01-foundation*
*Context gathered: 2026-02-09*
