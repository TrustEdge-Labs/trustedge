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

### Active

<!-- v1.2 scope — see REQUIREMENTS.md for full details -->
- [ ] Clear stable/experimental crate classification with Cargo.toml metadata
- [ ] Dependency audit and rationalization for core crates
- [ ] Experimental crates marked as beta (not deleted)
- [ ] Build times and supply chain surface reduced

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

## Current Milestone: v1.2 Scope Reduction & Dependency Rationalization

**Goal:** Make TrustEdge maintainable by a solo developer — clear stable/experimental split, trimmed dependencies, reduced build and maintenance burden.

**Target features:**
- Stable/experimental crate classification (5 core, 5 experimental) with Cargo.toml metadata and README markers
- Moderate dependency audit: remove unused, consolidate redundant, document what stays
- Facade crates reclassified from "deprecated with timeline" to "experimental/unsupported"
- CI focused on core crates only; experimental crates build but don't block

### Completed Milestones
- **v1.0 Consolidation** — Monolith core + thin shells, 343 tests, zero API breaks
- **v1.1 YubiKey Integration Overhaul** — Scorched-earth rewrite with fail-closed design, battle-tested libraries, 27 tests, unconditional CI

## Context

Shipped v1.1 with 30,144 Rust LOC across 10 crates.
Tech stack: Rust, AES-256-GCM, Ed25519, BLAKE3, XChaCha20-Poly1305, WASM, YubiKey PIV (ECDSA P-256, RSA-2048).
370+ tests passing (160+ in core including 18 YubiKey simulation, 9 hardware integration with #[ignore]).
Build time: 45s clean release.
CI unconditionally validates YubiKey feature on every PR.
Facade crates reclassified to experimental (v1.2) — no longer on deprecation timeline.
Key generation and attestation deferred to future (yubikey crate API limitations).
v1.2 focus: scope reduction and dependency rationalization for solo-dev sustainability.

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

---
*Last updated: 2026-02-11 after v1.2 milestone started*
