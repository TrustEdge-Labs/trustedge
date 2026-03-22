# Requirements: TrustEdge

**Defined:** 2026-03-22
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.4 Requirements

Requirements for Security Review Remediation. Each maps to roadmap phases.

### Crypto Hygiene

- [ ] **CRYP-01**: Custom base64 encoder/decoder in crypto.rs replaced with standard `base64` crate
- [ ] **CRYP-02**: PBKDF2 iteration count versioned in encrypted key file metadata with documented upgrade path

### Auth Hardening

- [ ] **AUTH-01**: Timestamp validation rejects future-dated auth responses (unidirectional check, past-only tolerance)
- [ ] **AUTH-02**: All `unwrap()`/`expect()` in auth.rs and envelope.rs security paths replaced with proper error propagation

### Key File Security

- [ ] **KEYF-01**: Generated key files have 0600 Unix permissions (owner-only read/write) on Unix systems
- [ ] **KEYF-02**: Envelope nonce construction explicitly guards against u32 chunk index overflow with error on exceeding 2^32 chunks

### Error Path Testing

- [ ] **TEST-01**: Negative tests for wrong passphrase, truncated key files, and corrupted key file JSON beyond existing SEC-08/09/10 coverage
- [ ] **TEST-02**: Negative tests for malformed metadata in archives and clock skew rejection in auth handshake

## Future Requirements

### YubiKey Capability Alignment

- **YUBI-01**: Key generation supported when yubikey crate exports PinPolicy/TouchPolicy
- **YUBI-02**: Attestation certificate retrieval implemented

## Out of Scope

| Feature | Reason |
|---------|--------|
| YubiKey PIN timing hardening (P1-5) | Verified false positive — yubikey crate's verify_pin() is already constant-time |
| Manual ASN.1 replacement (P2-3) | Already using proper libraries (rcgen, der, spki) — not manual encoding |
| Unused dependency cleanup (P2-6) | False positive from cargo-machete — all flagged deps are legitimately needed |
| YubiKey key generation (P2-2) | Blocked on yubikey crate v0.7 API limitation (PinPolicy not exported) |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CRYP-01 | — | Pending |
| CRYP-02 | — | Pending |
| AUTH-01 | — | Pending |
| AUTH-02 | — | Pending |
| KEYF-01 | — | Pending |
| KEYF-02 | — | Pending |
| TEST-01 | — | Pending |
| TEST-02 | — | Pending |

**Coverage:**
- v2.4 requirements: 8 total
- Mapped to phases: 0
- Unmapped: 8

---
*Requirements defined: 2026-03-22*
*Last updated: 2026-03-22 after initial definition*
