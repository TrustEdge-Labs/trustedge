<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-22)

**Core value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.
**Current focus:** Phase 35 — HKDF Infrastructure

## Current Position

Phase: 35 of 37 (HKDF Infrastructure)
Plan: 1 of TBD in current phase
Status: In progress
Last activity: 2026-02-23 — Completed 35-01: HKDF workspace dependency + HKDF-SHA256 envelope key derivation

Progress: [█░░░░░░░░░] 10%

## Performance Metrics

**Cumulative (all milestones):**
- v1.0: 8 phases, 17 plans, 31 tasks
- v1.1: 4 phases, 6 plans, 8 tasks
- v1.2: 2 phases, 4 plans, 9 tasks
- v1.3: 4 phases, 5 plans, 7 tasks
- v1.4: 5 phases, 5 plans, 10 tasks
- v1.5: 4 phases, 8 plans, 16 tasks
- v1.6: 3 phases, 6 plans, 11 tasks
- v1.7: 4 phases, 10 plans, 18 tasks
- **Total prior: 34 phases, 61 plans, 110 tasks**

**v1.8 (current):**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 35. HKDF Infrastructure | 1 | 2 tasks | 2/plan |
| 36. Envelope Format Migration | TBD | - | - |
| 37. Keyring Hardening | TBD | - | - |

*Updated after each plan completion*

## Accumulated Context

### Decisions

- Envelope.rs: Replace PBKDF2-per-chunk with HKDF hierarchical derivation (Tink AES-GCM-HKDF streaming model adapted to ECDH)
- Keyring backends: Harden PBKDF2 iterations (100k → 600k) and salt length (16 → 32 bytes)
- Experimental crates (pubky-advanced): Out of scope for this milestone
- auth.rs BLAKE3::derive_key: Already correct, no changes needed
- software_hsm.rs PBKDF2: Already hardened to 600k in prior commit, out of scope
- 35-01: HKDF-SHA256 chosen over PBKDF2 for ECDH key extraction — RFC 5869 Extract+Expand with b"TRUSTEDGE_ENVELOPE_V1" domain separation
- 35-01: pbkdf2_iterations field preserved in ChunkManifest at literal 100_000u32 for format compatibility until Phase 36

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-23
Stopped at: Completed 35-01-PLAN.md — HKDF dependency added, PBKDF2 CatKDF replaced with HKDF-SHA256
Resume file: None

---
*Last updated: 2026-02-23 after 35-01 completed*
