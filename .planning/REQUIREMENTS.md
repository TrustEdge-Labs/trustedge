<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v2.2

**Defined:** 2026-03-18
**Core Value:** Prove that data from an edge device has not been tampered with -- from capture to verification -- using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.2 Requirements

Requirements for the Security Remediation milestone. Each maps to roadmap phases.

### RSA Encryption

- [x] **RSA-01**: RSA key exchange uses OAEP-SHA256 padding instead of PKCS#1 v1.5 in asymmetric.rs
- [x] **RSA-02**: RSA decryption uses OAEP-SHA256 padding, rejects PKCS#1 v1.5 ciphertext

### Envelope Format

- [x] **ENV-01**: v1 envelope format is deprecated -- unseal() logs a deprecation warning when decrypting v1 envelopes
- [x] **ENV-02**: seal() always produces v2 envelopes (no code path creates v1 format)

### Key Derivation

- [x] **KDF-01**: Any PBKDF2 usage enforces a minimum of 300,000 iterations (fail if lower is requested)

### Key Protection

- [x] **KEY-01**: trst keygen encrypts private key files at rest using a passphrase (prompted via rpassword)
- [ ] **KEY-02**: trst wrap and trst unwrap prompt for passphrase to decrypt key files before use
- [ ] **KEY-03**: Unencrypted key files are rejected by default (with --unencrypted escape hatch for CI/automation)

## Future Requirements

Deferred to v2.3+. Tracked but not in current roadmap.

### Device Management
- **DEV-01**: Device enrollment via platform API with attestation
- **DEV-02**: Device key revocation and rotation

### Key Distribution
- **KEY-04**: Recipients can decrypt archives shared with them
- **KEY-05**: Key wrapping for multi-recipient archives

### Crypto Migration
- **KDF-02**: Migrate from PBKDF2 to Argon2 for passphrase-based key derivation

## Out of Scope

| Feature | Reason |
|---------|--------|
| Argon2 migration | Long-term goal, PBKDF2 at 300k+ is adequate for v2.2 |
| TPM support | No hardware to test against |
| Post-quantum crypto | Research phase only |
| Key rotation protocol | Requires device management infrastructure (v2.3+) |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| RSA-01 | Phase 45 | Complete |
| RSA-02 | Phase 45 | Complete |
| ENV-01 | Phase 46 | Complete |
| ENV-02 | Phase 46 | Complete |
| KDF-01 | Phase 46 | Complete |
| KEY-01 | Phase 47 | Complete |
| KEY-02 | Phase 47 | Pending |
| KEY-03 | Phase 47 | Pending |

**Coverage:**
- v2.2 requirements: 8 total
- Mapped to phases: 8
- Unmapped: 0

---
*Requirements defined: 2026-03-18*
*Last updated: 2026-03-18 after roadmap creation*
