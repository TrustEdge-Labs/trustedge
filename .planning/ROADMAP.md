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
- 🚧 **v1.7 Security & Quality Hardening** - Phases 31-34 (in progress)

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

### v1.7 Security & Quality Hardening (In Progress)

**Milestone Goal:** Address reviewer-identified security gaps, remove deprecated facade crates, harden platform quality, and add integration test coverage for the platform server.

- [x] **Phase 31: Secret Hardening** - Zeroize sensitive fields and eliminate secret leakage via Debug and serialization (completed 2026-02-22)
- [x] **Phase 32: Workspace Cleanup** - Delete deprecated facade crates and separate Tier 2 crates from shared dependency graph (completed 2026-02-22)
- [x] **Phase 33: Platform Quality** - Deduplicate verify handler logic, harden CORS, and resolve CA route ambiguity (completed 2026-02-22)
- [x] **Phase 34: Platform Testing** - Add integration tests for platform-server wiring and full HTTP verify round-trip (completed 2026-02-23)

## Phase Details

### Phase 31: Secret Hardening
**Goal**: Sensitive values (PIN, passphrase, JWT secret, passwords) cannot leak through debug output, serialization, or memory reuse
**Depends on**: Nothing (standalone security hardening, no cross-phase dependency)
**Requirements**: SEC-01, SEC-02, SEC-03, SEC-04
**Success Criteria** (what must be TRUE):
  1. `cargo test` passes with zeroize applied to all sensitive struct fields — no test regression
  2. `{:?}` formatting on any config or auth struct containing secrets outputs redacted placeholders, never plaintext values
  3. `serde::Serialize` and `serde::Deserialize` are absent from `YubiKeyConfig`, `SoftwareHsmConfig`, and `LoginRequest` — the compiler rejects any attempt to serialize them
  4. `LoginRequest.password` cannot be printed or serialized by accident — verified by inspecting derived trait list and Debug output in tests
**Plans:** 3/3 plans complete

Plans:
- [x] 31-01-PLAN.md — Create Secret<T> wrapper type with zeroize, redacted Debug, expose_secret()
- [x] 31-02-PLAN.md — Harden YubiKeyConfig and SoftwareHsmConfig (remove serde, builder pattern, Secret fields)
- [x] 31-03-PLAN.md — Harden LoginRequest, CAConfig, AuthService + CI regression check

### Phase 32: Workspace Cleanup
**Goal**: Deprecated facade crates are gone from the workspace, and Tier 2 experimental crates are isolated so their dependency graph does not contaminate the shared Cargo.lock
**Depends on**: Phase 31
**Requirements**: WRK-01, WRK-02, WRK-03, WRK-04
**Success Criteria** (what must be TRUE):
  1. `cargo build --workspace` succeeds with `trustedge-receipts` and `trustedge-attestation` absent from the crates list — they do not exist on disk
  2. CI scripts and documentation contain no references to the deleted facade crates
  3. Tier 2 pubky crates live in a separate workspace or are excluded via `[workspace]` membership, so their transitive deps are absent from the root `Cargo.lock`
  4. `cargo machete` on the root workspace reports no unused workspace-level dependencies introduced by the removed crates
**Plans:** 3/3 plans complete

Plans:
- [x] 32-01-PLAN.md — Delete deprecated facade crates from workspace
- [ ] 32-02-PLAN.md — Isolate pubky crates into experimental workspace and clean root deps
- [ ] 32-03-PLAN.md — Rewrite CI scripts and documentation to reflect new workspace structure

### Phase 33: Platform Quality
**Goal**: Platform verify logic is deduplicated into a single always-compiled path, the non-postgres build uses restrictive CORS, and the CA module's exposure is explicitly documented or wired
**Depends on**: Phase 32
**Requirements**: PLT-01, PLT-02, PLT-03
**Success Criteria** (what must be TRUE):
  1. The shared verify validation function compiles and is called by all feature variants of `verify_handler` — no duplicated validation branches exist
  2. Building `trustedge-platform` without the `postgres` feature produces a server that returns `403` or `405` on cross-origin requests rather than accepting all origins
  3. CA module routes are either reachable via `create_router()` or a code comment explicitly marks the module as library-only with no HTTP exposure
**Plans:** 2/2 plans complete

Plans:
- [ ] 33-01-PLAN.md -- Deduplicate verify handler validation and receipt construction
- [ ] 33-02-PLAN.md -- Harden CORS policy and document CA module as library-only

### Phase 34: Platform Testing
**Goal**: The platform-server binary has integration tests that verify startup wiring, and a full HTTP verify round-trip test confirms the pipeline works end-to-end
**Depends on**: Phase 33
**Requirements**: TST-01, TST-02, TST-03
**Success Criteria** (what must be TRUE):
  1. `cargo test -p trustedge-platform-server` runs integration tests that construct `AppState`, confirm required environment variables are wired, and assert the router starts without panicking
  2. `create_test_app()` applies the same CORS policy, tracing middleware, and auth middleware as `create_router()` — a test that passes through `create_test_app` exercises identical middleware to production
  3. A test submits a correctly signed payload to the verify endpoint over HTTP and receives a receipt response with HTTP 200 — the full sign-then-verify pipeline is exercised in a single test
**Plans:** 2/2 plans complete

Plans:
- [ ] 34-01-PLAN.md -- Platform-server wiring tests (Config, AppState, router health)
- [ ] 34-02-PLAN.md -- Router builder fidelity, CORS parity test, full verify round-trip

## Progress

| Phase | Milestone | Plans Complete | Status | Completed |
|-------|-----------|----------------|--------|-----------|
| 31. Secret Hardening | 3/3 | Complete    | 2026-02-22 | - |
| 32. Workspace Cleanup | 3/3 | Complete    | 2026-02-22 | - |
| 33. Platform Quality | 2/2 | Complete    | 2026-02-22 | - |
| 34. Platform Testing | 2/2 | Complete    | 2026-02-23 | - |

---
*Last updated: 2026-02-22 after executing 32-01*
