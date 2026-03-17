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
- 🚧 **v2.1 Data Lifecycle & Hardware Integration** - Phases 42-44 (in progress)

## Phases

<details>
<summary>v1.0 Consolidation (Phases 1-8) - SHIPPED 2026-02-11</summary>

Consolidated TrustEdge from 10 scattered crates into monolithic core with thin CLI/WASM shells. Zero API breaking changes, 98.6% test retention (343 tests), WASM compatibility preserved. Eliminated ~2,500 LOC duplication, removed 21 unused dependencies. Established 6-layer architecture, unified error types, migrated receipts and attestation into core, deprecated facade crates with 6-month migration window.

**See:** `.planning/milestones/v1.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.1 YubiKey Integration Overhaul (Phases 9-12) - SHIPPED 2026-02-11</summary>

Deleted broken YubiKey backend (8,117 lines) and rewrote from scratch with fail-closed design, battle-tested libraries only (yubikey crate stable API, rcgen for X.509), comprehensive test suite (18 simulation + 9 hardware), and unconditional CI validation on every PR.

**See:** `.planning/milestones/v1.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.2 Scope Reduction (Phases 13-14) - SHIPPED 2026-02-12</summary>

Made TrustEdge maintainable by a solo developer -- 2-tier crate classification (stable/experimental), full dependency audit with documentation, trimmed tokio features, tiered CI pipeline (core blocking, experimental non-blocking), dependency tree size tracking, and updated README with crate classification.

**See:** `.planning/milestones/v1.2-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.3 Dependency Audit & Rationalization (Phases 15-18) - SHIPPED 2026-02-13</summary>

Hardened the dependency tree across all 10 crates -- feature-gated heavy optional deps (git2, keyring), removed unused deps via cargo-machete, integrated cargo-audit into CI, and documented every remaining dependency with justification in DEPENDENCIES.md.

**See:** `.planning/milestones/v1.3-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.4 Placeholder Elimination (Phases 19-23) - SHIPPED 2026-02-13</summary>

Removed all placeholder code, incomplete features, and insecure defaults. Secured QUIC TLS by default, removed dead code and stubs from core and Pubky crates, enforced zero-TODO hygiene with CI enforcement on every push/PR.

**See:** `.planning/milestones/v1.4-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.5 Platform Consolidation (Phases 24-27) - SHIPPED 2026-02-22</summary>

Consolidated external service repos (platform-api, verify-core, shared-libs) into the main trustedge workspace. Created trustedge-types crate for shared wire types, merged platform-api and verify-core into unified trustedge-platform crate, replaced all manual crypto with trustedge-core primitives, and archived 5 scaffold repos with scope documentation.

**See:** `.planning/milestones/v1.5-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.6 Final Consolidation (Phases 28-30) - SHIPPED 2026-02-22</summary>

Brought all satellite code into the monorepo and finalized the GitHub org structure. Created standalone platform server binary (crates/platform-server) with Axum HTTP, clap CLI, and deployment artifacts. Moved SvelteKit dashboard into web/dashboard/ with TypeScript types generated from trustedge-types JSON schemas. Deleted 11 orphaned repos, reducing TrustEdge-Labs org to 3 repos (trustedge, trustedgelabs-website, shipsecure).

**See:** `.planning/milestones/v1.6-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.7 Security & Quality Hardening (Phases 31-34) - SHIPPED 2026-02-23</summary>

Hardened secret handling with in-house Secret<T> wrapper (zeroize, redacted Debug, no serde). Deleted deprecated facade crates and isolated experimental pubky crates into standalone workspace. Deduplicated verify handler validation, hardened CORS (same-origin for verify-only, restricted headers for postgres), documented CA module as library-only. Added 16 integration tests: platform-server wiring (5), CORS parity (1), full HTTP verify round-trip with JWS/JWKS verification (4), plus existing tests. 14/14 requirements shipped, 44 commits, 90 files changed.

**See:** `.planning/milestones/v1.7-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v1.8 KDF Architecture Fix (Phases 35-37) - SHIPPED 2026-02-24</summary>

Fixed incorrect KDF usage across the codebase. Replaced PBKDF2-per-chunk with HKDF-SHA256 hierarchical key derivation in envelope.rs, added versioned envelope format (v2 HKDF-once with deterministic counter nonces) with backward-compatible v1 decryption, and hardened keyring PBKDF2 parameters to OWASP 2023 levels (600k iterations, 32-byte salts). 3 phases, 4 plans, 8 tasks, 15/15 requirements complete.

**See:** `.planning/milestones/v1.8-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.0 End-to-End Demo (Phases 38-41) - SHIPPED 2026-03-16</summary>

Delivered working end-to-end demonstration of TrustEdge's full value proposition. Generic archive profiles (TrstManifest with ProfileMetadata enum), one-command Docker stack (platform + postgres + dashboard with auto-migration), demo script showing full lifecycle (keygen, wrap, verify, receipt), and README rewrite (465 to 128 lines with problem statement, 3-command quick start, 4 use cases). 4 phases, 8 plans, 17/17 requirements complete, 42 commits, 50 files changed.

**See:** `.planning/milestones/v2.0-ROADMAP.md` for full phase details.

</details>

---

### v2.1 Data Lifecycle & Hardware Integration (In Progress)

**Milestone Goal:** Complete the data lifecycle by adding decryption/unwrap capability, expose YubiKey hardware signing in the CLI, and add named archive profiles for specific use cases.

## Phase Details

### Phase 42: Named Archive Profiles
**Goal**: Users can wrap data with use-case-specific metadata schemas (sensor, audio, log) that produce valid, verifiable archives
**Depends on**: Nothing (pure type additions to trustedge-trst-protocols, no upstream workspace deps)
**Requirements**: PROF-05, PROF-06, PROF-07, PROF-08
**Success Criteria** (what must be TRUE):
  1. User can run `trst wrap --profile sensor` with sensor-specific fields (sample_rate, unit, sensor_model) and receive a valid .trst archive
  2. User can run `trst wrap --profile audio` with audio-specific fields (sample_rate, bit_depth, channels, codec) and receive a valid .trst archive
  3. User can run `trst wrap --profile log` with log-specific fields (application, host, log_level) and receive a valid .trst archive
  4. All three profile archives pass `trst verify` with exit code 0
  5. `trst verify` on a sensor/audio/log archive produces the same human-readable output format as a generic archive
**Plans:** 2/2 plans complete

Plans:
- [ ] 42-01-PLAN.md -- Add metadata structs, enum variants, validate/canonical, unit tests
- [ ] 42-02-PLAN.md -- Add CLI flags for sensor/audio/log, acceptance tests

### Phase 43: Archive Decryption (trst unwrap)
**Goal**: Users can recover original data from a .trst archive, completing the wrap/unwrap data lifecycle
**Depends on**: Phase 42 (profile validation must not reject new profile names before unwrap is tested)
**Requirements**: UNWRAP-01, UNWRAP-02, UNWRAP-03, UNWRAP-04
**Success Criteria** (what must be TRUE):
  1. User can run `trst unwrap <archive.trst> --device-key <key> --out <file>` and recover the exact original data
  2. `trst wrap` derives the encryption key from the device signing key via HKDF (no hardcoded demo key in the codebase)
  3. `trst unwrap` verifies the archive signature and continuity chain before producing any plaintext output
  4. A wrap-then-unwrap round-trip on arbitrary binary data produces byte-identical output
  5. `trst unwrap` on a tampered or incorrectly-keyed archive exits with a non-zero exit code and no plaintext output
**Plans**: TBD

### Phase 44: YubiKey CLI Integration
**Goal**: Users can sign archives with a hardware YubiKey from the CLI, and verify those hardware-signed archives
**Depends on**: Phase 43 (signature dispatch in trst verify must not be in-flight during unwrap development)
**Requirements**: YUBI-01, YUBI-02, YUBI-03, YUBI-04
**Success Criteria** (what must be TRUE):
  1. User can run `trst wrap --backend yubikey` and produce an archive signed with ECDSA P-256 from the connected YubiKey
  2. `trst verify` accepts and validates both Ed25519 (`"ed25519:..."`) and ECDSA P-256 (`"ecdsa-p256:..."`) archive signatures
  3. When the YubiKey requires a PIN, the CLI prompts interactively without echoing the PIN to the terminal
  4. `scripts/demo.sh --local` runs without error on a machine with no YubiKey (YubiKey steps are gracefully skipped)
**Plans**: TBD

---

## Progress

**Execution Order:** 42 → 43 → 44

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 42. Named Archive Profiles | 2/2 | Complete    | 2026-03-17 | - |
| 43. Archive Decryption (trst unwrap) | v2.1 | 0/? | Not started | - |
| 44. YubiKey CLI Integration | v2.1 | 0/? | Not started | - |

---
*Last updated: 2026-03-16 after phase 42 planning*
