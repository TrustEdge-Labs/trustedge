---
phase: 66-crypto-cli-hardening
plan: 01
subsystem: crypto
tags: [aes-gcm, nonce, network-chunk, transport, cryptography]

# Dependency graph
requires:
  - phase: 65-key-material-safety
    provides: PrivateKey field visibility hardening that this phase builds on
provides:
  - NetworkChunk::new() with mandatory explicit nonce parameter (4-arg signature)
  - Elimination of zero-nonce default constructor from all workspace crates
affects: [trustedge-core, transport, envelope, trustedge-client]

# Tech tracking
tech-stack:
  added: []
  patterns: ["Mandatory nonce parameter pattern: all NetworkChunk construction requires explicit nonce supply"]

key-files:
  created: []
  modified:
    - crates/core/src/lib.rs
    - crates/core/src/transport/tcp.rs
    - crates/core/src/transport/quic.rs
    - crates/core/src/envelope.rs
    - crates/core/src/bin/trustedge-client.rs

key-decisions:
  - "Merge new_with_nonce() into new() rather than deprecating: forces compile-time error at all old call sites, no migration path to old insecure behavior"
  - "Test callers use [1u8; NONCE_LEN] dummy nonce: test nonces are not cryptographically sensitive; real production paths already supply proper random nonces"

patterns-established:
  - "NetworkChunk construction: always call NetworkChunk::new(seq, data, manifest, nonce) with a real nonce from the encryption step"

requirements-completed: [CRYPT-01]

# Metrics
duration: 15min
completed: 2026-03-25
---

# Phase 66 Plan 01: NetworkChunk Mandatory Nonce Summary

**NetworkChunk::new() now requires an explicit nonce parameter, eliminating the silent zero-nonce default that was a cryptographic hazard in AES-256-GCM usage**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-03-25T00:30:00Z
- **Completed:** 2026-03-25T00:45:00Z
- **Tasks:** 1
- **Files modified:** 5

## Accomplishments

- Removed the 3-argument `NetworkChunk::new()` that silently defaulted to `[0; NONCE_LEN]`
- Renamed `new_with_nonce()` to `new()` with the same 4-parameter signature (nonce mandatory)
- Updated 3 test call sites (tcp.rs x2, quic.rs x1) to pass `[1u8; NONCE_LEN]` test nonces
- Updated 5 production call sites (envelope.rs x1, trustedge-client.rs x4) to use renamed `new()`
- All 22 transport tests and 20 envelope tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1: Merge new_with_nonce into new and update all callers** - `967c7cd` (fix)

## Files Created/Modified

- `crates/core/src/lib.rs` - Removed zero-nonce `new()`, renamed `new_with_nonce()` to `new()` with 4-param signature
- `crates/core/src/transport/tcp.rs` - Updated 2 test callers to pass explicit nonce
- `crates/core/src/transport/quic.rs` - Updated 1 test caller to pass explicit nonce
- `crates/core/src/envelope.rs` - Renamed `new_with_nonce` call to `new`
- `crates/core/src/bin/trustedge-client.rs` - Renamed 4 `new_with_nonce` calls to `new`

## Decisions Made

- Merged `new_with_nonce()` into `new()` rather than deprecating: any code that called the old zero-nonce `new()` now fails to compile, forcing explicit nonce supply at all call sites.
- Test callers use `[1u8; NONCE_LEN]` (all-ones byte): acceptable for tests since the test code only exercises transport framing/validation, not AES-GCM decryption.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- A `git stash pop` during verification unexpectedly reverted some file changes. Re-applied all changes after confirming the stash had been a pre-existing worktree state, not related to this plan's changes. All changes verified complete before commit.

## Known Stubs

None.

## Next Phase Readiness

- Plan 66-01 complete: NetworkChunk nonce API is hardened
- Plan 66-02 (CliExitError for trst exit codes) is ready to execute next
- No blockers

---
*Phase: 66-crypto-cli-hardening*
*Completed: 2026-03-25*
