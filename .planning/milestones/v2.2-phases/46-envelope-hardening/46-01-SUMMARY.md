<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: 46-envelope-hardening
plan: 01
subsystem: crypto
tags: [envelope, aes-256-gcm, hkdf, ed25519, v2-format]

# Dependency graph
requires: []
provides:
  - v2-only envelope seal/unseal in crates/core/src/envelope.rs
  - No v1 decrypt code paths exist anywhere in envelope.rs
affects: [47-key-encryption, downstream envelope consumers]

# Tech tracking
tech-stack:
  added: []
  patterns: [v2-only envelope format with single HKDF derivation and deterministic counter nonces]

key-files:
  created: []
  modified:
    - crates/core/src/envelope.rs

key-decisions:
  - "v1 envelope format removed entirely (not deprecated) — no v1 envelopes exist in production (solo dev, no external users)"
  - "unseal() is now a straight-line v2 decrypt — no try/fallback branching"

patterns-established:
  - "Envelope v2 pattern: single HKDF derivation per envelope, deterministic counter nonces (nonce_prefix || chunk_index || last_flag)"

requirements-completed: [ENV-01, ENV-02]

# Metrics
duration: 13min
completed: 2026-03-19
---

# Phase 46 Plan 01: Envelope Hardening — v1 Format Removal Summary

**Deleted decrypt_chunk_v1(), default_envelope_version(), and test_v1_legacy_fallback; unseal() is now straight-line v2-only with single HKDF derivation and no fallback branch**

## Performance

- **Duration:** 13 min
- **Started:** 2026-03-19T00:50:40Z
- **Completed:** 2026-03-19T01:03:33Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments
- Deleted `decrypt_chunk_v1()` — 59 lines of legacy per-chunk-salt decryption code removed
- Simplified `unseal()` from 66 lines (with v2_result wrapping + v1 fallback loop) to 30 lines (straight-line v2 decrypt)
- Deleted `test_v1_legacy_fallback` — 110 lines of dead test code removed
- Removed `default_envelope_version()` helper and its `#[serde(default = ...)]` attribute
- Removed `#[serde(default)]` from `hkdf_salt` field — v1 envelopes with missing salt no longer deserialize
- All 20 envelope tests pass on v2-only paths; full workspace passes (169 core tests, all crates)

## Task Commits

Each task was committed atomically:

1. **Task 1: Remove v1 envelope code and update tests** - `8aff537` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/core/src/envelope.rs` — Deleted v1 code paths: decrypt_chunk_v1(), default_envelope_version(), test_v1_legacy_fallback; simplified unseal() to v2-only

## Decisions Made
- v1 envelope format removed entirely rather than deprecated — consistent with user decision that no v1 envelopes exist in production
- ENV-01 satisfied by removal (stronger guarantee than deprecation warning)
- ENV-02 confirmed: seal() already produced v2 only (version: 2, at line 192) — no change needed

## Deviations from Plan

None — plan executed exactly as written.

## Issues Encountered
- `cargo fmt` pre-commit hook flagged trailing blank lines introduced by the deletions — ran `cargo fmt -p trustedge-core` and recommitted cleanly.

## Next Phase Readiness
- envelope.rs is now v2-only with no dead code; ready for Phase 46 remaining plans
- No downstream breakage: full workspace test suite passes

---
*Phase: 46-envelope-hardening*
*Completed: 2026-03-19*
