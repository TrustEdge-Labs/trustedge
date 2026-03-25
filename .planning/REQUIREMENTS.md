# Requirements: TrustEdge

**Defined:** 2026-03-24
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.7 Requirements

Requirements for milestone v2.7 CI & Config Security. Each maps to roadmap phases.

### CI Supply Chain

- [x] **CISC-01**: CI installs wasm-pack without `curl | sh` (use cargo-install or pre-built binary with checksum verification)
- [x] **CISC-02**: All GitHub Actions in all 4 workflows are pinned to full commit SHAs (not tags)
- [x] **CISC-03**: `actions-rs/toolchain@v1` replaced with `dtolnay/rust-toolchain` in wasm-tests.yml

### Config & Credentials

- [x] **CONF-01**: `DATABASE_URL` has no hardcoded credential fallback in release builds (require explicit config)
- [x] **CONF-02**: PostgreSQL port not exposed to host in docker-compose (internal network only)
- [x] **CONF-03**: `CAConfig::default()` rejects placeholder JWT secret outside tests (panic or error if `"your-secret-key"` used in non-test code)

### Error Handling

- [ ] **ERRH-01**: Crypto verification error responses return generic message to clients; raw library errors logged server-side only

## Future Requirements

None — all findings addressed in this milestone.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full SLSA compliance | Beyond current review scope; tracked for future |
| Signed commits enforcement | Requires contributor setup; deferred |
| Container image signing | Not flagged in review; future hardening |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CISC-01 | Phase 61 | Complete |
| CISC-02 | Phase 61 | Complete |
| CISC-03 | Phase 61 | Complete |
| CONF-01 | Phase 62 | Complete |
| CONF-02 | Phase 62 | Complete |
| CONF-03 | Phase 62 | Complete |
| ERRH-01 | Phase 63 | Pending |

**Coverage:**
- v2.7 requirements: 7 total
- Mapped to phases: 7
- Unmapped: 0

---
*Requirements defined: 2026-03-24*
*Last updated: 2026-03-24 after roadmap created (all 7 requirements mapped)*
