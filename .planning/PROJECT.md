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
- ⚠️ YubiKey PKCS#11 integration — v1.0 (broken: manual DER, software fallbacks, untested flag — rewriting in v1.1)
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

### Active

- [ ] YubiKey backend rewritten from scratch (fail-closed, no software fallbacks)
- [ ] X.509 certificate generation via rcgen (no manual DER encoding)
- [ ] `yubikey` crate used without `untested` feature flag
- [ ] All YubiKey tests exercise real functionality (no placeholders or auto-passes)
- [ ] CI always compile-checks yubikey feature

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

## Current Milestone: v1.1 YubiKey Integration Overhaul

**Goal:** Delete the broken YubiKey backend and rewrite from scratch — `yubikey` crate (stable features only), `rcgen` for X.509, fail-closed design, zero software fallbacks.

**Target features:**
- Scorched-earth rewrite of yubikey.rs (3,263 lines) and all 8 test files
- Fail-closed design: hardware unavailable = error, never silent fallback
- X.509 certificate generation via rcgen (replace 1,000+ lines manual DER)
- `yubikey` crate stable API only (drop `untested` feature flag)
- Real test suite: every test exercises actual functionality, no auto-passes
- CI always compile-checks yubikey feature

## Context

Shipped v1.0 consolidation with 37,589 Rust LOC across 10 crates.
Tech stack: Rust, AES-256-GCM, Ed25519, BLAKE3, XChaCha20-Poly1305, WASM.
343 tests passing (160 in core, 183 across thin shells).
Build time: 45s clean release.
Zero API breaking changes throughout consolidation.
6 non-critical tech debt items carried forward (see MILESTONES.md).
Facade crate deprecation active — removal planned v0.4.0 (Aug 2026).
**v1.1 trigger:** External code review identified critical issues in YubiKey backend — manual ASN.1 DER encoding, silent software fallbacks, hardcoded placeholder keys, `untested` feature flag.

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
| Scorched-earth YubiKey rewrite | External review found critical issues: manual DER, silent fallbacks, placeholder keys | — Pending (v1.1) |
| yubikey crate stable API only | Drop `untested` feature — use only tested/stable functionality | — Pending (v1.1) |
| rcgen for X.509 certs | Replace 1,000+ lines manual DER with battle-tested library | — Pending (v1.1) |
| Fail-closed hardware design | Hardware unavailable = error, never silent software fallback | — Pending (v1.1) |
| No placeholder keys or signatures | Every key and signature must come from real cryptographic operations | — Pending (v1.1) |

---
*Last updated: 2026-02-11 after v1.1 milestone start*
