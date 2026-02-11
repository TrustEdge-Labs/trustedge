# Requirements: TrustEdge v1.1 — YubiKey Integration Overhaul

**Defined:** 2026-02-11
**Core Value:** A single, reliable trustedge-core library with production-quality YubiKey hardware integration — no manual crypto, no software fallbacks, no placeholder keys.

## Foundational Constraints

These apply to ALL requirements in this milestone:

1. **No manual/invented cryptography.** Use only battle-tested, well-maintained libraries and functions. Follow NIST and industry standards (SP 800-73-4, RFC 5280, PKCS#11 v2.40) at all times.
2. **All tests must be specific and accurate.** No placeholders, automatic passes, mocked success, or other assumptions. Every test must exercise actual features and capabilities with real assertions.
3. **Fail-closed design.** Hardware unavailable = error returned to caller. Never silently fall back to software crypto.

## v1.1 Requirements

### Cleanup (CLEAN)

- [ ] **CLEAN-01**: Old yubikey.rs (3,263 lines) is deleted entirely — no partial retention
- [ ] **CLEAN-02**: All 8 existing YubiKey test files are deleted (yubikey_simulation_tests.rs, yubikey_hardware_tests.rs, yubikey_integration.rs, yubikey_strict_hardware.rs, yubikey_piv_analysis.rs, yubikey_certificate_debug.rs, yubikey_hardware_detection.rs, yubikey_real_operations.rs)
- [ ] **CLEAN-03**: `untested` feature flag removed from yubikey dependency in Cargo.toml
- [ ] **CLEAN-04**: All placeholder keys, placeholder signatures, and manual DER encoding functions are gone from codebase (verified by grep)

### Backend Rewrite (BACK)

- [ ] **BACK-01**: New YubiKey backend implements UniversalBackend trait (perform_operation, supports_operation, get_capabilities, backend_info, list_keys)
- [ ] **BACK-02**: Backend uses `yubikey` crate stable API only — no `untested` feature, no PKCS#11 for core operations
- [ ] **BACK-03**: Backend returns `BackendError::HardwareError` when YubiKey hardware is unavailable — never falls back to software
- [ ] **BACK-04**: Ed25519 signing works via PIV slot with real hardware key
- [ ] **BACK-05**: ECDSA P-256 signing works via PIV slot with real hardware key
- [ ] **BACK-06**: RSA-2048 signing works via PIV slot with real hardware key
- [ ] **BACK-07**: Public key extraction retrieves actual DER-encoded SPKI from hardware slot
- [ ] **BACK-08**: PIV slot enumeration detects which slots (9a/9c/9d/9e) have keys
- [ ] **BACK-09**: X.509 certificate reading retrieves existing certificates from PIV slots
- [ ] **BACK-10**: X.509 certificate generation uses rcgen with hardware-backed signing (CustomKeyPair delegates to YubiKey)
- [ ] **BACK-11**: PKCS#11 error codes map to human-readable BackendError messages
- [ ] **BACK-12**: PIN verification with retry limit (max 3 attempts, then error)
- [ ] **BACK-13**: `backend_info()` reports `available: true` only when real hardware session exists
- [ ] **BACK-14**: Zero manual ASN.1/DER encoding — all encoding via der/spki/rcgen crates

### Testing (TEST)

- [ ] **TEST-01**: Simulation tests validate capability reporting, slot parsing, error mapping, and config validation — run without hardware in CI
- [ ] **TEST-02**: Hardware integration tests use `#[ignore]` and require physical YubiKey — test real signing, key extraction, certificate operations
- [ ] **TEST-03**: Anti-pattern tests verify: signing fails without hardware (no fallback), no placeholder keys returned for empty slots, no auto-pass behavior
- [ ] **TEST-04**: Every test function contains at least one `assert!`, `assert_eq!`, or `expect` that validates actual output — no tests that just call a function and return Ok(())
- [ ] **TEST-05**: Certificate generation round-trip test: generate cert via rcgen → parse back with x509-cert → verify signature matches hardware public key
- [ ] **TEST-06**: Negative tests for invalid slot IDs, wrong PIN, unsupported algorithms, and hardware disconnection scenarios

### CI (CI)

- [ ] **CI-01**: CI always compiles with `--features yubikey` — not conditional on dependency availability
- [ ] **CI-02**: CI runs simulation tests (non-#[ignore]) on every PR
- [ ] **CI-03**: Clippy passes with `--features yubikey` with zero warnings

## Future Requirements (v1.2+)

### Attestation
- **ATT-01**: Hardware attestation via slot F9 certificate chain
- **ATT-02**: Attestation verification against YubiKey root CA

### Advanced Operations
- **ADV-01**: On-device key generation (requires management key handling)
- **ADV-02**: PIN caching with zeroize for session duration
- **ADV-03**: Multi-slot differentiation (separate signing vs auth keys)
- **ADV-04**: Key usage validation against PIV spec

## Out of Scope

| Feature | Reason |
|---------|--------|
| PKCS#11 as primary interface | User chose yubikey crate stable API; PKCS#11 kept only where yubikey crate delegates to it |
| Software fallback of any kind | Core security violation — the exact bug we're fixing |
| Manual DER/ASN.1 encoding | Core security violation — the exact bug we're fixing |
| RSA-4096 support | Low demand, defer to v1.2+ |
| Smart card reader auto-detection | Defer until multi-reader scenarios emerge |
| Symmetric encryption via YubiKey | PIV doesn't expose AES for general encryption |
| Generic PKCS#11 device support | YubiKey-specific backend; generic can be separate backend later |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLEAN-01 | Phase 9 | Pending |
| CLEAN-02 | Phase 9 | Pending |
| CLEAN-03 | Phase 9 | Pending |
| CLEAN-04 | Phase 9 | Pending |
| BACK-01 | Phase 10 | Pending |
| BACK-02 | Phase 10 | Pending |
| BACK-03 | Phase 10 | Pending |
| BACK-04 | Phase 10 | Pending |
| BACK-05 | Phase 10 | Pending |
| BACK-06 | Phase 10 | Pending |
| BACK-07 | Phase 10 | Pending |
| BACK-08 | Phase 10 | Pending |
| BACK-09 | Phase 10 | Pending |
| BACK-10 | Phase 10 | Pending |
| BACK-11 | Phase 10 | Pending |
| BACK-12 | Phase 10 | Pending |
| BACK-13 | Phase 10 | Pending |
| BACK-14 | Phase 10 | Pending |
| TEST-01 | Phase 11 | Pending |
| TEST-02 | Phase 11 | Pending |
| TEST-03 | Phase 11 | Pending |
| TEST-04 | Phase 11 | Pending |
| TEST-05 | Phase 11 | Pending |
| TEST-06 | Phase 11 | Pending |
| CI-01 | Phase 12 | Pending |
| CI-02 | Phase 12 | Pending |
| CI-03 | Phase 12 | Pending |

**Coverage:**
- v1.1 requirements: 27 total
- Mapped to phases: 27/27 ✓
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-11*
*Last updated: 2026-02-11 after roadmap creation*
