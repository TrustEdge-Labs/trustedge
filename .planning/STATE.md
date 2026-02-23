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
Plan: 0 of TBD in current phase
Status: Ready to plan
Last activity: 2026-02-22 — v1.8 roadmap created, phases 35-37 defined

Progress: [░░░░░░░░░░] 0%

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
| 35. HKDF Infrastructure | TBD | - | - |
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

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from prior milestones)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)

## Session Continuity

Last session: 2026-02-22
Stopped at: Roadmap created — Phase 35 ready to plan
Resume file: None

---
*Last updated: 2026-02-22 after v1.8 roadmap created*
