<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# TrustEdge

## What This Is

TrustEdge is a Rust workspace providing hardware-backed cryptographic operations for edge devices and IoT. A consolidated monolithic core (`trustedge-core`) owns all cryptographic operations — envelope encryption, signing, digital receipts, software attestation, and .trst archives — with thin CLI and WASM shells as frontends. The workspace includes 10 crates, with `trustedge-core` as the single source of truth.

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

### Active

## Current Milestone: v1.4 Placeholder Elimination

**Goal:** Remove all placeholder code, incomplete features, and insecure defaults — if it doesn't work, it doesn't exist in the codebase.

**Target features:**
- QUIC TLS security: proper cert verification by default, insecure skip feature-gated
- Dead code removal: legacy server functions, reserved keyring functions, dead fields
- Stub elimination: delete envelope_v2_bridge.rs, remove Blake2b stub, clean Pubky placeholders
- TODO hygiene: zero remaining TODOs indicating unimplemented functionality

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

## Context

Shipped v1.3 with 30,144 Rust LOC across 10 crates (5 stable, 5 experimental).
Tech stack: Rust, AES-256-GCM, Ed25519, BLAKE3, XChaCha20-Poly1305, WASM, YubiKey PIV (ECDSA P-256, RSA-2048).
370+ tests passing (160+ in core including 18 YubiKey simulation, 9 hardware integration with #[ignore]).
Build time: 45s clean release. Dependency tree: 60 unique crates (baselined, warn at 70).
CI tiered: core crates blocking, experimental crates non-blocking. YubiKey feature validated unconditionally. cargo-audit runs as blocking check.
Crate classification: Tier 1 (stable) = core, cli, trst-protocols, trst-cli, trst-wasm. Tier 2 (experimental) = wasm, pubky, pubky-advanced, receipts, attestation.
Heavy optional deps (git2, keyring) feature-gated. All dependencies documented with justifications in DEPENDENCIES.md.
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

---
*Last updated: 2026-02-13 after v1.4 milestone started*
