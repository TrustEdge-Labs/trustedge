<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# TrustEdge

## What This Is

TrustEdge is a Rust workspace providing hardware-backed cryptographic operations for edge devices and IoT. A consolidated monolithic core (`trustedge-core`) owns all cryptographic operations — envelope encryption, signing, digital receipts, software attestation, and .trst archives — with thin CLI and WASM shells as frontends. The workspace includes 12 crates (including `trustedge-types` for shared wire types and `trustedge-platform` for the consolidated verification/CA service), with `trustedge-core` as the single source of truth for all crypto.

## Core Value

A single, reliable `trustedge-core` library that owns all cryptographic operations (envelope encryption, signing, receipts, attestation, archives) with production-quality YubiKey hardware integration — thin CLIs and WASM bindings are just frontends.

## Requirements

### Validated

- ✓ AES-256-GCM envelope encryption with Ed25519 signing — v1.0
- ✓ Universal Backend system (Software HSM, Keyring, YubiKey) — v1.0
- ✓ Network transport (TCP framed, QUIC with TLS) — v1.0
- ✓ Digital receipt system with ownership chains — v1.0 (migrated to core)
- ✓ Software attestation with provenance tracking — v1.0 (migrated to core)
- ✓ .trst archive format with cam.video profile — v1.0 (trst-protocols)
- ✓ YubiKey PIV integration rewritten — v1.1 (fail-closed, yubikey crate stable API, rcgen for X.509)
- ✓ WASM browser bindings — v1.0
- ✓ Pubky network integration — v1.0 (community contribution)
- ✓ Dependency graph analyzed and cross-crate duplication mapped — v1.0
- ✓ Layered module hierarchy (primitives/backends/protocols/applications/transport/io) — v1.0
- ✓ Test inventory baseline documented (348 tests) — v1.0
- ✓ Unified TrustEdgeError enum with 7 subsystem variants — v1.0
- ✓ 10+ duplicate error types consolidated into hierarchy — v1.0
- ✓ thiserror for libraries, anyhow for CLIs — v1.0
- ✓ trst-core manifest types merged via trst-protocols (WASM-compatible) — v1.0
- ✓ Receipts (1,281 LOC, 23 tests) merged into core — v1.0
- ✓ Attestation (826 LOC, 10 tests) merged into core — v1.0
- ✓ Feature flags consolidated into categories (backend, platform) — v1.0
- ✓ CI matrix tests critical feature combinations — v1.0
- ✓ Deprecated re-export facades with 6-month migration window — v1.0
- ✓ MIGRATION.md with import path changes — v1.0
- ✓ 343 tests preserved (98.6% of baseline) — v1.0
- ✓ WASM build verified — v1.0
- ✓ Zero API breakage (196 semver checks) — v1.0
- ✓ YubiKey backend rewritten from scratch (fail-closed, no software fallbacks) — v1.1
- ✓ X.509 certificate generation via rcgen (no manual DER encoding) — v1.1
- ✓ `yubikey` crate used without `untested` feature flag — v1.1
- ✓ 18 simulation tests + 9 hardware integration tests, all with real assertions — v1.1
- ✓ CI always compile-checks and tests yubikey feature unconditionally — v1.1
- ✓ 2-tier crate classification (stable/experimental) with Cargo.toml metadata — v1.2
- ✓ Full dependency audit with documented justifications (DEPENDENCIES.md) — v1.2
- ✓ Tokio features trimmed from "full" to minimal sets — v1.2
- ✓ Tiered CI pipeline (core blocking, experimental non-blocking) — v1.2
- ✓ Dependency tree size baseline and regression tracking — v1.2
- ✓ Root README documents stable/experimental crate split — v1.2
- ✓ git2 and keyring feature-gated behind opt-in flags (not compiled by default) — v1.3
- ✓ CI tests both default and feature-enabled builds — v1.3
- ✓ Unused dependencies removed (pkcs11, sha2, tokio-test) via cargo-machete — v1.3
- ✓ cargo-audit integrated into CI as blocking check — v1.3
- ✓ RSA Marvin Attack advisory risk-accepted with documented rationale — v1.3
- ✓ Cargo.lock tracked in git for reproducible security audits — v1.3
- ✓ DEPENDENCIES.md covers all 10 crates with per-dependency justifications — v1.3
- ✓ Security-critical dependencies documented with detailed rationale (15 entries) — v1.3
- ✓ QUIC TLS secure-by-default with webpki-roots trust store — v1.4
- ✓ Insecure TLS gated behind compile-time `insecure-tls` feature flag — v1.4
- ✓ Legacy dead code removed (server functions, keyring methods, struct fields) — v1.4
- ✓ All `#[allow(dead_code)]` audited — unjustified code deleted — v1.4
- ✓ envelope_v2_bridge.rs deleted from codebase — v1.4
- ✓ Blake2b hash variant removed (don't advertise unimplemented) — v1.4
- ✓ Unimplemented Pubky CLI commands (publish, migrate) removed — v1.4
- ✓ Zero TODO/FIXME/HACK/XXX markers indicating unimplemented functionality — v1.4
- ✓ CI enforces TODO hygiene on every push/PR — v1.4
- ✓ te_shared wire types centralized in workspace as trustedge-types crate — v1.5
- ✓ platform-api and verify-core merged into unified trustedge-platform crate — v1.5
- ✓ CA module preserved within trustedge-platform — v1.5
- ✓ Combined REST API (verify, JWKS, devices, receipts) serves all endpoints — v1.5
- ✓ All integration tests pass in consolidated platform crate — v1.5
- ✓ Manual crypto/chaining code deleted, replaced with trustedge-core — v1.5
- ✓ Verification uses trustedge_core::chain and trustedge_core::crypto — v1.5
- ✓ 5 empty scaffold repos archived on GitHub with scope documentation — v1.5
- ✓ Platform server binary (`trustedge-platform-server`) with Axum HTTP, clap CLI, graceful shutdown — v1.6
- ✓ Server reads PORT, DATABASE_URL, JWT_AUDIENCE from environment variables — v1.6
- ✓ Server boots router via `trustedge_platform::create_router()` (zero routing in main.rs) — v1.6
- ✓ Dashboard moved into `web/dashboard/` with npm build/check passing — v1.6
- ✓ Dashboard TypeScript types generated from `trustedge-types` JSON schemas — v1.6
- ✓ 11 orphaned repos deleted from TrustEdge-Labs GitHub org — v1.6
- ✓ CLAUDE.md and documentation updated for 3-repo org structure — v1.6
- ✓ Secret<T> wrapper type with zeroize, redacted Debug, no Display/Deref/Serialize — v1.7
- ✓ All sensitive fields (PIN, passphrase, JWT secret, password) wrapped in Secret<T> with ZeroizeOnDrop — v1.7
- ✓ Serde derives removed from YubiKeyConfig, SoftwareHsmConfig, LoginRequest — v1.7
- ✓ Builder pattern for config structs containing secrets — v1.7
- ✓ CI Step 23 enforces no Serialize derive regression on secret-holding structs — v1.7
- ✓ Deprecated facade crates (trustedge-receipts, trustedge-attestation) deleted from workspace — v1.7
- ✓ Experimental pubky crates isolated in crates/experimental/ as standalone workspace — v1.7
- ✓ CI simplified to --workspace (no tiered logic after experimental isolation) — v1.7
- ✓ verify_handler validation deduplicated via validate_verify_request_full() — v1.7
- ✓ Receipt construction shared via build_receipt_if_requested() — v1.7
- ✓ Non-postgres build uses restrictive CORS (CorsLayer::new(), same-origin only) — v1.7
- ✓ Postgres build restricts CORS headers to Content-Type, Authorization, Accept — v1.7
- ✓ CA module documented as library-only, Axum coupling removed — v1.7
- ✓ Shared build_base_router() ensures create_test_app mirrors create_router middleware — v1.7
- ✓ Platform-server wiring integration tests (Config, AppState, router health) — v1.7
- ✓ Full HTTP verify round-trip test with JWS receipt verified against JWKS endpoint — v1.7

### Active

### Deferred

- Pubky adapter merged into core protocols/pubky/ (feature-gated)
- Pubky-advanced hybrid encryption merged into core
- Prelude module for common imports
- Updated documentation with module-level security considerations

### Out of Scope

- TPM support — premature, no hardware to test against, adds complexity before core is stable
- Post-quantum cryptography — research phase only, no production use case yet
- no_std support — requires separate milestone, half-measures are worse
- Algorithm agility changes — hard-coded Ed25519/AES-256-GCM is sufficient

## Completed Milestones
- **v1.0 Consolidation** — Monolith core + thin shells, 343 tests, zero API breaks
- **v1.1 YubiKey Integration Overhaul** — Scorched-earth rewrite with fail-closed design, battle-tested libraries, 27 tests, unconditional CI
- **v1.2 Scope Reduction** — 2-tier crate classification, dependency audit, tiered CI pipeline, dep tree tracking
- **v1.3 Dependency Audit & Rationalization** — Feature-gated heavy deps, removed unused deps, cargo-audit CI, comprehensive DEPENDENCIES.md
- **v1.4 Placeholder Elimination** — Secure-by-default QUIC TLS, dead code removal, stub elimination, TODO hygiene with CI enforcement
- **v1.5 Platform Consolidation** — External repos merged into workspace (types, platform, verify), core owns all crypto, 5 scaffold repos archived
- **v1.6 Final Consolidation** — Platform server binary, dashboard in monorepo with generated types, 11 orphaned repos deleted, 3-repo org
- **v1.7 Security & Quality Hardening** — Secret<T> zeroize wrapper, facade crates deleted, experimental workspace isolated, verify handler deduplicated, CORS hardened, 16 new integration tests

## Context

Shipped v1.7 with 9 crates in root workspace + 2 experimental crates in crates/experimental/ + SvelteKit dashboard at web/dashboard/.
Tech stack: Rust, AES-256-GCM, Ed25519, BLAKE3, XChaCha20-Poly1305, WASM, YubiKey PIV (ECDSA P-256, RSA-2048), Axum, PostgreSQL (sqlx), SvelteKit (TypeScript).
TrustEdge-Labs GitHub org has exactly 3 repos: trustedge (main workspace), trustedgelabs-website, shipsecure.
trustedge-core owns all crypto — platform calls core::chain and core::crypto; re-exports SigningKey/VerifyingKey for downstream use.
Platform server binary (`crates/platform-server/`) boots Axum via `trustedge_platform::create_router()` with deploy/ artifacts (Dockerfile, docker-compose.yml).
Dashboard TypeScript types generated from trustedge-types JSON schemas — no hand-written type definitions for shared types.
CI uses `--workspace` for root workspace. Experimental crates (pubky) isolated in standalone workspace at crates/experimental/. YubiKey feature validated unconditionally. cargo-audit + TODO hygiene enforced on every push/PR. Secret struct regression check (CI Step 23) ensures no serde derives on secret-holding structs.
Heavy optional deps (git2, keyring) feature-gated. Platform features: http, postgres, ca, openapi, yubikey. Dependency tree baseline: 70.
All sensitive config fields wrapped in Secret<T> with ZeroizeOnDrop. CORS: same-origin for verify-only, restricted headers for postgres. CA module is library-only (no HTTP exposure).
Key generation and attestation deferred to future (yubikey crate API limitations).

## Constraints

- **Preservation**: All prior work must be preserved — no functionality loss
- **Consistency**: No feature or parameter drift between consolidated modules
- **Tests**: All existing tests must continue to pass
- **Backward compatibility**: Public API surface of `trustedge-core` must support all current consumers
- **Architecture**: Monolith core + thin shells — CLIs and WASM are frontends only

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Monolith core + thin shells | Eliminates duplication, single source of truth for crypto ops | ✓ Good — ~2,500 LOC duplication eliminated |
| Best implementation wins for merges | Pragmatic — don't union-merge everything, pick the better code | ✓ Good — clean migrations |
| Envelope encryption is the core product | YubiKey hardware signing is the differentiator | ✓ Good — stable foundation |
| No new features during consolidation | Adding features while reorganizing risks bugs | ✓ Good — zero breakage |
| trst-core renamed to trst-protocols | Better reflects purpose as protocol definitions | ✓ Good — clear naming |
| Scoped error types per submodule | ManifestFormatError, ChunkFormatError etc. for granularity | ✓ Good — precise error handling |
| Module-level #![deprecated] for facades | Rust limitation: per-item re-export deprecation doesn't propagate | ✓ Good — visible warnings |
| 6-month deprecation timeline (v0.3.0 → v0.4.0) | Follows RFC 1105, gives consumers time to migrate | — Pending (Aug 2026) |
| Feature categories: Backend + Platform | Semantic organization prevents combinatorial explosion | ✓ Good — clean CI matrix |
| cargo-semver-checks with HEAD~1 baseline | Track API changes commit-to-commit | ✓ Good — 196 checks, 0 breaks |
| Scorched-earth YubiKey rewrite | External review found critical issues: manual DER, silent fallbacks, placeholder keys | ✓ Good — 8,117 lines deleted, clean 487-line rewrite |
| yubikey crate stable API only | Drop `untested` feature — use only tested/stable functionality | ✓ Good — stable API sufficient for all PIV operations |
| rcgen for X.509 certs | Replace 1,000+ lines manual DER with battle-tested library | ✓ Good — RemoteKeyPair + hardware-backed signing |
| Fail-closed hardware design | Hardware unavailable = error, never silent software fallback | ✓ Good — ensure_connected() gates every operation |
| No placeholder keys or signatures | Every key and signature must come from real cryptographic operations | ✓ Good — 27 tests, all with real assertions |
| Arc<Mutex> for RemoteKeyPair | rcgen's KeyPair::from_remote takes ownership, needs shared YubiKey access | ✓ Good — clean shared ownership |
| ECDSA P-256 only for certs | Simplicity for initial release, RSA cert generation deferred | ✓ Good — sufficient for v1.1 |
| Unconditional CI for YubiKey | Remove conditional if-checks, fail loudly if deps missing | ✓ Good — prevents silent breakage |
| Scope reduction, not deletion | Mark experimental crates, don't destroy — rebuild later would be wasted effort | ✓ Good — all code preserved |
| [package.metadata.trustedge] for tier classification | Machine-readable tier in Cargo.toml metadata | ✓ Good — tooling-friendly |
| Trim tokio to minimal features | "full" pulled unnecessary features; minimal sets sufficient | ✓ Good — 8 features core, 2 trst-cli |
| Keep trustedge-cli crypto deps | Direct instantiation, not redundancy with core | ✓ Good — correct architecture |
| Tiered CI (core blocking, experimental non-blocking) | Experimental issues shouldn't block core development | ✓ Good — continue-on-error for tier 2 |
| Dep tree baseline at 60 + warn at 70 | Informational tracking, non-blocking | ✓ Good — catches regression early |
| Feature-gate git2 behind git-attestation flag | Heavy dep not needed by default | ✓ Good — default build skips git2 |
| Feature-gate keyring behind keyring flag | Platform-specific dep not needed by default | ✓ Good — default build skips keyring |
| dep:keyring syntax for feature disambiguation | Cargo feature naming conflict with dependency | ✓ Good — clean feature/dep separation |
| Integration tests gated behind keyring feature | Tests depend on KeyringBackend unavailable without feature | ✓ Good — tests pass with/without feature |
| Remove pkcs11 from trustedge-core | Genuinely unused — no imports found | ✓ Good — cleaner dep tree |
| Accept RSA Marvin Attack advisory (RUSTSEC-2023-0071) | TrustEdge doesn't use RSA for production encryption | ✓ Good — documented in .cargo/audit.toml |
| Track Cargo.lock in git | Reproducible security audits require pinned dep versions | ✓ Good — cargo-audit runs on exact versions |
| DEPENDENCIES.md covers all 10 crates | v1.2 only documented 5 stable crates | ✓ Good — complete audit trail |
| webpki-roots for QUIC TLS trust store | Consistent cross-platform behavior vs OS-native certs | ✓ Good — no platform-specific cert store issues |
| insecure-tls as compile-time feature flag | Compile-time enforcement stronger than runtime config | ✓ Good — insecure code excluded from release builds |
| Delete dead code, don't annotate | #[allow(dead_code)] hides problems; removal is permanent fix | ✓ Good — cleaner codebase, no hidden dead code |
| Remove incomplete features rather than TODOs | If it doesn't work, it shouldn't exist in the codebase | ✓ Good — eliminates misleading API surface |
| Fail-closed error messages with guidance | YubiKey generate_key directs users to external tools | ✓ Good — actionable rather than confusing |
| "Feature-disabled" not "stub" terminology | Cfg-gated code returns errors, not placeholders — name it accurately | ✓ Good — clearer documentation |
| CI TODO hygiene enforcement | Scan for TODO/FIXME/HACK/XXX on every push/PR | ✓ Good — prevents regression to incomplete code |
| trustedge-types as standalone crate | Wire types shared across platform and CLI without pulling in core | ✓ Good — clean dependency separation |
| schemars 0.8 (not 1.x) for types | Exact JSON schema fixture compatibility | ✓ Good — fixture tests pass without rewrite |
| trustedge-platform unified crate | Single service crate instead of separate platform-api + verify-core | ✓ Good — simplified deployment and testing |
| CA module as private mod (not pub) | Internal only, exposed through HTTP layer | ✓ Good — encapsulated implementation detail |
| trustedge-core always-on for platform | Core crypto needed by all platform features, not just CA | ✓ Good — eliminated optional dep complexity |
| Core re-exports SigningKey/VerifyingKey | Downstream crates use core's ed25519 types without direct dep | ✓ Good — single source of truth for key types |
| format_b3() with STANDARD base64 | Wire format consistency for BLAKE3 digests | ✓ Good — matches existing platform-api format |
| Archive 5 repos (not 6 as planned) | trustedge-audit was never created; document gap rather than fail | ✓ Good — accurate over bureaucratic |
| trustedge-dashboard not archived | 29-file SvelteKit codebase has meaningful code | ✓ Good — deferred to future milestone |
| Platform server as thin binary | main.rs is pure wiring — zero routing logic | ✓ Good — all routes in trustedge-platform |
| TypeScript types generated from JSON schemas | json-schema-to-typescript as devDependency | ✓ Good — no hand-written type drift |
| Delete repos (not archive) | All code consolidated, no external audience | ✓ Good — clean 3-repo org |
| debian-slim over alpine for Dockerfile | glibc compatibility with sqlx native-tls | ✓ Good — stable runtime base |
| In-house Secret<T> over secrecy crate | zeroize already in workspace, API surface is small | ✓ Good — zero new deps, compile-time safety |
| Builder pattern for secret-holding configs | Prevents accidental bypass of Secret<T> wrapping | ✓ Good — clean construction API |
| LoginRequest custom Deserialize | Password wrapped in Secret at JSON parsing boundary via private raw struct | ✓ Good — no exposure window |
| CI Step 23 for secret regression | grep-based check catches Serialize derives on secret structs | ✓ Good — automated safety net |
| Delete facade crates immediately | Not published to crates.io, git history preserves them | ✓ Good — clean workspace |
| Standalone experimental workspace | crates/experimental/ with own Cargo.lock, no coupling to root | ✓ Good — clean dependency isolation |
| validate_verify_request_full as public API | Single always-compiled function, first-error-wins ordering | ✓ Good — no duplicated validation |
| build_receipt_if_requested with manifest_digest_fn | Avoids feature-flag coupling in shared code | ✓ Good — clean separation |
| CorsLayer::new() for verify-only | Same-origin only, denies all cross-origin by default | ✓ Good — secure default |
| CA module as library-only | Axum coupling removed, plain service functions, annotated sub-modules | ✓ Good — clear API boundary |
| build_base_router shared builder | Both create_router and create_test_app call same route builder | ✓ Good — middleware parity guaranteed |
| OnceLock<Mutex> for env-var tests | Serializes tests manipulating PORT env var | ✓ Good — no parallel-thread races |

---
*Last updated: 2026-02-23 after v1.7 milestone completion*
