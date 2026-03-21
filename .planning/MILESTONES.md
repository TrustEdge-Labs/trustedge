## v2.0 End-to-End Demo (Shipped: 2026-03-16)

## v2.3 Security Testing (Shipped: 2026-03-21)

**Phases completed:** 4 phases (48-51), 4 plans, 6 tasks
**Timeline:** 2 days (2026-03-20 → 2026-03-21)
**Stats:** 8 files changed, 1,326 insertions, 2 deletions, 6 commits

**Delivered:** Implemented targeted security tests proving TrustEdge's tamper-evidence, nonce uniqueness, key protection, and replay resistance claims with 31 new security tests across 4 threat model categories.

**Key accomplishments:**
- 8 archive integrity tests: byte mutation, chunk injection, chunk reordering, and post-signing manifest modification all detected by `trst verify` with specific exit codes and error messages
- 6 nonce/key derivation tests: HashSet-based nonce uniqueness proof within archives, cross-archive nonce divergence, and HKDF key-binding via direct `derive_chunk_key()` calls
- 14 key file protection tests: TRUSTEDGE-KEY-V1 truncation (5 boundaries), JSON corruption (6 variants), and wrong-passphrase rejection — never panics or produces partial keys
- 3 receipt binding tests: duplicate submissions yield distinct verification_ids, manifest_digest is BLAKE3-bound to content, deterministic binding verified

**Known gaps:**
- Phase 51 missing VERIFICATION.md (process gap — tests pass, integration verified)

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)

**Git range:** v2.2..v2.3

**Archives:**
- `.planning/milestones/v2.3-ROADMAP.md`
- `.planning/milestones/v2.3-REQUIREMENTS.md`
- `.planning/milestones/v2.3-MILESTONE-AUDIT.md`

---

## v2.2 Security Remediation (Shipped: 2026-03-19)

**Phases completed:** 3 phases (45-47), 5 plans
**Timeline:** 2 days (2026-03-17 → 2026-03-18)
**Stats:** 33 files changed, 2,498 insertions, 401 deletions, 23 commits

**Delivered:** Fixed critical cryptographic flaws — replaced insecure RSA padding, removed legacy envelope format, enforced key derivation minimums, and added passphrase-encrypted device keys.

**Key accomplishments:**
- RSA PKCS#1 v1.5 replaced with OAEP-SHA256 in asymmetric.rs (RUSTSEC-2023-0071 resolved after being carried since v1.3)
- v1 envelope format removed entirely (not just deprecated) — unseal() simplified from 66 to 30 lines
- PBKDF2 minimum 300k iterations enforced at 4 points (builder + backend, both APIs)
- Device keys encrypted at rest: TRUSTEDGE-KEY-V1 format with PBKDF2-SHA256 (600k) + AES-256-GCM
- `--unencrypted` escape hatch for CI/automation, secure-by-default for interactive use

**Tech debt resolved:**
- RUSTSEC-2023-0071 risk acceptance removed (carried from v1.3 — 7 milestones)
- v1 envelope dead code eliminated

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)

**Git range:** v2.1..v2.2

**Archives:**
- `.planning/milestones/v2.2-ROADMAP.md`
- `.planning/milestones/v2.2-REQUIREMENTS.md`

---

## v2.1 Data Lifecycle & Hardware Integration (Shipped: 2026-03-18)

**Phases completed:** 3 phases (42-44), 6 plans
**Timeline:** 2 days (2026-03-16 → 2026-03-17)
**Stats:** 28 files changed, 4,397 insertions, 117 deletions, 27 commits

**Delivered:** Completed the data lifecycle with decryption capability, exposed YubiKey hardware signing in the CLI, and added named archive profiles for real-world use cases.

**Key accomplishments:**
- Named profiles (sensor, audio, log) with typed metadata structs and profile-conditional CLI flags (SensorMetadata with geo fields, AudioMetadata, LogMetadata)
- `trst unwrap` command: HKDF-SHA256 key derivation from device signing key replaces hardcoded demo key, nonce-prepended chunk format, mandatory verify-before-decrypt
- Multi-algorithm verify dispatch: `trst verify` accepts both `ed25519:` and `ecdsa-p256:` signatures with prefix-based dispatch
- `trst wrap --backend yubikey` wires ECDSA P-256 hardware signing via PIV slot 9c with interactive `rpassword` PIN prompt
- Demo script auto-detects YubiKey and adds optional hardware signing step
- 28 acceptance tests (10 new), 30+ unit tests in trst-protocols

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)
- Demo-key archives (from before v2.1) are not decryptable by `trst unwrap` — accepted, test artifacts only

**Git range:** v2.0..v2.1

**Archives:**
- `.planning/milestones/v2.1-ROADMAP.md`
- `.planning/milestones/v2.1-REQUIREMENTS.md`

---

**Phases completed:** 4 phases (38-41), 8 plans
**Timeline:** 2 days (2026-03-15 → 2026-03-16)
**Stats:** 50 files changed, 4,867 insertions, 574 deletions, 42 commits

**Delivered:** Working end-to-end demonstration of TrustEdge's full value proposition — data-agnostic archive wrapping, one-command Docker deployment, automated demo script, and developer-focused documentation.

**Key accomplishments:**
- TrstManifest with ProfileMetadata enum (generic + cam.video) — generic profile as default, backward-compatible
- Three-service Docker Compose stack (platform + postgres + dashboard) with auto-migration and health checks
- `trst keygen` subcommand for Ed25519 device key pair generation
- `scripts/demo.sh` showing full lifecycle (keygen, wrap, verify, receipt) with docker/local auto-detect
- README rewritten from 465 to 128 lines — problem statement, 3-command quick start, 4 use cases with copy-paste commands
- Architecture and YubiKey content reorganized into docs/

**Issues fixed during deployment:**
- `.dockerignore` added (132GB target/ excluded from Docker context)
- Dockerfile Rust version bumped to 1.88 (base64ct edition2024 + time crate MSRV)
- sqlx re-added to platform postgres feature (was removed but code still used it)
- /healthz excluded from auth middleware in postgres builds
- Non-root container user given home directory and WORKDIR

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)

**Git range:** v1.8..v2.0

**Archives:**
- `.planning/milestones/v2.0-ROADMAP.md`
- `.planning/milestones/v2.0-REQUIREMENTS.md`

---

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


## v1.8 KDF Architecture Fix (Shipped: 2026-02-24)

**Phases completed:** 3 phases (35-37), 4 plans, 8 tasks
**Timeline:** 2 days (2026-02-22 → 2026-02-24)
**Stats:** 23 files changed, 1,903 insertions, 153 deletions

**Delivered:** Fixed incorrect KDF usage across the codebase — replaced PBKDF2-per-chunk with HKDF hierarchical key derivation in envelope.rs, added versioned envelope format with backward-compatible decryption, and hardened keyring PBKDF2 parameters to OWASP 2023 levels.

**Key accomplishments:**
- Replaced ad-hoc PBKDF2 CatKDF with HKDF-SHA256 (RFC 5869) Extract+Expand using "TRUSTEDGE_ENVELOPE_V1" domain separation
- v2 envelope seal path: single HKDF derivation produces 40-byte OKM (32-byte AES-256-GCM key + 8-byte nonce prefix) with deterministic counter nonces
- Backward-compatible unseal() with try-v2-first then v1-fallback — existing encrypted data decrypts without modification
- Hardened both keyring backends to OWASP 2023 PBKDF2 parameters: 600,000 iterations and 32-byte salts

**Tech debt carried forward:**
- Hardware tests require physical YubiKey 5 series (carried from v1.1)
- RSA Marvin Attack advisory (RUSTSEC-2023-0071) accepted (carried from v1.3)

**Git range:** v1.7..1c7ac82

**Archives:**
- `.planning/milestones/v1.8-ROADMAP.md`
- `.planning/milestones/v1.8-REQUIREMENTS.md`

---

