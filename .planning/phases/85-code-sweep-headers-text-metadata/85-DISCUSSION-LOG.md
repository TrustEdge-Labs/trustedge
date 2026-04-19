# Phase 85: Code Sweep — Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-18
**Phase:** 85-code-sweep-headers-text-metadata
**Areas discussed:** Crypto byte-literal domains, Copyright holder line, Cargo.toml URL timing, Brand casing rules

---

## Crypto byte-literal domains

### Q1: How should Phase 85 handle TRUSTEDGE_TRST_CHUNK_KEY and TRUSTEDGE_SESSION_KEY_V1?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename clean-break | Rename inside Phase 85 as byte-literal domain separator changes. Same shape as Phase 84 envelope. Existing .seal archives become undecryptable; live sessions break. | ✓ |
| Insert decimal Phase 84.1 | Treat as Phase 84 crypto-wire-format work that was missed. Keep Phase 85 = user-visible strings only. | |
| Leave as-is (internal only) | Never user-visible. Allowlist them for criterion 5. | |

**User's choice:** Rename clean-break (Recommended)
**Notes:** Consistent with v6.0 clean-break preference. Matches Phase 84 precedent for envelope HKDF domain.

### Q2: New names for the renamed crypto domain tags?

| Option | Description | Selected |
|--------|-------------|----------|
| SEALEDGE_SEAL_CHUNK_KEY + SEALEDGE_SESSION_KEY_V1 | Follows Phase 83's .trst → .seal naming. Keeps V1 suffix for session key. | ✓ |
| SEALEDGE_CHUNK_KEY + SEALEDGE_SESSION_KEY_V1 | Drops SEAL_ infix. Simpler but loses archive-format linkage. | |
| SEALEDGE_SEAL_CHUNK_KEY_V1 + SEALEDGE_SESSION_KEY_V1 | Adds V1 suffix to both. Future-proofs for v2 schemes. | |

**User's choice:** SEALEDGE_SEAL_CHUNK_KEY + SEALEDGE_SESSION_KEY_V1 (Recommended)

### Q3: D-02 clean-break rejection tests for the two new domains?

| Option | Description | Selected |
|--------|-------------|----------|
| Full D-02 treatment | Inline shadow consts + rejection tests + KAT. Matches Phase 84 quality bar. | ✓ |
| KAT sanity check only | Prove distinct OKMs only. Simpler, less comprehensive. | |
| No new tests | Rely on existing crypto test coverage + AES-GCM tag failure. | |

**User's choice:** Full D-02 treatment (Recommended)

### Q4: Handle TRUSTEDGE_AUDIO_V2 in experimental/pubky-advanced?

| Option | Description | Selected |
|--------|-------------|----------|
| Rename for consistency | Change to SEALEDGE_AUDIO_V2. Trivial, keeps criterion 5 green. | ✓ |
| Leave (experimental, frozen) | Skip and allowlist. Experimental crate not in CI. | |

**User's choice:** Rename for consistency (Recommended)

---

## Copyright holder line

### Q1: How should the copyright/license header change in every .rs file?

| Option | Description | Selected |
|--------|-------------|----------|
| Keep LLC, change Project only | `Copyright (c) 2025 TRUSTEDGE LABS LLC` stays. `Project: trustedge` → `Project: sealedge`. Legal entity unchanged. | ✓ |
| Change both to sealedge | `Copyright (c) 2025 SEALEDGE LABS LLC`. Inaccurate unless LLC legally renames. | |
| Keep LLC, add Sealedge tagline | Hybrid with `(dba Sealedge)` annotation. Adds chars × 127 files. | |
| Keep LLC, no Project line | Drop the Project line entirely. Loses grep-as-namespace. | |

**User's choice:** Keep LLC, change Project only (Recommended)
**Notes:** Matches standard practice — copyright tracks legal entity, not product brand.

### Q2: Repository URL line in the copyright header?

| Option | Description | Selected |
|--------|-------------|----------|
| Update to sealedge URL in Phase 85 | Change to sealedge repo URL. GitHub redirect handles Phase 85 → Phase 87 gap. | ✓ |
| Keep pointing to old URL until Phase 87 | Phase 87's rename triggers a follow-up sweep. | |
| Drop the GitHub line from headers | Remove entirely. Reduces churn on rename. | |

**User's choice:** Update to sealedge URL in Phase 85 (Recommended)

---

## Cargo.toml URL timing

### Q1: When should Cargo.toml metadata URLs flip?

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 85 updates to sealedge URL | Phase 85 owns URL edits, Phase 87 owns the GitHub operation. | ✓ |
| Phase 87 updates URLs as part of the rename | Synchronized URL + GitHub rename. Cleaner atomicity but adds scope to Phase 87. | |

**User's choice:** Phase 85 updates to sealedge URL (Recommended)

### Q2: Cargo.toml description field treatment?

| Option | Description | Selected |
|--------|-------------|----------|
| Full brand replacement | Every description with trustedge → sealedge. Matches criterion 5. | ✓ |
| Minimal (fix only broken refs) | Leave legacy descriptions. Fails criterion 5. | |

**User's choice:** Full brand replacement (Recommended)

---

## Brand casing rules

### Q1: Brand casing in user-visible strings?

| Option | Description | Selected |
|--------|-------------|----------|
| Context-dependent | Title case in UI (matches criterion 4), lowercase in prose, ALL_CAPS in constants. Matches existing TrustEdge usage. | ✓ |
| Always lowercase `sealedge` | Single rule. Diverges from criterion 4. | |
| Always Title case `Sealedge` | Single rule. Reads odd in log/error prose. | |

**User's choice:** Context-dependent (Recommended)

### Q2: SvelteKit dashboard UI scope — Phase 85 vs Phase 86?

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 85 = compiled text, Phase 86 = content | Titles/labels/nav/footer/toast in 85; long-form content/onboarding/help in 86. | ✓ |
| Phase 85 owns all dashboard text | Every dashboard string in 85. Simpler boundary, larger blast radius. | |
| Dashboard UI entirely Phase 85 | Plus HTML, favicons, manifest.json, OG tags in 85. Full external presentation. | |

**User's choice:** Phase 85 = compiled text, Phase 86 = content (Recommended)

### Q3: Inline `//` line comments — Phase 85 or Phase 86?

| Option | Description | Selected |
|--------|-------------|----------|
| Phase 85 | Comments live alongside code — keeps source files internally consistent. | ✓ |
| Phase 86 (with rustdoc) | Single comment-sweep phase. Simpler model, leaves inline comments mismatched during Phase 85. | |

**User's choice:** Phase 85 (Recommended)

---

## Claude's Discretion

- Plan granularity (5-plan split suggested, planner's call)
- Per-plan file-count budget (Plan 02 headers: ~127 files; planner may split)
- Commit-granularity rule (Phase 83/84 carry-forward)
- Dashboard build verification gate after dashboard plan
- Generic English nouns that aren't brand words ("edge", "trust") stay

## Deferred Ideas

- Rustdoc comments (`///`, `//!`) → Phase 86
- Long-form dashboard content → Phase 86
- `docs/**`, README.md, CLAUDE.md, SECURITY.md narrative → Phase 86
- GitHub repo rename operation → Phase 87
- External Action / Marketplace / product website → Phase 88
- Final validation → Phase 89
- Copyright year bump → not a rename concern
- Worktree cleanup → housekeeping, not a phase concern
