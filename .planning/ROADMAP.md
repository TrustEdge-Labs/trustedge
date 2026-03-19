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
- 🚧 **v2.2 Security Remediation** - Phases 45-47 (in progress)

## Phases

<details>
<summary>v1.0-v1.8 (Phases 1-37) - See milestone archives</summary>

See `.planning/milestones/v1.0-ROADMAP.md` through `.planning/milestones/v1.8-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.0 End-to-End Demo (Phases 38-41) - SHIPPED 2026-03-16</summary>

Delivered working end-to-end demonstration of TrustEdge's full value proposition. Generic archive profiles (TrstManifest with ProfileMetadata enum), one-command Docker stack (platform + postgres + dashboard with auto-migration), demo script showing full lifecycle (keygen, wrap, verify, receipt), and README rewrite (465 to 128 lines with problem statement, 3-command quick start, 4 use cases). 4 phases, 8 plans, 17/17 requirements complete, 42 commits, 50 files changed.

**See:** `.planning/milestones/v2.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.1 Data Lifecycle & Hardware Integration (Phases 42-44) - SHIPPED 2026-03-18</summary>

Completed the data lifecycle with decryption capability, exposed YubiKey hardware signing in the CLI, and added named archive profiles for real-world use cases. Named profiles (sensor, audio, log) with typed metadata structs. `trst unwrap` with HKDF key derivation and verify-before-decrypt. Multi-algorithm verify dispatch (Ed25519 + ECDSA P-256). `--backend yubikey` for hardware signing with interactive PIN. 3 phases, 6 plans, 12/12 requirements complete, 27 commits, 28 files changed.

**See:** `.planning/milestones/v2.1-ROADMAP.md` for full phase details.

</details>

---

### v2.2 Security Remediation (In Progress)

**Milestone Goal:** Fix critical cryptographic flaws — replace insecure RSA PKCS#1 v1.5 with OAEP, deprecate v1 envelope format, enforce PBKDF2 minimums, and encrypt device keys at rest.

#### Phase Summary

- [x] **Phase 45: RSA OAEP Migration** - Replace PKCS#1 v1.5 with OAEP-SHA256 padding in asymmetric.rs (encrypt and decrypt paths) (completed 2026-03-18)
- [x] **Phase 46: Envelope Hardening** - Remove v1 envelope format, enforce v2-only sealing, enforce PBKDF2 minimum iterations (completed 2026-03-19)
- [ ] **Phase 47: Key Protection at Rest** - Encrypt device key files with passphrase, require passphrase on wrap/unwrap, reject unencrypted keys by default

## Phase Details

### Phase 45: RSA OAEP Migration
**Goal**: RSA asymmetric operations are resistant to padding oracle attacks
**Depends on**: Nothing (isolated to asymmetric.rs)
**Requirements**: RSA-01, RSA-02
**Success Criteria** (what must be TRUE):
  1. `trustedge` CLI encrypting with RSA produces OAEP-SHA256 ciphertext (PKCS#1 v1.5 ciphertext is rejected on decrypt)
  2. Decryption of PKCS#1 v1.5 ciphertext returns an error rather than silently succeeding
  3. All existing RSA tests pass using OAEP padding (no test is skipped or weakened)
  4. cargo-audit no longer flags RSA Marvin Attack advisory as a live concern for the encrypt/decrypt path
**Plans:** 1/1 plans complete

Plans:
- [x] 45-01-PLAN.md -- Replace PKCS#1 v1.5 with OAEP-SHA256 and update cargo-audit config

### Phase 46: Envelope Hardening
**Goal**: v1 envelope format is removed and PBKDF2 iteration minimums are enforced everywhere
**Depends on**: Phase 45
**Requirements**: ENV-01, ENV-02, KDF-01
**Success Criteria** (what must be TRUE):
  1. Calling `seal()` always produces a v2 envelope — no code path in the library produces v1 format
  2. No v1 decrypt code path exists — `decrypt_chunk_v1()` and v1 fallback in `unseal()` are deleted
  3. Any call to a PBKDF2 function with fewer than 300,000 iterations returns an error rather than proceeding
  4. CI passes with existing test suite (v1-only tests deleted, v2 tests pass)
**Plans:** 2/2 plans complete

Plans:
- [x] 46-01-PLAN.md -- Remove v1 envelope format entirely from envelope.rs
- [x] 46-02-PLAN.md -- Enforce PBKDF2 minimum 300k iterations in all backends

### Phase 47: Key Protection at Rest
**Goal**: Device key files are encrypted at rest and the CLI refuses to use unencrypted keys by default
**Depends on**: Phase 46
**Requirements**: KEY-01, KEY-02, KEY-03
**Success Criteria** (what must be TRUE):
  1. `trst keygen` prompts for a passphrase and writes an encrypted private key file (not plaintext)
  2. `trst wrap` prompts for passphrase and decrypts the key before signing — no plaintext key is written to disk during the operation
  3. `trst unwrap` prompts for passphrase and decrypts the key before reassembling — operation fails if wrong passphrase is supplied
  4. Passing a plaintext (unencrypted) key file to `trst wrap` or `trst unwrap` returns an error unless `--unencrypted` flag is provided
  5. CI/automation can pass `--unencrypted` to bypass the passphrase requirement without interactive input
**Plans:** 1/2 plans executed

Plans:
- [ ] 47-01-PLAN.md -- Add encrypted key export/import to DeviceKeypair in crypto.rs
- [ ] 47-02-PLAN.md -- Integrate passphrase prompts and --unencrypted flag into CLI + tests + demo

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 45. RSA OAEP Migration | 1/1 | Complete    | 2026-03-18 | - |
| 46. Envelope Hardening | 2/2 | Complete    | 2026-03-19 | - |
| 47. Key Protection at Rest | 1/2 | In Progress|  | - |

---
*Last updated: 2026-03-19 after phase 47 planning*
