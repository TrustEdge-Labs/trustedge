<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# Documentation Audit Report

**Date**: October 19, 2025  
**Auditor**: AI Assistant  
**Scope**: All markdown documentation in trustedge repository

## Executive Summary

Found **5 major categories** of documentation issues requiring attention:

1. **WASM Documentation Duplication** - 5 files covering similar topics
2. **Outdated Command Examples** - Pre-workspace commands in 15+ files
3. **Installation Guide Duplication** - 3 separate installation guides
4. **Missing Cross-References** - New docs not linked from existing docs
5. **Archive Files** - Old release notes and drafts in main repo

---

## 🔴 Critical Issues (Breaking/Confusing)

### 1. Outdated CLI Commands in Multiple Files

**Problem**: Many docs use old `trustedge-core --input` syntax instead of workspace-aware commands.

**Affected Files**:
- `docs/user/examples/getting-started.md` - Lines 15-84 (uses `./target/release/trustedge-core`)
- `docs/user/examples/installation.md` - Line 79 (uses `./target/release/trustedge-core`)
- `docs/user/cli.md` - Multiple instances (205, 212, 215, 285, 349)
- `README.md` - Line 159-162 (partially correct, but mixed)

**Current State (INCORRECT)**:
```bash
./target/release/trustedge-core --input document.txt --envelope document.trst --key-out mykey.hex
```

**Should Be (CORRECT)**:
```bash
cargo run -p trustedge-core -- --input document.txt --envelope document.trst --key-out mykey.hex
# OR for installed binary:
./target/release/trustedge-core --input document.txt --envelope document.trst --key-out mykey.hex
```

**Impact**: Medium - Commands may work but don't follow workspace conventions.

**Recommendation**: 
- Update all examples to use `cargo run -p trustedge-core` for development
- Show `./target/release/trustedge-core` as alternative for installed binaries
- Add note about workspace organization

---

### 2. Archive CLI Command Inconsistency

**Problem**: Some docs use `--bin trst` (old) vs `cargo run -p trustedge-trst-cli` (current).

**Affected Files**:
- `archive/POST_RELEASE_CHECKLIST_0.2.0.md` - Line 29
- Various older docs in `archive/`

**Current State**:
```bash
cargo run --bin trst -- wrap ...  # OLD (doesn't specify package)
```

**Should Be**:
```bash
cargo run -p trustedge-trst-cli -- wrap ...  # NEW (workspace-aware)
```

**Impact**: Low - Archive folder is for historical docs, but could confuse.

**Recommendation**: Add note at top of archive/ README explaining these are historical.

---

## 🟡 Medium Issues (Duplication/Redundancy)

### 3. WASM Documentation Overlap

**Problem**: 5 separate documents cover WASM building with partial overlap.

**Files**:
1. **`WASM.md`** (NEW - 400 lines) - Comprehensive, authoritative ✅
2. **`docs/developer/wasm-testing.md`** (402 lines) - Testing-focused, browser tests
3. **`web/demo/README.md`** (174 lines) - Demo-specific, quick start
4. **`crates/wasm/README.md`** (273 lines) - Core WASM package API
5. **`crates/trst-wasm/README.md`** (273 lines) - Archive verification WASM API

**Analysis**:
- `WASM.md` is the new comprehensive guide (keep as primary)
- `wasm-testing.md` has unique browser testing content (keep, cross-reference)
- `web/demo/README.md` is demo-specific (keep, simplify, cross-reference)
- `crates/wasm/README.md` is package-specific API (keep, cross-reference)
- `crates/trst-wasm/README.md` is package-specific API (keep, cross-reference)

**Overlap Examples**:
- All 5 mention `wasm-pack build` command
- 3 mention installation of `wasm-pack`
- 2 have identical "Quick Start" sections

**Recommendation**:
- **Keep all files** but establish hierarchy
- `WASM.md` = Comprehensive guide (primary)
- Others = Package-specific details + link to `WASM.md`
- Add cross-references in each file

---

### 4. Installation Guide Duplication

**Problem**: 3 separate installation guides with overlapping content.

**Files**:
1. **`README.md`** - Section "Installation" (lines 140-150)
2. **`docs/user/examples/installation.md`** - Complete guide (101 lines)
3. **`docs/user/examples/getting-started.md`** - Includes installation (196 lines)

**Overlap**:
- All 3 explain audio dependencies
- All 3 show YubiKey setup
- Similar verification commands

**Recommendation**:
- **`README.md`**: Quick install (3-5 commands) + link to full guide
- **`installation.md`**: Authoritative, complete installation guide
- **`getting-started.md`**: Link to `installation.md`, focus on usage examples

---

### 5. Missing Cross-References to New Documentation

**Problem**: New comprehensive docs (`FEATURES.md`, `WASM.md`, `RFC_K256_SUPPORT.md`) not linked from existing docs.

**New Files Created**:
- `FEATURES.md` - Comprehensive feature flag reference
- `WASM.md` - Complete WASM build/deploy guide  
- `RFC_K256_SUPPORT.md` - secp256k1 implementation plan

**Missing Links From**:
- `README.md` - Should link to FEATURES.md, WASM.md in relevant sections
- `crates/*/README.md` - Should link to FEATURES.md for feature flag info
- `docs/developer/wasm-testing.md` - Should link to WASM.md
- `web/demo/README.md` - Should link to WASM.md

**Recommendation**: Add "See also" sections with proper cross-references.

---

## 🟢 Low Issues (Minor/Cosmetic)

### 6. Development Guide Has Pre-Workspace Commands

**File**: `docs/developer/development.md`

**Problem**: Line 155 shows outdated test commands without `-p` flags.

**Current**:
```bash
cargo test --lib                 # Unit tests (79)
```

**Should Reference**:
```bash
cargo test -p trustedge-core --lib                 # Core unit tests
cargo test --workspace --all-features              # All workspace tests
```

**Impact**: Low - File is for developers who understand cargo.

**Recommendation**: Update commands, add workspace context.

---

### 7. Archive Folder Contains Draft/Old Content

**Files in `archive/`**:
- `bolt-new-prompt.md` - Draft prompt for AI
- `linkedin-release-post.md` - Social media post
- `POST_RELEASE_CHECKLIST_0.2.0.md` - Release checklist
- `RELEASE_NOTES_0.2.0.md` - Old release notes
- `ROADMAP_OLD.md` - Superseded roadmap

**Problem**: These clutter searches and could confuse contributors.

**Recommendation**: 
- Add `archive/README.md` explaining these are historical
- Consider moving to `.archive/` (hidden) or deleting
- CHANGELOG.md already has release notes

---

## 📋 Recommended Action Plan

### Phase 1: Critical Fixes (Today)

1. ✅ **Update CLI command examples** in:
   - `docs/user/examples/getting-started.md`
   - `docs/user/examples/installation.md`
   - `docs/user/cli.md`
   - Add workspace context notes

2. ✅ **Add cross-references** to new docs:
   - `README.md` → link to `FEATURES.md`, `WASM.md`
   - `crates/*/README.md` → link to `FEATURES.md`
   - WASM-related docs → link to `WASM.md`

3. ✅ **Create WASM documentation hierarchy**:
   - Add "See also" sections to all 5 WASM docs
   - Establish `WASM.md` as primary reference

### Phase 2: Medium Priority (This Week)

4. **Consolidate installation guides**:
   - Simplify `README.md` installation
   - Enhance `docs/user/examples/installation.md`
   - Update `getting-started.md` to focus on examples

5. **Add archive/ README**:
   - Explain historical nature of content
   - Link to current versions

### Phase 3: Low Priority (Next Sprint)

6. **Update development guide**:
   - Modernize `docs/developer/development.md`
   - Add workspace architecture section

7. **Review all crate READMEs**:
   - Ensure consistency
   - Add feature flag references

---

## 📊 Documentation Inventory

### Root Level (8 files)
- `README.md` - Main project README ✅
- `FEATURES.md` - **NEW** Feature flags guide ✅
- `WASM.md` - **NEW** WASM build guide ✅
- `RFC_K256_SUPPORT.md` - **NEW** K1 curve RFC ✅
- `P0_IMPLEMENTATION.md` - P0 completion status ✅
- `SECURITY.md` - Security policy ✅
- `CONTRIBUTING.md` - Contribution guidelines ✅
- `CHANGELOG.md` - Version history ✅

### docs/ Directory (30+ files)
- `docs/README.md` - Documentation index
- `docs/roadmap.md` - Development roadmap
- `docs/developer/` - 7 files (development, testing, WASM, etc.)
- `docs/user/` - 10+ files (CLI, authentication, troubleshooting)
- `docs/technical/` - 4 files (protocol, format, backend, threat model)
- `docs/legal/` - 4 files (licensing, CLA, DCO, copyright)

### Crate READMEs (9 files)
- `crates/core/README.md`
- `crates/trst-cli/` - (no README)
- `crates/trst-core/` - (no README)
- `crates/attestation/README.md`
- `crates/receipts/README.md`
- `crates/wasm/README.md`
- `crates/trst-wasm/README.md`
- `crates/pubky/README.md`
- `crates/pubky-advanced/README.md`

### Other (5+ files)
- `.github/copilot-instructions.md` - **UPDATED** AI guide ✅
- `.github/README.md` - GitHub-specific docs
- `examples/cam.video/README.md` - P0 demo
- `web/demo/README.md` - WASM demo
- `scripts/README.md` - Script documentation

**Total**: ~65 markdown files

---

## 🎯 Quality Metrics

### Documentation Coverage
- **Excellent**: FEATURES.md, WASM.md, RFC_K256_SUPPORT.md (new)
- **Good**: README.md, SECURITY.md, copilot-instructions.md
- **Needs Update**: CLI docs, installation guides, development guide
- **Historical**: archive/ folder (7 files)

### Cross-Reference Density
- **Strong**: Copilot instructions → other docs
- **Weak**: Crate READMEs → feature flags
- **Missing**: New docs ← existing docs

### Command Accuracy
- **Current**: New docs (FEATURES.md, WASM.md, copilot-instructions.md)
- **Mixed**: README.md (some old, some new)
- **Outdated**: docs/user/ examples (pre-workspace)

---

## ✅ Verification Checklist

After implementing fixes, verify:

- [ ] All `cargo run` commands use `-p package-name` format
- [ ] All `cargo build` commands specify package where applicable
- [ ] All `cargo test` commands use workspace-aware syntax
- [ ] FEATURES.md is linked from README.md and relevant crates
- [ ] WASM.md is referenced by all WASM-related docs
- [ ] Installation guide is primary source (not duplicated)
- [ ] Archive folder has explanatory README
- [ ] All new docs have copyright headers
- [ ] Cross-references work (no broken links)

---

## 📝 Notes for Future

### Documentation Principles Established
1. **Single Source of Truth**: Each topic has one authoritative doc
2. **Cross-Reference Liberally**: Link to details, don't duplicate
3. **Workspace-Aware**: Always use `-p` flags for cargo commands
4. **Feature Flag Clarity**: Reference FEATURES.md for all feature info
5. **WASM Hierarchy**: WASM.md is comprehensive, others are specific

### Maintenance Guidelines
- Update FEATURES.md when adding new cargo features
- Update WASM.md when changing build process
- Keep copilot-instructions.md aligned with main docs
- Archive old release-specific docs after each version

---

For implementation details of these fixes, see the following sections of this report.
