# Requirements: TrustEdge v2.5

**Defined:** 2026-03-22
**Core Value:** Prove that data from an edge device has not been tampered with — from capture to verification — using cryptographic signatures, continuity chains, and verifiable receipts.

## v2.5 Requirements

Requirements for critical security fixes. Each maps to roadmap phases.

### Transport Security

- [x] **TSEC-01**: QUIC `HardwareBackedVerifier` performs actual TLS signature verification instead of returning unconditional `HandshakeSignatureValid::assertion()`
- [x] **TSEC-02**: MITM attack against QUIC TLS handshake is rejected (test proves verification catches bad signatures)

### Platform HTTP

- [x] **HTTP-01**: `/v1/verify` endpoint enforces a request body size limit (1-10 MB) via `RequestBodyLimitLayer`
- [x] **HTTP-02**: HTTP endpoints enforce rate limiting to prevent CPU-exhaustion abuse of BLAKE3+Ed25519 verify
- [x] **HTTP-03**: JWKS signing key path is configurable via environment variable (not hardcoded to `target/dev/`)
- [x] **HTTP-04**: JWKS signing key is not persisted as unencrypted plaintext in a build-artifact directory

### WASM

- [x] **WASM-01**: `trst-wasm` decrypt logic calls `.decrypt()` exactly once per ciphertext (double-decrypt bug fixed)
- [x] **WASM-02**: Browser-based archive verification completes successfully (test proves end-to-end WASM verify works)

## Future Requirements (v2.6)

Deferred P1 findings — tracked but not in current roadmap.

### Core Hardening

- **CORE-01**: Missing zeroization on 4 key-holding structs (PrivateKey, SessionInfo.session_key, ClientAuthResult.session_key, SymmetricKey)
- **CORE-02**: Minimum PBKDF2 iteration count enforced on encrypted key import

### Platform Hardening

- **PLAT-01**: `/v1/verify` works correctly in postgres mode (OrgContext extraction fixed)
- **PLAT-02**: CORS origins configurable via environment variable (not hardcoded localhost)

### CLI Hardening

- **CLI-01**: AES-256 key not printed to stderr without explicit `--show-key` flag

### Deploy Hardening

- **DEPL-01**: TLS termination in deploy stack (nginx HTTPS, encrypted Bearer tokens)
- **DEPL-02**: API key not exposed in dashboard client-side bundle

## Out of Scope

| Feature | Reason |
|---------|--------|
| Full mTLS client certificate auth | Out of proportion for current threat model — P0 TLS fix is sufficient |
| WAF/DDoS protection | Infrastructure-level concern, not application code |
| Key rotation automation | Good practice but not a P0 vulnerability |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| TSEC-01 | Phase 54 | Complete |
| TSEC-02 | Phase 54 | Complete |
| HTTP-01 | Phase 55 | Complete |
| HTTP-02 | Phase 55 | Complete |
| HTTP-03 | Phase 55 | Complete |
| HTTP-04 | Phase 55 | Complete |
| WASM-01 | Phase 56 | Complete |
| WASM-02 | Phase 56 | Complete |

**Coverage:**
- v2.5 requirements: 8 total
- Mapped to phases: 8
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-22*
*Last updated: 2026-03-22 after roadmap creation (traceability complete)*
