---
phase: quick
plan: 01
type: execute
wave: 1
depends_on: []
files_modified:
  - SECURITY.md
  - CONTRIBUTING.md
  - docs/README.md
  - docs/architecture.md
  - docs/developer/development.md
  - docs/developer/testing.md
  - docs/technical/format.md
  - docs/developer/coding-standards.md
autonomous: true
requirements: [DOC-UPDATE]

must_haves:
  truths:
    - "All version references say v2.4, not v1.0 or v1.7"
    - "Test counts say 406, not 109 or 144 or 270"
    - "Crate references match current workspace (no trustedge-receipts, includes types/platform/trst-cli)"
    - "Git clone URLs reference TrustEdge-Labs org"
    - "No references to non-existent doc files (WASM.md, claude.md)"
  artifacts:
    - path: "SECURITY.md"
      provides: "Current security posture"
    - path: "CONTRIBUTING.md"
      provides: "Current contributor guide"
    - path: "docs/developer/development.md"
      provides: "Current development guide"
    - path: "docs/developer/testing.md"
      provides: "Current test reference"
    - path: "docs/architecture.md"
      provides: "Current architecture overview"
    - path: "docs/README.md"
      provides: "Current docs index"
    - path: "docs/technical/format.md"
      provides: "Current format spec"
  key_links: []
---

<objective>
Update all outdated markdown documentation files to reflect TrustEdge v2.4 state.

Purpose: Documentation references v1.0-v1.7 era state (test counts, crate names, version numbers, security features) and needs to reflect v2.4 with 406 tests, 9 crates, encrypted keys, HKDF-SHA256, and TrustEdge-Labs org.
Output: 8 updated markdown files with current, accurate information.
</objective>

<execution_context>
@~/.claude/get-shit-done/workflows/execute-plan.md
@~/.claude/get-shit-done/templates/summary.md
</execution_context>

<context>
@CLAUDE.md
@SECURITY.md
@CONTRIBUTING.md
@docs/README.md
@docs/architecture.md
@docs/developer/development.md
@docs/developer/testing.md
@docs/technical/format.md
@docs/developer/coding-standards.md
</context>

<tasks>

<task type="auto">
  <name>Task 1: Update root-level docs (SECURITY.md, CONTRIBUTING.md)</name>
  <files>SECURITY.md, CONTRIBUTING.md</files>
  <action>
Read both files fully, then update:

**SECURITY.md** (228 lines):
- Version table: change current version from v1.7.x to v2.4
- Section header "Current Security Status (v1.7)" -> "Current Security Status (v2.4)"
- Add to security features: HKDF-SHA256 key derivation (v1.8+), RSA OAEP-SHA256 (v2.2+), encrypted keys at rest via TRUSTEDGE-KEY-V1 format with PBKDF2-HMAC-SHA256 600k iterations + AES-256-GCM (v2.2+)
- Update key derivation description to mention both PBKDF2 (for key files) and HKDF-SHA256 (for envelope KDF)
- Update test count references to 406 tests across 9 workspace crates, including 45+ dedicated security tests (v2.3-v2.4)
- "Next Audit: Planned for post-v1.0" -> "Previous audit: v2.4 Security Review Remediation completed March 2026"
- Add mention of sensor, audio, and log archive profiles alongside cam.video
- Last Updated date -> March 2026

**CONTRIBUTING.md** (376 lines):
- "Current Status: v1.0 released (February 2026)" -> "Current Status: v2.4 released (March 2026)"
- Remove or update Issue #16 reference (project tracking is now via GSD workflow, not GitHub issues)
- Clone URL: change `yourusername/trustedge` to `TrustEdge-Labs/trustedge`
- Verify test commands match CLAUDE.md (cargo test --workspace, etc.)
  </action>
  <verify>
    <automated>grep -n "v1\.0\|v1\.7\|yourusername\|109 \|144 \|270 " SECURITY.md CONTRIBUTING.md | grep -v "v1\.0 through\|v1\.0-v2\|since v1\|TRUSTEDGE-KEY-V1" | head -20; echo "Exit: $?"</automated>
  </verify>
  <done>SECURITY.md reflects v2.4 with all current security features; CONTRIBUTING.md references v2.4 and TrustEdge-Labs org; no stale version or count references remain</done>
</task>

<task type="auto">
  <name>Task 2: Update docs/ directory files (development, testing, architecture, format, README, coding-standards)</name>
  <files>docs/developer/development.md, docs/developer/testing.md, docs/architecture.md, docs/technical/format.md, docs/README.md, docs/developer/coding-standards.md</files>
  <action>
Read all 6 files fully, then update:

**docs/developer/development.md** (696 lines):
- Remove or replace the entire "Roadmap" section with Phases 1-6 checkboxes. Replace with a brief note: "See CLAUDE.md for current build/test commands. TrustEdge has shipped through v2.4 with 53 phases of development."
- "144 Total Tests" -> "406 Total Tests" (or remove specific count table and reference `cargo test --workspace` for live count)
- Update per-crate test breakdowns: trustedge-types (18), trustedge-core (160+), trustedge-platform (19+), trustedge-trst-cli (7), plus others. Match CLAUDE.md test commands.
- Remove all references to `trustedge-receipts` crate (merged into core in v1.5)
- Clone URL: `yourusername/trustedge` -> `TrustEdge-Labs/trustedge`
- `cd trustedge/trustedge-core` -> `cd trustedge` (workspace-level)
- Current Backends: add Software HSM and YubiKey alongside Keyring
- Project structure: update to match current workspace layout from CLAUDE.md (9 crates: types, core, platform, platform-server, cli, wasm, trst-protocols, trst-cli, trst-wasm, plus web/dashboard)
- Fix `.ci-check.sh` -> `./scripts/ci-check.sh`
- "Phase 3: Network Operations (Current)" -> remove phase status markers, all complete

**docs/developer/testing.md** (1252 lines):
- "109 Total Tests" -> "406 Total Tests"
- Update all per-crate breakdowns to match current state
- Remove `trustedge-receipts` test category
- Add test categories for: trustedge-types, trustedge-platform, trustedge-trst-cli, trustedge-trst-wasm, trustedge-wasm
- Reference the test commands from CLAUDE.md for accuracy

**docs/architecture.md** (217 lines):
- "270+ automated tests" -> "406 automated tests"
- Update test breakdown numbers to match current state
- Add encrypted key format: TRUSTEDGE-KEY-V1 (PBKDF2-HMAC-SHA256 600k iterations + AES-256-GCM)
- Working with Archives section: add --device-key and --device-pub flags to example commands (match CLAUDE.md archive examples)

**docs/technical/format.md** (451 lines):
- Change "Status: Draft" -> "Status: Stable"
- Update date from "September 6, 2025" to "March 2026"
- Add v2.0 entry to version history table: "2.0 | February 2026 | HKDF-SHA256 KDF, versioned envelope format, encrypted key files"
- Fix duplicate "Section 9" numbering — renumber the second occurrence to Section 10 and adjust subsequent sections

**docs/README.md** (242 lines):
- Remove links to non-existent files: WASM.md, claude.md (these do not exist in docs/)
- If listing doc count/metrics, update or remove (counts have changed)
- Verify all other doc links point to files that actually exist

**docs/developer/coding-standards.md** (403 lines):
- Update module structure references to match current workspace layout if any are outdated
- Light touch — this file is noted as "generally OK"
  </action>
  <verify>
    <automated>grep -rn "trustedge-receipts\|109 Total\|144 Total\|270.\|yourusername\|Phase 3.*Current\|Status: Draft" docs/ | head -20; echo "---"; grep -l "WASM.md\|claude.md" docs/README.md 2>/dev/null; echo "Exit: $?"</automated>
  </verify>
  <done>All docs/ files reflect v2.4 state: correct test counts (406), current crate names (no trustedge-receipts), TrustEdge-Labs org URLs, current project structure, no broken doc links, format.md is Stable status</done>
</task>

</tasks>

<verification>
Run across all updated files:
```bash
# No stale version references (excluding legitimate historical mentions)
grep -rn "v1\.7\|v1\.0 released\|Current.*v1\." SECURITY.md CONTRIBUTING.md docs/ | grep -v "shipped\|through\|since\|history\|TRUSTEDGE-KEY-V1"

# No stale crate references
grep -rn "trustedge-receipts" SECURITY.md CONTRIBUTING.md docs/

# No stale test counts
grep -rn "109 Total\|144 Total\|86 Core\|23 Receipt" SECURITY.md CONTRIBUTING.md docs/

# No broken clone URLs
grep -rn "yourusername" SECURITY.md CONTRIBUTING.md docs/
```
All should return empty (exit 1).
</verification>

<success_criteria>
- All 8 markdown files updated to reflect TrustEdge v2.4 state
- Version references: v2.4 (not v1.0/v1.7)
- Test count: 406 (not 109/144/270)
- Crate references: current 9-crate workspace (no trustedge-receipts)
- Org references: TrustEdge-Labs (not yourusername)
- No links to non-existent doc files
- format.md status is Stable with correct section numbering
</success_criteria>

<output>
After completion, create `.planning/quick/260322-jgi-review-and-update-out-of-date-markdown-d/260322-jgi-SUMMARY.md`
</output>
