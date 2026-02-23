<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v1.8 KDF Architecture Fix

**Defined:** 2026-02-22
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.

## v1.8 Requirements

Fix incorrect KDF usage across the codebase. Critical: replace PBKDF2-per-chunk with HKDF hierarchical key derivation in envelope.rs. Moderate: harden keyring backend parameters.

### Envelope KDF (Critical)

- [ ] **ENV-01**: Envelope encryption derives encryption keys from ECDH shared secrets using HKDF-SHA256, not PBKDF2
- [ ] **ENV-02**: Encryption key is derived once per envelope via HKDF-Extract + Expand, producing DerivedKey and NoncePrefix — not re-derived per chunk
- [ ] **ENV-03**: Per-chunk nonces use deterministic counter mode (NoncePrefix || chunk_index || last_flag) instead of random salt generation
- [ ] **ENV-04**: HKDF info parameter includes domain separation string for cryptographic binding to the TrustEdge envelope context
- [ ] **ENV-05**: Ad-hoc CatKDF construction (concatenating shared_secret + salt + sequence + metadata as IKM) is eliminated in favor of structured HKDF inputs
- [ ] **ENV-06**: `hkdf` crate added as workspace dependency with appropriate version

### Envelope Versioning

- [ ] **VER-01**: Envelope format includes version field to distinguish v1 (legacy PBKDF2-per-chunk) from v2 (HKDF-once) formats
- [ ] **VER-02**: Decryption path supports both v1 and v2 envelope formats via version-based dispatch, preserving backward compatibility for existing encrypted data

### Keyring Hardening (Moderate)

- [ ] **KEY-01**: Keyring backend (`keyring.rs`) PBKDF2 iterations increased from 100,000 to 600,000 per OWASP 2023 recommendation
- [ ] **KEY-02**: Keyring backend (`keyring.rs`) salt length increased from 16 bytes to 32 bytes
- [ ] **KEY-03**: Universal keyring backend (`universal_keyring.rs`) PBKDF2 iterations increased from 100,000 to 600,000
- [ ] **KEY-04**: Universal keyring backend (`universal_keyring.rs`) salt length increased from 16 bytes to 32 bytes

### Verification

- [ ] **TST-01**: All existing envelope tests pass with updated KDF architecture (no regression)
- [ ] **TST-02**: Multi-chunk encryption/decryption verified end-to-end with new HKDF-based format
- [ ] **TST-03**: Keyring encryption/decryption tests pass with updated PBKDF2 parameters

## Future Requirements

### Performance Optimization

- **PERF-01**: Benchmark envelope encryption throughput before/after HKDF migration
- **PERF-02**: Profile PBKDF2 overhead for keyring operations at 600k iterations

## Out of Scope

| Feature | Reason |
|---------|--------|
| Experimental pubky-advanced KDF fixes | Isolated standalone workspace, not part of root workspace or CI |
| auth.rs KDF changes | Already uses BLAKE3::derive_key correctly — no fix needed |
| software_hsm.rs PBKDF2 changes | Already hardened to 600k iterations in prior commit — correct usage (passphrase → key, not ECDH) |
| Post-quantum KDF migration | Research phase only, no production use case yet |
| Streaming AEAD as separate API | This milestone fixes the internal KDF; a public streaming API is future work |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| ENV-01 | Phase 36 | Pending |
| ENV-02 | Phase 36 | Pending |
| ENV-03 | Phase 36 | Pending |
| ENV-04 | Phase 35 | Pending |
| ENV-05 | Phase 35 | Pending |
| ENV-06 | Phase 35 | Pending |
| VER-01 | Phase 36 | Pending |
| VER-02 | Phase 36 | Pending |
| KEY-01 | Phase 37 | Pending |
| KEY-02 | Phase 37 | Pending |
| KEY-03 | Phase 37 | Pending |
| KEY-04 | Phase 37 | Pending |
| TST-01 | Phase 36 | Pending |
| TST-02 | Phase 36 | Pending |
| TST-03 | Phase 37 | Pending |

**Coverage:**
- v1.8 requirements: 15 total
- Mapped to phases: 15
- Unmapped: 0

---
*Requirements defined: 2026-02-22*
*Last updated: 2026-02-22 after roadmap created — traceability complete*
