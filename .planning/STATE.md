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
**Current focus:** Phase 37 — Keyring Hardening

## Current Position

Phase: 37 of 37 (Keyring Hardening) — IN PROGRESS
Plan: 1 of 1 complete (Phase 37 Plan 01 done)
Status: Phase 37 Plan 01 complete — PBKDF2 hardened to OWASP 2023 parameters
Last activity: 2026-02-24 — Completed 37-01: PBKDF2 600k iterations, 32-byte salts in both keyring backends, all 162 tests pass

Progress: [█████░░░░░] 50%

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
| 36. Envelope Format Migration | 2 | 4 tasks | 2/plan |
| 37. Keyring Hardening | 1 | 2 tasks | 2/plan |

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
- 36-01: Single HKDF derivation per envelope produces 40-byte OKM — 32-byte AES-256-GCM key + 8-byte nonce prefix
- 36-01: Deterministic nonce layout: nonce_prefix[0..8] || chunk_index[1..4] (BE u32 low 3 bytes) || last_flag (0xFF/0x00)
- 36-01: Envelope.version field (serde default=1) enables v1/v2 dispatch in Plan 02 decrypt path
- 36-01: ChunkManifest key_derivation_salt and pbkdf2_iterations zeroed for v2 envelopes (serde shape preserved)
- 36-02: try-then-fallback over version-field dispatch: AES-GCM auth tag failure is the definitive v2 vs v1 discriminator
- 36-02: decrypt_chunk_v2 takes no SigningKey param — pre-derived encryption_key passed in; avoids clippy unused-param warning
- 36-02: V2 key zeroized before fallback decision; v1 path re-derives per-chunk with inline zeroize
- 37-01: PBKDF2 iterations raised from 100k to 600k per OWASP 2023 PBKDF2-HMAC-SHA256 recommendation
- 37-01: Salt length raised from 16 to 32 bytes in both keyring backends and CLI
- 37-01: derive_key key_id ([u8; 16]) uses first 16 bytes of 32-byte salt as key isolator in CLI (signature preserved)

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-24
Stopped at: Completed 37-01-PLAN.md — PBKDF2 hardened to OWASP 2023 parameters (600k iterations, 32-byte salts); 162 tests pass
Resume file: None

---
*Last updated: 2026-02-24 after 37-01 completed*
