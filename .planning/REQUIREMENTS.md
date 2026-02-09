# Requirements: TrustEdge Consolidation

**Defined:** 2026-02-09
**Core Value:** A single, reliable trustedge-core library that owns all cryptographic operations with production-quality YubiKey integration — thin CLIs and WASM bindings are just frontends.

## v1 Requirements

Requirements for consolidation milestone. Each maps to roadmap phases.

### Foundation

- [ ] **FOUND-01**: Dependency graph analyzed and cross-crate duplication mapped
- [ ] **FOUND-02**: Layered module hierarchy created in trustedge-core (primitives/backends/protocols/applications/transport/io)
- [ ] **FOUND-03**: Test inventory baseline documented (exact count per crate)

### Error Handling

- [ ] **ERR-01**: Unified TrustEdgeError enum with subsystem variants (Crypto, Backend, Transport, Archive, Manifest)
- [ ] **ERR-02**: All 10+ duplicate error types consolidated into hierarchy
- [ ] **ERR-03**: thiserror for library code, anyhow restricted to CLI binaries only

### Code Integration

- [ ] **INTG-01**: trst-core manifest types merged into core applications/archives/ (WASM-compatible)
- [ ] **INTG-02**: Duplicate ManifestError between core and trst-core resolved
- [ ] **INTG-03**: Receipts system (1,281 LOC, 23 tests) merged into core applications/receipts/
- [ ] **INTG-04**: Attestation system merged into core applications/attestation/

### Feature Flags

- [ ] **FEAT-01**: Feature flags consolidated into categories (backend, platform, format)
- [ ] **FEAT-02**: CI matrix tests critical feature combinations (default, yubikey, audio, all-features)

### Backward Compatibility

- [ ] **COMPAT-01**: Deprecated re-export facades created for merged crates
- [ ] **COMPAT-02**: Migration guide documenting import path changes

### Validation

- [ ] **VAL-01**: 150+ tests preserved (before/after count validated)
- [ ] **VAL-02**: WASM build succeeds (cargo check --target wasm32-unknown-unknown)
- [ ] **VAL-03**: No API breakage verified via cargo semver-checks

## v2 Requirements

Deferred to future milestone. Tracked but not in current roadmap.

### Pubky Integration

- **PUBKY-01**: Pubky adapter merged into core protocols/pubky/ (feature-gated)
- **PUBKY-02**: Pubky-advanced hybrid encryption merged into core

### Hardware Validation

- **HW-01**: YubiKey hardware tests pass after consolidation (requires physical hardware)

### Developer Experience

- **DX-01**: Prelude module for common imports
- **DX-02**: Updated documentation with module-level security considerations

## Out of Scope

Explicitly excluded. Documented to prevent scope creep.

| Feature | Reason |
|---------|--------|
| TPM support | Premature, no hardware to test against |
| Post-quantum cryptography | Research phase only, no production use case |
| New crypto features | This milestone is consolidation only |
| no_std support | Requires separate milestone, half-measures are worse |
| Deleting any code | Everything is preserved, just reorganized |
| Algorithm agility changes | Hard-coded Ed25519/AES-256-GCM is sufficient |

## Traceability

Which phases cover which requirements. Updated during roadmap creation.

| Requirement | Phase | Status |
|-------------|-------|--------|
| FOUND-01 | Phase 1 | Pending |
| FOUND-02 | Phase 1 | Pending |
| FOUND-03 | Phase 1 | Pending |
| ERR-01 | Phase 2 | Pending |
| ERR-02 | Phase 2 | Pending |
| ERR-03 | Phase 2 | Pending |
| INTG-01 | Phase 3 | Pending |
| INTG-02 | Phase 3 | Pending |
| INTG-03 | Phase 4 | Pending |
| INTG-04 | Phase 5 | Pending |
| FEAT-01 | Phase 6 | Pending |
| FEAT-02 | Phase 6 | Pending |
| COMPAT-01 | Phase 7 | Pending |
| COMPAT-02 | Phase 7 | Pending |
| VAL-01 | Phase 8 | Pending |
| VAL-02 | Phase 8 | Pending |
| VAL-03 | Phase 8 | Pending |

**Coverage:**
- v1 requirements: 17 total
- Mapped to phases: 17
- Unmapped: 0 ✓

---
*Requirements defined: 2026-02-09*
*Last updated: 2026-02-09 after initial definition*
