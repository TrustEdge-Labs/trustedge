<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge v1.4 Placeholder Elimination

**Defined:** 2026-02-13
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — no misleading or incomplete features.

## v1.4 Requirements

Requirements for placeholder elimination. Each maps to roadmap phases.

### QUIC Security

- [ ] **QUIC-01**: QUIC client uses proper TLS certificate verification by default
- [ ] **QUIC-02**: Insecure TLS skip is gated behind `insecure-tls` feature flag, only available for development

### Dead Code Removal

- [ ] **DEAD-01**: Legacy `handle_connection()`, `process_and_decrypt_chunk()`, `save_chunk_to_disk()` removed from trustedge-server.rs
- [ ] **DEAD-02**: Reserved `encrypt_aes_gcm`/`decrypt_aes_gcm` removed from universal_keyring.rs
- [ ] **DEAD-03**: `ProcessingSession` dead fields removed or activated
- [ ] **DEAD-04**: All `#[allow(dead_code)]` attributes audited — each either justified or code deleted

### Stub Elimination (Core)

- [ ] **STUB-01**: `envelope_v2_bridge.rs` deleted from trustedge-core
- [ ] **STUB-02**: Blake2b hash variant removed from Software HSM (don't advertise what you can't do)
- [ ] **STUB-03**: YubiKey `generate_key` returns clear error with external tool instructions (verify and clean up TODO comment)

### Stub Elimination (Pubky)

- [ ] **PUBK-01**: Unimplemented `publish_key` CLI command removed from trustedge-pubky
- [ ] **PUBK-02**: Placeholder `discover_identities()` removed or returns proper error
- [ ] **PUBK-03**: Placeholder `migrate` command removed from trustedge-pubky
- [ ] **PUBK-04**: TODO comments in pubky-advanced `batch_resolve` addressed (remove TODO, document as known limitation, or implement)

### TODO Hygiene

- [ ] **TODO-01**: Zero remaining TODO comments that indicate unimplemented functionality (informational TODOs about future optimization are acceptable only if the code works correctly as-is)

## Future Requirements

Deferred to future release. Tracked but not in current roadmap.

### QUIC Infrastructure

- **QUIC-F01**: Proper certificate management (CA infrastructure, cert provisioning)

### Pubky Integration

- **PUBK-F01**: Full Pubky network integration (when upstream API stabilizes)

### Cryptography

- **CRYP-F01**: Blake2b hash support (when concrete use case emerges)

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| Implementing missing Pubky features | Removing stubs, not building Pubky integration |
| YubiKey key generation implementation | Blocked by upstream yubikey crate 0.7 API limitations |
| New cryptographic algorithms | Cleaning up, not adding |
| Audio feature stubs | Proper cfg-gated feature design, not placeholders |
| Test-only placeholder data | Test fixtures with "placeholder" strings are fine |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| QUIC-01 | Phase 19 | Pending |
| QUIC-02 | Phase 19 | Pending |
| DEAD-01 | Phase 20 | Pending |
| DEAD-02 | Phase 20 | Pending |
| DEAD-03 | Phase 20 | Pending |
| DEAD-04 | Phase 20 | Pending |
| STUB-01 | Phase 21 | Pending |
| STUB-02 | Phase 21 | Pending |
| STUB-03 | Phase 21 | Pending |
| PUBK-01 | Phase 22 | Pending |
| PUBK-02 | Phase 22 | Pending |
| PUBK-03 | Phase 22 | Pending |
| PUBK-04 | Phase 22 | Pending |
| TODO-01 | Phase 23 | Pending |

**Coverage:**
- v1.4 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-13*
*Last updated: 2026-02-13 after roadmap creation*
