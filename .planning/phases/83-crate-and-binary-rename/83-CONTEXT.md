# Phase 83: Crate & Binary Rename - Context

**Gathered:** 2026-04-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Rename every Cargo package, every binary target, every crate library name, and the archive file extension across the sealedge workspace so the workspace presents as sealedge end-to-end. Clean break — no aliases, no backward-compat re-exports.

**In scope:**
- 9 root-workspace crates (`trustedge-*` and `trustedge-trst-*` → `sealedge-*` and `sealedge-seal-*`)
- 5 binary targets (`trustedge`, `trst`, `trustedge-server`, `trustedge-client`, `trustedge-platform-server`)
- Inter-crate dependency references in every `Cargo.toml`
- Library module names (derived from package name via hyphen→underscore)
- Dashboard `package.json` name field
- Experimental crates in `crates/experimental/` (separate workspace)
- `.trst` archive file extension across archive read/write and tests
- Workspace `cargo build --workspace --release`, `cargo check --workspace`, `cargo test --workspace` all green

**Out of scope (Phase 83 — handled elsewhere):**
- Crypto wire-format constants (`TRUSTEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1`) → Phase 84
- `.te-attestation.json` file extension → Phase 84
- Copyright headers, error messages, log output, CLI help text → Phase 85
- Env var prefixes (`TRUSTEDGE_*` → `SEALEDGE_*`) → Phase 85
- Cargo.toml metadata URLs (`repository`, `homepage`, `documentation`) → Phase 85
- Doc comments (`///`, `//!`) and prose docs → Phase 86
- GitHub repo rename → Phase 87
- Dashboard directory rename (`web/dashboard/` → `web/sealedge-dashboard/`) — out of scope entirely, no reason to move it

</domain>

<decisions>
## Implementation Decisions

### Binary Naming
- **D-01:** `trst` binary renamed to **`seal`** — 4 chars, natural English verb, strongest brand fit. Drives the package-name pattern for the former `trustedge-trst-*` crates (see D-03).
- **Mechanical renames** (no discussion needed — direct substitution):
  - `trustedge` → `sealedge`
  - `trustedge-server` → `sealedge-server`
  - `trustedge-client` → `sealedge-client`
  - `trustedge-platform-server` → `sealedge-platform-server`

### Archive File Extension
- **D-02:** `.trst` archive extension renamed to **`.seal`** — parallels the new `seal` binary (`seal wrap input.bin → input.seal`). Clean break, no reader for legacy `.trst` files.
- ⚠ **Requirement gap:** REBRAND-04 currently names only `.te-attestation.json`. Before executing Phase 83, REBRAND-04 must be amended (or a REBRAND-04b added) to also cover `.trst` → `.seal`. Flag this for the planner.

### Crate Package Naming
- **D-03:** Double-prefix crates use direct parallel — mechanical, zero restructure:
  - `trustedge-trst-protocols` → `sealedge-seal-protocols`
  - `trustedge-trst-cli` → `sealedge-seal-cli`
  - `trustedge-trst-wasm` → `sealedge-seal-wasm`
- **Simple renames** (no discussion needed — direct substitution):
  - `trustedge-core` → `sealedge-core`
  - `trustedge-types` → `sealedge-types`
  - `trustedge-platform` → `sealedge-platform`
  - `trustedge-platform-server` → `sealedge-platform-server`
  - `trustedge-cli` → `sealedge-cli`
  - `trustedge-wasm` → `sealedge-wasm`

### Scope Edges
- **D-04:** Dashboard `web/dashboard/package.json` name field renamed `trustedge-dashboard` → `sealedge-dashboard`. The `web/dashboard/` **directory itself stays put** — no import-path churn, no CI workflow rewrites.
- **D-05:** Experimental crates renamed too (separate workspace at `crates/experimental/`) — for full monorepo consistency:
  - `trustedge-pubky` → `sealedge-pubky`
  - `trustedge-pubky-advanced` → `sealedge-pubky-advanced`
  - Small blast radius — separate workspace, isolated deps, not in root CI. Still commit + verify under new names.

### Claude's Discretion
- **Commit granularity:** Single atomic workspace commit per logical step (e.g., "rename all crate packages + inter-crate deps in one commit"). The workspace must not be in a half-renamed state between commits — `cargo check --workspace` green at every commit boundary. Planner may split into 3-5 atomic commits (e.g., root crates → trst crates → dashboard → experimental → `.trst` extension) if each step remains workspace-coherent.
- **Library module names:** Derive automatically from new package names via Rust's hyphen→underscore convention:
  - `sealedge-core` → `use sealedge_core::*`
  - `sealedge-seal-protocols` → `use sealedge_seal_protocols::*`
  - All `use trustedge_*` statements across the codebase must be updated in lock-step with the package rename.
- **Crate directory naming:** Normalize the outlier `crates/trustedge-cli/` directory → `crates/cli/` (aligning with the other short dirs: `core/`, `types/`, `platform/`, `wasm/`). The `trst-cli`, `trst-protocols`, `trst-wasm` directories also rename to `seal-cli/`, `seal-protocols/`, `seal-wasm/` to match the new package names — keeps directory and package names aligned for developer ergonomics.
- **`.trst` extension search surface:** The extension literal appears in archive.rs, hybrid.rs, io/mod.rs, vectors.rs, inspect-trst.rs, trst-cli/main.rs + all trst-cli tests, and example files. A ripgrep sweep for `\.trst\b` is the starting point; planner confirms exhaustive coverage.
- **Binary target name in Cargo.toml:** Under `[[bin]]` sections, `name = "trst"` → `name = "seal"`, etc. The package-level `name = "trustedge-trst-cli"` is a separate field that also renames (D-03).

### Folded Todos
None.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` — v6.0 requirements (REBRAND-01, REBRAND-02 assigned to Phase 83; REBRAND-04 needs amendment to cover `.trst` rename, see ⚠ flag under D-02)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target features and clean-break scope
- `.planning/ROADMAP.md` §"Phase 83: Crate & Binary Rename" — goal, depends-on, success criteria

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — full rebrand scope, clean-break rationale, reserved phase numbers
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — user's clean-break preference for breaking changes (no aliases, no fallbacks)

### Code surface (starting points for the planner's grep sweep)
- `Cargo.toml` (root) — workspace members + workspace deps declaration
- `crates/*/Cargo.toml` (9 files) — package name, inter-crate deps, `[[bin]]` target names
- `crates/experimental/Cargo.toml` — separate workspace root
- `crates/experimental/pubky/Cargo.toml`, `crates/experimental/pubky-advanced/Cargo.toml`
- `web/dashboard/package.json` — `name` field
- `crates/core/src/archive.rs`, `crates/core/src/hybrid.rs`, `crates/core/src/io/mod.rs`, `crates/core/src/vectors.rs`, `crates/core/src/bin/inspect-trst.rs` — `.trst` extension literals
- `crates/trst-cli/src/main.rs`, `crates/trst-cli/tests/*.rs` — `.trst` extension literals, binary name references
- `crates/core/examples/attest.rs`, `crates/core/examples/verify_attestation.rs` — `.trst` references

### Not yet read (planner should consult)
- `scripts/*.sh` — demo scripts invoke binary names and file extensions; must stay in lock-step with renamed binaries (though prose/comment sweep is Phase 86, the invocation targets matter here)
- `.github/workflows/*.yml` — CI workflows may reference crate names (e.g., `cargo test -p trustedge-core`); rename alongside package rename to keep CI green

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Cargo workspace structure** — all 9 crates already participate in a single `[workspace]` at repo root with workspace-level dependency management. Rename propagates through `workspace.dependencies` cleanly if present.
- **Hyphen→underscore Rust convention** — mechanical and deterministic; `sed`-able with care.

### Established Patterns
- **Short crate dir names** (`core/`, `types/`, `platform/`, `wasm/`) vs outlier `trustedge-cli/`. The rename is a natural moment to normalize — aligns directory and package naming (see Claude's Discretion).
- **Binary target defined under `[[bin]]` with explicit `name = "..."`** — the package name and binary name are separate Cargo concepts. Both rename (often to different values, e.g., package `sealedge-cli` + binary `sealedge`).
- **Per-crate `src/bin/*.rs`** (e.g., `core/src/bin/trustedge-server.rs`, `inspect-trst.rs`) — these files have names matching their binary targets and must be renamed alongside the binary rename.

### Integration Points
- **Inter-crate deps in Cargo.toml** — every crate that depends on another trustedge crate has a `path` or `version` dep; all get new package names.
- **`use` statements across the codebase** — `use trustedge_core::*`, `use trustedge_types::*`, etc. A grep-and-rename pass is mandatory.
- **`pkg::` references in macros, test names, error messages** — some patterns (e.g., `format!("trustedge_core::...")`) may embed the crate name; discoverable via grep.
- **CI workflows** — `cargo test -p trustedge-core` style invocations in `.github/workflows/`; also rename alongside.

</code_context>

<specifics>
## Specific Ideas

- **"seal wrap input.bin → input.seal"** reads as the canonical demo invocation — verb + noun + natural output filename. Use this as the sanity-check mental model: if the rename produces an incoherent command pattern, something's wrong.
- **Binary `seal` replaces `trst`**, not `trustedge-cli`. The envelope-encryption CLI (`trustedge` binary) becomes `sealedge` — it's a different tool.
- **Clean-break verification** — a test must prove old `.trst` files and old crate imports produce clear rejection (not silent fallback). Planner designs the test shape.

</specifics>

<deferred>
## Deferred Ideas

### Amendment needed before planning
- ⚠ **REBRAND-04 scope amendment** — extend to cover `.trst` → `.seal` in addition to `.te-attestation.json` → `.se-attestation.json`. Without this amendment, Phase 83 doing the `.trst` rename is technically scope-creep. The cleanest resolution is to amend REBRAND-04 in-place (it's already about file extensions) and map it to both Phase 83 (`.trst`) AND Phase 84 (`.te-attestation.json`). Planner should flag this to the user or the /gsd-plan-phase workflow should surface it before Phase 83 plan is approved.

### Scope creep caught and deferred
- None — discussion stayed within phase scope.

</deferred>

---

*Phase: 83-crate-and-binary-rename*
*Context gathered: 2026-04-18*
