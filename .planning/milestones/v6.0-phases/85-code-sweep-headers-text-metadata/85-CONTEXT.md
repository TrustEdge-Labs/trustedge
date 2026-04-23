# Phase 85: Code Sweep — Headers, Text, Metadata - Context

**Gathered:** 2026-04-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Every human-readable string **compiled into the binary or written in Rust source** says "sealedge". Covers:

- Copyright/license headers in every `.rs` file
- CLI help text, error messages, log lines visible to a user
- Environment variable names (`TRUSTEDGE_*` → `SEALEDGE_*`)
- Cargo.toml metadata (description, repository, homepage, documentation URLs)
- SvelteKit dashboard UI compiled text (titles, headings, labels, nav, buttons, footer, `<title>`, meta description, aria-labels, toast/error messages)
- Inline `//` line comments in source (not rustdoc — see below)
- Three crypto byte-literal domain tags that Phase 84 did not cover (see D-01)

**In scope (boundary with Phase 86):**
- Phase 85 = strings baked into the compiled binary or in `.rs` source (including inline `//` comments)
- Phase 86 = rustdoc comments (`///`, `//!`), README.md, CLAUDE.md, `docs/**`, long-form content pages in dashboard, help/onboarding text, MDX/markdown assets, scripts/ echo-string prose (if not already renamed in Phase 85)

**Out of scope (explicit carve-outs):**
- `TrustEdge-Labs` GitHub org name — stays in org URLs, legal references, and anywhere it identifies the labs brand (per v6.0 memory)
- `TRUSTEDGE LABS LLC` legal entity name in copyright line — stays (see D-02)
- Rustdoc comments (`///`, `//!`) — Phase 86
- README.md, CLAUDE.md, `docs/**` prose — Phase 86
- GitHub repository rename operation itself — Phase 87 (only URLs change in Phase 85)
- External-surface work (new Action repo, Marketplace listing, `trustedgelabs.com` updates) — Phase 88

</domain>

<decisions>
## Implementation Decisions

### Crypto byte-literal domain separators (Phase 84 follow-up)

> **Audit amendment (2026-04-18):** Original D-01 scope asserted "no other HKDF/BLAKE3 domain tags in the codebase". Planner's ground-truth grep found **5 additional wire-format crypto constants** CONTEXT's scout missed. D-01 and D-02 below are the expanded post-audit scope. See D-01a below for the audit-finding rationale.

- **D-01:** Rename all crypto/wire-format byte-literal and domain constants. Clean break — same shape as Phase 84 envelope rename:

  **Production core (`crates/core/src/`):**
  - `b"TRUSTEDGE_TRST_CHUNK_KEY"` (HKDF-SHA256 info at `crates/core/src/crypto.rs:291`, per-chunk AES-GCM key derivation) → **`b"SEALEDGE_SEAL_CHUNK_KEY"`**
  - `"TRUSTEDGE_SESSION_KEY_V1"` (BLAKE3 derive_key context at `crates/core/src/auth.rs:320`, TCP/QUIC session key after ECDH) → **`"SEALEDGE_SESSION_KEY_V1"`**
  - `b"trustedge:genesis"` (`GENESIS_SEED` const, BLAKE3 continuity-chain genesis seed at `crates/core/src/chain.rs:10`) → **`b"sealedge:genesis"`**
  - `b"trustedge.manifest.v1"` (`MANIFEST_DOMAIN_SEP` const, Ed25519 manifest-signature domain separation at `crates/core/src/format.rs:141`) → **`b"sealedge.manifest.v1"`**
  - `b"TRST"` (`MAGIC` const, 4-byte legacy core-envelope file-format magic at `crates/core/src/format.rs:14`) → **`b"SEAL"`**

  **Experimental (`crates/experimental/pubky-advanced/src/`):**
  - `b"TRUSTEDGE_X25519_DERIVATION"` (BLAKE3 key derivation at `keys.rs:132`) → **`b"SEALEDGE_X25519_DERIVATION"`**
  - `b"TRUSTEDGE_V2_SESSION_KEY"` (HKDF info for hybrid-envelope session key at `envelope.rs:251`) → **`b"SEALEDGE_V2_SESSION_KEY"`**

  **Experimental demo:**
  - `b"TRUSTEDGE_AUDIO_V2"` (magic header in `crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs:149`) → **`b"SEALEDGE_AUDIO_V2"`**

- **D-02:** Treatment by category:

  **Full D-02 treatment** (shadow const + rejection test + KAT distinctness) for the **6 crypto-meaningful domain constants**:
  | Constant | Shadow const name | Test module location |
  |----------|-------------------|----------------------|
  | `SEALEDGE_SEAL_CHUNK_KEY` | `OLD_CHUNK_KEY_DOMAIN` | `crates/core/src/crypto.rs` (inline `#[cfg(test)]`) |
  | `SEALEDGE_SESSION_KEY_V1` | `OLD_SESSION_KEY_DOMAIN` | `crates/core/src/auth.rs` (inline `#[cfg(test)]`) |
  | `sealedge:genesis` | `OLD_GENESIS_SEED` | `crates/core/src/chain.rs` (inline `#[cfg(test)]`) |
  | `sealedge.manifest.v1` | `OLD_MANIFEST_DOMAIN_SEP` | `crates/core/tests/domain_separation_test.rs` (adjacent to existing `MANIFEST_DOMAIN_SEP` assertion at line 128) |
  | `SEALEDGE_X25519_DERIVATION` | `OLD_X25519_DERIVATION` | `crates/experimental/pubky-advanced/src/keys.rs` (inline `#[cfg(test)]`) |
  | `SEALEDGE_V2_SESSION_KEY` | `OLD_V2_SESSION_KEY` | `crates/experimental/pubky-advanced/src/envelope.rs` (inline `#[cfg(test)]`) |

  Each shadow-const site gets:
  - Inline `#[cfg(test)]` shadow const for the old tag
  - Rejection test: data sealed/derived/signed under old domain fails cleanly under new domain (AES-GCM tag failure, BLAKE3 hash mismatch, or Ed25519 signature verification failure depending on primitive)
  - KAT sanity check: identical IKM/input + two different domain values produce distinct outputs

  **Test names** (all 12):
  - `test_old_chunk_key_domain_rejected_cleanly`, `test_old_chunk_key_domain_produces_distinct_okm`
  - `test_old_session_key_domain_rejected_cleanly`, `test_old_session_key_domain_produces_distinct_okm`
  - `test_old_genesis_seed_rejected_cleanly`, `test_old_genesis_seed_produces_distinct_hash`
  - `test_old_manifest_domain_rejected_cleanly`, `test_old_manifest_domain_produces_distinct_signature`
  - `test_old_x25519_derivation_rejected_cleanly`, `test_old_x25519_derivation_produces_distinct_key`
  - `test_old_v2_session_key_rejected_cleanly`, `test_old_v2_session_key_produces_distinct_okm`

  **Plain rename + fixture update** (no D-02 cryptographic-distinctness test — wire-format magic header, same pattern as Phase 84 `ENCRYPTED_KEY_HEADER: "TRUSTEDGE-KEY-V1"` → `"SEALEDGE-KEY-V1"`):
  - `MAGIC: &[u8; 4] = b"TRST"` → `b"SEAL"` (`crates/core/src/format.rs:14`)
  - Any test fixtures containing raw legacy-envelope bytes need regeneration or magic-byte update (planner enumerates via `grep -l 'TRST\b'` on test files)

  **Plain rename only** (demo code, not security-critical, no test treatment):
  - `b"TRUSTEDGE_AUDIO_V2"` → `b"SEALEDGE_AUDIO_V2"`

- **Blast radius:** All of the following break as expected under clean-break rebrand:
  - Any existing `.seal` archive chunk decryption fails (chunk key derivation uses `SEALEDGE_SEAL_CHUNK_KEY`)
  - Any existing live TCP/QUIC session fails to authenticate (session key uses `SEALEDGE_SESSION_KEY_V1`)
  - Any existing continuity chain rooted at `b"trustedge:genesis"` is cryptographically distinct from a new chain — chain verification rejects old-rooted chains (expected — different genesis = different chain identity)
  - Any existing Ed25519 manifest signature verifies only under the old `b"trustedge.manifest.v1"` domain — signatures over old-domain-rebuilt manifests fail verification under the new domain (expected)
  - Any existing `.trst` legacy core-envelope file fails magic check (byte 0 = `b"TRST"`, new reader expects `b"SEAL"`)
  - Any experimental pubky-advanced encrypted payload using old X25519 derivation or V2 session key cannot be decrypted
  Consistent with v6.0 clean-break preference and Phase 84 envelope precedent.

- **Scope (post-audit):** D-01 and D-02 apply to the 8 constants above. Verified via `grep -rn 'b"[Tt][Rr][Uu][Ss][Tt][Ee][Dd][Gg][Ee]' --include='*.rs' crates/core/src/ crates/experimental/` during the audit — no additional hits outside the D-02 shadow consts already installed by Phase 84 (`OLD_ENVELOPE_DOMAIN`, `OLD_KEY_HEADER`) and Phase 84's `TRST` file-magic path which this phase now closes. Planner re-runs the grep at the start of Plan 01 to confirm the surface is still closed.

### D-01a: Source-audit amendment (2026-04-18)

The original D-01 scope (3 byte literals) reflected an incomplete scout pass. The pre-planning ground-truth grep found 5 additional constants of the same semantic category:

| Constant | Location | Why D-01 now covers it |
|----------|----------|------------------------|
| `GENESIS_SEED` = `b"trustedge:genesis"` | `crates/core/src/chain.rs:10` | BLAKE3 continuity-chain domain — same category as the HKDF/BLAKE3 tags originally listed |
| `MANIFEST_DOMAIN_SEP` = `b"trustedge.manifest.v1"` | `crates/core/src/format.rs:141` | Ed25519 signature domain separation — exact use case `DOMAIN_SEP` exists to prevent; wire-format |
| `b"TRUSTEDGE_X25519_DERIVATION"` | `crates/experimental/pubky-advanced/src/keys.rs:132` | BLAKE3 key derivation — production-shape crypto even in experimental crate |
| `b"TRUSTEDGE_V2_SESSION_KEY"` | `crates/experimental/pubky-advanced/src/envelope.rs:251` | HKDF info — identical pattern to `SEALEDGE_SEAL_CHUNK_KEY` |
| `MAGIC` = `b"TRST"` | `crates/core/src/format.rs:14` | File-format magic header — rename sibling of Phase 83 `.trst`→`.seal` and Phase 84 `SEALEDGE-KEY-V1`; Phase 83/84 overlook |

Phase 85 ROADMAP success criterion 5 (repo-wide case-insensitive grep for `trustedge` returning only `TrustEdge-Labs` org references) cannot pass while any of these remain under the old brand. They are functionally the same as the originally-listed HKDF `info` and BLAKE3 `derive_key` context — all are wire-format domain/magic constants baked into the binary and visible under `grep -i trustedge` against `.rs` source.

### Copyright / license header format

- **D-03:** Copyright line unchanged: `Copyright (c) 2025 TRUSTEDGE LABS LLC` stays. `TRUSTEDGE LABS LLC` is the legal entity name — company is NOT renaming, only the product is. Standard practice: copyright attribution tracks the legal entity, not the product brand.
- **D-04:** `Project: trustedge` → **`Project: sealedge`** in every `.rs` file MPL-2.0 header. One-word literal substitution.
- **D-05:** `GitHub: https://github.com/TrustEdge-Labs/trustedge` → **`GitHub: https://github.com/TrustEdge-Labs/sealedge`** in every `.rs` file header. Phase 85 flips the URL; Phase 87 renames the actual GitHub repo. GitHub's automatic redirect (built-in on repo rename) covers the gap between Phase 85 commit and Phase 87 rename.
- **D-06:** MPL-2.0 license-text link (`https://mozilla.org/MPL/2.0/`) unchanged. Copyright year (`2025`) unchanged — renames are not a reason to bump copyright year.

### Cargo.toml metadata URLs + description

- **D-07:** Update `repository`, `homepage`, `documentation` URLs in every Cargo.toml to `https://github.com/TrustEdge-Labs/sealedge` (or the appropriate subpath). Same timing rationale as D-05 — Phase 85 owns URL edits, Phase 87 owns the GitHub operation. `cargo publish` is not gated on URL reachability, and the workspace is not currently published to crates.io anyway (per v6.0 memory).
- **D-08:** `description` field in every Cargo.toml replaces `trustedge`/`TrustEdge` brand words with `sealedge`/`Sealedge` per the casing rules in D-11 through D-13. Workspace `description` in root `Cargo.toml` (if present) included.
- **D-09:** Phase 87 scope explicitly does NOT re-edit Cargo.toml URLs — all that work is in Phase 85. Phase 87 = rename repo on GitHub, update local `git remote`, verify redirects.

### Env var rename

- **D-10:** Clean rename `TRUSTEDGE_*` → `SEALEDGE_*` across all env-var read sites. Current env var names in the codebase (from scout):
  - `TRUSTEDGE_DEVICE_ID` → `SEALEDGE_DEVICE_ID`
  - `TRUSTEDGE_SALT` → `SEALEDGE_SALT`
  - Default fallback values in `.unwrap_or_else(|_| "trustedge-...")` calls: literal string fallback values also renamed (e.g., `"trustedge-abc123"` → `"sealedge-abc123"`, `"trustedge-demo-salt"` → `"sealedge-demo-salt"`).
  - No dual-read support. Downstream developer workflows requiring `.env` updates are an expected one-time cost of v6.0.
- **Callsite scan:** Planner's first task is a repo-wide grep of `TRUSTEDGE_[A-Z_]+` in `.rs` files to produce a definitive rename list (the scout found 8 distinct names; full enumeration happens in planning).

### Brand casing rules (user-visible strings)

- **D-11:** **Dashboard UI** (titles, headings, labels, nav, buttons, footer, meta tags, aria-labels, toast/error messages): `Sealedge` (Title case). Matches ROADMAP Phase 85 success criterion 4.
- **D-12:** **CLI help text, error messages, log lines, prose in strings**: `sealedge` (lowercase product name). Reads naturally in `error: sealedge envelope format mismatch` / `log: sealedge-cli started` prose.
- **D-13:** **Environment variable names, constants, byte-literal domain tags**: `SEALEDGE_*` (ALL_CAPS, existing convention carried forward).
- **D-14:** **Brand word at start of a user-visible sentence (e.g., log line starting with the brand)**: follow sentence-case grammar → `Sealedge` (Title case at sentence start). Rare; call out as edge case in plans if encountered.
- **D-14a:** **Phase 83 carry-forward: clap `#[command(name = ...)]` attributes.** Phase 83 Plan 02 (REBRAND-02) renamed Cargo binary target names but missed the `clap::Parser` `#[command(name = "trustedge-*")]` attributes on binaries and examples. These print in `--help` output and are user-visible CLI text per D-12. Plan 03 scope — lowercase rename. Known sites (planner re-greps `name = "trustedge` and `#\[command(name =` during Plan 03 enumeration to catch any additional):
  - `crates/core/src/bin/sealedge-client.rs:57` — `name = "trustedge-client"` → `"sealedge-client"`
  - `crates/core/src/bin/sealedge-server.rs:27` — `name = "trustedge-server"` → `"sealedge-server"`
  - `crates/core/examples/attest.rs:18` — `name = "trustedge-attest"` → `"sealedge-attest"`
  - `crates/core/examples/verify_attestation.rs:18` — `name = "trustedge-verify"` → `"sealedge-verify"`
- **Rationale:** Matches how `TrustEdge` / `trustedge` / `TRUSTEDGE_*` were used throughout the codebase before the rename. Preserving the casing convention means downstream readers don't notice the rename disrupts their mental model of where each form appears.

### Dashboard UI scope (Phase 85 vs Phase 86 boundary)

- **D-15:** **Phase 85 covers compiled UI text:**
  - Page `<title>`, meta `description`, OpenGraph tags
  - Navigation labels, buttons, links, footer
  - Form labels, input placeholders, helper text
  - Toast notifications, inline error messages, loading states
  - Aria-labels, accessibility text, tooltips
  - `manifest.json` `name` / `short_name` (treat as compiled brand surface)
  - Any string constants in TypeScript/Svelte source files
- **D-16:** **Phase 86 covers content, not compiled UI:**
  - Long-form content pages (about, pricing, docs overview)
  - Onboarding flows, help text, walkthroughs
  - Marketing copy, landing-page prose
  - Any MDX/markdown assets under `web/dashboard/src/**`
- **Rationale:** Compiled UI text has tight rename coupling (labels, nav, errors) — one coordinated pass. Long-form content has different editorial voice and can take more time; separating it into Phase 86 lets Phase 85 close faster.

### Inline code comments vs rustdoc boundary

- **D-17:** Inline `//` line comments in `.rs` source are **Phase 85** scope. These live alongside the code they explain — renaming them keeps each source file internally consistent after Phase 85 lands.
- **D-18:** Rustdoc comments (`///` on items, `//!` on modules) are **Phase 86** scope. They render into `cargo doc` output — they're documentation, not code, and Phase 86 already covers `cargo doc --workspace --no-deps` rendering correctness per ROADMAP.
- **Grep rule for success criterion 5:** After Phase 85, `grep -i trustedge` across `.rs` files should only match `TrustEdge-Labs` org references AND rustdoc comments (which Phase 86 sweeps). After Phase 86, only `TrustEdge-Labs` remains.

### Scripts directory scope

- **D-19:** `scripts/*.sh` echo strings that reference the product brand are **Phase 85** scope. They're user-visible when the script runs (same category as CLI log lines). Phase 84 CONTEXT deferred `demo-attestation.sh` echo strings to "Phase 85/86" — resolved here as Phase 85. Scripts that reference long-form narrative (rare) may defer to Phase 86 — planner's call per script.

### Claude's Discretion

- **Plan granularity:** The planner splits Phase 85 into plans at natural workspace boundaries. Suggested split (planner's call):
  - Plan 01: Crypto byte-literal domain rename + D-02 tests (REBRAND-05 partial via criterion 5; isolated crypto work that can be verified independently)
  - Plan 02: Copyright/license headers + `Project:` + `GitHub:` across all `.rs` files (REBRAND-05; biggest file count, mechanical)
  - Plan 03: CLI help text + error messages + log lines + env vars + scripts echo prose (REBRAND-06; production-code user-visible surface)
  - Plan 04: Cargo.toml metadata URLs + descriptions across all workspace crates (REBRAND-07)
  - Plan 05: SvelteKit dashboard UI compiled text (REBRAND-06 dashboard portion; web/dashboard/ surface)
- **File-count budget per plan:** Plan 02 touches ~127 `.rs` files for header edits; the planner may split it further if needed to stay within a per-plan complexity budget.
- **Commit-granularity rule:** `cargo check --workspace --locked` green at every commit boundary (Phase 83/84 carry-forward). For Plan 02 (headers), a single atomic commit is fine — header edits don't affect compilation. For Plan 03 (env vars + error messages), split commits by subsystem if needed.
- **Dashboard build verification:** After Plan 05, `cd web/dashboard && npm run build && npm run check` must pass. TypeScript/Svelte compile errors from renamed UI labels would surface here.
- **Generic English nouns that shouldn't rename:** Anything containing "edge" / "trust" outside the brand-word form stays. E.g., "trust-boundary", "edge-device", "edge computing" in prose — these are domain vocabulary, not brand words.

### Folded Todos

None — no backlog items fold into this phase.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` — REBRAND-05 (copyright headers), REBRAND-06 (user-facing text + env vars), REBRAND-07 (Cargo.toml metadata)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target features; clean-break preference locked
- `.planning/ROADMAP.md` §"Phase 85: Code Sweep — Headers, Text, Metadata" (line 158) — goal + 5 success criteria

### Phase 84 decisions that carry forward
- `.planning/phases/84-crypto-constants-file-extension/84-CONTEXT.md` — D-02 clean-break rejection test pattern being replicated in Plan 01 (crypto byte-literal domains)
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` — `.trst` → `.seal` naming carry-forward (informs `SEALEDGE_SEAL_CHUNK_KEY`)

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — scope boundaries, reserved phase numbers, clean-break rationale
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — user's clean-break preference

### Code surface (starting points for the planner)

**Crypto byte-literal domains (D-01/D-02):**
- `crates/core/src/crypto.rs:285-291` — chunk-key HKDF domain + surrounding doc
- `crates/core/src/auth.rs:319-321` — session-key BLAKE3 derive_key
- `crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs:148-150` — audio V2 header magic

**Copyright headers (D-03 through D-06):**
- `crates/*/src/**/*.rs` — ~127 files with `Project: trustedge` headers (scout count)
- `examples/**/*.rs` — example programs with headers
- `crates/experimental/**/*.rs` — experimental crate headers (separate workspace but still in-repo)

**Env vars + user-facing text (D-10 through D-14):**
- `crates/cli/src/main.rs:986-987` — `TRUSTEDGE_DEVICE_ID` + `TRUSTEDGE_SALT` env reads (primary demo surface)
- All `env::var("TRUSTEDGE_*")` call sites — planner's first task is to enumerate via grep
- All `anyhow!`/`bail!`/`log::*!` invocations containing brand words — planner enumerates

**Cargo.toml metadata (D-07 through D-09):**
- Root `Cargo.toml` (workspace metadata if any)
- `crates/*/Cargo.toml` for all 9 workspace crates
- `crates/experimental/*/Cargo.toml` for experimental crates (separate workspace)

**Dashboard UI (D-11, D-15):**
- `web/dashboard/src/app.html` — `<title>`, meta tags
- `web/dashboard/src/routes/**/*.svelte` — page components
- `web/dashboard/static/manifest.json` — PWA manifest
- `web/dashboard/package.json` — name, description (metadata)

**Scripts (D-19):**
- `scripts/*.sh` — demo + CI scripts with echo prose
- `scripts/demo-attestation.sh` — specifically called out by Phase 84 CONTEXT as a Phase 85/86 carry-forward

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **Phase 84 D-02 test shape** — the `clean_break_tests` module pattern in `crates/core/src/envelope.rs` is the template for Plan 01's chunk-key and session-key rejection tests. Copy the structure verbatim (inline `#[cfg(test)]` shadow consts + three tests: rejection + KAT distinctness + optional decrypt-fails-with-old-data variant).
- **MPL-2.0 header block** — uniform across `.rs` files. A single sed-style replacement or a small Rust tool can handle all 127 files in one pass. The `./scripts/fix-copyright.sh` script already exists (per `CLAUDE.md`) — verify it handles the new `Project: sealedge` line or extend it.
- **`blake3::derive_key` and `hkdf::Hkdf::expand`** — both accept variable-length domain tag / context strings. Old vs new tag length differences don't affect layout or output size. KAT tests prove distinctness.

### Established Patterns
- **Brand word casing convention** — `TrustEdge` (Title case for UI) vs `trustedge` (lowercase for prose/crate names) vs `TRUSTEDGE_*` (ALL_CAPS for constants/env vars) was consistently applied throughout the codebase. Preserve the same casing distinctions under `Sealedge` / `sealedge` / `SEALEDGE_*`.
- **env::var with default fallback** — consistent pattern: `env::var("TRUSTEDGE_*").unwrap_or_else(|_| "default-value".into())`. Fallback string values also contain brand words and need renaming alongside the env-var key.
- **SvelteKit compile-time checks** — `npm run check` (svelte-check + tsc) catches label/prop typos. Use as a verification gate after Plan 05.

### Integration Points
- **web/verify/index.html ↔ platform-server binary** — updating HTML in source flows through the next cargo build via `include_str!`. Phase 84 already updated `.te-attestation.json` references here; Phase 85 sweeps remaining brand words in the HTML (title, H1, footer).
- **scripts/fix-copyright.sh ↔ all .rs files** — script-driven update path keeps Plan 02 atomic.
- **Cargo.toml `workspace.package` inheritance** — if the workspace root Cargo.toml sets shared metadata (description template, license, repository), editing one spot cascades. Verify this pattern exists before enumerating individual crate files.

</code_context>

<specifics>
## Specific Ideas

- **Test naming (carry-forward from Phase 84 convention):**
  - `test_old_chunk_key_domain_rejected_cleanly`, `test_old_chunk_key_domain_produces_distinct_okm` (in `crypto.rs`)
  - `test_old_session_key_domain_rejected_cleanly`, `test_old_session_key_domain_produces_distinct_okm` (in `auth.rs`)
- **Scope of grep criterion 5** — after Phase 85 lands, run `grep -ri trustedge --include='*.rs' --include='*.toml' --include='*.svelte' --include='*.ts' --include='*.json' --include='*.html' --include='*.sh' . | grep -vE 'TrustEdge-Labs|.planning/milestones/|.git/|target/|node_modules/|.claude/worktrees/'` and expect only rustdoc/docs prose hits (those are Phase 86). Save as a check the planner can include in each plan's verification.
- **Workspace-level `Cargo.toml` `workspace.package` inheritance check** — if set, it's the single source of truth for all shared metadata; a single edit there cascades to every member crate. Include this check in Plan 04 as a first step.
- **`TrustEdge` brand word preservation exceptions:**
  - `TrustEdge-Labs` (hyphenated, always) — org name, never changes
  - `TRUSTEDGE LABS LLC` (all caps, company legal entity) — never changes
  - Any URL containing `/TrustEdge-Labs/` — org URL, stays
- **Cache-sensitive check:** After Plan 01 (crypto byte-literal rename), run `cargo clean && cargo test --workspace` to ensure no cached test artifacts hide the rename breakage.

</specifics>

<deferred>
## Deferred Ideas

- **Rustdoc comment sweep** (`///`, `//!`) — Phase 86 scope per D-18. Phase 85 explicitly excludes these so `cargo doc` output is a Phase 86 concern, not a Phase 85 concern.
- **Long-form dashboard content** (marketing copy, help pages, onboarding) — Phase 86 per D-16.
- **`docs/**` prose sweep** — Phase 86.
- **`README.md`, `CLAUDE.md`, `SECURITY.md` narrative** — Phase 86.
- **GitHub repo rename operation itself** — Phase 87. Phase 85 updates the URLs; Phase 87 does the GitHub-side rename and sets up redirects.
- **External Action / Marketplace / product website** — Phase 88.
- **Final validation (full test matrix, CI, E2E)** — Phase 89.
- **Copyright year bump to 2026** — not a rename concern; if needed, handle as a separate annual sweep.
- **Removing the `.claude/worktrees/` leftover directories** — housekeeping, not a phase concern. Planner may add a cleanup task if worktrees are still present when Phase 85 starts.

</deferred>

---

*Phase: 85-code-sweep-headers-text-metadata*
*Context gathered: 2026-04-18*
