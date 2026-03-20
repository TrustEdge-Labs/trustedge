<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v2.3

**Defined:** 2026-03-20
**Core Value:** Prove that data from an edge device has not been tampered with -- from capture to verification -- using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.3 Requirements

Requirements for the Security Testing milestone. Each maps to roadmap phases. Tests target threat model categories T1-T8.

### Archive Tampering (T1, T2)

- [ ] **SEC-01**: Test that modifying any byte of an encrypted chunk causes trst verify to fail (AES-GCM auth tag detection)
- [ ] **SEC-02**: Test that injecting an extra chunk file into a .trst archive causes verification failure (BLAKE3 chain break)
- [ ] **SEC-03**: Test that reordering chunk files in a .trst archive causes verification failure (continuity chain)
- [ ] **SEC-04**: Test that modifying manifest.json after signing causes signature verification failure

### Nonce & Key Derivation (T5, T6)

- [ ] **SEC-05**: Test that nonces across chunks within an archive are unique (no nonce reuse)
- [ ] **SEC-06**: Test that the same plaintext encrypted twice with the same device key produces different nonces (random nonce generation)
- [ ] **SEC-07**: Test that HKDF derivation with different device keys produces different encryption keys

### Key Protection (T3)

- [ ] **SEC-08**: Test that truncated encrypted key files are rejected (not silently corrupted)
- [ ] **SEC-09**: Test that corrupted JSON header in encrypted key files is rejected with clear error
- [ ] **SEC-10**: Test that wrong passphrase returns a clear error, not garbled data

### Replay & Receipt (T8)

- [ ] **SEC-11**: Test that the same archive submitted twice to /v1/verify produces receipts with different verification IDs and timestamps
- [ ] **SEC-12**: Test that a receipt's manifest_digest is bound to the specific archive content

## Future Requirements

Deferred to v2.4+.

### Device Management
- **DEV-01**: Device enrollment via platform API with attestation
- **DEV-02**: Device key revocation and rotation

### Key Distribution
- **KEY-04**: Recipients can decrypt archives shared with them
- **KEY-05**: Key wrapping for multi-recipient archives

### Replay Hardening
- **SEC-13**: Sliding-window nonce validation for high-volume /v1/verify
- **SEC-14**: Request-level idempotency tokens with server-side deduplication

## Out of Scope

| Feature | Reason |
|---------|--------|
| Fuzzing / property-based testing | Separate initiative, not targeted security tests |
| Penetration testing | Requires external security team |
| Side-channel timing analysis | Requires specialized tooling and hardware |
| TPM integration tests | No hardware available |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| SEC-01 | TBD | Pending |
| SEC-02 | TBD | Pending |
| SEC-03 | TBD | Pending |
| SEC-04 | TBD | Pending |
| SEC-05 | TBD | Pending |
| SEC-06 | TBD | Pending |
| SEC-07 | TBD | Pending |
| SEC-08 | TBD | Pending |
| SEC-09 | TBD | Pending |
| SEC-10 | TBD | Pending |
| SEC-11 | TBD | Pending |
| SEC-12 | TBD | Pending |

**Coverage:**
- v2.3 requirements: 12 total
- Mapped to phases: 0
- Unmapped: 12

---
*Requirements defined: 2026-03-20*
*Last updated: 2026-03-20 after initial definition*
