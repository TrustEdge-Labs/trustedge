# Phase 48: Archive Integrity Attacks - Context

**Gathered:** 2026-03-20
**Status:** Ready for planning

<domain>
## Phase Boundary

Security tests that attack .trst archives at the byte, chunk, and manifest level to verify detection. Tests prove the system's tamper-evidence claims from the threat model (T1, T2).

</domain>

<decisions>
## Implementation Decisions

### Test Scope
- SEC-01: Byte-level mutation of encrypted chunks (AES-GCM auth tag detection)
- SEC-02: Inject extra chunk file into archive (BLAKE3 chain break)
- SEC-03: Reorder chunk files (continuity chain detection)
- SEC-04: Modify manifest.json fields after signing (signature verification failure)

### Claude's Discretion
- Test file location and naming (separate security_tests.rs or extend existing acceptance.rs)
- Which specific byte(s) to mutate for SEC-01
- Which manifest field to modify for SEC-04
- Whether to use multi-chunk archives for more thorough testing
- Test helper functions for archive creation/mutation

</decisions>

<canonical_refs>
## Canonical References

- `crates/trst-cli/tests/integration_tests.rs` — existing A1-A5 tests (some overlap, new tests should go deeper)
- `crates/trst-cli/tests/acceptance.rs` — acceptance test patterns
- `crates/core/src/archive.rs` — read_archive, validate_archive
- `crates/core/src/chain.rs` — BLAKE3 continuity chain
- `crates/core/src/crypto.rs` — verify_manifest, sign_manifest
- `docs/technical/threat-model.md` — T1 and T2 threat categories
- `.planning/REQUIREMENTS.md` — SEC-01 through SEC-04

</canonical_refs>

---

*Phase: 48-archive-integrity-attacks*
*Context gathered: 2026-03-20*
