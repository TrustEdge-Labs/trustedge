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
- 🚧 **v1.8 KDF Architecture Fix** - Phases 35-37 (in progress)

## Phases

<details>
<summary>✅ v1.0 Consolidation (Phases 1-8) - SHIPPED 2026-02-11</summary>

Consolidated TrustEdge from 10 scattered crates into monolithic core with thin CLI/WASM shells. Zero API breaking changes, 98.6% test retention (343 tests), WASM compatibility preserved. Eliminated ~2,500 LOC duplication, removed 21 unused dependencies. Established 6-layer architecture, unified error types, migrated receipts and attestation into core, deprecated facade crates with 6-month migration window.

**See:** `.planning/milestones/v1.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.1 YubiKey Integration Overhaul (Phases 9-12) - SHIPPED 2026-02-11</summary>

Deleted broken YubiKey backend (8,117 lines) and rewrote from scratch with fail-closed design, battle-tested libraries only (yubikey crate stable API, rcgen for X.509), comprehensive test suite (18 simulation + 9 hardware), and unconditional CI validation on every PR.

**See:** `.planning/milestones/v1.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.2 Scope Reduction (Phases 13-14) - SHIPPED 2026-02-12</summary>

Made TrustEdge maintainable by a solo developer — 2-tier crate classification (stable/experimental), full dependency audit with documentation, trimmed tokio features, tiered CI pipeline (core blocking, experimental non-blocking), dependency tree size tracking, and updated README with crate classification.

**See:** `.planning/milestones/v1.2-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.3 Dependency Audit & Rationalization (Phases 15-18) - SHIPPED 2026-02-13</summary>

Hardened the dependency tree across all 10 crates — feature-gated heavy optional deps (git2, keyring), removed unused deps via cargo-machete, integrated cargo-audit into CI, and documented every remaining dependency with justification in DEPENDENCIES.md.

**See:** `.planning/milestones/v1.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.4 Placeholder Elimination (Phases 19-23) - SHIPPED 2026-02-13</summary>

Removed all placeholder code, incomplete features, and insecure defaults. Secured QUIC TLS by default, removed dead code and stubs from core and Pubky crates, enforced zero-TODO hygiene with CI enforcement on every push/PR.

**See:** `.planning/milestones/v1.4-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.5 Platform Consolidation (Phases 24-27) - SHIPPED 2026-02-22</summary>

Consolidated external service repos (platform-api, verify-core, shared-libs) into the main trustedge workspace. Created trustedge-types crate for shared wire types, merged platform-api and verify-core into unified trustedge-platform crate, replaced all manual crypto with trustedge-core primitives, and archived 5 scaffold repos with scope documentation.

**See:** `.planning/milestones/v1.5-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.6 Final Consolidation (Phases 28-30) - SHIPPED 2026-02-22</summary>

Brought all satellite code into the monorepo and finalized the GitHub org structure. Created standalone platform server binary (crates/platform-server) with Axum HTTP, clap CLI, and deployment artifacts. Moved SvelteKit dashboard into web/dashboard/ with TypeScript types generated from trustedge-types JSON schemas. Deleted 11 orphaned repos, reducing TrustEdge-Labs org to 3 repos (trustedge, trustedgelabs-website, shipsecure).

**See:** `.planning/milestones/v1.6-ROADMAP.md` for full phase details.

</details>

<details>
<summary>✅ v1.7 Security & Quality Hardening (Phases 31-34) - SHIPPED 2026-02-23</summary>

Hardened secret handling with in-house Secret<T> wrapper (zeroize, redacted Debug, no serde). Deleted deprecated facade crates and isolated experimental pubky crates into standalone workspace. Deduplicated verify handler validation, hardened CORS (same-origin for verify-only, restricted headers for postgres), documented CA module as library-only. Added 16 integration tests: platform-server wiring (5), CORS parity (1), full HTTP verify round-trip with JWS/JWKS verification (4), plus existing tests. 14/14 requirements shipped, 44 commits, 90 files changed.

**See:** `.planning/milestones/v1.7-ROADMAP.md` for full phase details.

</details>

---

### 🚧 v1.8 KDF Architecture Fix (In Progress)

**Milestone Goal:** Fix incorrect KDF usage across the codebase — replace PBKDF2-per-chunk with HKDF hierarchical key derivation in envelope.rs, and harden keyring backend parameters.

## Phases

- [ ] **Phase 35: HKDF Infrastructure** - Add hkdf workspace dependency and establish correct HKDF input structure in envelope.rs, eliminating the ad-hoc CatKDF construction
- [ ] **Phase 36: Envelope Format Migration** - Rewrite envelope encryption to HKDF-once with counter nonces and versioned format supporting both v1 (legacy) and v2 (HKDF) decryption paths
- [ ] **Phase 37: Keyring Hardening** - Increase PBKDF2 parameters in both keyring backends to OWASP 2023 recommended levels

## Phase Details

### Phase 35: HKDF Infrastructure
**Goal**: The hkdf crate is wired into the workspace and envelope.rs uses correctly structured HKDF inputs with domain separation — no ad-hoc key material concatenation
**Depends on**: Nothing (first phase of this milestone)
**Requirements**: ENV-04, ENV-05, ENV-06
**Success Criteria** (what must be TRUE):
  1. `hkdf` crate appears as a workspace dependency with a pinned version in the root Cargo.toml
  2. The ad-hoc CatKDF construction (concatenating shared_secret + salt + sequence + metadata as IKM) is gone from envelope.rs
  3. HKDF inputs use structured fields: ECDH shared secret as IKM, a TrustEdge-specific domain separation string as info
  4. `cargo test -p trustedge-core --lib` passes with no regressions
**Plans**: TBD

### Phase 36: Envelope Format Migration
**Goal**: Envelope encryption uses HKDF-once key derivation with deterministic counter nonces, and the format version field enables backward-compatible decryption of both old (PBKDF2-per-chunk) and new (HKDF-once) envelopes
**Depends on**: Phase 35
**Requirements**: ENV-01, ENV-02, ENV-03, VER-01, VER-02, TST-01, TST-02
**Success Criteria** (what must be TRUE):
  1. Encrypting a multi-chunk envelope derives DerivedKey exactly once via HKDF-Extract + Expand, not once per chunk
  2. Per-chunk nonces are deterministic counters (NoncePrefix || chunk_index || last_flag), not randomly generated salts
  3. Encrypted envelopes carry a version field; v2 envelopes are produced by default
  4. Decrypting a v1 (legacy PBKDF2-per-chunk) envelope succeeds without modification to the stored data
  5. All existing envelope tests pass; a new multi-chunk round-trip test covering the v2 format passes
**Plans**: TBD

### Phase 37: Keyring Hardening
**Goal**: Both keyring backends use OWASP 2023-recommended PBKDF2 parameters — 600,000 iterations and 32-byte salts — so keyring-encrypted secrets resist modern brute-force attacks
**Depends on**: Phase 35
**Requirements**: KEY-01, KEY-02, KEY-03, KEY-04, TST-03
**Success Criteria** (what must be TRUE):
  1. `keyring.rs` PBKDF2 iteration count reads 600,000 (not 100,000) in source
  2. `keyring.rs` salt length reads 32 bytes (not 16 bytes) in source
  3. `universal_keyring.rs` PBKDF2 iteration count reads 600,000 in source
  4. `universal_keyring.rs` salt length reads 32 bytes in source
  5. Keyring encryption/decryption tests pass with updated parameters; `cargo test -p trustedge-core --lib` passes
**Plans**: TBD

## Progress

**Execution Order:** 35 → 36 → 37 (Phase 37 may run after Phase 36 or parallel once Phase 35 completes)

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 35. HKDF Infrastructure | 0/TBD | Not started | - |
| 36. Envelope Format Migration | 0/TBD | Not started | - |
| 37. Keyring Hardening | 0/TBD | Not started | - |

---
*Last updated: 2026-02-22 after v1.8 roadmap created*
