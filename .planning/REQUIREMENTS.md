<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Requirements: TrustEdge

**Defined:** 2026-02-22
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations — thin CLIs and WASM bindings are just frontends.

## v1.7 Requirements

Requirements for v1.7 Security & Quality Hardening. Each maps to roadmap phases.

### Secret Handling

- [x] **SEC-01**: All sensitive fields (PIN, passphrase, JWT secret) are zeroized on drop
- [x] **SEC-02**: Debug output redacts sensitive fields instead of printing plaintext
- [x] **SEC-03**: Serialize/Deserialize removed from config structs that contain secrets (YubiKeyConfig, SoftwareHsmConfig)
- [ ] **SEC-04**: LoginRequest.password is not leaked via Debug or accidental serialization

### Workspace Hygiene

- [ ] **WRK-01**: Deprecated facade crates (trustedge-receipts, trustedge-attestation) are deleted from workspace
- [ ] **WRK-02**: CI scripts and documentation updated to remove facade references
- [ ] **WRK-03**: Tier 2 experimental crates separated from Tier 1 dependency graph (pubky dep tree no longer in shared Cargo.lock)
- [ ] **WRK-04**: Workspace Cargo.toml cleaned of unused workspace dependencies after separation

### Platform Quality

- [ ] **PLT-01**: verify_handler shared validation logic extracted into single always-compiled function
- [ ] **PLT-02**: Verify-only (non-postgres) build uses restrictive CORS instead of permissive
- [ ] **PLT-03**: CA module routes either wired into router or documented as library-only

### Platform Testing

- [ ] **TST-01**: Platform-server binary crate has integration tests validating AppState wiring
- [ ] **TST-02**: create_test_app faithfully mirrors create_router (CORS, trace, auth middleware)
- [ ] **TST-03**: Full verify round-trip tested over HTTP (valid signature, receipt returned)

## Future Requirements

### Deferred from v1.7

- Pubky adapter merged into core protocols/pubky/ (feature-gated)
- Pubky-advanced hybrid encryption merged into core
- Prelude module for common imports
- Updated documentation with module-level security considerations
- CI-automated TypeScript type generation from trustedge-types schemas

## Out of Scope

| Feature | Reason |
|---------|--------|
| Postgres CI integration tests | Requires Docker infrastructure, separate milestone |
| TPM support | No hardware to test against |
| Post-quantum cryptography | Research phase only |
| no_std support | Requires separate milestone |
| Algorithm agility | Ed25519/AES-256-GCM sufficient |
| secrecy crate adoption | zeroize already in workspace, sufficient for v1.7 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SEC-01 | Phase 31 | Complete |
| SEC-02 | Phase 31 | Complete |
| SEC-03 | Phase 31 | Complete |
| SEC-04 | Phase 31 | Pending |
| WRK-01 | Phase 32 | Pending |
| WRK-02 | Phase 32 | Pending |
| WRK-03 | Phase 32 | Pending |
| WRK-04 | Phase 32 | Pending |
| PLT-01 | Phase 33 | Pending |
| PLT-02 | Phase 33 | Pending |
| PLT-03 | Phase 33 | Pending |
| TST-01 | Phase 34 | Pending |
| TST-02 | Phase 34 | Pending |
| TST-03 | Phase 34 | Pending |

**Coverage:**
- v1.7 requirements: 14 total
- Mapped to phases: 14
- Unmapped: 0

---
*Requirements defined: 2026-02-22*
*Last updated: 2026-02-22 after 31-02 — SEC-03 complete (YubiKeyConfig + SoftwareHsmConfig serde removed)*
