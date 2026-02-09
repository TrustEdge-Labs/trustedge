<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# Project Research Summary

**Project:** TrustEdge Workspace Consolidation
**Domain:** Rust Cryptographic Library Organization
**Researched:** 2026-02-09
**Confidence:** HIGH

## Executive Summary

TrustEdge's consolidation from 10 crates to a monolithic core with thin shells is a well-trodden path in the Rust crypto ecosystem. The research confirms that successful crypto libraries (ring, RustCrypto, rustls) follow a consistent pattern: capability-based layering (primitives → protocols → applications), minimal feature flags for platform integration (not algorithms), and ruthless focus on table stakes features over premature abstraction.

The current TrustEdge architecture already implements the hardest organizational pattern—the Universal Backend system for pluggable key management. Consolidation should preserve this differentiator while addressing the three critical gaps: (1) unified error handling (10+ duplicate error types), (2) consistent module hierarchy (18 flat modules → layered structure), and (3) WASM compatibility preservation. The recommended approach is incremental merge in dependency order (trst-core → receipts → attestation → pubky) over 9-11 weeks with rigorous test preservation (150+ tests must survive intact).

Key risks center on WASM compatibility (trst-core's minimal dependencies must not be polluted by core's tokio/quinn/cpal), hardware integration fragility (YubiKey's 3236 lines of PKCS#11 code is brittle), and test loss during migration. Mitigation requires feature-gating non-WASM dependencies, pinning hardware dependency versions, and pre-merge test inventory with validation scripts.

## Key Findings

### Recommended Stack

The consolidation requires no new technologies—only tooling for validation. Rust 1.82+ provides robust workspace dependency inheritance and feature resolution. The critical tooling stack includes cargo-semver-checks (API compatibility validation), cargo-modules (dependency visualization), cargo-hack (feature matrix testing), and cargo-machete (unused dependency cleanup). All tools are mature (90-100% confidence) and actively maintained as of January 2025.

**Core technologies:**
- **cargo-semver-checks (v0.36)**: API compatibility validation during consolidation — prevents accidental breaking changes
- **cargo-modules (v0.15)**: Dependency graph visualization — detects circular dependencies early
- **cargo-hack (v0.6.36)**: Feature matrix testing — validates all feature combinations work
- **cargo-machete (v0.7)**: Unused dependency cleanup — reduces bloat post-merge

**Critical version requirements:**
- Rust 1.82+ (stable as of Jan 2025) for workspace feature resolution
- No new crypto dependencies (use existing aes-gcm, ed25519-dalek, blake3)

### Expected Features

Consolidation focuses on organizational features (how code is structured), not product features (what users can do). The research identifies three tiers based on analysis of established crypto libraries.

**Must have (table stakes):**
- Unified error type hierarchy (currently 10+ separate error enums across crates)
- Consistent feature flag architecture (backend, platform, format support)
- Re-export facade pattern (preserve public API surface during migration)
- Clear module hierarchy (primitives → backends → protocols → applications → transport → io)
- Documentation standards (module-level security considerations, usage examples)

**Should have (differentiators):**
- Backend plugin system (ALREADY EXISTS — Universal Backend is the crown jewel)
- Compile-time feature selection (feature-gated code doesn't compile when disabled)
- Trait-based API surface for extensibility (already correct for backends)
- Prelude module pattern for common imports (low effort, high DX value)

**Defer (anti-features to avoid):**
- Algorithm agility (hard-coded Ed25519/AES-256-GCM is fine, no need for RSA/ChaCha variants)
- no_std support (requires separate milestone, half-measures are worse than honest "requires std")
- Over-abstraction (crypto code needs performance and audit-ability, not extensibility)
- Kitchen sink approach (only consolidate code that's actually used)

### Architecture Approach

The target architecture follows a layered capability model proven by ring and rustls. TrustEdge's existing backend/ and transport/ directories already demonstrate this pattern, making consolidation an extension rather than a restructuring.

**Major components:**
1. **Primitives Layer (Layer 0)** — Pure crypto operations (AES-256-GCM, Ed25519, BLAKE3) with no internal dependencies
2. **Backends Layer (Layer 1)** — Universal Backend system for key management abstraction (software HSM, keyring, YubiKey)
3. **Protocols Layer (Layer 2)** — Wire formats and cryptographic protocols (Envelope, ContinuityChain, Auth, Pubky)
4. **Applications Layer (Layer 3)** — Business logic (Receipts, Attestation, Archives with .trst format)
5. **Transport Layer (Layer 4)** — Network I/O (TCP, QUIC) independent of application protocols
6. **I/O Layer (Layer 5)** — Platform abstractions (InputReader trait, AudioCapture)

**Data flow invariants:**
- Lower layers NEVER import from higher layers
- Primitives are leaf dependencies (no internal imports)
- Protocols use backends for key management (never direct primitives)
- Applications use protocols, never primitives directly

**Module organization:**
```
trustedge-core/src/
├── primitives/       # Layer 0: aead, signatures, hashing
├── backends/         # Layer 1: universal, software_hsm, keyring, yubikey
├── protocols/        # Layer 2: envelope, chain, auth, pubky (feature-gated)
├── applications/     # Layer 3: receipts, attestation, archives
├── transport/        # Layer 4: tcp, quic
└── io/               # Layer 5: reader, audio (feature-gated)
```

### Critical Pitfalls

Research identified 12 consolidation pitfalls, prioritized for TrustEdge context:

1. **WASM vs Native Dependency Conflicts (HIGH)** — trst-core must remain WASM-compatible. Merging with core (tokio, quinn, cpal, pkcs11) will break browser verification unless dependencies are target-gated. Prevention: Use `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` for native-only deps, test with `cargo check --target wasm32-unknown-unknown`.

2. **Test Namespace Collisions and Lost Coverage (HIGH)** — 150+ tests across 10 crates risk silent loss during merge. Prevention: Pre-merge inventory (document test count per crate), namespace preservation (receipts/tests/integration_tests.rs → core/tests/receipts_integration.rs), validation script comparing before/after counts.

3. **YubiKey Hardware Integration Regression (HIGH)** — YubiKey backend (3236 LOC) is fragile with PKCS#11 dependencies. Prevention: Pin exact versions (`pkcs11 = "=0.5.0"`), preserve hardware tests with `#[ignore]` markers, manual test protocol before/after merge.

4. **Feature Flag Explosion (MEDIUM)** — With yubikey, audio, and inherited features, combinatorial complexity (2^n configurations) becomes untestable. Prevention: Feature categorization (backend vs platform vs format), test critical combinations only (default, yubikey, audio, all-features), avoid per-subsystem features like `receipt-ops`.

5. **API Surface Regression (MEDIUM)** — Thin shells (trustedge-cli, trst-cli, wasm, trst-wasm) break if library paths change. Prevention: Re-export facades with deprecation warnings, test all consumers after each merge (`cargo test -p trustedge-cli`), semver-checks validation.

## Implications for Roadmap

Based on combined research, the consolidation naturally divides into 6 phases over 9-11 weeks (80h effort, solo developer at 10h/week).

### Phase 1: Foundation and Analysis
**Rationale:** Establish baseline before making changes. Dependency analysis, test inventory, and API documentation prevent pitfalls #1, #2, #4, #5.
**Delivers:** Module hierarchy skeleton, dependency graph visualization, test count baseline, API surface snapshot.
**Addresses:** Table stakes #4 (clear module hierarchy design), pitfall prevention infrastructure.
**Avoids:** Pitfall #2 (test loss), #4 (feature explosion), #5 (API breakage).
**Duration:** Week 1 (8h)
**Research flag:** SKIP RESEARCH — standard Rust tooling patterns.

### Phase 2: Unified Error Hierarchy
**Rationale:** Blocks everything else. Must unify 10+ error types before merging code. Addresses table stakes #1 (unified errors).
**Delivers:** Single `TrustEdgeError` enum with subsystem variants, migration path for old error types.
**Uses:** thiserror (already in core/Cargo.toml), anyhow for CLI propagation.
**Implements:** Primitives layer error types (CryptoError, BackendError, etc.).
**Avoids:** Pitfall #7 (error consolidation gone wrong).
**Duration:** Week 2-3 (8h)
**Research flag:** SKIP RESEARCH — well-documented Rust error patterns.

### Phase 3: Integrate trst-core (Manifest Types)
**Rationale:** WASM-safe code merges first to establish compatibility baseline. Resolves duplicate manifest types. Addresses WASM pitfall #1.
**Delivers:** Archive manifest types in `applications/archives/manifest/`, updated trst-cli and trst-wasm imports.
**Addresses:** Applications layer (Layer 3) foundation, .trst format support.
**Avoids:** Pitfall #1 (WASM breakage) by merging WASM-safe code early.
**Duration:** Week 3-4 (12h)
**Research flag:** SKIP RESEARCH — canonical cam.video manifest types already defined.

### Phase 4: Integrate Receipts
**Rationale:** Receipts are dependency leaf (only depends on core::Envelope). Clean merge with no circular deps.
**Delivers:** Receipts in `applications/receipts/`, 23 tests preserved, deprecated `trustedge-receipts` facade.
**Addresses:** Applications layer digital receipt system, receipts feature (table stakes).
**Avoids:** Pitfall #5 (circular dependencies) by merging in dependency order.
**Duration:** Week 4-5 (12h)
**Research flag:** SKIP RESEARCH — receipt logic unchanged, pure code move.

### Phase 5: Integrate Attestation
**Rationale:** Attestation depends on receipts (optional envelope integration). Must come after receipts.
**Delivers:** Attestation in `applications/attestation/`, envelope feature unified (always enabled).
**Addresses:** Applications layer software attestation, attestation feature.
**Uses:** Protocols layer (envelope) and backends (signing).
**Duration:** Week 5-6 (8h)
**Research flag:** SKIP RESEARCH — standard envelope pattern.

### Phase 6: Integrate Pubky (Community/Experimental)
**Rationale:** Pubky is experimental, feature-gated. Merging last allows easy removal if unmaintained.
**Delivers:** Pubky in `protocols/pubky/`, feature flag `pubky = ["dep:pubky", "x25519-dalek"]`.
**Addresses:** Differentiator #7 (algorithm agility for X25519 ECDH), community integration.
**Avoids:** Pitfall #4 (feature explosion) by keeping behind single `pubky` flag.
**Duration:** Week 6-7 (8h)
**Research flag:** NEEDS RESEARCH IF CHANGES REQUIRED — Pubky integration is niche, sparse docs.

### Phase 7: Feature Flag Consolidation
**Rationale:** After code merge, consolidate features to prevent explosion. Addresses table stakes #2.
**Delivers:** Unified feature set (backend, platform, format), CI matrix testing, feature documentation.
**Addresses:** Table stakes #2 (consistent feature flags), differentiator #8 (compile-time selection).
**Avoids:** Pitfall #4 (feature explosion) with feature categorization.
**Duration:** Week 7-8 (8h)
**Research flag:** SKIP RESEARCH — cargo features are well-documented.

### Phase 8: Re-export Facades and Documentation
**Rationale:** Preserve backward compatibility before deprecating old crates. Addresses table stakes #3 and #5.
**Delivers:** Deprecated crate facades, migration guide, updated CLAUDE.md and examples.
**Addresses:** Table stakes #3 (re-export facade), #5 (documentation standards).
**Avoids:** Pitfall #5 (API breakage), #9 (stale documentation).
**Duration:** Week 8-9 (8h)
**Research flag:** SKIP RESEARCH — standard deprecation practices.

### Phase 9: Validation and Cleanup
**Rationale:** Final validation of all pitfall prevention strategies before declaring success.
**Delivers:** Test count validation (150+ preserved), WASM build verification, YubiKey manual tests, benchmark comparison.
**Addresses:** All pitfall validation (#1-#12), build time measurement (#6), performance regression check (#8).
**Avoids:** Silent breakage through comprehensive validation.
**Duration:** Week 9-11 (12h with 2-week buffer)
**Research flag:** SKIP RESEARCH — testing and validation.

### Phase Ordering Rationale

- **WASM-safe first (Phase 3):** Establishes compatibility baseline early, prevents late-stage WASM breakage.
- **Dependency order (Phases 4-5-6):** trst-core → receipts → attestation → pubky follows natural dependency graph, avoids circular deps.
- **Features after code (Phase 7):** Can't consolidate features until code is merged and patterns are clear.
- **Facades near end (Phase 8):** Need stable API surface before creating backward-compat layer.
- **Validation last (Phase 9):** Final gate before release, catches issues from earlier phases.

### Research Flags

Phases with standard patterns (skip research-phase):
- **Phases 1, 2, 4, 5, 7, 8, 9:** Well-documented Rust patterns (workspace consolidation, error handling, code moves, feature flags, documentation).
- **Phase 3:** Canonical types already defined in trst-core.

Phases needing deeper research during planning:
- **Phase 6 (Pubky):** IF changes required beyond pure code move. Pubky integration is community/experimental with sparse documentation. Current plan is pure move, no research needed unless implementation encounters unknowns.

### Success Metrics

The consolidation is complete when:

| Metric | Target | Validation |
|--------|--------|------------|
| Total crates | 5 (core + 4 shells) | `ls crates/ | wc -l` |
| Test count | 150+ preserved | Validation script comparison |
| WASM build | Success | `cargo check --target wasm32-unknown-unknown` |
| YubiKey tests | All passing | Manual hardware test protocol |
| Build time | <2x baseline | `cargo build --timings` |
| API breakage | Zero (semver) | `cargo semver-checks check-release` |
| Documentation | 100% examples compile | `cargo test --doc` |

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH (95%) | Mature Rust tooling, verified current versions, proven in production |
| Features | HIGH (90%) | Based on established crypto library patterns (ring, RustCrypto, rustls) |
| Architecture | HIGH (95%) | Layered capability model is standard practice, TrustEdge already follows it |
| Pitfalls | HIGH (90%) | 12 pitfalls identified from Rust workspace consolidation experience |

**Overall confidence:** HIGH (92%)

### Gaps to Address

1. **Duplicate manifest.rs resolution:** Both core/src/manifest.rs (460 LOC) and trst-core/src/manifest.rs (449 LOC) exist. Resolution during Phase 3: Manual inspection to determine if trst-core is canonical cam.video spec (keep that) or if core has TrustEdge extensions (merge both into manifest/types.rs + manifest/serialization.rs).

2. **YubiKey hardware availability:** Validation requires physical YubiKey for manual testing. If unavailable, document as untested and flag for future validation.

3. **External consumers unknown:** Research assumes no external dependents on trustedge-receipts/attestation/trst-core. Action during Phase 1: Search crates.io and GitHub to verify. If found, extend deprecation window from 6 months to 12 months.

4. **Build time actual impact:** Cold build estimate is ~60s for monolith vs ~90s for 10 crates, but this is theoretical. Action during Phase 9: Measure with `cargo build --timings`, optimize if >2x baseline.

## Sources

### Primary (HIGH confidence)
- **Rust Cargo Book** — Workspace consolidation patterns, feature flag semantics
- **ring repository** (briansmith/ring) — Monolithic crypto crate architecture, zero feature flags
- **RustCrypto organization** — Multi-crate workspace patterns, feature strategies
- **rustls repository** — Protocol layer separation, CryptoProvider trait, dangerous_configuration pattern
- **cargo-semver-checks** (v0.36) — API compatibility validation tool
- **cargo-modules** (v0.15) — Dependency graph visualization
- **tokio 1.0 migration guide** — Re-export facade deprecation strategy

### Secondary (MEDIUM confidence)
- **sodiumoxide/orion** — Pure Rust crypto library organization patterns
- **Rust API Guidelines** — Error handling, documentation standards
- **Community best practices** — Feature flag anti-patterns, WASM compatibility

### Tertiary (LOW confidence, needs validation)
- **Pubky network integration** — Community contribution, sparse official docs, may need validation during implementation

---
*Research completed: 2026-02-09*
*Ready for roadmap: yes*
*Estimated timeline: 9-11 weeks (80h effort)*
*Critical path: Phases 1-3-4 (foundation → WASM-safe → dependency order)*
