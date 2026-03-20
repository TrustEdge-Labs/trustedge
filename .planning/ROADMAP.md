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
- 🚧 **v2.3 Security Testing** - Phases 48-51 (in progress)

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

---

### v2.3 Security Testing (In Progress)

**Milestone Goal:** Implement targeted security tests that attempt to exploit vulnerabilities identified in the threat model — providing concrete evidence for TrustEdge's security claims across archive integrity, cryptographic properties, key protection, and receipt binding.

## Phases (v2.3)

- [x] **Phase 48: Archive Integrity Attacks** - Tests that byte-level tampering, chunk injection, reordering, and manifest modification are all detected by trst verify (completed 2026-03-20)
- [ ] **Phase 49: Nonce and Key Derivation** - Tests that nonces are unique across and within archives, and that HKDF produces distinct keys for distinct device keys
- [ ] **Phase 50: Encrypted Key File Protection** - Tests that truncated, corrupted, and wrong-passphrase key files are rejected with clear errors, not silently mishandled
- [ ] **Phase 51: Verification Receipt Binding** - Tests that duplicate archive submissions produce distinct receipts and that receipt digest is bound to archive content

## Phase Details

### Phase 48: Archive Integrity Attacks
**Goal**: Users have concrete evidence that any modification to a .trst archive — at the byte, chunk, or manifest level — is detected and rejected by trst verify
**Depends on**: Nothing (first phase of milestone)
**Requirements**: SEC-01, SEC-02, SEC-03, SEC-04
**Success Criteria** (what must be TRUE):
  1. Flipping any byte in an encrypted chunk causes `trst verify` to return a non-zero exit code with an authentication tag error
  2. Adding a spurious chunk file to the archive directory causes `trst verify` to fail with a BLAKE3 chain break error
  3. Swapping the order of two chunk files causes `trst verify` to fail with a continuity chain error
  4. Changing any field in manifest.json after signing causes `trst verify` to fail with a signature verification error
**Plans**: 1 plan

Plans:
- [ ] 48-01: Archive tampering attack tests (byte mutation, chunk injection, reorder, manifest modification)

### Phase 49: Nonce and Key Derivation
**Goal**: Users have concrete evidence that TrustEdge never reuses nonces within or across archives, and that HKDF key derivation is key-bound
**Depends on**: Phase 48
**Requirements**: SEC-05, SEC-06, SEC-07
**Success Criteria** (what must be TRUE):
  1. Inspecting all chunk nonces within a single archive confirms no two chunks share the same nonce
  2. Encrypting the same plaintext twice with the same device key produces two archives with different chunk nonces
  3. Deriving envelope keys from two different device keys produces two non-equal AES-256-GCM keys
**Plans**: 1 plan

Plans:
- [ ] 49-01: Nonce uniqueness and HKDF key derivation tests

### Phase 50: Encrypted Key File Protection
**Goal**: Users have concrete evidence that malformed, corrupted, or wrong-passphrase key files are rejected safely — no garbled output, no silent data corruption
**Depends on**: Phase 49
**Requirements**: SEC-08, SEC-09, SEC-10
**Success Criteria** (what must be TRUE):
  1. Loading a key file truncated to an arbitrary byte length returns an explicit error, not a panic or partial key
  2. Loading a key file with a corrupted JSON header returns a clear parse error identifying the problem
  3. Supplying the wrong passphrase to a valid encrypted key file returns a clear authentication error, not garbled key material
**Plans**: 1 plan

Plans:
- [ ] 50-01: Encrypted key file format attack tests (truncation, corruption, wrong passphrase)

### Phase 51: Verification Receipt Binding
**Goal**: Users have concrete evidence that the verification receipt system cannot be exploited via replay — each submission produces a distinct receipt bound to the exact archive content
**Depends on**: Phase 50
**Requirements**: SEC-11, SEC-12
**Success Criteria** (what must be TRUE):
  1. Submitting the same archive to /v1/verify twice returns two receipts with different verification IDs and different timestamps
  2. The manifest_digest field in a receipt matches the BLAKE3 digest of the exact archive submitted — a different archive produces a different digest
**Plans**: 1 plan

Plans:
- [ ] 51-01: Replay resistance and receipt content binding tests

## Progress

**Execution Order:** 48 → 49 → 50 → 51

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 48. Archive Integrity Attacks | 1/1 | Complete    | 2026-03-20 | - |
| 49. Nonce and Key Derivation | v2.3 | 0/1 | Not started | - |
| 50. Encrypted Key File Protection | v2.3 | 0/1 | Not started | - |
| 51. Verification Receipt Binding | v2.3 | 0/1 | Not started | - |

---
*Last updated: 2026-03-20 after v2.3 roadmap created*
