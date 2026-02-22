# Phase 32: Workspace Cleanup - Context

**Gathered:** 2026-02-22
**Status:** Ready for planning

<domain>
## Phase Boundary

Delete deprecated facade crates (trustedge-receipts, trustedge-attestation) from the workspace and isolate Tier 2 experimental crates (trustedge-pubky, trustedge-pubky-advanced) into a separate workspace so their dependency graph does not contaminate the shared Cargo.lock. Clean all references from documentation and CI.

</domain>

<decisions>
## Implementation Decisions

### Facade crate removal
- Check crates.io first before deleting — if published, yank before removing from disk
- Delete crate directories entirely from disk (rm -rf crates/receipts/ and crates/attestation/) — git history preserves them
- Remove all traces from documentation — no historical notes, clean deletion
- Don't scan for internal imports — confident nothing depends on them; cargo build will catch any issues

### Pubky isolation strategy
- Move to separate workspace at `crates/experimental/` with its own Cargo.toml workspace
- Experimental workspace depends on trustedge-core via path dependency (`../../core`)
- CI only builds/tests the main workspace — experimental crates are opt-in for developers, no CI job
- Experimental workspace Cargo.lock is gitignored — not tracked in git

### Reference cleanup scope
- Section rewrite, not surgical removal — rewrite entire affected sections (Architecture Overview, etc.) to reflect the new reality
- Update crate count in CLAUDE.md, drop Tier 1/Tier 2 classification since experimental crates are in a separate workspace
- Delete dependency entries for removed/moved crates from DEPENDENCIES.md entirely
- Simplify ci-check.sh — remove tiered CI logic (core blocking, experimental non-blocking), only cover the main workspace

### Dependency hygiene
- Run cargo-machete to identify and remove unused workspace-level deps after crate removal
- Lower the dependency tree baseline (currently 70) to match the new, smaller dep tree
- Explicitly verify that pubky-specific heavy deps (rsa, x25519-dalek) are gone from the root Cargo.lock after isolation

### Claude's Discretion
- Exact order of operations (delete facades first vs move pubky first)
- How to structure the experimental workspace Cargo.toml
- Which workspace deps to remove (cargo-machete determines this)
- New dependency tree baseline number (based on actual count after cleanup)

</decisions>

<specifics>
## Specific Ideas

No specific requirements — open to standard approaches

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope

</deferred>

---

*Phase: 32-workspace-cleanup*
*Context gathered: 2026-02-22*
