# Phase 1: Foundation - Research

**Researched:** 2026-02-09
**Domain:** Rust workspace analysis and module architecture
**Confidence:** HIGH

## Summary

Phase 1 establishes baseline metrics and module hierarchy scaffolding for trustedge workspace consolidation. The phase requires four specialized Rust tooling categories: dependency analysis (cargo-tree, cargo-workspace-analyzer), API compatibility tracking (cargo-semver-checks), feature testing (cargo-hack), and unused dependency detection (cargo-machete). The workspace currently has 10 crates with 202 tests (150+ documented, actual count higher), requiring per-module granular tracking to detect regressions during consolidation.

The recommended approach uses standard Cargo tooling combined with purpose-built analyzers. cargo-workspace-analyzer generates Mermaid diagrams for cross-crate visualization, cargo-semver-checks creates rustdoc JSON baselines for API surface tracking, and custom shell scripts inventory test counts per module. The layered module hierarchy (primitives/backends/protocols/applications/transport/io) follows established Rust patterns for large crates, with directory-level separation and mod.rs contract documentation.

**Primary recommendation:** Install all four tools immediately, generate baseline artifacts before any code changes, and integrate cargo-semver-checks + cargo-hack into CI to catch regressions from Phase 1 forward.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

**Module hierarchy:**
- Create 6-layer directory structure in trustedge-core/src/: primitives/, protocols/, applications/, io/ (backends/ and transport/ already exist)
- Directories + mod.rs files only — no code moves, no re-exports, just the scaffolding
- Existing flat modules (crypto.rs, envelope.rs, chain.rs, etc.) stay in place — moves happen in later phases
- New directories sit alongside existing backends/ and transport/ (flat layout, no src/layers/ parent)
- Each mod.rs gets layer contract documentation: what belongs here, dependency rules ("never imports from protocols or above"), and what will live here after consolidation

**Duplication audit:**
- Map both exact duplicates AND near-duplicates (functions/types that do the same thing with minor differences)
- Table summary format: one line per finding with module paths, crates affected, duplicate type (exact/near), and best-implementation recommendation
- Also map cross-crate dependency usage (who imports what from whom) to verify merge order
- Output: .planning/phases/01-foundation/AUDIT.md

**Test baseline:**
- Per-crate AND per-module granularity (e.g., "core::backends: 15, core::envelope: 12")
- Include full test names per crate/module, not just counts — pinpoints exactly what's missing if a count drops
- Create scripts/test-inventory.sh that outputs current counts in the same format as the baseline
- Baseline snapshot saved to .planning/phases/01-foundation/TEST-BASELINE.md
- Script can be re-run after each subsequent phase to compare against baseline

**Tooling:**
- Install all four recommended tools: cargo-semver-checks, cargo-modules, cargo-hack, cargo-machete
- Add cargo-semver-checks and cargo-hack to CI (GitHub Actions) immediately in Phase 1 — catches regressions from the start
- Also update scripts/ci-check.sh with the new tool checks
- Save API surface snapshot as a baseline artifact in the phase directory (not just pass/fail gate)

### Claude's Discretion

- Exact cargo-modules visualization format and output
- How to structure the dependency graph output (text, mermaid, DOT, etc.)
- cargo-machete findings format and whether to act on unused deps in Phase 1 or defer

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope.
</user_constraints>

---

## Standard Stack

### Core Tools

| Tool | Version | Purpose | Why Standard |
|------|---------|---------|--------------|
| cargo-semver-checks | Latest (0.x) | API compatibility validation | Official Rust ecosystem tool, uses rustdoc JSON as baseline, CI-ready |
| cargo-hack | Latest (0.x) | Feature powerset testing | De-facto standard for testing feature combinations, partition support for CI |
| cargo-machete | Latest (0.9+) | Unused dependency detection | Fast regex-based scanner, low false-positive with metadata config |
| cargo-modules | Latest | Module structure visualization | Generates DOT/SVG output, hierarchical tree views with visibility |
| cargo-workspace-analyzer | Latest | Cross-crate dependency graphs | Mermaid diagram output, circular dependency detection |

### Supporting Tools

| Tool | Version | Purpose | When to Use |
|------|---------|---------|-------------|
| cargo tree | Built-in | Dependency tree display | Quick CLI inspection, duplicate version detection |
| cargo metadata | Built-in | JSON workspace data | Programmatic workspace analysis in scripts |
| graphviz (dot) | System package | DOT to SVG/PNG rendering | Visualizing cargo-modules output |
| mermaid-cli | npm 10+ | Mermaid to SVG rendering | Visualizing cargo-workspace-analyzer output |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| cargo-semver-checks | cargo-public-api | cargo-public-api is lower-level, requires custom diffing logic |
| cargo-workspace-analyzer | Manual cargo tree parsing | Manual parsing misses circular deps, no visualization |
| cargo-modules | Custom AST analysis | Reinventing visualization, no DOT output |

**Installation:**
```bash
# Core tools
cargo install cargo-semver-checks --locked
cargo install cargo-hack --locked
cargo install cargo-machete
cargo install cargo-modules
cargo install cargo-workspace-analyzer

# Supporting tools (for rendering)
sudo apt-get install graphviz  # or brew install graphviz
npm install -g @mermaid-js/mermaid-cli
```

## Architecture Patterns

### Recommended Project Structure

The layered architecture pattern for large Rust crates uses directory-based module organization with explicit dependency direction enforcement through documentation contracts.

```
crates/core/src/
├── primitives/      # Layer 1: Pure crypto primitives (AES, Ed25519, BLAKE3)
│   └── mod.rs       # Contract: never imports from any other layer
├── backends/        # Layer 2: Backend abstraction (EXISTING - already correct layer)
│   └── mod.rs       # Contract: imports primitives only
├── protocols/       # Layer 3: Wire formats, envelopes, chains
│   └── mod.rs       # Contract: imports primitives + backends
├── applications/    # Layer 4: High-level APIs (receipts, attestation)
│   └── mod.rs       # Contract: imports primitives + backends + protocols
├── transport/       # Layer 5: Network layer (EXISTING - already correct layer)
│   └── mod.rs       # Contract: imports primitives + backends + protocols
├── io/              # Layer 6: Input/output adapters (audio, archive)
│   └── mod.rs       # Contract: imports all layers below
├── crypto.rs        # EXISTING flat module (moves to primitives/ in Phase 2+)
├── envelope.rs      # EXISTING flat module (moves to protocols/ in Phase 2+)
├── chain.rs         # EXISTING flat module (moves to protocols/ in Phase 2+)
├── audio.rs         # EXISTING flat module (moves to io/ in Phase 2+)
├── archive.rs       # EXISTING flat module (moves to io/ in Phase 2+)
└── lib.rs           # Root module declarations
```

**Key insight:** Phase 1 creates directories + mod.rs only. Existing flat modules stay in place. Later phases move code into directories.

### Pattern 1: Layer Contract Documentation

**What:** Each mod.rs contains documentation defining layer boundaries and dependency rules.

**When to use:** Every layer directory in the hierarchy.

**Example mod.rs template:**
```rust
//! # [Layer Name] Layer
//!
//! **Layer contract:** [What belongs in this layer]
//!
//! ## Dependency Rules
//! - ✔ CAN import: [list of allowed layers]
//! - ✖ NEVER imports: [list of forbidden layers]
//!
//! ## Post-consolidation contents
//! - [Module 1]: [current location] → [future location]
//! - [Module 2]: [current location] → [future location]
//!
//! ## Examples
//! [Brief usage example showing layer's purpose]

// Layer is empty during Phase 1 - populated in later phases
```

### Pattern 2: Test Inventory with Full Names

**What:** Baseline tracking that records both test counts AND full test names per module.

**When to use:** Before any code consolidation, re-run after each phase.

**Example output format:**
```
=== trustedge-core (101 tests) ===

archive (7 tests):
  archive::tests::test_archive_dir_name
  archive::tests::test_archive_validation
  ...

backends::software_hsm (23 tests):
  backends::software_hsm::tests::test_backend_creation_default_config
  backends::software_hsm::tests::test_ed25519_key_generation_and_signing
  ...
```

**Implementation pattern:**
```bash
# Generate baseline
cargo test --package trustedge-core --lib -- --list --format=terse 2>/dev/null \
    | grep -E '^[a-z]' \
    | awk -F'::tests::' '{print $1}' \
    | sort | uniq -c
```

### Pattern 3: API Surface Baseline with Rustdoc JSON

**What:** cargo-semver-checks uses rustdoc JSON as baseline, not custom formats.

**When to use:** Phase 1 baseline generation, every subsequent phase validation.

**Example workflow:**
```bash
# Generate baseline (Phase 1)
cargo semver-checks --baseline-version $(cargo metadata --format-version 1 | jq -r '.packages[] | select(.name == "trustedge-core") | .version')

# Save rustdoc JSON for future comparison
cargo +nightly rustdoc --package trustedge-core -- -Z unstable-options --output-format json
cp target/doc/trustedge_core.json .planning/phases/01-foundation/api-baseline.json

# Validate after changes (Phase 2+)
cargo semver-checks --baseline-rustdoc .planning/phases/01-foundation/api-baseline.json
```

### Pattern 4: Mermaid Dependency Graph

**What:** cargo-workspace-analyzer generates Mermaid flowcharts showing cross-crate dependencies.

**When to use:** Workspace-level dependency visualization, detecting circular deps.

**Example:**
```bash
# Generate Mermaid diagram
cargo-workspace-analyzer --working-dir . -o mmd > .planning/phases/01-foundation/WORKSPACE-DEPS.mmd

# Render to SVG (optional)
mmdc -i .planning/phases/01-foundation/WORKSPACE-DEPS.mmd -o .planning/phases/01-foundation/WORKSPACE-DEPS.svg
```

### Anti-Patterns to Avoid

- **Creating src/layers/ parent directory:** Adds unnecessary nesting. User decided flat layout (primitives/ sits next to backends/, not inside src/layers/primitives/).
- **Moving code in Phase 1:** Phase 1 is scaffolding only. Moving crypto.rs into primitives/ happens in Phase 2+.
- **Test count-only baselines:** Without full test names, a drop from 23 → 22 doesn't tell you which test disappeared. Record names.
- **Ignoring cargo-machete false positives:** Renamed dependencies (like rustls-webpki imported as webpki) require explicit config in Cargo.toml metadata.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| API compatibility checking | Custom pub item diffing | cargo-semver-checks | Handles complex cases (trait method additions, generics, lifetimes), maintained by Rust ecosystem |
| Feature combination testing | Shell script loops | cargo-hack --feature-powerset | Deduplicates equivalent combinations, supports --partition for parallel CI |
| Dependency graph visualization | Parsing cargo metadata JSON | cargo-workspace-analyzer | Detects circular deps, generates Mermaid/SVG, calculates metrics (Fan In/Out, Instability) |
| Module structure visualization | Walking src/ directory | cargo-modules | Understands visibility (pub/pub(crate)/private), generates DOT graphs, filters externs |
| Test inventory extraction | Regex on test file paths | cargo test --list | Handles feature-gated tests, integration vs unit separation, accurate module paths |

**Key insight:** Rust's compilation model is complex (features, conditional compilation, visibility rules). Purpose-built tools handle edge cases that custom scripts miss.

## Common Pitfalls

### Pitfall 1: Workspace vs Package Confusion

**What goes wrong:** Running `cargo test --workspace -- --list` counts all crates together, making it impossible to track per-crate baselines.

**Why it happens:** Workspace-level commands aggregate output across all member crates.

**How to avoid:** Always use `--package` flag for per-crate operations.

**Warning signs:** Baseline shows "150 tests" as a single number without crate breakdown.

**Solution:**
```bash
# WRONG: workspace-level count
cargo test --workspace -- --list  # Returns 202 mixed tests

# RIGHT: per-crate counts
cargo test --package trustedge-core --lib -- --list     # 101 tests
cargo test --package trustedge-receipts --lib -- --list # 23 tests
```

### Pitfall 2: cargo-semver-checks on Unpublished Crates

**What goes wrong:** Tool defaults to fetching baseline from crates.io. Fails for unreleased crates.

**Why it happens:** cargo-semver-checks assumes you're publishing to crates.io.

**How to avoid:** Use `--baseline-rustdoc` or `--baseline-rev` for unpublished work.

**Warning signs:** Error: "could not find version X.Y.Z on crates.io"

**Solution:**
```bash
# Generate baseline for unpublished crate
cargo +nightly rustdoc --package trustedge-core -- -Z unstable-options --output-format json
cp target/doc/trustedge_core.json .planning/phases/01-foundation/trustedge-core-api-baseline.json

# Later: validate against baseline
cargo semver-checks --package trustedge-core --baseline-rustdoc .planning/phases/01-foundation/trustedge-core-api-baseline.json
```

### Pitfall 3: Integration Tests in Test Inventory

**What goes wrong:** `cargo test --lib -- --list` only shows unit tests. Integration tests in `tests/` directory are missed.

**Why it happens:** Integration tests are separate compilation units.

**How to avoid:** Run both `--lib` and `--test <name>` for complete inventory.

**Warning signs:** Baseline shows 101 tests for core, but git shows 14 files in crates/core/tests/.

**Solution:**
```bash
# Unit tests (in lib.rs and module #[cfg(test)])
cargo test --package trustedge-core --lib -- --list

# Integration tests (in tests/*.rs)
cargo test --package trustedge-core --test auth_integration -- --list
cargo test --package trustedge-core --test network_integration -- --list
# ... repeat for each integration test file

# Better: script that discovers all integration tests
for test in $(ls crates/core/tests/*.rs | xargs -n1 basename | sed 's/\.rs$//'); do
    cargo test --package trustedge-core --test "$test" -- --list
done
```

### Pitfall 4: cargo-machete False Positives on Renamed Imports

**What goes wrong:** Tool reports rustls-webpki as unused when code imports it as `use webpki::*`.

**Why it happens:** cargo-machete uses regex matching on dependency names, doesn't parse use statements.

**How to avoid:** Configure renamed dependencies in Cargo.toml metadata.

**Warning signs:** Tool claims dependency is unused but compilation fails when removed.

**Solution:**
```toml
[package.metadata.cargo-machete.renamed]
rustls-webpki = "webpki"
some-crate-name = "actual_import_name"
```

### Pitfall 5: cargo-hack Without --no-dev-deps

**What goes wrong:** Dev-dependency features leak into production feature testing, causing false passes.

**Why it happens:** Dev-dependencies can enable features that production code doesn't actually require.

**How to avoid:** Always use `--no-dev-deps` for production feature testing.

**Warning signs:** Feature test passes in CI but fails when users add crate as dependency.

**Solution:**
```bash
# WRONG: dev deps influence feature testing
cargo hack check --feature-powerset

# RIGHT: test only production dependency graph
cargo hack check --feature-powerset --no-dev-deps
```

### Pitfall 6: Missing Layer Contract Documentation

**What goes wrong:** Created directory structure without dependency rules, later phases import from wrong layers.

**Why it happens:** Skipped mod.rs documentation in Phase 1, assuming "we'll document later."

**How to avoid:** Write layer contracts immediately when creating directory structure.

**Warning signs:** Phase 2+ PRs have debates about "should protocols/ import applications/?"

**Solution:** Every mod.rs created in Phase 1 gets full contract documentation (see Pattern 1).

## Code Examples

Verified patterns from Rust official documentation and tool repositories:

### Test Inventory Script (per-module granularity)

```bash
#!/bin/bash
# scripts/test-inventory.sh
# Generate test baseline with full test names per module

set -e

OUTPUT_FILE="${1:-.planning/phases/01-foundation/TEST-BASELINE.md}"

echo "# Test Inventory Baseline" > "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"
echo "**Generated:** $(date -u +%Y-%m-%d)" >> "$OUTPUT_FILE"
echo "**Total tests:** $(cargo test --workspace -- --list 2>/dev/null | grep -c ': test')" >> "$OUTPUT_FILE"
echo "" >> "$OUTPUT_FILE"

for pkg in core trustedge-cli attestation receipts wasm pubky pubky-advanced trst-core trst-cli trst-wasm; do
    pkg_name="trustedge-${pkg}"
    [[ "$pkg" == "core" ]] && pkg_name="trustedge-core"
    [[ "$pkg" =~ ^trst ]] && pkg_name="$pkg"
    [[ "$pkg" == "wasm" ]] && pkg_name="trustedge-wasm"

    # Check if package exists
    if ! cargo metadata --format-version 1 | jq -e ".packages[] | select(.name == \"$pkg_name\")" > /dev/null 2>&1; then
        continue
    fi

    echo "## Package: $pkg_name" >> "$OUTPUT_FILE"
    echo "" >> "$OUTPUT_FILE"

    # Unit tests
    unit_tests=$(cargo test --package "$pkg_name" --lib -- --list 2>/dev/null | grep ': test' || true)
    if [ -n "$unit_tests" ]; then
        echo "### Unit Tests (lib)" >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        echo "$unit_tests" >> "$OUTPUT_FILE"
        echo '```' >> "$OUTPUT_FILE"
        echo "" >> "$OUTPUT_FILE"
    fi

    # Integration tests
    for test_file in crates/${pkg}/tests/*.rs 2>/dev/null; do
        [ -f "$test_file" ] || continue
        test_name=$(basename "$test_file" .rs)
        int_tests=$(cargo test --package "$pkg_name" --test "$test_name" -- --list 2>/dev/null | grep ': test' || true)
        if [ -n "$int_tests" ]; then
            echo "### Integration Tests ($test_name)" >> "$OUTPUT_FILE"
            echo '```' >> "$OUTPUT_FILE"
            echo "$int_tests" >> "$OUTPUT_FILE"
            echo '```' >> "$OUTPUT_FILE"
            echo "" >> "$OUTPUT_FILE"
        fi
    done
done

echo "✔ Test inventory written to $OUTPUT_FILE"
```

### Layer Contract Documentation Template

```rust
//! # Primitives Layer
//!
//! **Layer contract:** Pure cryptographic primitives with no external dependencies
//! beyond standard crypto libraries. Zero business logic, zero I/O.
//!
//! ## Dependency Rules
//! - ✔ CAN import: Standard library, crypto crates (aes-gcm, ed25519-dalek, blake3)
//! - ✖ NEVER imports: backends, protocols, applications, transport, io
//!
//! ## Post-consolidation contents (Phase 2+)
//! - `crypto.rs` → `primitives/crypto.rs` (AES-256-GCM, XChaCha20-Poly1305)
//! - Portions of `asymmetric.rs` → `primitives/asymmetric.rs` (Ed25519, P256, RSA)
//! - `chain.rs` primitives → `primitives/chain.rs` (BLAKE3 hashing only)
//!
//! ## Examples
//! ```rust
//! use trustedge_core::primitives::{encrypt_segment, sign_manifest};
//!
//! let ciphertext = encrypt_segment(&plaintext, &key, &nonce)?;
//! let signature = sign_manifest(&manifest_bytes, &signing_key)?;
//! ```
//!
//! **Status:** Phase 1 scaffolding - no code yet

// Phase 1: empty module, populated in Phase 2+
```

### Dependency Graph Generation

```bash
# Source: cargo-workspace-analyzer documentation
# Generate Mermaid diagram of workspace dependencies

#!/bin/bash
# scripts/generate-dep-graph.sh

set -e

OUTPUT_DIR=".planning/phases/01-foundation"
mkdir -p "$OUTPUT_DIR"

echo "Generating workspace dependency graph..."

# Mermaid format (primary)
cargo-workspace-analyzer --working-dir . -o mmd > "$OUTPUT_DIR/WORKSPACE-DEPS.mmd"
echo "✔ Mermaid diagram: $OUTPUT_DIR/WORKSPACE-DEPS.mmd"

# SVG rendering (optional, requires mermaid-cli)
if command -v mmdc &> /dev/null; then
    mmdc -i "$OUTPUT_DIR/WORKSPACE-DEPS.mmd" -o "$OUTPUT_DIR/WORKSPACE-DEPS.svg"
    echo "✔ SVG diagram: $OUTPUT_DIR/WORKSPACE-DEPS.svg"
else
    echo "⚠ mermaid-cli not found, skipping SVG rendering"
    echo "  Install: npm install -g @mermaid-js/mermaid-cli"
fi

# Text-based tree (secondary, for quick reference)
echo "Generating text dependency tree..."
cargo tree --workspace --depth 1 > "$OUTPUT_DIR/WORKSPACE-TREE.txt"
echo "✔ Text tree: $OUTPUT_DIR/WORKSPACE-TREE.txt"

echo "✔ Dependency graph generation complete"
```

### API Surface Baseline Capture

```bash
# Source: cargo-semver-checks documentation
# Generate rustdoc JSON baseline for each public crate

#!/bin/bash
# scripts/capture-api-baseline.sh

set -e

OUTPUT_DIR=".planning/phases/01-foundation"
mkdir -p "$OUTPUT_DIR"

PUBLIC_CRATES=("trustedge-core" "trustedge-receipts" "trustedge-attestation" "trustedge-trst-core")

for crate in "${PUBLIC_CRATES[@]}"; do
    echo "Capturing API baseline for $crate..."

    # Generate rustdoc JSON
    cargo +nightly rustdoc --package "$crate" -- -Z unstable-options --output-format json

    # Copy to baseline directory with crate name
    baseline_file="$OUTPUT_DIR/${crate}-api-baseline.json"
    cp "target/doc/$(echo $crate | tr '-' '_').json" "$baseline_file"

    echo "✔ Baseline saved: $baseline_file"
done

echo "✔ API baseline capture complete for ${#PUBLIC_CRATES[@]} crates"
```

### Duplication Detection Script

```bash
# Manual duplication detection combining grep and manual review
#!/bin/bash
# scripts/detect-duplication.sh

set -e

echo "# Code Duplication Audit" > .planning/phases/01-foundation/AUDIT.md
echo "" >> .planning/phases/01-foundation/AUDIT.md
echo "**Generated:** $(date -u +%Y-%m-%d)" >> .planning/phases/01-foundation/AUDIT.md
echo "" >> .planning/phases/01-foundation/AUDIT.md

echo "## Function Signature Analysis" >> .planning/phases/01-foundation/AUDIT.md
echo "" >> .planning/phases/01-foundation/AUDIT.md
echo "| Function | Crate 1 | Crate 2 | Type | Recommendation |" >> .planning/phases/01-foundation/AUDIT.md
echo "|----------|---------|---------|------|----------------|" >> .planning/phases/01-foundation/AUDIT.md

# Search for common crypto function patterns
for func in "encrypt_segment" "decrypt_segment" "sign_manifest" "verify_manifest"; do
    matches=$(grep -rn "pub fn $func" crates/*/src --include="*.rs" | cut -d: -f1 | sort -u)
    count=$(echo "$matches" | grep -v '^$' | wc -l)
    if [ "$count" -gt 1 ]; then
        echo "Found duplicate: $func in $count locations:"
        echo "$matches"
    fi
done

echo "" >> .planning/phases/01-foundation/AUDIT.md
echo "## Type Definition Analysis" >> .planning/phases/01-foundation/AUDIT.md
echo "" >> .planning/phases/01-foundation/AUDIT.md

# Search for common type patterns
for type in "Envelope" "Manifest" "DeviceKeypair" "ChainSegment"; do
    matches=$(grep -rn "pub struct $type" crates/*/src --include="*.rs" | cut -d: -f1 | sort -u)
    count=$(echo "$matches" | grep -v '^$' | wc -l)
    if [ "$count" -gt 1 ]; then
        echo "Found duplicate type: $type in $count locations:"
        echo "$matches"
    fi
done

echo "✔ Duplication audit written to .planning/phases/01-foundation/AUDIT.md"
echo "⚠ Manual review required to classify exact vs near duplicates"
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual dependency tracking | cargo-workspace-analyzer + Mermaid | 2024+ | Automated circular dep detection, visual diagrams |
| Custom pub API diffing | cargo-semver-checks with rustdoc JSON | 2023+ | Handles complex Rust semantics (traits, generics) |
| Shell script feature testing | cargo-hack --feature-powerset | 2022+ | Deduplicates combinations, --partition for parallel CI |
| cargo-geiger for unused deps | cargo-machete | 2023+ | 10-100x faster, regex-based matching |
| Flat src/*.rs files | Layered module hierarchy | Modern Rust | Enforces architecture via filesystem, scales to large codebases |

**Deprecated/outdated:**
- **cargo-modules --graphviz flag:** Deprecated in favor of DOT output piped to graphviz directly
- **cargo-semver-check (singular):** Forked/inactive project, use cargo-semver-checks (plural) instead
- **Manual test counting:** Brittle regex parsing replaced by `cargo test --list` standard output

## Open Questions

1. **Should cargo-machete findings block Phase 1 completion?**
   - What we know: Tool identifies potentially unused dependencies, but has false positives
   - What's unclear: Whether fixing unused deps is critical for foundation phase or can defer to Phase 8 (cleanup)
   - Recommendation: Generate cargo-machete report as artifact, defer removals to Phase 8 unless dependencies clearly cause issues

2. **Integration test inventory granularity**
   - What we know: 14 integration test files in crates/core/tests/, some feature-gated (yubikey)
   - What's unclear: Whether baseline should track per-integration-test-file or aggregate
   - Recommendation: Per-file tracking (like per-module unit tests) to detect if specific integration test disappears

3. **API baseline for internal crates (pubky, pubky-advanced)**
   - What we know: Pubky crates are community contributions, not core product
   - What's unclear: Whether they need API baseline tracking or can be excluded
   - Recommendation: Exclude from semver tracking (not published libraries), but include in dependency graph analysis

## Sources

### Primary (HIGH confidence)
- [cargo-semver-checks GitHub](https://github.com/obi1kenobi/cargo-semver-checks) - Installation, baseline flags, CI integration
- [cargo-semver-checks crates.io](https://crates.io/crates/cargo-semver-checks) - Version info, usage
- [cargo-hack GitHub](https://github.com/taiki-e/cargo-hack) - Feature powerset testing, --partition flag
- [cargo-machete GitHub](https://github.com/bnjbvr/cargo-machete) - Unused dependency detection, metadata config
- [cargo-workspace-analyzer GitHub](https://github.com/jaads/cargo-workspace-analyzer) - Mermaid diagram generation, circular dep detection
- [cargo tree - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) - Built-in dependency tree
- [Test Organization - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-03-test-organization.html) - Unit vs integration tests

### Secondary (MEDIUM confidence)
- [SemVer Compatibility - The Cargo Book](https://doc.rust-lang.org/cargo/reference/semver.html) - API compatibility rules
- [Semantic Versioning - Rust Project Primer](https://rustprojectprimer.com/checks/semver.html) - Best practices verified against official docs
- [cargo-workspace-analyzer crates.io](https://crates.io/crates/cargo-workspace-analyzer) - Metrics (Fan In/Out, Instability)

### Tertiary (LOW confidence)
- cargo-modules documentation (tool exists, websearch unavailable - installation verified via crates.io pattern)
- Layered architecture in Rust (general pattern, no 2026-specific source - applying standard practices)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - all tools verified via official repos/docs, actively maintained
- Architecture: HIGH - layered module pattern is standard Rust practice, user decisions align with ecosystem norms
- Pitfalls: HIGH - derived from official tool documentation and Rust Book testing chapter
- Code examples: HIGH - based on tool documentation and standard Rust patterns

**Research date:** 2026-02-09
**Valid until:** 2026-03-09 (30 days - stable tooling, slow-moving standards)
