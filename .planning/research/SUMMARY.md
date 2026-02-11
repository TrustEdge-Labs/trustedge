<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# Project Research Summary

**Project:** TrustEdge v1.1 YubiKey Integration Overhaul
**Domain:** Hardware cryptographic backend (YubiKey PIV)
**Researched:** 2026-02-11
**Confidence:** MEDIUM-HIGH

## Executive Summary

The YubiKey backend rewrite involves replacing a broken 3,263-line implementation that uses manual cryptography, software fallbacks, and placeholder keys with a clean, fail-closed architecture. Research reveals the current implementation has critical security vulnerabilities: 93 occurrences of "placeholder" code, 41 instances of software fallback logic, and extensive manual ASN.1/DER encoding (lines 95-200). The recommended approach eliminates the `yubikey` crate's `untested` feature flag, migrates from manual DER encoding to `rcgen` for certificate generation, and maintains strict PKCS#11-based hardware integration with fail-closed error handling.

The rewrite targets 500-800 lines (75% reduction) by using battle-tested libraries exclusively: `rcgen` for X.509 certificate generation, `pkcs11` for hardware operations, and `der`/`spki` from RustCrypto for encoding. Architecture follows the existing Universal Backend pattern, requiring no trait changes. The key constraint is absolute: hardware unavailable must return errors, never silently fallback to software crypto. All tests must exercise actual functionality with hardware or be properly separated as simulation tests.

Critical risks center on three areas: (1) rcgen's custom signing API for hardware-backed keys may require complex callback integration, (2) PKCS#11 session management leaks if not using RAII patterns, and (3) test infrastructure must distinguish simulation (always-run) from strict hardware tests (gated with `#[ignore]`). The recommended phase structure starts with core backend rewrite (remove fallbacks, eliminate manual crypto), followed by test infrastructure (strict hardware tests + proper simulation), and concludes with documentation and integration testing.

## Key Findings

### Recommended Stack

The stack overhaul eliminates unstable dependencies and manual cryptography in favor of battle-tested libraries. Current implementation uses the `yubikey` crate with the `untested` feature flag (forbidden by milestone constraints) and manual ASN.1/DER encoding for certificates (150+ lines of fragile code in yubikey.rs lines 95-200).

**Core technologies:**
- `rcgen ~0.13` — X.509 certificate generation with hardware signing support. Eliminates manual DER encoding, provides NIST-compliant certificate builder. HIGH confidence for approach, LOW confidence for exact version (training data from January 2025).
- `pkcs11 =0.5.0` — PKCS#11 interface for hardware operations. Exact version lock prevents transitive dependency conflicts (Pitfall #11). Already working with 90+ simulation tests.
- `yubikey =0.7.0` (NO `untested` feature) — YubiKey PIV operations via stable API only. Remove `untested` feature flag to meet milestone requirements.
- `der 0.7`, `spki 0.7`, `signature 2.2` — RustCrypto ecosystem for encoding and signature traits. Keep current versions (stable, compatible).

**Dependencies to REMOVE:**
- Manual ASN.1/DER encoding functions (lines 95-150 in yubikey.rs)
- `x509-cert` direct usage (migrate to rcgen high-level API)
- `yubikey` crate with `untested` feature (use stable API only)

**Build requirements unchanged:** PCSC daemon (pcscd) on Linux/macOS, smart card service on Windows. Runtime requires YubiKey 5 series with PIV applet enabled.

### Expected Features

Research identifies clear table stakes (must-have), differentiators (competitive advantage), and anti-features (commonly requested but problematic).

**Must have (table stakes):**
- Ed25519, ECDSA P-256, RSA-2048 signing via PIV slots
- Public key extraction from hardware
- PIV slot enumeration (9a/9c/9d/9e)
- Fail-closed hardware check — MUST error if hardware unavailable, never silent fallback
- PKCS#11 session management with proper cleanup
- Certificate management (read existing certs from PIV slots)
- Hardware-specific error handling — distinguish "no hardware", "wrong PIN", "slot empty", "operation failed"

**Should have (competitive):**
- Hardware attestation — cryptographic proof operation happened in hardware (HIGH complexity, defer to v1.x)
- Certificate generation (self-signed) — use rcgen with hardware signing (HIGH complexity, needed for TLS integration)
- PIN caching (secure) — cache PIN in memory with zeroize for session duration (MEDIUM complexity, UX enhancement)
- Multi-slot operations — use different PIV slots for different purposes (LOW complexity, already supported by PIV)
- Verbose diagnostics mode — config flag for debugging hardware issues (LOW complexity, include in v1)

**Defer (v2+):**
- Key generation on-device — requires PIV management key handling, initialization flows
- RSA-4096 support — slower, less common, defer unless compatibility requirement emerges
- Certificate revocation checking — complex OCSP/CRL integration
- Smart card reader auto-detection — handle multiple readers, auto-select YubiKey

**Anti-features (FORBIDDEN):**
- Software fallback — silently downgrades security, user thinks they have hardware protection but don't
- Placeholder keys — insecure test vectors that ship to production (current implementation has 93 occurrences)
- Manual crypto (DER encoding) — bug-prone, unaudited, violates "battle-tested only" rule
- Auto-generated PINs — defeats YubiKey security model
- Tests that don't test anything — auto-pass stubs, always `Ok(())` without assertions

### Architecture Approach

The rewrite integrates into existing Universal Backend architecture at `crates/core/src/backends/`. No changes to the `UniversalBackend` trait required. Architecture separates three layers: hardware operations (yubikey crate wrapper), certificate generation (rcgen integration), and Universal Backend integration (trait implementation + error handling).

**Major components:**
1. **YubiKeyBackend struct** (~150 lines) — Configuration (PIN, default slot, verbose flag), YubiKey connection lifecycle, optional key cache for performance
2. **Hardware operations layer** (~200 lines) — PIV signing, public key extraction, attestation. Thin wrapper around `yubikey` crate with fail-closed error conversion
3. **Certificate generation layer** (~150 lines) — rcgen integration with custom `KeyPair` implementation that delegates signing to YubiKey hardware
4. **UniversalBackend implementation** (~150 lines) — Maps `CryptoOperation` enum to hardware ops (Sign, GetPublicKey, GenerateKeyPair, Attest), explicit unsupported operation handling
5. **Error conversion** (~50 lines) — Convert `yubikey::Error` to `BackendError` variants with user-actionable messages
6. **Helper functions and tests** (~100 lines) — Slot parsing, capability reporting, inline unit tests

**Target:** 500-800 lines total (vs 3,263 current), single file `backends/yubikey.rs` following `software_hsm.rs` pattern (1,419 lines, works well).

**Data flow (signing operation):**
```
perform_operation(key_id="9c", Sign{data, algorithm})
  → Parse slot "9c" → PivSlot::Signature
  → Validate algorithm supported
  → Hash data with SHA256
  → Authenticate with PIN
  → yubikey.sign_data(slot, digest, algorithm)
  → Return CryptoResult::Signed(signature)
  → ALL errors → BackendError::HardwareError (fail-closed)
```

**Certificate generation flow:**
```
generate_certificate(slot, subject)
  → Extract public key from YubiKey
  → Create rcgen CertificateParams
  → Wrap as CustomKeyPair (delegates signing to YubiKey)
  → rcgen generates TBS cert → calls CustomKeyPair::sign()
  → YubiKey signs TBS data
  → rcgen assembles certificate DER
  → Return certificate_der
```

### Critical Pitfalls

Research identified 8 critical and 3 moderate pitfalls from codebase analysis (93 "placeholder" occurrences, 41 "fallback" instances, 1000+ lines manual DER encoding).

1. **Silent fallback to software crypto** — Current implementation has 41 fallback instances. Hardware unavailable silently uses software keys, user believes they have hardware security but don't. Prevention: Fail-closed architecture, zero placeholder implementations in production, capability checks verify actual hardware state. Phase 1 must eliminate all fallback logic.

2. **Manual ASN.1/DER encoding** — Lines 95-200 in yubikey.rs contain handwritten ASN.1 INTEGER/SEQUENCE encoding. Subtle encoding bugs, security vulnerabilities (integer overflow), incompatibility with parsers. Prevention: NEVER implement ASN.1/DER manually, use `rcgen` for certificates, `der`/`spki` for low-level encoding. Phase 1 migrates to rcgen entirely.

3. **Hardcoded test vectors as placeholder keys** — Current implementation has 93 "placeholder" occurrences, using NIST test vectors in backend code. Catastrophic if private key found, reproducible certificates, false security theater. Prevention: Zero tolerance for hardcoded keys in `src/backends/`, test vectors in `tests/` only, fail with `BackendError::HardwareUnavailable` when hardware missing. Phase 1 removes all placeholder keys.

4. **Tests that don't test anything** — Auto-pass tests (always `Ok(())` without assertions), mocked success stubs, "does it panic?" tests. Prevention: Every test must have assertions about actual output, hardware tests use `#[ignore]` + strict failure if hardware absent, simulation tests check API contracts not hardware ops. Phase 2 implements strict test infrastructure.

5. **PKCS#11 session management leaks** — Resource exhaustion (max 16-32 sessions per slot), stale sessions after hardware removal, login state confusion. Prevention: RAII pattern (session guard with Drop), open → operate → close (never cache long-term), logout before close_session. Phase 1 implements session guard pattern.

## Implications for Roadmap

Based on research, recommended 3-phase structure with clear dependencies and pitfall avoidance:

### Phase 1: Core Backend Rewrite
**Rationale:** Foundation phase eliminates security vulnerabilities before any feature work. Addresses pitfalls #1-3, #5 (silent fallbacks, manual crypto, placeholder keys, session leaks). Dependencies: rcgen, pkcs11 (both already in workspace or well-documented). Architecture is well-defined (500-800 line target, follows software_hsm.rs pattern).

**Delivers:**
- New YubiKeyBackend struct with config (PIN, slot, verbose)
- PIV hardware operations (sign, get_public_key, attest) with fail-closed error handling
- rcgen certificate generation with custom hardware signer
- UniversalBackend trait implementation
- PKCS#11 session guard with proper Drop cleanup
- Zero placeholder keys, zero software fallbacks, zero manual DER encoding

**Addresses features:**
- Ed25519/ECDSA P-256/RSA-2048 signing (table stakes)
- Public key extraction (table stakes)
- PIV slot enumeration (table stakes)
- Fail-closed hardware check (table stakes)
- PKCS#11 session management (table stakes)
- Verbose diagnostics mode (should-have, LOW complexity)

**Avoids pitfalls:**
- Silent fallback (#1) — architecture enforces fail-closed
- Manual crypto (#2) — rcgen for all certificate operations
- Placeholder keys (#3) — errors on hardware unavailable
- Session leaks (#5) — RAII guard pattern

**Research needed:** Medium. rcgen custom signer API (callback-based, complex control flow) needs investigation. PKCS#11 key attribute extraction may vary by YubiKey firmware version.

### Phase 2: Test Infrastructure
**Rationale:** Cannot validate Phase 1 without proper test infrastructure. Current tests have auto-pass stubs, no hardware validation. Must separate simulation tests (always-run, no hardware) from strict hardware tests (require YubiKey, gated with `#[ignore]`). Addresses pitfall #4 (tests that don't test) and #8 (feature flag testing gaps in CI).

**Delivers:**
- Simulation tests (unit) — capability reporting, slot parsing, error mapping, config validation (no hardware required)
- Strict hardware tests (integration) — real signing with YubiKey, certificate generation, PIN authentication, slot enumeration (requires `#[ignore]`)
- Anti-pattern tests — verify no software fallback, no placeholder keys, hardware tests fail without YubiKey
- CI configuration — unit tests always run, integration tests run with `--ignored` on hardware-equipped runners
- Test documentation — which tests require hardware, expected YubiKey state (e.g., "slot 9a must have Ed25519 key")

**Uses stack:**
- YubiKey backend from Phase 1
- Test harness patterns from existing yubikey_strict_hardware.rs
- CI setup already has cargo-hack for feature powerset testing

**Implements architecture:**
- Test file structure: `tests/yubikey_backend_unit.rs` (always-run), `tests/yubikey_backend_integration.rs` (hardware-required)
- Test gating: `#[ignore]` for hardware tests, `YUBIKEY_HARDWARE_TEST_MUTEX` for sequential execution
- Hardware detection: `YubikeyTestEnvironment::detect()`, skip gracefully if no hardware in unit tests, fail in integration tests

**Avoids pitfalls:**
- Tests that don't test (#4) — every test has >=1 assertion validating actual behavior
- Feature flag testing gaps (#8) — CI fails if feature enabled but deps missing, strict test tier enforced

**Research needed:** Low. Testing patterns well-established in codebase (existing yubikey test files), standard Rust testing practices.

### Phase 3: Documentation and Integration Testing
**Rationale:** Phase 1+2 deliver working backend + validated tests. Phase 3 ensures usability (docs) and validates integration with existing TrustEdge systems (network transport, envelope encryption). Deferred features (attestation, certificate generation for TLS) not included in v1.1 scope.

**Delivers:**
- Updated CLAUDE.md with YubiKey backend patterns, build instructions
- README.md section on hardware backend usage
- Example code for YubiKey backend initialization, signing operations
- Integration tests with network transport (client/server with YubiKey backend)
- Error message improvements based on usability testing
- Performance baseline (operation latency with YubiKey vs software_hsm)

**Uses stack:**
- YubiKey backend from Phase 1
- Test infrastructure from Phase 2
- Existing network transport (TCP/QUIC) for integration validation

**Implements architecture:**
- No new components, validates existing integration points
- Error message mapping (PKCS#11 codes → user-actionable messages)
- Performance profiling (identify slow operations, document expected latency)

**Avoids pitfalls:**
- UX pitfalls from PITFALLS.md — cryptic error codes → human messages, no blocking ops in async, feedback during slow operations

**Research needed:** None. Documentation patterns established, integration testing uses existing network stack.

### Phase Ordering Rationale

- **Phase 1 first:** Cannot build features on broken foundation. Security vulnerabilities (fallbacks, placeholder keys, manual crypto) must be eliminated before any feature work. Dependencies are clear (rcgen, pkcs11), architecture is well-defined (500-800 lines, follows existing patterns).

- **Phase 2 before Phase 3:** Cannot document or integrate without validated implementation. Tests verify Phase 1 correctness before exposing to users or integrating with other systems. Strict hardware tests catch bugs that simulation tests miss.

- **Phase 3 last:** Documentation and integration assume working, tested backend. Usability improvements (error messages, examples) build on stable API from Phase 1. Deferred features (attestation, TLS cert generation) explicitly out of scope for v1.1.

- **Dependencies discovered:** rcgen custom signer API is only unknown (Phase 1 research flag). PKCS#11 session management, PIV operations, test patterns all well-documented in existing codebase or RustCrypto ecosystem.

- **Grouping based on architecture:** Hardware layer (Phase 1), validation layer (Phase 2), integration layer (Phase 3). Clean separation of concerns matches Universal Backend design.

- **Pitfall avoidance:** Phase 1 eliminates critical security pitfalls (#1-3, #5), Phase 2 addresses testing pitfalls (#4, #8), Phase 3 addresses UX pitfalls. Sequential phases prevent cascading failures (e.g., can't fix tests before fixing implementation).

### Research Flags

Phases likely needing deeper research during planning:

- **Phase 1:** rcgen custom signer callback — Complex control flow, need to understand TBS certificate signing exactly. Callback-based API requires understanding how rcgen invokes hardware signing during certificate generation. MEDIUM priority, well-documented in rcgen examples but complex integration pattern.

- **Phase 1:** PKCS#11 key attribute extraction — Different YubiKey firmware versions may return different formats (raw vs DER for public keys). Need to normalize at extraction point. LOW priority, can be addressed during implementation with hardware testing.

Phases with standard patterns (skip research-phase):

- **Phase 2:** Testing patterns well-established in codebase (existing yubikey test files), standard Rust testing with `#[ignore]` and `cargo test --features yubikey`. No additional research needed.

- **Phase 3:** Documentation follows existing CLAUDE.md patterns, integration testing uses established network transport (TCP/QUIC). No additional research needed.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | MEDIUM | PKCS#11/YubiKey deps HIGH (existing codebase, stable), rcgen version LOW (training data may be outdated), rcgen API MEDIUM (standard patterns but external signer needs verification) |
| Features | HIGH | Based on codebase analysis (Universal Backend trait, existing implementations), PIV standard operations (NIST SP 800-73-4), clear table stakes vs differentiators |
| Architecture | HIGH | Existing Universal Backend pattern is stable, codebase analysis shows 500-800 line target feasible, integration points well-defined, no trait changes needed |
| Pitfalls | HIGH | Direct observation in codebase (93 "placeholder", 41 "fallback", 1000+ lines manual DER), improvement-plan.md security findings, PKCS#11 specification session limits |

**Overall confidence:** MEDIUM-HIGH

Core approach is sound (eliminate fallbacks, use rcgen, fail-closed architecture), but rcgen exact version and custom signer API need verification. Architecture is well-understood (Universal Backend stable), pitfalls are concrete (observed in codebase), features are clearly categorized (table stakes vs deferred).

### Gaps to Address

Research areas that need validation during implementation:

- **rcgen exact version:** Training data suggests 0.13, but version as of 2026-02-11 unknown. Action: Verify rcgen version immediately in Phase 1, check compatibility with `signature`, `der`, `spki` crates.

- **rcgen custom signer API:** How to integrate hardware-backed key signing with rcgen? May require implementing `rcgen::KeyPair` trait or using `serialize_der_with_signer()`. Action: Dedicate first Phase 1 task to dependency verification and API compatibility testing, consult rcgen documentation/examples.

- **YubiKey stable API coverage:** Does stable API (without `untested` feature) provide ALL operations needed for Universal Backend? Action: Audit current yubikey.rs usage in Phase 1 kickoff, verify sign_data, fetch_pubkey, generate available in stable API.

- **Certificate validation approach:** Should we add `webpki` or similar for certificate chain validation? Not strictly needed for v1.1 (just generation), but may be required for attestation in v1.x. Action: Defer to v1.x milestone planning, document as future consideration.

- **PKCS#11 version conflicts:** Locking to 0.5.0 may conflict with transitive dependencies from rcgen or other crates. Action: Run `cargo tree -p trustedge-core --features yubikey` after adding rcgen, resolve conflicts before committing.

## Sources

### Primary (HIGH confidence)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/yubikey.rs` (3,263 lines, 93 "placeholder", 41 "fallback", manual DER lines 95-200)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/universal.rs` (UniversalBackend trait, CryptoOperation enum)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/crates/core/src/backends/software_hsm.rs` (1,419 lines, reference implementation)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/improvement-plan.md` (security audit findings)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/Cargo.toml` (dependencies: pkcs11 0.5, yubikey 0.7 with untested)
- Codebase analysis: `/home/john/vault/projects/github.com/trustedge/crates/core/tests/yubikey_*_tests.rs` (test patterns)

### Secondary (MEDIUM confidence)
- NIST SP 800-73-4 (PIV specification) — mandatory algorithms (P-256, RSA-2048), slot purposes
- PKCS#11 v2.40 specification — session management, error codes, max session limits
- RFC 5280 (X.509 Certificate Profile) — extension requirements, critical flags
- RustCrypto ecosystem documentation — `der`, `spki`, `signature`, `x509-cert` crates
- rcgen documentation (training data from January 2025) — certificate generation patterns

### Tertiary (LOW confidence)
- rcgen version 0.13 (training data) — needs verification for 2026-02-11
- YubiKey stable API surface (inferred from codebase patterns) — needs verification in official yubikey crate docs
- Custom signer API details — needs verification with current rcgen documentation

---
*Research completed: 2026-02-11*
*Ready for roadmap: yes*
