# Project State

## Project Reference

See: .planning/PROJECT.md (updated 2026-02-11)

**Core value:** A single, reliable trustedge-core library that owns all cryptographic operations (envelope encryption, signing, receipts, attestation, archives) with production-quality YubiKey hardware integration — thin CLIs and WASM bindings are just frontends.

**Current focus:** Phase 12 complete — v1.1 milestone complete, all 4 phases done

## Current Position

Phase: 12 of 12 (CI Integration)
Plan: 1 of 1 in current phase
Status: Complete
Last activity: 2026-02-12 — Plan 12-01 complete (Unconditional YubiKey CI validation)

Progress: [███████████████████░] 79% (23/29 total plans across all phases)

## Performance Metrics

**Velocity:**
- Total plans completed: 23 (17 v1.0 + 6 v1.1)
- Average duration: 5.5 min
- Total execution time: 2.1 hours

**By Phase (v1.0 complete):**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 1. Foundation | 4 | 32 min | 8.0 min |
| 2. Error Handling | 3 | 15 min | 5.0 min |
| 3. trst-core Integration | 2 | 9 min | 4.5 min |
| 4. Receipts Integration | 1 | 6 min | 6.0 min |
| 5. Attestation Integration | 1 | 5 min | 5.0 min |
| 6. Feature Flags | 2 | 10 min | 5.0 min |
| 7. Backward Compatibility | 2 | 11 min | 5.5 min |
| 8. Validation | 2 | 8 min | 4.0 min |

**By Phase (v1.1 in progress):**

| Phase | Plans | Total | Avg/Plan |
|-------|-------|-------|----------|
| 9. Cleanup | 1 | 5 min | 5.0 min |
| 10. Backend Rewrite | 2 | 15 min | 7.5 min |
| 11. Test Infrastructure | 2 | 10 min | 5.0 min |
| 12. CI Integration | 1 | 3 min | 3.0 min |

**Recent Trend:**
- Last 5 plans: 3-8 min range
- Trend: Stable (CI integration and test infrastructure complete)
| Phase 12 P01 | 3 | 2 tasks | 4 files |

## Accumulated Context

### Decisions

Decisions are logged in PROJECT.md Key Decisions table.
Recent decisions affecting current work:

- **v1.1 Start**: Scorched-earth YubiKey rewrite — external review found critical issues: manual DER, silent fallbacks, placeholder keys
- **v1.1 Stack**: yubikey crate stable API only (drop `untested` feature), rcgen for X.509 (replace 1,000+ lines manual DER), fail-closed hardware design
- **v1.1 Testing**: No placeholder keys or signatures — every key and signature must come from real cryptographic operations
- **Phase 9 Cleanup**: Deleted entire YubiKey implementation (8,117 lines: backend, tests, examples) for clean v1.1 rewrite; preserved yubikey dependency and feature flag for reuse
- **Phase 10-01 Backend**: Clean YubiKey PIV backend (487 lines) with ECDSA P-256/RSA-2048 signing, public key extraction, slot enumeration, PIN verification, fail-closed design; key generation and attestation deferred (private policy types in yubikey 0.7)
- **Phase 10-02 Certificates**: X.509 certificate generation via rcgen's RemoteKeyPair with hardware-backed signing; YubiKey backend registered in UniversalBackendRegistry (auto-discovery when feature enabled)
- [Phase 11]: Remove unused create_test_config() helper - tests use inline config creation
- [Phase 11]: All hardware tests marked with #[ignore] to prevent CI failures
- [Phase 12]: CI enforces YubiKey validation unconditionally (fail-loud if dependencies missing) to prevent silent breakage
- [Phase 12]: Use --lib flag to run only simulation tests (18 tests), skip hardware integration tests (4 tests marked #[ignore])

### Pending Todos

None yet.

### Blockers/Concerns

**Phase 10 (Backend Rewrite):**
- rcgen custom signer API callback pattern needs investigation during planning (how to integrate hardware-backed key signing)
- PKCS#11 key attribute extraction may vary by YubiKey firmware version

**General:**
- Hardware tests require physical YubiKey 5 series with PIV applet enabled
- PCSC daemon (pcscd) must be running for hardware operations

## Session Continuity

Last session: 2026-02-12 (phase 12 execution)
Stopped at: Phase 12 Plan 01 complete (Unconditional YubiKey CI validation)
Resume file: Phase 12 complete - CI enforces YubiKey compilation and simulation tests on every PR
