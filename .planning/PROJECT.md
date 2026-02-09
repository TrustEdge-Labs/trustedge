# TrustEdge Consolidation

## What This Is

TrustEdge is a Rust workspace providing hardware-backed cryptographic operations for edge devices and IoT. The codebase has grown to 10 crates with overlapping functionality, duplicated patterns, and unclear boundaries. This milestone consolidates everything into a monolith core with thin CLI/WASM shells — preserving all prior work while eliminating duplication and establishing a clear architecture.

## Core Value

A single, reliable `trustedge-core` library that owns all cryptographic operations (envelope encryption, signing, receipts, attestation, archives) with production-quality YubiKey hardware integration — thin CLIs and WASM bindings are just frontends.

## Requirements

### Validated

<!-- Shipped and confirmed valuable. -->

- ✓ AES-256-GCM envelope encryption with Ed25519 signing — existing in `crates/core/`
- ✓ Universal Backend system (Software HSM, Keyring, YubiKey) — existing in `crates/core/src/backends/`
- ✓ Network transport (TCP framed, QUIC with TLS) — existing in `crates/core/src/transport/`
- ✓ Digital receipt system with ownership chains — existing in `crates/receipts/`
- ✓ Software attestation with provenance tracking — existing in `crates/attestation/`
- ✓ .trst archive format with cam.video profile — existing in `crates/trst-core/` + `crates/trst-cli/`
- ✓ YubiKey PKCS#11 integration — existing in `crates/core/src/backends/yubikey.rs`
- ✓ WASM browser bindings — existing in `crates/wasm/` + `crates/trst-wasm/`
- ✓ Pubky network integration — existing in `crates/pubky/` + `crates/pubky-advanced/`
- ✓ 150+ tests across workspace — existing

### Active

<!-- Current scope. Building toward these. -->

- [ ] Audit and map all cross-crate duplication (crypto, serialization, error types, manifest handling)
- [ ] Consolidate duplicated functionality into `trustedge-core` using best-implementation-wins strategy
- [ ] Merge receipt logic into core (preserving all capabilities)
- [ ] Merge attestation logic into core (preserving all capabilities)
- [ ] Merge trst-core manifest types into core (preserving WASM compatibility)
- [ ] Unify WASM bindings into single crate wrapping core
- [ ] Reduce CLIs to thin shells over core library APIs
- [ ] Ensure no feature or parameter drift between crates after consolidation
- [ ] All existing tests pass after reorganization
- [ ] Production-quality YubiKey end-to-end workflow (encrypt, sign, verify with hardware key)

### Out of Scope

<!-- Explicit boundaries. Includes reasoning to prevent re-adding. -->

- TPM support — premature, no hardware to test against, adds complexity before core is stable
- Post-quantum cryptography — research phase only, no production use case yet
- New features — this milestone is consolidation only, no new capabilities
- Deleting crates or code — everything is preserved, just reorganized

## Context

- Solo developer / small team — scope must be realistic
- Existing 10-crate workspace with significant working code
- Codebase map available at `.planning/codebase/` (architecture, stack, conventions, concerns, testing)
- Key duplication areas likely: crypto primitives across core/receipts/attestation, manifest types across core/trst-core, error types, serialization patterns
- Merge strategy: best implementation wins — pick the better version, move to core, update callers

## Constraints

- **Preservation**: All prior work must be preserved — no functionality loss
- **Consistency**: After consolidation, no feature or parameter drift between what were separate crates
- **Tests**: All 150+ existing tests must continue to pass
- **Backward compatibility**: Public API surface of `trustedge-core` must support all current consumers
- **Architecture**: Monolith core + thin shells — CLIs and WASM are frontends only

## Key Decisions

<!-- Decisions that constrain future work. Add throughout project lifecycle. -->

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Monolith core + thin shells | Eliminates duplication, single source of truth for crypto ops | — Pending |
| Best implementation wins for merges | Pragmatic — don't union-merge everything, pick the better code | — Pending |
| Envelope encryption is the core product | YubiKey hardware signing is the differentiator, focus stability here | — Pending |
| No new features this milestone | Consolidation only — adding features while reorganizing is a recipe for bugs | — Pending |

---
*Last updated: 2026-02-09 after initialization*
