# Phase 06: Feature Flags - Research

**Researched:** 2026-02-10
**Domain:** Rust cargo feature flag architecture and CI testing strategies
**Confidence:** HIGH

## Summary

This research investigates how to organize feature flags in a consolidated Rust workspace to prevent combinatorial explosion and ensure maintainable CI testing. The investigation focused on Rust best practices from official documentation, community patterns from mature projects (tokio, rustls, RustCrypto), and cargo tooling (cargo-hack) for feature testing matrices.

**Key Finding:** Feature flags should be organized into semantic categories (not algorithm variants), tested with cargo-hack's --feature-powerset for critical combinations, and documented using cargo doc --all-features. The existing TrustEdge feature structure (`audio`, `yubikey`) is already correct — Phase 6 consolidates this pattern workspace-wide and adds proper CI testing.

**Primary recommendation:** Organize features into 3 categories: **backend** (hardware integrations: yubikey, tpm), **platform** (I/O capabilities: audio, network), and **format** (protocol support: archive, receipts, attestation). Test default, each individual feature, and all-features configurations in CI. Document feature requirements using `#[doc(cfg(feature = "..."))]` annotations.

## Standard Stack

### Core Tools

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| cargo-hack | 0.6+ | Feature powerset testing | Official cargo team recommendation for CI feature testing |
| cargo-semver-checks | Latest | API compatibility | Already in CI (scripts/ci-check.sh:101), prevents feature-related breakage |
| document-features | Optional | Feature docs automation | Community standard for generating feature flag documentation |

### Installation

```bash
# Already installed in CI (workflows/ci.yml:35-36)
cargo install cargo-hack --locked
cargo install cargo-semver-checks --locked

# Optional: auto-generate feature docs from Cargo.toml comments
cargo install document-features
```

### Supporting Documentation

| Crate Metadata | Purpose | When to Use |
|----------------|---------|-------------|
| `[package.metadata.docs.rs]` | Control docs.rs build | All library crates with features |
| `all-features = true` | Include all feature APIs in docs | Default for comprehensive docs |
| `#[doc(cfg(feature = "..."))]` | Annotate feature requirements | Every feature-gated public API |

**Configuration:**
```toml
# In each library crate Cargo.toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Architecture Patterns

### Pattern 1: Categorical Feature Organization

**What:** Group features by purpose/category, not by implementation details or algorithm variants.

**When to use:** All workspace-consolidated crates with optional dependencies.

**Anti-pattern:** Per-subsystem feature flags like `receipt-ops`, `attestation-ops` lead to combinatorial explosion.

**Example:**
```toml
[features]
# GOOD: Categorical organization
default = []

# Backend category: Hardware/storage integrations
yubikey = ["pkcs11", "dep:yubikey", "x509-cert", "der", "spki", "signature"]
tpm = ["tss-esapi"]  # Future

# Platform category: I/O and system capabilities
audio = ["cpal"]
network = ["tokio", "quinn", "rustls"]

# Format category: Protocol/data format support
# (Phase 6: Do NOT create these yet — consolidation only)
# archive = []     # Future: if .trst becomes optional
# receipts = []    # Future: if receipts become optional
```

**Why categorical:**
- Avoids explosion: 3 categories with 2-3 features each = 8-27 combinations
- VS algorithm features: `aes`, `chacha`, `ed25519`, `rsa` = 2^4 = 16 combinations (and grows exponentially)
- User mental model: "I need hardware" vs "I need AES-256-GCM vs AES-128-GCM"

**Source:** [Effective Rust Item 26: Be wary of feature creep](https://effective-rust.com/features.html), [Tokio Feature Organization](https://docs.rs/tokio)

### Pattern 2: Additive-Only Features

**What:** Features only add capabilities, never remove or change behavior.

**When to use:** Always. This is a Rust/Cargo requirement, not optional.

**Anti-pattern:**
```toml
# BAD: Subtractive features
no-std = []  # Disables std
minimal = []  # Removes default functionality
```

**Correct pattern:**
```toml
# GOOD: Additive features
default = ["std"]  # Std is the default
std = []           # Enables std library
alloc = []         # Heap allocation without full std (future: no_std work)
```

**Why:** Cargo unifies features across dependency graph. If crate A enables `std` and crate B enables `no-std` for same dependency, Cargo cannot resolve this conflict. Additive features unify cleanly (A+B = both features enabled).

**Source:** [Cargo Book: Features](https://doc.rust-lang.org/cargo/reference/features.html)

### Pattern 3: cargo-hack Feature Powerset Testing

**What:** Test all valid feature combinations automatically in CI to catch feature interaction bugs.

**When to use:** For crates with multiple independent features (trustedge-core has 2 currently: audio, yubikey).

**Example:**
```bash
# Test all combinations: [], [audio], [yubikey], [audio,yubikey]
cargo hack check --feature-powerset --no-dev-deps --package trustedge-core
```

**Advanced: Excluding invalid combinations:**
```bash
# If features conflict (none currently in TrustEdge)
cargo hack check --feature-powerset \
  --mutually-exclusive-features runtime-tokio,runtime-async-std \
  --exclude-features nightly-only \
  --package trustedge-core
```

**CI Integration:**
```yaml
# Already in .github/workflows/ci.yml:74-75
- name: Feature compatibility check (cargo-hack)
  run: cargo hack check --feature-powerset --no-dev-deps --package trustedge-core
```

**Source:** [cargo-hack GitHub](https://github.com/taiki-e/cargo-hack), [GitHub Actions Best Practices for Rust](https://www.infinyon.com/blog/2021/04/github-actions-best-practices/)

### Pattern 4: CI Testing Matrix (Critical Combinations)

**What:** Test specific high-value feature combinations even if not testing full powerset.

**When to use:** When powerset is too large (2^n becomes expensive), test the critical real-world configurations.

**TrustEdge Critical Combinations:**
1. **default** (no features) — Fast CI, maximum portability
2. **audio** — Edge devices with microphone capture
3. **yubikey** — Hardware-backed security deployments
4. **all-features** — Full functionality test

**Current CI strategy (scripts/ci-check.sh):**
```bash
# Already implemented correctly:
cargo clippy --workspace --no-default-features  # (1) default
cargo clippy -p trustedge-core --features audio # (2) audio
cargo clippy -p trustedge-core --features yubikey # (3) yubikey
# Missing: all-features test (should add)
```

**Recommendation: Add all-features test:**
```bash
# Add to scripts/ci-check.sh after step 11
echo "■ Step 12: Build and test all features together..."
cargo build -p trustedge-core --all-features
cargo test -p trustedge-core --all-features
```

**Source:** [GitHub Actions Matrix Strategy](https://codefresh.io/learn/github-actions/github-actions-matrix/), [Advanced Matrix Testing](https://partial.solutions/2023/advanced-matrix-testing-with-github-actions.html)

### Pattern 5: Feature Documentation

**What:** Document which features are required for each public API using rustdoc annotations.

**When to use:** Every feature-gated public type, function, or module.

**Example:**
```rust
// Current pattern in core/src/lib.rs:91
#[cfg(feature = "audio")]
pub use audio::AudioCapture;

// Enhanced with documentation
#[cfg(feature = "audio")]
#[doc(cfg(feature = "audio"))]  // Rustdoc shows "Available on crate feature audio only"
pub use audio::AudioCapture;
```

**Crate-level documentation (lib.rs):**
```rust
//! # Feature Flags
//!
//! ## Backend Features
//! - `yubikey` — YubiKey PIV hardware support (requires PCSC libraries)
//!
//! ## Platform Features
//! - `audio` — Live audio capture (requires ALSA/CoreAudio/WASAPI)
//!
//! ## Building
//! ```bash
//! # Default: no features (fast CI build)
//! cargo build
//!
//! # With audio support
//! cargo build --features audio
//! ```
```

**Cargo.toml metadata for docs.rs:**
```toml
[package.metadata.docs.rs]
all-features = true  # Build docs with all features enabled
rustdoc-args = ["--cfg", "docsrs"]  # Enable doc_cfg annotations
```

**Source:** [document-features crate](https://crates.io/crates/document-features), [Cargo Book: cargo rustdoc](https://doc.rust-lang.org/cargo/commands/cargo-rustdoc.html)

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Feature combination testing | Custom bash loops over feature flags | `cargo hack --feature-powerset` | Handles powerset generation, dependency resolution, proper error reporting |
| Feature documentation | Manual feature list maintenance | `#[doc(cfg(...))]` + `document-features` | Automated from Cargo.toml, stays in sync |
| CI matrix for features | GitHub matrix strategy yaml | cargo-hack with --partition for parallel jobs | Simpler config, built for Rust features |
| Detecting unused features | Manual code review | `cargo machete` (already planned Phase 8) | Detects unused dependencies including feature-gated ones |

**Key insight:** Feature flag testing is complex enough that tooling exists specifically for it. Don't reinvent cargo-hack's powerset logic or feature dependency resolution.

## Common Pitfalls

### Pitfall 1: Feature Explosion (Subsystem Features)

**What goes wrong:** Creating per-subsystem feature flags like `receipt-ops`, `attestation-ops`, `archive-ops` leads to combinatorial explosion and unclear semantics.

**Why it happens:** Intuition that "if receipts is a module, it should have a feature flag." But features are for *optional dependencies*, not code organization.

**How to avoid:**
- Ask: "Does disabling this feature remove external dependencies?"
  - YES → Legitimate feature (audio removes cpal dependency)
  - NO → Don't create a feature (receipts is pure Rust code, no optional deps)
- Rule: Features control dependencies, not code structure

**Warning signs:**
- Feature count > 5 in a single crate
- Features that only gate internal pure Rust code
- Users confused about which features to enable

**Current TrustEdge status:** SAFE. Only 2 features (audio, yubikey), both gate external dependencies.

**Source:** [Cargo Workspace Feature Unification Pitfall](https://nickb.dev/blog/cargo-workspace-and-the-feature-unification-pitfall/)

### Pitfall 2: Default Features Too Large

**What goes wrong:** Setting `default = ["audio", "yubikey", "network"]` makes CI slow and reduces portability.

**Why it happens:** Developer convenience — "most users want most features."

**How to avoid:**
- Default should be: minimum viable configuration for most portable build
- CI should always test with `--no-default-features`
- Users opt-in to hardware/platform features

**Warning signs:**
- CI requires installing audio/hardware libraries for basic build
- Cross-compilation fails due to default features
- Binary size bloat from unused features

**TrustEdge current state:** CORRECT. `default = []` in core/Cargo.toml:92.

**Source:** [Features - The Cargo Book](https://doc.rust-lang.org/cargo/reference/features.html)

### Pitfall 3: Feature Testing Only at Crate Level

**What goes wrong:** Testing features only on `trustedge-core` misses issues in downstream crates (cli, wasm, etc.).

**Why it happens:** Assumption that "if core works, everything works."

**How to avoid:**
- Test workspace-level: `cargo hack check --feature-powerset --workspace` (may be too slow)
- OR test critical downstream crates: `cargo hack check --feature-powerset -p trustedge-cli`
- Verify that feature propagation works: `trustedge-cli --features audio` must enable `trustedge-core/audio`

**Warning signs:**
- Features work in `core` but fail when used via `cli`
- WASM build failures not caught until deployment
- Dependency resolution works locally but fails in workspace

**Current gap:** CI only tests core, not cli with features.

**Recommendation:**
```bash
# Add to ci-check.sh
cargo hack check --feature-powerset -p trustedge-cli
```

**Source:** [cargo-hack: Testing Multiple Packages](https://github.com/taiki-e/cargo-hack#usage)

### Pitfall 4: Missing #[doc(cfg)] Annotations

**What goes wrong:** Users see `AudioCapture` in documentation but cargo complains "type not found" because they forgot `--features audio`.

**Why it happens:** Feature-gated code is `#[cfg(feature)]` but documentation doesn't indicate feature requirement.

**How to avoid:**
- Every `#[cfg(feature = "X")]` should have companion `#[doc(cfg(feature = "X"))]`
- Set `all-features = true` in Cargo.toml metadata so docs.rs builds with all features
- Test documentation build: `cargo doc --all-features --open`

**Warning signs:**
- User confusion about "missing types" that are documented
- Documentation doesn't show feature requirements
- docs.rs build different from local docs

**Current gap:** core/src/lib.rs:91 has `#[cfg(feature = "audio")]` but no `#[doc(cfg(...))]`.

**Fix:**
```rust
#[cfg(feature = "audio")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio")))]  // Shows requirement on docs.rs
pub use audio::AudioCapture;
```

**Source:** [Rust RFC: doc_cfg](https://rust-lang.github.io/rfcs/3692-feature-unification.html)

## Code Examples

Verified patterns from official sources:

### Categorical Feature Organization (Tokio Pattern)

```toml
# Source: https://docs.rs/tokio (simplified for clarity)
[features]
default = []

# I/O drivers
net = []
io-util = []
io-std = []

# Runtime
rt = []
rt-multi-thread = ["rt"]

# Platform
fs = []
signal = []
process = []

# All (for testing)
full = ["net", "io-util", "io-std", "rt-multi-thread", "fs", "signal", "process"]
```

**TrustEdge application:**
```toml
# trustedge-core/Cargo.toml (proposed Phase 6 structure)
[features]
default = []

# Backend category: Hardware/storage
yubikey = ["pkcs11", "dep:yubikey", "x509-cert", "der", "spki", "signature"]
# tpm = ["tss-esapi"]  # Future Phase 7+

# Platform category: I/O capabilities
audio = ["cpal"]
# network = ["tokio", "quinn", "rustls"]  # Future: if networking becomes optional

# Convenience: Enable all features for testing
all = ["yubikey", "audio"]  # Note: NOT named "full" to avoid confusion with --all-features
```

### cargo-hack CI Integration

```yaml
# Source: https://github.com/taiki-e/cargo-hack#usage
# .github/workflows/ci.yml (enhanced)
jobs:
  feature-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Install cargo-hack
        run: cargo install cargo-hack --locked

      - name: Check feature powerset
        run: |
          cargo hack check --feature-powerset \
            --no-dev-deps \
            --package trustedge-core \
            --package trustedge-cli

      # Optional: Parallel execution for large matrices
      - name: Test feature combinations (partition 1/4)
        run: |
          cargo hack test --feature-powerset \
            --partition 1/4 \
            --package trustedge-core
```

**TrustEdge current state:** Already has feature-powerset check at ci.yml:74-75, good foundation.

### Feature Documentation Pattern

```rust
// Source: https://doc.rust-lang.org/cargo/reference/features.html
// Combined with document-features pattern

//! # trustedge-core
//!
//! Core cryptographic library for TrustEdge.
//!
//! ## Feature Flags
//!
//! ### Backend Features
#![doc = document_features::document_features!()]  // Auto-generates from Cargo.toml comments
//!
//! ### Usage Examples
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! trustedge-core = { version = "0.2", features = ["audio"] }
//! ```

/// Audio capture from live microphone input.
///
/// Requires the `audio` feature to be enabled.
///
/// # Platform Support
/// - Linux: ALSA backend
/// - macOS: CoreAudio backend
/// - Windows: WASAPI backend (untested)
#[cfg(feature = "audio")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio")))]
pub struct AudioCapture {
    // ...
}
```

### Mutually Exclusive Features (If Needed)

```bash
# Source: https://github.com/taiki-e/cargo-hack#mutually-exclusive-features
# Example: If TrustEdge adds multiple runtime options in future

cargo hack check --feature-powerset \
  --mutually-exclusive-features runtime-tokio,runtime-async-std \
  --package trustedge-core

# Current TrustEdge: NOT NEEDED
# audio and yubikey are independent, can be combined
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual feature testing in CI | `cargo-hack --feature-powerset` | ~2020 (cargo-hack 0.4+) | Automated detection of feature interaction bugs |
| Default features = "everything" | Default features = minimal viable | ~2021 (Rust Edition 2021 era) | Faster CI, better portability |
| Feature docs in README only | `#[doc(cfg(feature))]` + docs.rs metadata | 2019 (Rust 1.37+) | Documentation shows feature requirements inline |
| Test default + all-features only | Test critical combinations matrix | ~2022 (CI cost awareness) | Balances coverage with CI runtime |

**Deprecated/outdated:**
- `default = ["std"]` as universal pattern → Now `default = []` is preferred for libraries (executables can have richer defaults)
- Testing only `--no-default-features` and `--all-features` → Insufficient, misses valid intermediate combinations
- Feature flags for algorithm selection (`aes`, `chacha`) → Algorithm agility should be runtime, not compile-time

**TrustEdge alignment:** Current feature structure (`default = []`, 2 features) aligns with 2026 best practices. Phase 6 adds testing rigor, not structural changes.

## Open Questions

### Question 1: Should format support (archive, receipts, attestation) be feature-gated?

**What we know:**
- These are pure Rust code with no optional external dependencies
- Adding them as features creates combinatorial explosion (2^3 = 8 more combinations)
- Current structure: always compiled into core (consolidated in Phases 3-5)

**What's unclear:**
- Are there deployment scenarios where users want core crypto WITHOUT receipts/attestation? (e.g., embedded constrained environments)
- Would no_std support (future work) require making these optional?

**Recommendation:**
- **Phase 6: Do NOT add format features.** Receipts, attestation, archive are core functionality.
- **Future (Phase 7+):** If no_std work requires it, reconsider. Acceptable pattern:
  ```toml
  [features]
  default = ["std"]
  std = ["receipts", "attestation", "archive"]  # Std enables formats
  alloc = []  # Heap without std, but no formats
  ```

### Question 2: Should network transport be a feature?

**What we know:**
- Network currently uses tokio, quinn, rustls (heavy dependencies)
- Not all TrustEdge use cases need client-server networking
- Network transport is in `core/src/transport/` (tcp.rs, quic.rs)

**What's unclear:**
- How many users use only local envelope encryption (no network)?
- Is network code substantial enough to justify optional compilation?

**Recommendation:**
- **Phase 6: Investigate dependency tree.** Run `cargo tree -p trustedge-core` to see tokio/quinn impact on compile time and binary size.
- **If tokio adds >5s to compile or >500KB to binary:** Create `network` feature in Phase 6.
- **Otherwise:** Defer to Phase 7+. Don't create features prematurely.

### Question 3: How to handle workspace feature unification?

**What we know:**
- Cargo unifies features across workspace (if core and cli both depend on same crate, features are merged)
- trustedge-cli has `audio = ["trustedge-core/audio"]` feature propagation (Cargo.toml:42)
- WASM crate cannot use audio or yubikey (platform incompatible)

**What's unclear:**
- Should workspace members have independent feature sets or unified?
- How to prevent WASM builds from accidentally enabling yubikey?

**Recommendation:**
- **Keep current pattern:** Each crate declares its own features, thin shells propagate to core.
- **Verify in Phase 6:** `cargo build -p trustedge-wasm` must NOT enable yubikey even if core has it.
- **Protection mechanism:** WASM crate doesn't declare yubikey feature → cargo won't enable it for wasm target even if enabled elsewhere.

## Sources

### Primary (HIGH confidence)

- [Cargo Book: Features](https://doc.rust-lang.org/cargo/reference/features.html) - Official Cargo feature documentation
- [cargo-hack GitHub Repository](https://github.com/taiki-e/cargo-hack) - Official cargo-hack tool documentation
- [Effective Rust: Item 26 - Feature Creep](https://effective-rust.com/features.html) - Authoritative best practices
- [Tokio Documentation: Features](https://docs.rs/tokio) - Real-world categorical feature organization

### Secondary (MEDIUM confidence)

- [GitHub Actions Best Practices for Rust](https://www.infinyon.com/blog/2021/04/github-actions-best-practices/) - CI integration patterns
- [GitHub Actions Matrix Strategy](https://codefresh.io/learn/github-actions/github-actions-matrix/) - Matrix testing strategies
- [Serde Feature Flags](https://serde.rs/feature-flags.html) - Feature organization example
- [Cargo Workspace Feature Unification Pitfall](https://nickb.dev/blog/cargo-workspace-and-the-feature-unification-pitfall/) - Common workspace issues

### Tertiary (LOW confidence - community patterns)

- [document-features crate](https://crates.io/crates/document-features) - Feature documentation automation
- [Advanced Matrix Testing with GitHub Actions](https://partial.solutions/2023/advanced-matrix-testing-with-github-actions.html) - Advanced CI patterns

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - cargo-hack and cargo-semver-checks are official/widely adopted tools
- Architecture patterns: HIGH - Based on Cargo Book, Effective Rust, and tokio/serde real-world patterns
- Pitfalls: MEDIUM-HIGH - Derived from community experience (nickb.dev blog) and official docs

**Research date:** 2026-02-10
**Valid until:** ~90 days (cargo-hack stable, patterns mature, low churn expected)

**TrustEdge-specific context:**
- Current features: 2 (audio, yubikey) — well below explosion threshold
- Current CI: Already uses cargo-hack (ci.yml:74-75) — foundation in place
- Missing: all-features testing, doc(cfg) annotations, downstream crate feature testing
- Risk: LOW — Phase 6 is refinement, not architectural change
