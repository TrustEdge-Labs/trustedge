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

### v2.0 End-to-End Demo (In Progress)

**Milestone Goal:** Deliver a working end-to-end demonstration that shows TrustEdge's full value proposition -- device captures data, signs/encrypts it, wraps into a tamper-evident archive, submits to a verification service, and receives a cryptographic receipt proving provenance.

- [x] **Phase 38: Archive Profiles** - Data-agnostic archive wrapping with generic profile as default (completed 2026-03-15)
- [ ] **Phase 39: Deployment Stack** - One-command docker-compose stack with platform, postgres, and dashboard
- [ ] **Phase 40: Demo Script** - End-to-end lifecycle script showing keygen, wrap, verify, and receipt
- [ ] **Phase 41: Documentation** - README rewrite focused on use cases, quick start, and copy-paste commands

## Phase Details

### Phase 38: Archive Profiles
**Goal**: Users can wrap any data type into a tamper-evident .trst archive without being limited to cam.video
**Depends on**: Nothing (builds on existing .trst archive system in crates/trst-protocols and crates/trst-cli)
**Requirements**: PROF-01, PROF-02, PROF-03, PROF-04
**Success Criteria** (what must be TRUE):
  1. User can run `trst wrap --profile generic --in data.bin --out archive.trst` and get a valid archive
  2. Generic profile manifest includes optional metadata fields (device type, data source, capture context) that the user can populate
  3. Running `trst wrap --in data.bin --out archive.trst` without specifying `--profile` uses the generic profile by default
  4. Running `trst wrap --profile cam.video --in sample.bin --out archive.trst` still works exactly as before
**Plans**: 2 plans

Plans:
- [ ] 38-01-PLAN.md -- Define TrstManifest type with ProfileMetadata enum and update core re-exports
- [ ] 38-02-PLAN.md -- Update CLI (generic default), WASM, acceptance tests, and examples

### Phase 39: Deployment Stack
**Goal**: Users can start the entire TrustEdge platform with a single docker-compose command and have all services running and connected
**Depends on**: Phase 38 (generic profile needed for demo data wrapping)
**Requirements**: DEPL-01, DEPL-02, DEPL-03, DEPL-04
**Success Criteria** (what must be TRUE):
  1. User runs `docker-compose up` from the deploy/ directory and platform server, postgres, and dashboard all start without errors
  2. Postgres schema tables exist after first startup without any manual SQL commands
  3. Dashboard loads in a browser and shows the platform UI without editing any .env files
  4. Hitting `/healthz` on the platform server returns an OK response, and the dashboard renders its home page
**Plans**: 2 plans

Plans:
- [ ] 39-01-PLAN.md -- Containerize dashboard with static adapter, Dockerfile.dashboard, and nginx
- [ ] 39-02-PLAN.md -- Update docker-compose.yml with three-service stack, auto-migration, and health checks

### Phase 40: Demo Script
**Goal**: Users can see the complete TrustEdge lifecycle in action by running a single script
**Depends on**: Phase 38 (generic profile for wrapping), Phase 39 (running stack for verification)
**Requirements**: DEMO-01, DEMO-02, DEMO-03, DEMO-04
**Success Criteria** (what must be TRUE):
  1. User runs `./scripts/demo.sh` and sees the full lifecycle: key generation, data wrapping into .trst archive, submission to verification service, and receipt returned
  2. Demo script works when run against the docker-compose stack and also when run against locally built cargo binaries
  3. Each step of the demo prints clear output showing what is happening and ends with a visible PASS or FAIL verification result
  4. Demo script runs without any manual file preparation -- it generates or includes its own sample data
**Plans**: TBD

Plans:
- [ ] 40-01: TBD
- [ ] 40-02: TBD

### Phase 41: Documentation
**Goal**: A new user understands what TrustEdge does and can run the demo within 5 minutes of reading the README
**Depends on**: Phase 39 (docker-compose for quick start), Phase 40 (demo script to reference)
**Requirements**: DOCS-01, DOCS-02, DOCS-03, DOCS-04, DOCS-05
**Success Criteria** (what must be TRUE):
  1. Root README.md opens with a clear problem statement and 3-4 concrete use cases (drone inspection, sensor logs, body cam, audio capture)
  2. README includes a 3-command quick start: clone the repo, run docker-compose up, run the demo script
  3. Architecture details, crate descriptions, and internal module documentation are in docs/ or collapsed sections -- not cluttering the main README flow
  4. README is a single self-contained file (no redirects to scattered docs for essential information)
  5. Each use case example (drone, sensor, body cam, audio) includes copy-paste commands that a user can run
**Plans**: TBD

Plans:
- [ ] 41-01: TBD
- [ ] 41-02: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 38 -> 39 -> 40 -> 41

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 38. Archive Profiles | 2/2 | Complete    | 2026-03-15 | - |
| 39. Deployment Stack | 1/2 | In Progress|  | - |
| 40. Demo Script | v2.0 | 0/TBD | Not started | - |
| 41. Documentation | v2.0 | 0/TBD | Not started | - |

---
*Last updated: 2026-03-15 after phase 39 planning*
