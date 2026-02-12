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

