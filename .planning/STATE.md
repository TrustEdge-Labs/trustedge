---
gsd_state_version: 1.0
milestone: v2.1
milestone_name: Data Lifecycle & Hardware Integration
status: planning
stopped_at: Completed 44-01-PLAN.md
last_updated: "2026-03-18T00:41:01.908Z"
last_activity: 2026-03-16 -- Roadmap created for v2.1
progress:
  total_phases: 3
  completed_phases: 2
  total_plans: 6
  completed_plans: 5
  percent: 0
---

<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-03-16)

**Core value:** Prove that data from an edge device has not been tampered with -- from capture to verification -- using cryptographic signatures, continuity chains, and verifiable receipts.
**Current focus:** v2.1 Data Lifecycle & Hardware Integration -- Phase 42: Named Archive Profiles

## Current Position

Phase: 42 of 44 (Named Archive Profiles)
Plan: Ready to plan
Status: Ready to plan
Last activity: 2026-03-16 -- Roadmap created for v2.1

Progress: [..........] 0%

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
- v1.8: 3 phases, 4 plans, 8 tasks
- v2.0: 4 phases, 8 plans, ~17 tasks
- **Total: 41 phases, 69 plans, ~135 tasks**

## Accumulated Context

### Decisions

Cleared -- see PROJECT.md Key Decisions table for full history.

Key design decisions to lock before coding each phase:
- [Phase 42 — before code]: Typed enum variants (Sensor, Audio, Log) each need at least one required field not present in Generic for unambiguous serde deserialization; write canonical JSON fixture tests before adding any new variant
- [Phase 43 — before code]: HKDF-SHA256 derives XChaCha20Poly1305 chunk key from Ed25519 signing key (domain tag "TRUSTEDGE_TRST_CHUNK_KEY"); wrap + unwrap updated atomically; v2.0 demo archives (hardcoded key) will not decrypt with new unwrap — document in CHANGELOG
- [Phase 44 — before code]: Signature format for ECDSA P-256 must be decided before implementation: "ecdsa-p256:<base64_der>" alongside existing "ed25519:..." format; confirm p256 crate version and workspace compatibility
- [Phase 42-named-archive-profiles]: ProfileMetadata variant order: CamVideo, Sensor, Audio, Log, Generic - each typed variant has unique required fields for unambiguous untagged serde deserialization
- [Phase 42-named-archive-profiles]: AudioMetadata.sample_rate_hz is u32 (integer Hz); SensorMetadata.sample_rate_hz is f64 (fractional Hz for precision sensors)
- [Phase 42-named-archive-profiles]: Negative float CLI values require --flag=VALUE syntax; leading '-' is parsed as a flag prefix by clap
- [Phase 42-named-archive-profiles]: SensorMetadata, AudioMetadata, LogMetadata re-exported from trustedge-core to keep downstream import paths consistent
- [Phase 43]: HKDF-SHA256 with empty salt + domain tag TRUSTEDGE_TRST_CHUNK_KEY derives deterministic chunk key from Ed25519 device key
- [Phase 43]: Chunk files now [nonce:24][ciphertext:N]; BLAKE3 hashes cover nonce+ciphertext to match validate_archive disk reads
- [Phase 43]: Used process::exit() on all error paths in handle_unwrap() to guarantee no partial output file is written on signature/continuity/decryption failure
- [Phase 44-yubikey-cli-integration]: ECDSA P-256 signature format: ecdsa-p256:<base64_sec1_uncompressed> for keys, ecdsa-p256:<base64_der> for DER signatures; p256 crate handles SHA-256 hashing internally

### Pending Todos

None.

### Blockers/Concerns

- Hardware tests require physical YubiKey 5 series (carried from v1.1) — Phase 44 acceptance tests need hardware
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) risk-accepted (carried from v1.3)
- Phase 44: Confirm p256 crate workspace version compatibility before coding (low risk, well-maintained crate family)

## Session Continuity

Last session: 2026-03-18T00:41:01.906Z
Stopped at: Completed 44-01-PLAN.md
Resume file: None

---
*Last updated: 2026-03-16 after v2.1 roadmap created*
