<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


---
phase: quick
plan: 260320-ayg
type: execute
wave: 1
depends_on: []
files_modified:
  - docs/technical/threat-model.md
autonomous: true
requirements: []

must_haves:
  truths:
    - "docs/technical/threat-model.md accurately describes v2.2 architecture (not the old August 2025 AI-privacy system)"
    - "All crypto primitives in use are listed with correct algorithm identifiers"
    - "RSA vulnerability history (RUSTSEC-2023-0071) is documented from risk-acceptance in v1.3 through resolution in v2.2"
    - "Each threat category has a current mitigation status (implemented/partial/planned)"
    - "Network components (HTTP /v1/verify, QUIC, Docker) are correctly described"
    - "Key-at-rest format (TRUSTEDGE-KEY-V1) is documented"
  artifacts:
    - path: "docs/technical/threat-model.md"
      provides: "Accurate v2.2 threat model document"
      contains: "TRUSTEDGE-KEY-V1"
  key_links: []
---

<objective>
Rewrite docs/technical/threat-model.md to accurately describe the TrustEdge v2.2 threat model.

Purpose: The current file (dated August 2025) describes an obsolete "AI privacy edge" system that bears no relation to what TrustEdge actually is. It must be replaced with an accurate document covering the real v2.2 architecture, crypto stack, and mitigation history.

Output: A complete, accurate threat model document covering architecture, network components, crypto primitives, threat categories with per-threat mitigation status, RSA vulnerability history, and key-at-rest protection.
</objective>

<execution_context>
@/home/john/.claude/get-shit-done/workflows/execute-plan.md
@/home/john/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@.planning/PROJECT.md
@.planning/STATE.md
@README.md
@docs/architecture.md

<interfaces>
<!-- Current architecture facts the executor must reflect accurately. -->

**Architecture (v2.2, from PROJECT.md and codebase):**
- 9 crates in root workspace + 2 experimental in crates/experimental/ + SvelteKit dashboard at web/dashboard/
- Monolith core (trustedge-core) + thin CLI/WASM shells + platform service
- Full data lifecycle: `trst keygen` → `trst wrap` → `trst verify` → `trst unwrap`

**Crypto primitives actually in use:**
- Ed25519 — archive manifest signing (software keygen), mutual auth
- ECDSA P-256 — YubiKey PIV signing (slot 9c), YubiKey-generated X.509 certs
- AES-256-GCM — per-chunk envelope encryption; key-at-rest encryption (TRUSTEDGE-KEY-V1)
- XChaCha20-Poly1305 — .trst chunk encryption (crypto.rs)
- HKDF-SHA256 (RFC 5869) — v2 envelope key derivation (single Extract+Expand, 40-byte OKM)
- BLAKE3 — continuity chain (genesis seed: blake3("trustedge:genesis"), segment linking)
- RSA-OAEP-SHA256 — hybrid encryption in hybrid.rs (replaced PKCS#1 v1.5 in v2.2)
- PBKDF2-HMAC-SHA256 — key-at-rest (TRUSTEDGE-KEY-V1: 600k iterations, 32-byte salt); keyring backend (600k); minimum enforced at 300k (PBKDF2_MIN_ITERATIONS constant in universal.rs)
- X25519 ECDH — session key derivation for mutual-auth network transport

**Key-at-rest format (TRUSTEDGE-KEY-V1):**
- Format: `TRUSTEDGE-KEY-V1` magic prefix
- KDF: PBKDF2-HMAC-SHA256, 600,000 iterations, 32-byte salt
- Encryption: AES-256-GCM
- CLI: passphrase prompted via rpassword at runtime; `--unencrypted` escape for CI/automation

**Network components:**
- HTTP POST /v1/verify — Axum platform server, JWT bearer auth, CORS restricted
- QUIC transport — secure-by-default (webpki-roots trust store); `insecure-tls` compile-time feature blocks it in release builds (build.rs compile-time guard)
- Docker Compose stack: platform-server (Rust), postgres, dashboard (nginx static)
- Docker internal networking; no external port exposure for postgres

**Replay attack mitigations:**
- Receipt system: cryptographic receipts tied to specific archive content (BLAKE3 digest)
- Mutual auth: challenge-response with BLAKE3 domain-separated KDF, time-limited sessions
- Per-chunk deterministic counter nonces (nonce_prefix || chunk_index || last_flag) — no nonce reuse

**Dashboard / API auth:**
- JWT bearer tokens; JWT secret wrapped in Secret<T> (zeroize on drop)
- LoginRequest uses custom Deserialize: password wrapped in Secret<T> at JSON parse boundary
- CORS: CorsLayer::new() (same-origin only) for verify-only; restricted headers (Content-Type, Authorization, Accept) for postgres build
- /healthz excluded from auth middleware (docker-compose health checks)

**Secret<T> wrapper (v1.7):**
- In-house implementation using zeroize crate
- Redacted Debug (shows "[REDACTED]"), no Display, no Deref, no Serialize
- ZeroizeOnDrop on all secret-holding fields: PIN, passphrase, JWT secret, DB password
- CI Step 23 enforces no Serialize derive regression on secret-holding structs

**RSA vulnerability history:**
- v1.3 (Feb 2026): RUSTSEC-2023-0071 (RSA Marvin Attack / timing side-channel on PKCS#1 v1.5 decryption) risk-accepted with documented rationale; added to .cargo/audit.toml ignore list
  - Rationale at the time: TrustEdge didn't use RSA for production encryption
- v2.2 Phase 45 (Mar 2026): PKCS#1 v1.5 (`Pkcs1v15Encrypt`) replaced with OAEP (`Oaep::new::<sha2::Sha256>()`) in asymmetric.rs (hybrid.rs); RUSTSEC-2023-0071 removed from audit.toml ignore list

**v1 envelope format removal:**
- v1.8 introduced versioned envelopes (v1 = PBKDF2 KDF, v2 = HKDF-once)
- v2.2 Phase 46: v1 format removed entirely (not deprecated) — no v1 envelopes in production
- HKDF domain separation: info = "TRUSTEDGE_ENVELOPE_V1" (context-bound key)

**PBKDF2 minimum enforcement:**
- PBKDF2_MIN_ITERATIONS = 300_000 constant in universal.rs
- Enforced at builder level (assert!) and backend level (error return) — belt-and-suspenders
- Software HSM default: 600,000 iterations (OWASP 2023 PBKDF2-HMAC-SHA256 recommendation)
</interfaces>
</context>

<tasks>

<task type="auto">
  <name>Task 1: Rewrite docs/technical/threat-model.md for v2.2</name>
  <files>docs/technical/threat-model.md</files>
  <action>
Read the existing docs/technical/threat-model.md to understand its structure, then REPLACE it entirely with an accurate v2.2 document. The current file is from August 2025 and describes an obsolete "AI privacy edge" system — it is not an update, it is a full replacement.

The new document must cover:

**1. Header / version block**
- Version: 2.2, Date: 2026-03-20
- Note that it replaces the August 2025 draft
- Link to SECURITY.md for vulnerability reporting

**2. System overview (1 paragraph)**
TrustEdge is a cryptographic provenance system for edge device data. Prove data from capture to verification has not been tampered with using BLAKE3 continuity chains, Ed25519/ECDSA P-256 signatures, and verifiable receipts.

**3. Architecture diagram (ASCII)**
Show the actual system components:
```
[Edge Device / CLI (trst)]
    |-- keygen: TRUSTEDGE-KEY-V1 (PBKDF2 + AES-GCM, passphrase-protected)
    |-- wrap: chunk → XChaCha20-Poly1305 + BLAKE3 chain → Ed25519/ECDSA P-256 sign
    |-- unwrap: verify-then-decrypt
         |
         v
[.trst Archive] ─── HTTP POST /v1/verify ───▶ [Platform Server (Axum)]
                                                     |── JWT bearer auth
                                                     |── BLAKE3 + Ed25519/ECDSA verify
                                                     |── PostgreSQL (receipts, devices)
                                                     |── JWKS endpoint
                                                     v
                                              [Cryptographic Receipt (JWS)]
                                                     |
                                              [Dashboard (SvelteKit + nginx)]
                                              [Browser WASM (trst-wasm)]
```

**4. Assets under protection**
- Archive content (plaintext data, chunk-encrypted at rest and in transit)
- Device private keys (encrypted at rest: TRUSTEDGE-KEY-V1)
- Verification receipts (JWS-signed)
- Platform JWT secret (Secret<T>, zeroize on drop)
- PostgreSQL credentials (Secret<T>)
- YubiKey PIV PIN (Secret<T>, rpassword prompt)

**5. Crypto primitives table**
| Primitive | Algorithm | Where Used | Notes |
(one row per primitive from the interfaces block above)

**6. Threat categories — each with Description, Attack Vector, Current Status (MITIGATED / PARTIAL / PLANNED), and Mitigation Details**

Cover these threats (use the context block for accurate details):

**T1: Data tampering in transit**
- Mitigated: AES-GCM auth tags on chunks; Ed25519/ECDSA P-256 signature on manifest; BLAKE3 continuity chain (any missing/reordered chunk breaks chain); verify-before-decrypt in trst unwrap

**T2: Data tampering at rest**
- Mitigated: .trst archives are read-only tamper-evident bundles; signature over manifest.json; BLAKE3 chain links all chunks to genesis seed

**T3: Key compromise (private key at rest)**
- Mitigated (v2.2): TRUSTEDGE-KEY-V1 format — PBKDF2-HMAC-SHA256 (600k iterations, 32-byte salt) + AES-256-GCM protects private key on disk; passphrase prompted via rpassword; --unencrypted escape for CI with explicit opt-in
- Hardware option: YubiKey PIV — private key never leaves hardware

**T4: Padding oracle / RSA PKCS#1 v1.5 (RUSTSEC-2023-0071)**
- History section (see below): risk-accepted v1.3, fully resolved v2.2
- Mitigated (v2.2): RSA uses OAEP-SHA256 exclusively (Oaep::new::<sha2::Sha256>()); PKCS#1 v1.5 code path eliminated

**T5: Weak key derivation**
- Mitigated: PBKDF2_MIN_ITERATIONS = 300,000 enforced at builder + backend; default 600,000 (OWASP 2023); envelope key derivation uses HKDF-SHA256 (correct KDF for high-entropy input per NIST SP 800-56C)

**T6: Legacy envelope format**
- Mitigated (v2.2): v1 envelope format removed entirely — v2-only codebase; no v1 envelopes in production; HKDF domain separation via "TRUSTEDGE_ENVELOPE_V1" info parameter

**T7: Insecure transport**
- Mitigated: QUIC uses webpki-roots trust store by default; insecure-tls is a compile-time feature flag blocked in release builds by build.rs compile-time guard (cfg!(not(debug_assertions))); HTTP platform API uses TLS in production Docker stack

**T8: Replay attacks on verification receipts**
- Partial: Receipts are cryptographically bound to specific archive content (BLAKE3 digest); per-chunk deterministic nonces prevent chunk-level replay; challenge-response in mutual auth uses BLAKE3 domain-separated KDF with time-limited sessions
- Planned: Sliding-window nonce validation for high-volume verification endpoints

**T9: Dashboard / API authentication bypass**
- Mitigated: JWT bearer tokens required for all platform endpoints (except /healthz); JWT secret in Secret<T> with ZeroizeOnDrop; CORS restricted (CorsLayer::new() same-origin for verify-only, restricted headers for postgres build); LoginRequest password wrapped in Secret<T> at JSON parse boundary

**T10: Secret material in memory**
- Mitigated: Secret<T> wrapper with ZeroizeOnDrop on all sensitive fields (PIN, passphrase, JWT secret, DB password); redacted Debug output; no Serialize/Display/Deref derives; CI Step 23 enforces no regression; Rust memory safety prevents use-after-free

**T11: Supply chain (dependency vulnerabilities)**
- Mitigated: cargo-audit integrated into CI as blocking check; Cargo.lock tracked in git for reproducible audits; RUSTSEC-2023-0071 now resolved and removed from audit.toml ignore list

**T12: YubiKey hardware failure / fallback**
- Mitigated: Fail-closed design — hardware unavailable = error, no silent software fallback; ensure_connected() gates every PIV operation; 18 simulation tests + 9 hardware integration tests

**7. RSA vulnerability history (dedicated section)**
Narrate the lifecycle:
- v1.3 (2026-02-13): RUSTSEC-2023-0071 (Marvin Attack timing side-channel on PKCS#1 v1.5 decrypt in `rsa` crate) identified during cargo-audit integration. Risk-accepted: TrustEdge's RSA usage was in hybrid.rs for non-production encryption scenarios. Added to .cargo/audit.toml ignore list with documented rationale.
- v1.3 through v2.1: Advisory carried as acknowledged risk. Any code using hybrid.rs RSA was potentially vulnerable to timing-based key recovery.
- v2.2 Phase 45 (2026-03-19): PKCS#1 v1.5 (`Pkcs1v15Encrypt` trait) fully replaced with OAEP (`Oaep::new::<sha2::Sha256>()`). RUSTSEC-2023-0071 removed from audit.toml ignore list. cargo-audit now passes with no ignored advisories.

**8. Current mitigation status table**
Quick reference:
| Threat | Status | Version Resolved |
| Data tampering | MITIGATED | v1.0 |
| Key compromise (at rest) | MITIGATED | v2.2 |
| RSA padding oracle | MITIGATED | v2.2 |
| Weak KDF | MITIGATED | v1.8 / v2.2 |
| Legacy envelope | MITIGATED | v2.2 |
| Insecure transport | MITIGATED | v1.4 / v2.2 |
| Replay (receipts) | PARTIAL | — |
| API auth bypass | MITIGATED | v1.7 |
| Secret material in memory | MITIGATED | v1.7 |
| Supply chain | MITIGATED | v1.3 / v2.2 |
| YubiKey fallback | MITIGATED | v1.1 |

**9. Out of scope**
- TPM support (planned future milestone)
- Post-quantum cryptography (research only)
- Physical device security (OS-level)
- Regulatory compliance (GDPR, CCPA)

**10. Document maintenance note**
Review before each major release and after security incidents.

Format requirements:
- MPL-2.0 copyright header at top (HTML comment, same as other docs)
- Markdown with H2 sections
- No emoji — use plain text status labels (MITIGATED, PARTIAL, PLANNED)
- Tables for primitives and mitigation status
- ASCII diagram for architecture
  </action>
  <verify>
    <automated>grep -c "TRUSTEDGE-KEY-V1" /home/john/vault/projects/github.com/trustedge/docs/technical/threat-model.md && grep -c "RUSTSEC-2023-0071" /home/john/vault/projects/github.com/trustedge/docs/technical/threat-model.md && grep -c "OAEP" /home/john/vault/projects/github.com/trustedge/docs/technical/threat-model.md && grep -c "HKDF" /home/john/vault/projects/github.com/trustedge/docs/technical/threat-model.md</automated>
  </verify>
  <done>docs/technical/threat-model.md exists, is dated 2026-03-20, contains TRUSTEDGE-KEY-V1, RUSTSEC-2023-0071, OAEP-SHA256, HKDF-SHA256, and all threat categories from the context block. The August 2025 AI-privacy content is gone.</done>
</task>

</tasks>

<verification>
After writing the document:
1. `grep -c "TRUSTEDGE-KEY-V1" docs/technical/threat-model.md` returns >= 1
2. `grep -c "RUSTSEC-2023-0071" docs/technical/threat-model.md` returns >= 1
3. `grep -c "OAEP" docs/technical/threat-model.md` returns >= 1
4. `grep -c "Version.*2.2" docs/technical/threat-model.md` returns >= 1
5. Old content ("AI workloads", "Edge AI", "August 2025") is absent
</verification>

<success_criteria>
docs/technical/threat-model.md is a complete, accurate threat model for TrustEdge v2.2. A security engineer reading it would have a correct understanding of what the system does, what crypto it uses, what threats it mitigates (and how), and the history of the RSA vulnerability. No content from the August 2025 AI-privacy draft remains.
</success_criteria>

<output>
After completion, create `.planning/quick/260320-ayg-create-docs-technical-threat-model-md-wi/260320-ayg-SUMMARY.md` using the summary template.

Commit: `docs: rewrite threat model for TrustEdge v2.2`
Files: `docs/technical/threat-model.md`
</output>
