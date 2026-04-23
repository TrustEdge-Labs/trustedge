# Phase 83: Crate & Binary Rename - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-18
**Phase:** 83-crate-and-binary-rename
**Areas discussed:** New `trst` binary name, `.trst` archive extension, Double-prefix crate pattern, Scope edges (dashboard + experimental)

---

## New `trst` binary name

| Option | Description | Selected |
|--------|-------------|----------|
| `seal` | 4 chars, matches product name, reads naturally (`seal wrap input.bin output.seal`). Strongest brand fit, memorable, natural English verb. Downside: generic word — shell command collisions possible on some systems. | ✓ |
| `sedg` | 4 chars, abbreviation of sealedge, more unique (like `trst` was). Less memorable, no natural read, cryptic. | |
| `se` | 2 chars, maximum brevity. Too generic, likely collisions (`se` is a common Unix alias). | |
| `sealedge` | Match product name directly — unambiguous but longer to type (8 chars vs 4). | |

**User's choice:** `seal`
**Notes:** Natural English verb, matches product, `seal wrap → .seal` reads well.

---

## `.trst` archive extension

| Option | Description | Selected |
|--------|-------------|----------|
| `.seal` | Parallels the `seal` binary name (`seal wrap input.bin → input.seal`). Reads naturally, matches brand, one-to-one with binary. | ✓ |
| `.sealed` | More descriptive (`input.sealed`). Slightly longer but reads as a past-tense state ("this is sealed"). | |
| `.sedg` | Direct analog to `.trst` — abbreviation of product name. Cryptic, loses the natural "seal" verb connection. | |
| Keep `.trst` | Don't rename the extension at all — scope-restricted to binary and crate names only. | |

**User's choice:** `.seal`
**Notes:** Claude flagged that REBRAND-04 only names `.te-attestation.json` — the `.trst` rename needs a requirement amendment before Phase 83 planning is complete.

---

## Double-prefix crate pattern

| Option | Description | Selected |
|--------|-------------|----------|
| `sealedge-seal-*` | Direct parallel: `trustedge-trst-protocols` → `sealedge-seal-protocols`, same for cli/wasm. Mechanical, preserves the existing namespace shape. Zero restructure risk, predictable. | ✓ |
| `sealedge-archive-*` | Descriptive: `sealedge-archive-protocols`, `sealedge-archive-cli`, `sealedge-archive-wasm`. Reads more clearly (these ARE the archive crates). Slight restructure while renaming. | |
| Drop the middle — `sealedge-protocols`, `sealedge-cli-archive`, `sealedge-wasm-archive` | Flatten the namespace. Cleaner but there's also `trustedge-cli` (envelope CLI) — two CLI crates need disambiguation. | |
| `sealedge-*` with suffixes (`sealedge-protocols`, `sealedge-archive`, `sealedge-verify-wasm`) | Semantic names. Biggest restructure. | |

**User's choice:** `sealedge-seal-*`
**Notes:** Mechanical parallel, zero restructure risk during what's already a big rename.

---

## Scope edges: dashboard

| Option | Description | Selected |
|--------|-------------|----------|
| Rename `package.json` name to `sealedge-dashboard` | Trivial one-line change, keeps naming consistent. No reason to leave it inconsistent during the core rename. | ✓ |
| Leave dashboard alone in Phase 83 | Defer dashboard naming to the docs/code sweep (Phase 85 or 86). | |
| Rename the directory too: `web/dashboard/` → `web/sealedge-dashboard/` | Bigger change — touches every import path and CI workflow. Probably overkill since it's the only package in web/. | |

**User's choice:** Rename package.json to `sealedge-dashboard`
**Notes:** Directory stays put.

---

## Scope edges: experimental

| Option | Description | Selected |
|--------|-------------|----------|
| Rename experimental crates too (`sealedge-pubky`, `sealedge-pubky-advanced`) | Consistency: every crate in the monorepo is sealedge-*. Small blast radius (separate workspace, isolated deps). | ✓ |
| Leave experimental as `trustedge-pubky` | Experimental is isolated and not in the root workspace; REBRAND-01 literally says "workspace crates" so this is technically out of scope. Can be renamed later if promoted to stable. | |

**User's choice:** Rename experimental too
**Notes:** Full monorepo consistency; small blast radius makes this cheap.

---

## Claude's Discretion

- **Commit granularity** — single atomic workspace commit per logical step; planner may split into 3-5 atomic commits if each remains `cargo check --workspace` green.
- **Crate directory normalization** — rename `crates/trustedge-cli/` → `crates/cli/` (outlier), and `trst-cli/` / `trst-protocols/` / `trst-wasm/` → `seal-cli/` / `seal-protocols/` / `seal-wasm/` for directory/package alignment.
- **Library module names** — auto-derive from package rename (hyphen→underscore).

## Deferred Ideas

- **REBRAND-04 amendment** — needs to be extended to cover `.trst` → `.seal` in addition to `.te-attestation.json`. Must be resolved before Phase 83 plan is approved, either by amending the requirement in-place or by logging a scope delta.
- No scope creep was surfaced during discussion; all gray areas stayed within Phase 83's rename domain.
