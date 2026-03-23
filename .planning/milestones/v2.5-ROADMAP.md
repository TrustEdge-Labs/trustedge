<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Integration Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security & Quality Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Architecture Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle & Hardware Integration** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)
- ✅ **v2.3 Security Testing** - Phases 48-51 (shipped 2026-03-21)
- ✅ **v2.4 Security Review Remediation** - Phases 52-53 (shipped 2026-03-22)
- 🔄 **v2.5 Critical Security Fixes** - Phases 54-56 (active)

## Phases

<details>
<summary>v1.0-v1.8 (Phases 1-37) - See milestone archives</summary>

See `.planning/milestones/v1.0-ROADMAP.md` through `.planning/milestones/v1.8-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.0 End-to-End Demo (Phases 38-41) - SHIPPED 2026-03-16</summary>

Delivered working end-to-end demonstration of TrustEdge's full value proposition. Generic archive profiles, one-command Docker stack, demo script, and README rewrite. 4 phases, 8 plans, 17/17 requirements complete.

**See:** `.planning/milestones/v2.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.1 Data Lifecycle & Hardware Integration (Phases 42-44) - SHIPPED 2026-03-18</summary>

Completed the data lifecycle with decryption capability, exposed YubiKey hardware signing in the CLI, and added named archive profiles. 3 phases, 6 plans, 12/12 requirements complete.

**See:** `.planning/milestones/v2.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.2 Security Remediation (Phases 45-47) - SHIPPED 2026-03-19</summary>

Fixed critical cryptographic flaws. RSA OAEP-SHA256 replaces PKCS#1 v1.5, v1 envelope format removed entirely, PBKDF2 minimum 300k iterations enforced, device keys encrypted at rest with passphrase protection. RUSTSEC-2023-0071 fully resolved. 3 phases, 5 plans, 8/8 requirements complete, 23 commits.

**See:** `.planning/milestones/v2.2-ROADMAP.md` for full phase details.

</details>


<details>
<summary>v2.3 Security Testing (Phases 48-51) - SHIPPED 2026-03-21</summary>

31 new security tests proving TrustEdge's tamper-evidence, nonce uniqueness, key protection, and replay resistance claims across 4 threat model categories (T1/T2/T3/T5/T6/T8). 4 phases, 4 plans, 12/12 requirements complete, 6 commits.

**See:** `.planning/milestones/v2.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.4 Security Review Remediation (Phases 52-53) - SHIPPED 2026-03-22</summary>

Addressed all P1/P2 findings from the code & security review. Custom base64 replaced, key format versioned, timestamp replay fixed, envelope panics eliminated, key file permissions enforced, nonce overflow guarded, 14 error path tests added. 2 phases, 3 plans, 8/8 requirements complete, 19 commits.

**See:** `.planning/milestones/v2.4-ROADMAP.md` for full phase details.

</details>

### v2.5 Critical Security Fixes (Phases 54-56) — Active

- [x] **Phase 54: Transport Security** - Fix QUIC TLS certificate verification no-op (MITM vulnerability) (completed 2026-03-23)
- [x] **Phase 55: Platform HTTP Hardening** - Body size limit, rate limiting, and configurable JWKS signing key (completed 2026-03-23)
- [x] **Phase 56: WASM Fix** - Fix double-decrypt bug in trst-wasm browser verification (completed 2026-03-23)

## Phase Details

### Phase 54: Transport Security
**Goal**: QUIC connections verify server certificates cryptographically — no MITM attack possible
**Depends on**: Nothing (independent fix)
**Requirements**: TSEC-01, TSEC-02
**Success Criteria** (what must be TRUE):
  1. A QUIC connection attempt with a forged or self-signed certificate (not matching the trusted root) is rejected with a TLS error
  2. `HardwareBackedVerifier::verify_tls12_signature` and `verify_tls13_signature` perform actual cryptographic verification using the provided certificate and message/signature inputs
  3. A MITM test proves that substituting a different certificate causes the handshake to fail
  4. Legitimate QUIC connections with a valid certificate continue to succeed
**Plans:** 1/1 plans complete
Plans:
- [x] 54-01-PLAN.md — Fix signature verification, gate dev mode, unit + integration tests

### Phase 55: Platform HTTP Hardening
**Goal**: The HTTP platform endpoints are protected against body-flood DoS, verify-loop CPU abuse, and plaintext key leakage
**Depends on**: Nothing (independent fix)
**Requirements**: HTTP-01, HTTP-02, HTTP-03, HTTP-04
**Success Criteria** (what must be TRUE):
  1. A POST to `/v1/verify` with a body exceeding the configured limit (1-10 MB) receives a 413 response without OOM risk
  2. Repeated rapid calls to `/v1/verify` beyond the rate limit receive a 429 response
  3. The JWKS signing key path is read from an environment variable, not hardcoded to `target/dev/`
  4. No unencrypted signing key file appears under `target/dev/` or any build-artifact directory during server startup
**Plans:** 2/2 plans complete
Plans:
- [x] 55-01-PLAN.md — Body size limit (RequestBodyLimitLayer) and per-IP rate limiting (governor) on /v1/verify
- [x] 55-02-PLAN.md — JWKS signing key path configuration via env var, 0600 permissions, remove target/dev/ hardcoding
**UI hint**: no

### Phase 56: WASM Fix
**Goal**: Browser-based archive verification decrypts data correctly — no double-decrypt corruption
**Depends on**: Nothing (independent fix)
**Requirements**: WASM-01, WASM-02
**Success Criteria** (what must be TRUE):
  1. The trst-wasm `decrypt` path calls `.decrypt()` exactly once per ciphertext chunk
  2. An end-to-end WASM test wraps an archive, passes it through the browser WASM bindings, and verifies the recovered data matches the original input
  3. The previously failing browser verification path now returns the correct plaintext without corruption
**Plans:** 1/1 plans complete
Plans:
- [x] 56-01-PLAN.md — Fix double-decrypt bug, add aes-gcm/rand deps, wire crypto module, round-trip tests
**UI hint**: no

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 54. Transport Security | 1/1 | Complete    | 2026-03-23 |
| 55. Platform HTTP Hardening | 2/2 | Complete    | 2026-03-23 |
| 56. WASM Fix | 1/1 | Complete    | 2026-03-23 |

---
*Last updated: 2026-03-23 after Phase 56 planning complete*
