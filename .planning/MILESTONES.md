<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Milestones

## v1.0 Consolidation (Shipped: 2026-02-11)

**Phases completed:** 8 phases, 17 plans, 31 tasks
**Timeline:** 177 days (2025-08-17 → 2026-02-10)
**Stats:** 310 files modified, 37,589 Rust LOC, 343 tests, 216 commits

**Delivered:** Consolidated TrustEdge from 10 scattered crates into a monolithic core with thin CLI/WASM shells — zero API breaking changes, 98.6% test retention, WASM compatibility preserved.

**Key accomplishments:**
- Established 6-layer architecture in trustedge-core with CI tooling and 348-test baseline
- Unified 10+ duplicate error types into hierarchical TrustEdgeError enum with 7 subsystem variants
- Eliminated 454 lines of duplicate manifest code by wiring core to trst-protocols
- Migrated receipts (1,281 LOC, 23 tests) and attestation (826 LOC, 10 tests) into core
- Consolidated feature flags with CI matrix testing, WASM verification, and docs.rs metadata
- Deprecated facade crates with 6-month migration window and 228-line migration guide
- Validated zero breaking changes (196 semver checks), removed 21 unused dependencies

**Tech debt carried forward:**
- TODO comments in envelope_v2_bridge.rs for Pubky integration (future work)
- ~~Placeholder ECDSA key in yubikey.rs~~ (resolved in v1.1 — full rewrite)
- YubiKey manual testing requires physical hardware (protocol documented, 580 lines)
- 2 cargo-machete false positives (serde_bytes, getrandom)

**Git range:** Initial commit → efe05a2 (docs(phase-8): complete phase execution)

**Archives:**
- `.planning/milestones/v1.0-ROADMAP.md`
- `.planning/milestones/v1.0-REQUIREMENTS.md`
- `.planning/milestones/v1.0-MILESTONE-AUDIT.md`

---


## v1.1 YubiKey Integration Overhaul (Shipped: 2026-02-11)

**Phases completed:** 4 phases (9-12), 6 plans, 8 tasks
**Timeline:** 1 day (2026-02-11)
**Stats:** 158 files changed, 10,664 insertions, 11,347 deletions, 30,144 Rust LOC, 45 commits

**Delivered:** Deleted the broken YubiKey backend (8,117 lines) and rewrote from scratch with fail-closed design, battle-tested libraries only (yubikey crate stable API, rcgen for X.509), comprehensive test suite, and unconditional CI validation.

**Key accomplishments:**
- Scorched-earth deletion of broken YubiKey backend (3,263 lines), 8 test files, 8 examples, all placeholder keys and manual DER encoding
- Production-quality YubiKey PIV backend (487 lines) with ECDSA P-256/RSA-2048 signing, public key extraction, slot enumeration, PIN verification, fail-closed design
- X.509 certificate generation via rcgen RemoteKeyPair with hardware-backed signing — zero manual ASN.1/DER encoding
- 18 simulation tests (no hardware, run in CI) + 9 hardware integration tests (#[ignore], require physical YubiKey)
- CI unconditionally compiles and tests YubiKey feature on every PR — broken code can never merge silently

**Tech debt carried forward:**
- Key generation and attestation deferred (yubikey 0.7 has PinPolicy/TouchPolicy in private module)
- Certificate generation uses ECDSA P-256 only (RSA cert generation deferred)
- TODO comments in envelope_v2_bridge.rs for Pubky integration (carried from v1.0)

**Git range:** v1.0..ef596cf (docs(phase-12): complete phase execution)

**Archives:**
- `.planning/milestones/v1.1-ROADMAP.md`
- `.planning/milestones/v1.1-REQUIREMENTS.md`

---


## v1.2 Scope Reduction & Dependency Rationalization (Shipped: 2026-02-12)

**Phases completed:** 2 phases (13-14), 4 plans, 9 tasks
**Timeline:** 1 day (2026-02-12)
**Stats:** 34 files changed, 1,645 insertions, 110 deletions

**Delivered:** Made TrustEdge maintainable by a solo developer — clear stable/experimental crate split, dependency audit and optimization, tiered CI pipeline that prioritizes core crates.

**Key accomplishments:**
- 2-tier crate classification (Stable/Experimental) with metadata in all 10 Cargo.toml files and README banners
- Full dependency audit with justification documentation (DEPENDENCIES.md) for all core crate deps
- Tokio features trimmed from "full" to minimal sets (8 features for core, 2 for trst-cli)
- Tiered CI pipeline — core crates blocking merge, experimental crates non-blocking (continue-on-error)
- Dependency tree size baseline (60 crates) with regression tracking in CI and local script
- Root README crate classification section with tier table for user visibility

**Tech debt carried forward:**
- Key generation and attestation deferred (yubikey 0.7 API limitations, carried from v1.1)
- TODO comments in envelope_v2_bridge.rs for Pubky integration (carried from v1.0)
- 2 cargo-machete false positives (serde_bytes, getrandom) suppressed via config

**Git range:** v1.1..5852b52 (docs(phase-14): complete phase execution)

**Archives:**
- `.planning/milestones/v1.2-ROADMAP.md`
- `.planning/milestones/v1.2-REQUIREMENTS.md`

---


## v1.3 Dependency Audit & Rationalization (Shipped: 2026-02-13)

**Phases completed:** 4 phases (15-18), 5 plans, 7 tasks
**Timeline:** 1 day (2026-02-12 → 2026-02-13)
**Stats:** 37 files changed, 10,360 insertions, 185 deletions, 26 commits

**Delivered:** Hardened the dependency tree across all 10 crates — feature-gated heavy optional deps (git2, keyring), removed unused deps, ran security audit with cargo-audit, and documented every remaining dependency with justification.

**Key accomplishments:**
- Feature-gated git2 and keyring behind opt-in flags — default builds skip heavy optional dependencies
- CI validates feature-gated builds in both ci-check.sh and GitHub Actions
- Removed unused dependencies (pkcs11, sha2, tokio-test) via cargo-machete audit
- Integrated cargo-audit into CI as blocking check with RSA Marvin Attack advisory risk acceptance documented
- Comprehensive DEPENDENCIES.md covering all 10 crates with per-dependency justifications and 15-entry security-critical rationale section

**Tech debt carried forward:**
- Key generation and attestation deferred (yubikey 0.7 API limitations, carried from v1.1)
- TODO comments in envelope_v2_bridge.rs for Pubky integration (carried from v1.0)
- 2 cargo-machete false positives (serde_bytes, getrandom) suppressed via config
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted — TrustEdge does not use RSA for production encryption

**Git range:** v1.2..6c51e44 (docs(phase-18): complete phase execution)

**Archives:**
- `.planning/milestones/v1.3-ROADMAP.md`
- `.planning/milestones/v1.3-REQUIREMENTS.md`

---


## v1.4 Placeholder Elimination (Shipped: 2026-02-13)

**Phases completed:** 5 phases (19-23), 5 plans, 10 tasks
**Timeline:** 1 day (2026-02-13)
**Stats:** 36 files changed, 2,626 insertions, 734 deletions, 15 commits

**Delivered:** Removed all placeholder code, incomplete features, and insecure defaults from the codebase — if it doesn't work, it doesn't exist. Added CI enforcement to prevent regression.

**Key accomplishments:**
- Secured QUIC TLS by default with webpki-roots trust store; insecure bypass requires compile-time `insecure-tls` feature flag
- Removed all dead code: legacy server functions, reserved keyring methods, unused struct fields, unjustified `#[allow(dead_code)]`
- Eliminated core stubs: deleted envelope_v2_bridge.rs, removed Blake2b hash variant, cleaned YubiKey generate_key error
- Cleaned experimental Pubky crates: removed unimplemented CLI commands (publish, migrate), placeholder functions, resolved batch_resolve TODOs
- Enforced TODO hygiene: zero unimplemented markers, renamed "stub" to "feature-disabled" terminology, CI scans on every push/PR

**Tech debt carried forward:**
- Key generation and attestation deferred (yubikey 0.7 API limitations, carried from v1.1)
- 2 cargo-machete false positives (serde_bytes, getrandom) suppressed via config
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)

**Git range:** v1.3..84ab414 (docs(23-01): complete TODO hygiene sweep plan)

**Archives:**
- `.planning/milestones/v1.4-ROADMAP.md`
- `.planning/milestones/v1.4-REQUIREMENTS.md`

---


## v1.5 Platform Consolidation (Shipped: 2026-02-22)

**Phases completed:** 4 phases (24-27), 8 plans, 16 tasks
**Timeline:** 2 days (2026-02-21 → 2026-02-22)
**Stats:** 84 files changed, 10,940 insertions, 393 deletions, 34,526 Rust LOC, 49 commits

**Delivered:** Consolidated external service repos (platform-api, verify-core, shared-libs) into the main trustedge workspace, mandated trustedge-core for all crypto, and archived 5 empty scaffold repos with scope documentation.

**Key accomplishments:**
- Created `trustedge-types` crate centralizing te_shared wire types with Uuid/DateTime from platform-api and JSON schema generation from shared-libs
- Merged trustedge-platform-api and trustedge-verify-core into unified `trustedge-platform` crate with Axum HTTP layer, PostgreSQL backend, CA module, and BLAKE3+Ed25519 verify engine
- Replaced all manual crypto/chaining code with `trustedge_core::chain` and `trustedge_core::crypto` — removed blake3 and ed25519-dalek from platform production deps
- Added `SigningKey`/`VerifyingKey` re-exports to trustedge-core for downstream JWKS key management
- Archived 5 scaffold repos (billing, device, identity, infra, ingestion) on GitHub with redirect READMEs; documented intended scope in CLAUDE.md

**Tech debt carried forward:**
- `trustedge-types` listed as production dep of platform but not imported — unused compile surface
- Platform defines parallel VerifyRequest/VerifyOptions/SegmentDigest types instead of consuming from trustedge-types — schema divergence risk
- 11 platform-api DB-backed integration tests require live PostgreSQL (#[ignore]) — matches YubiKey test pattern
- trustedge-dashboard (29-file SvelteKit codebase) flagged but not archived — deferred to future milestone
- Key generation and attestation deferred (yubikey 0.7 API limitations, carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)

**Git range:** v1.4..HEAD (49 commits)

**Archives:**
- `.planning/milestones/v1.5-ROADMAP.md`
- `.planning/milestones/v1.5-REQUIREMENTS.md`
- `.planning/milestones/v1.5-MILESTONE-AUDIT.md`

---


## v1.6 Final Consolidation (Shipped: 2026-02-22)

**Phases completed:** 3 phases (28-30), 6 plans, 11 tasks
**Timeline:** 1 day (2026-02-22)
**Stats:** 353 files changed, 50,237 insertions, 12,730 deletions, 20 commits

**Delivered:** Brought all satellite code into the monorepo, created a standalone platform server binary, and deleted all orphaned GitHub repos — reducing the TrustEdge-Labs org from 14 repos to 3.

**Key accomplishments:**
- Created `trustedge-platform-server` binary crate with Axum HTTP, clap CLI, graceful shutdown, and startup banner
- Created deployment artifacts: multi-stage Dockerfile (debian-slim runtime), docker-compose.yml, .env.example
- Moved trustedge-dashboard (SvelteKit) into `web/dashboard/` with npm build/check passing
- Generated TypeScript types from `trustedge-types` JSON schemas via json-schema-to-typescript — replacing hand-written type definitions
- Deleted 11 orphaned repos from TrustEdge-Labs GitHub org via `gh repo delete`
- Updated CLAUDE.md and all documentation to reflect final 3-repo org structure (trustedge, trustedgelabs-website, shipsecure)

**Tech debt carried forward:**
- Key generation and attestation deferred (yubikey 0.7 API limitations, carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)
- Platform defines parallel VerifyRequest/VerifyOptions types instead of consuming from trustedge-types (carried from v1.5)
- Type generation is manual (one-time script, not CI-automated) — deferred to future milestone

**Git range:** v1.5..70de3d1 (20 commits)

**Archives:**
- `.planning/milestones/v1.6-ROADMAP.md`
- `.planning/milestones/v1.6-REQUIREMENTS.md`

---


## v1.7 Security & Quality Hardening (Shipped: 2026-02-23)

**Phases completed:** 4 phases (31-34), 10 plans, 18 tasks
**Timeline:** 1 day (2026-02-23)
**Stats:** 90 files changed, 6,082 insertions, 1,855 deletions, 44 commits

**Delivered:** Hardened secret handling, deleted deprecated code, isolated experimental crates, deduplicated verify handler, hardened CORS, and added comprehensive integration tests.

**Key accomplishments:**
- In-house Secret<T> wrapper type with zeroize, redacted Debug, no Display/Deref/Serialize — all sensitive fields wrapped
- Builder pattern for config structs containing secrets (YubiKeyConfig, SoftwareHsmConfig) with CI regression check (Step 23)
- Deleted deprecated facade crates (trustedge-receipts, trustedge-attestation) and isolated experimental pubky crates into standalone workspace at crates/experimental/
- Deduplicated verify handler validation via validate_verify_request_full() and shared receipt construction via build_receipt_if_requested()
- Hardened CORS: CorsLayer::new() for verify-only (same-origin), restricted headers for postgres build
- CA module documented as library-only with Axum coupling removed; build_base_router() shared builder ensures test/production middleware parity
- 16 new integration tests: platform-server wiring (5), HTTP verify round-trip with JWS/JWKS (4), CORS parity, and more

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)

**Git range:** v1.6..v1.7 (44 commits)

**Archives:**
- `.planning/milestones/v1.7-ROADMAP.md`
- `.planning/milestones/v1.7-REQUIREMENTS.md`

---

