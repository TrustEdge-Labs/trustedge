<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- 🚧 **v1.1 YubiKey Integration Overhaul** - Phases 9-12 (in progress)

## Phases

<details>
<summary>✅ v1.0 Consolidation (Phases 1-8) - SHIPPED 2026-02-11</summary>

Consolidated TrustEdge from 10 scattered crates into monolithic core with thin CLI/WASM shells. Zero API breaking changes, 98.6% test retention (343 tests), WASM compatibility preserved. Eliminated ~2,500 LOC duplication, removed 21 unused dependencies. Established 6-layer architecture, unified error types, migrated receipts and attestation into core, deprecated facade crates with 6-month migration window.

**See:** `.planning/milestones/v1.0-ROADMAP.md` for full phase details.

</details>

### 🚧 v1.1 YubiKey Integration Overhaul (In Progress)

**Milestone Goal:** Delete broken YubiKey backend and rewrite from scratch with fail-closed design, battle-tested libraries only, and comprehensive real testing.

#### Phase 9: Cleanup

**Goal:** Remove all broken YubiKey code — no partial retention, scorched-earth deletion of 3,263-line backend and 8 test files.

**Depends on:** Phase 8 (v1.0 complete)

**Requirements:** CLEAN-01, CLEAN-02, CLEAN-03, CLEAN-04

**Success Criteria** (what must be TRUE):
1. Old yubikey.rs (3,263 lines) is completely deleted from crates/core/src/backends/
2. All 8 YubiKey test files are deleted from crates/core/tests/
3. `untested` feature flag is removed from yubikey dependency in Cargo.toml
4. Codebase contains zero placeholder keys, placeholder signatures, or manual DER encoding functions (verified by grep)

**Plans:** 1 plan

Plans:
- [x] 09-01-PLAN.md — Scorched-earth deletion of YubiKey backend, tests, examples, and placeholder code

#### Phase 10: Backend Rewrite

**Goal:** Implement production-quality YubiKey backend using yubikey crate stable API, rcgen for X.509, and fail-closed error handling — no software fallbacks, no manual crypto.

**Depends on:** Phase 9

**Requirements:** BACK-01, BACK-02, BACK-03, BACK-04, BACK-05, BACK-06, BACK-07, BACK-08, BACK-09, BACK-10, BACK-11, BACK-12, BACK-13, BACK-14

**Success Criteria** (what must be TRUE):
1. YubiKey backend implements full UniversalBackend trait (perform_operation, supports_operation, get_capabilities, backend_info, list_keys)
2. Backend returns BackendError::HardwareError when hardware unavailable — never falls back to software crypto
3. ECDSA P-256 and RSA-2048 signing work via PIV slots with real hardware keys (Ed25519 not supported by PIV hardware — returns UnsupportedOperation)
4. X.509 certificate generation uses rcgen library with hardware-backed signing (zero manual ASN.1/DER encoding)
5. All cryptographic operations use battle-tested libraries only — yubikey crate (stable API), rcgen, der/spki crates from RustCrypto

**Plans:** 2 plans (2 waves)

Plans:
- [x] 10-01-PLAN.md — Backend foundation + PIV operations + UniversalBackend trait implementation (Wave 1)
- [x] 10-02-PLAN.md — X.509 certificate generation with rcgen + registry integration (Wave 2)

#### Phase 11: Test Infrastructure

**Goal:** Build comprehensive test suite with simulation tests (no hardware, always-run in CI) and strict hardware integration tests (require physical YubiKey, gated with #[ignore]) — every test validates actual behavior with real assertions.

**Depends on:** Phase 10

**Requirements:** TEST-01, TEST-02, TEST-03, TEST-04, TEST-05, TEST-06

**Success Criteria** (what must be TRUE):
1. Simulation tests validate capability reporting, slot parsing, error mapping, and config validation without requiring hardware — run in CI on every commit
2. Hardware integration tests use #[ignore] and verify real signing operations, key extraction, and certificate generation with physical YubiKey
3. Anti-pattern tests prove: signing fails without hardware (no fallback), empty slots return errors (no placeholder keys), no tests auto-pass
4. Every test function contains at least one assertion (assert!, assert_eq!, or expect) that validates actual output
5. Certificate generation round-trip works: generate cert via rcgen, parse with x509-cert, verify signature matches hardware public key

**Plans:** 2 plans (1 wave)

Plans:
- [ ] 11-01-PLAN.md — Simulation tests in yubikey.rs #[cfg(test)] module (Wave 1)
- [ ] 11-02-PLAN.md — Hardware integration tests in yubikey_integration.rs with #[ignore] (Wave 1)

#### Phase 12: CI Integration

**Goal:** Enable continuous validation of YubiKey feature — CI always compiles with --features yubikey, runs simulation tests on every PR, and clippy passes with zero warnings.

**Depends on:** Phase 11

**Requirements:** CI-01, CI-02, CI-03

**Success Criteria** (what must be TRUE):
1. CI workflow always compiles crates/core with --features yubikey regardless of dependency availability
2. CI runs YubiKey simulation tests (non-#[ignore]) on every pull request
3. Clippy passes with --features yubikey with zero warnings

**Plans:** TBD

Plans:
- [ ] TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 9 → 10 → 11 → 12

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 1-8. [v1.0 phases] | v1.0 | 17/17 | Complete | 2026-02-11 |
| 9. Cleanup | v1.1 | 1/1 | Complete | 2026-02-11 |
| 10. Backend Rewrite | v1.1 | 2/2 | Complete | 2026-02-11 |
| 11. Test Infrastructure | v1.1 | 0/2 | Planned | - |
| 12. CI Integration | v1.1 | 0/? | Not started | - |

---
*Last updated: 2026-02-11 after Phase 11 planning complete*
