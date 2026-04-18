# Phase 84: Crypto Constants & File Extension - Context

**Gathered:** 2026-04-18
**Status:** Ready for planning

<domain>
## Phase Boundary

Replace the trustedge-branded cryptographic wire-format constants and attestation file extension with sealedge equivalents, clean-break with no backward-compatible decrypt path. The underlying cryptographic scheme (HKDF-SHA256 → 40-byte OKM, AES-256-GCM, deterministic counter nonces, Ed25519 signatures, BLAKE3 hashing) is UNCHANGED — only the domain-separation strings and file-extension labels announce sealedge.

**In scope:**
- Encrypted key file magic header: `TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1` in `crates/core/src/crypto.rs` (const, detection fn, code-level doc mentions); `crates/seal-cli/tests/security_key_file_protection.rs` assertions
- HKDF info parameter (envelope domain separation): `TRUSTEDGE_ENVELOPE_V1` → `SEALEDGE_ENVELOPE_V1` in `crates/core/src/envelope.rs` (one byte literal + one comment referencing "TrustEdge envelope v2 context" — the brand word "TrustEdge" in the comment is Phase 86 prose; only the byte literal constant is in Phase 84 scope)
- Attestation file extension: `.te-attestation.json` → `.se-attestation.json` across:
  - `crates/core/src/point_attestation.rs` (code-level doc comment on `Serialize to pretty-printed JSON for writing to a .te-attestation.json file`)
  - `crates/seal-cli/src/main.rs` (CLI help text, default output path `attestation.te-attestation.json` → `attestation.se-attestation.json`, `value_name` description)
  - `crates/seal-cli/tests/acceptance.rs` (test fixture paths and assertions)
  - `web/verify/index.html` (deployed HTML verifier — UI label + file-input accept, if any)
  - `actions/attest-sbom-action/action.yml` (input description + OUT_PATH template)
  - `actions/attest-sbom-action/README.md` (action documentation prose — extension literal only; surrounding brand words stay for Phase 86)
  - `scripts/demo-attestation.sh` (demo output path)
  - `deploy/digitalocean/README-deploy.md` (deployment doc prose — extension literal only; surrounding brand words stay for Phase 86)

**Out of scope (Phase 84 — handled elsewhere):**
- README.md, CLAUDE.md, CHANGELOG.md, SECURITY.md root prose and doc comments → Phase 86
- `docs/architecture.md`, `docs/roadmap.md`, `docs/user/cli.md`, `docs/technical/format.md`, `docs/technical/threat-model.md`, `docs/third-party-attestation-guide.md`, `docs/landing-page.md` → Phase 86
- The word "TrustEdge" in error messages, log output, surrounding prose near the renamed constants → Phase 85
- Cargo.toml metadata fields → Phase 85
- Copyright headers → Phase 85
- The Phase 88 external surface work: re-publishing the attest-sbom-action under a sealedge marketplace listing, deprecating the old one, updating `trustedgelabs.com` product page. Phase 84 updates the in-repo `actions/attest-sbom-action/` source of truth; Phase 88 handles the external marketplace work.

</domain>

<decisions>
## Implementation Decisions

### Version & Format Invariants
- **D-01:** Envelope `version` JSON field STAYS at `2`. Domain separation string is changing but the cryptographic scheme is unchanged — version field tracks crypto scheme, not branding. Clean break is achieved via AES-GCM tag failure when old envelopes hit the new HKDF domain; no need to bump version just to signal the rename. All 5 existing `assert_eq!(envelope.version, 2, ...)` test sites remain unchanged.
- **Encrypted key file format version** (`{"version": 1, ...}` in JSON metadata after the magic header): also UNCHANGED. Only the magic header string changes from `TRUSTEDGE-KEY-V1` to `SEALEDGE-KEY-V1`. The `"version": 1` field in the JSON metadata describes the iteration-count encoding format, not the crypto brand.

### Clean-Break Rejection Test
- **D-02:** Inline shadow constant test. In a `#[cfg(test)]` module, define local consts:
  ```rust
  const OLD_ENVELOPE_DOMAIN: &[u8] = b"TRUSTEDGE_ENVELOPE_V1";
  const OLD_KEY_HEADER: &[u8] = b"TRUSTEDGE-KEY-V1";
  ```
  Test that:
  1. An envelope sealed with `OLD_ENVELOPE_DOMAIN` fails to unseal under the new `SEALEDGE_ENVELOPE_V1` domain (AES-GCM tag verification error, NOT silent fallback)
  2. A key file with the `OLD_KEY_HEADER` byte prefix is rejected by `is_encrypted_key_file()` / import paths with a clear error message (NOT silently accepted)
  3. KAT sanity check: `hkdf_expand` with the two domains produces distinct OKMs for identical inputs (proves domain separation is active)
- **Rationale:** Zero production footprint of old constants, fully self-contained, exercises the real unseal/import paths end-to-end. Tests live in `crates/core/src/envelope.rs` (or a new test module) and `crates/core/src/crypto.rs`.

### In-Repo External Assets
- **D-03:** Phase 84 updates both `web/verify/index.html` AND `actions/attest-sbom-action/action.yml` (+ README.md). These are the monorepo source-of-truth files. Phase 88 is about re-publishing the action to the Marketplace under a new sealedge repo name and updating `trustedgelabs.com` product-page content — Phase 88 does NOT re-rename the file extension, it just republishes the already-renamed artifacts. Keeping the monorepo sources aligned with the new extension in Phase 84 keeps the codebase internally consistent.

### UX Policy — Clean Rename Everywhere
- **D-04:** No dual-accept. UI labels say `.se-attestation.json`, file-input `accept` attribute (if present in `web/verify/index.html`) filters to `.se-attestation.json` only, demo script writes `.se-attestation.json`. Consistent with the project-wide clean-break preference (see `feedback_clean_break_compat` memory) and the fact that there are no production users with legacy attestation files.

### Claude's Discretion
- **HKDF info byte length:** `b"SEALEDGE_ENVELOPE_V1"` is 21 bytes vs `b"TRUSTEDGE_ENVELOPE_V1"` at 22 bytes. HKDF-Expand accepts variable-length `info` parameters — no layout or compatibility impact from the 1-byte length change. OKM output stays 40 bytes (32-byte AES-256 key + 8-byte nonce prefix) regardless.
- **CLI default output filename:** `attestation.te-attestation.json` (hard-coded default in `seal-cli/src/main.rs:1461`) → `attestation.se-attestation.json`. The prefix word "attestation" stays — it's the generic English noun, not a brand.
- **Magic header detection function:** `is_encrypted_key_file()` in `crypto.rs` reads ONLY the new `b"SEALEDGE-KEY-V1\n"` prefix. It does NOT fall through to accept the old `b"TRUSTEDGE-KEY-V1\n"` prefix (that would be backward-compat — explicitly rejected).
- **action.yml OUT_PATH template:** `${{ runner.temp }}/${BINARY_NAME}.te-attestation.json` → `${{ runner.temp }}/${BINARY_NAME}.se-attestation.json` at line 89 of `actions/attest-sbom-action/action.yml`.
- **Phase 83 carried-forward commit-granularity rule:** `cargo check --workspace --locked` green at every commit boundary. The planner will likely split Phase 84 into 2-3 atomic plans (crypto constants in core, then CLI/test/script sweep, then HTML + action.yml) — planner's call.
- **Error message wording for rejection of old headers/domains:** Leave the generic "invalid envelope" / "not an encrypted key file" error messages as-is. Adding a helpful "looks like a trustedge-era file, which is no longer supported" would be legacy-aware UX, contradicting the clean-break preference. Phase 85 will handle any brand-word prose in those error messages.

### Folded Todos
None.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Milestone-level (v6.0 scope)
- `.planning/REQUIREMENTS.md` — REBRAND-03 (crypto constants), REBRAND-04b (`.te-attestation.json` extension)
- `.planning/PROJECT.md` §Current Milestone — v6.0 target features; clean-break preference locked
- `.planning/ROADMAP.md` §"Phase 84: Crypto Constants & File Extension" — goal + 4 success criteria

### Phase 83 decisions that carry forward
- `.planning/phases/83-crate-and-binary-rename/83-CONTEXT.md` — clean break rationale for `.trst` → `.seal`; same shape applies to this phase
- Phase 83 landed 7 commits (f38bd31 through fbe8ba8) — the workspace is sealedge-native at the code level; Phase 84 finalizes the crypto wire format

### Memory / user context
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/project_v6_rebrand.md` — scope boundaries, reserved phase numbers
- `~/.claude/projects/-home-john-vault-projects-github-com-trustedge/memory/feedback_clean_break_compat.md` — user's clean-break preference; no legacy aliases

### Code surface (starting points for the planner)
- `crates/core/src/crypto.rs` — `ENCRYPTED_KEY_HEADER` const and `is_encrypted_key_file` fn
- `crates/core/src/envelope.rs:186` — `version: 2` stays; the HKDF `info` byte-literal is the single production rename point (plus 1 comment on the prior line about "TrustEdge envelope v2 context" — the "TrustEdge" brand word is Phase 86, only the byte literal is Phase 84)
- `crates/core/src/point_attestation.rs:234` — doc-comment extension reference
- `crates/seal-cli/src/main.rs:322,331,1461` — CLI help + default path
- `crates/seal-cli/tests/security_key_file_protection.rs` — header assertions
- `crates/seal-cli/tests/acceptance.rs` — attestation extension assertions
- `web/verify/index.html:123,126` — UI labels, potentially file-input `accept` attribute
- `actions/attest-sbom-action/action.yml:30,89` — input description + OUT_PATH template
- `actions/attest-sbom-action/README.md` — extension literal occurrences
- `scripts/demo-attestation.sh` — demo output path
- `deploy/digitalocean/README-deploy.md` — extension literal occurrences (prose brand words stay for Phase 86)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- **HKDF-Expand API** is already wired through `hkdf` crate; changing the `info` byte literal is a one-line change and cryptographically correct.
- **Magic-header detection fn** `is_encrypted_key_file(data: &[u8]) -> bool` already centralizes the prefix check — a single edit cascades to all callers.
- **`ENCRYPTED_KEY_HEADER` const** is the single source of truth for the header string in production code; `format!` + `b"..."` usages should all go through it.

### Established Patterns
- **Byte-literal domain separation** — HKDF info is a byte-literal, not a `String`. Keep that shape.
- **`assert_eq!(envelope.version, 2, ...)` pattern** — 5 test sites use this shape; version field stays 2 so these assertions stay as-is.
- **`include_str!` for HTML verify page** — `web/verify/index.html` is bundled into the platform-server binary via `include_str!` at compile time. Updating the HTML in source flows through the next build. No runtime asset deployment needed; just rebuild the platform server.

### Integration Points
- **envelope.rs ↔ crypto.rs** — HKDF domain lives in envelope.rs, header magic lives in crypto.rs. Independent edits; no cross-file consistency needed beyond the test constants in both places.
- **platform server ↔ verify page** — updating `web/verify/index.html` requires rebuilding/redeploying the platform server so the `include_str!` picks up the new bytes. Not a Phase 84 concern (deploy is elsewhere), but worth noting for the demo sanity check.
- **action.yml ↔ separate action repo** — monorepo source of truth is `actions/attest-sbom-action/`. Phase 88 will sync the external repo; Phase 84 just updates the monorepo source.

</code_context>

<specifics>
## Specific Ideas

- **Test name convention:** Name the new rejection test `test_old_domain_rejected_cleanly` (for envelope) and `test_old_header_rejected_cleanly` (for key file). Easy to grep, pairs with the existing security_* test files in `seal-cli/tests/`.
- **Known-answer fixture:** Since this is a clean break, the KAT vectors produced by `hkdf_expand(secret, salt, b"SEALEDGE_ENVELOPE_V1")` can be captured now and committed as golden vectors — future changes to HKDF code will be caught by the KAT diff.
- **Single envelope.rs edit:** The production code change for envelope domain is literally one line: `let info = b"TRUSTEDGE_ENVELOPE_V1";` → `let info = b"SEALEDGE_ENVELOPE_V1";`. All complexity is in the tests.

</specifics>

<deferred>
## Deferred Ideas

- **Helpful error messages for old-format detection** — adding "this looks like a trustedge-era file, which is no longer supported" would be legacy-aware UX. Explicitly rejected here because it contradicts the clean-break preference. If users ever report confusion, this can be revisited as a future phase.
- **`scripts/demo-attestation.sh` brand-word prose** — the script's echo statements containing "TrustEdge" stay for Phase 85. Only the `.te-attestation.json` → `.se-attestation.json` output paths change in Phase 84.
- **Docs prose sweep (README, CHANGELOG, SECURITY, docs/**)** — all `TRUSTEDGE-KEY-V1` / `TRUSTEDGE_ENVELOPE_V1` / `.te-attestation.json` mentions in prose are Phase 86 scope. Phase 84 keeps the code-level source of truth correct; Phase 86 aligns the narrative.
- **Phase 88 external republish** — the `actions/attest-sbom-action/` monorepo sources will be updated here in Phase 84, but syncing them to the separate TrustEdge-Labs/attest-sbom-action GitHub repo and cutting a new Marketplace listing is Phase 88 work.

</deferred>

---

*Phase: 84-crypto-constants-file-extension*
*Context gathered: 2026-04-18*
