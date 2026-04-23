# Phase 84: Crypto Constants & File Extension - Discussion Log

> **Audit trail only.** Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-18
**Phase:** 84-crypto-constants-file-extension
**Areas discussed:** Envelope version field, Rejection test shape, Scope (HTML verifier + action.yml), File picker UX + demo script

---

## Envelope `version` JSON field: bump or stay?

| Option | Description | Selected |
|--------|-------------|----------|
| Stay at `version: 2` | Domain separation string changes but cryptographic scheme unchanged — version field tracks crypto scheme, not branding. Old envelopes fail AES-GCM tag verification under the new domain naturally. 5 test assertions unchanged. | ✓ |
| Bump to `version: 3` | Explicit signaling: `version: 3` = sealedge; `version: 2` = trustedge-era, rejected at version dispatch. Costs: 5 test assertions + serialization sites. Benefit: cleaner rejection path than AES-GCM tag failure. | |
| Keep `V1` naming but bump JSON version to 3 | Decouple constant name from JSON version (`SEALEDGE_ENVELOPE_V1` + `version: 3`). Semantic mismatch; not recommended. | |

**User's choice:** Stay at `version: 2`
**Notes:** Crypto scheme is unchanged; version field tracks the scheme, not the brand. AES-GCM tag failure under new domain is a sufficient clean-break signal.

---

## Clean-break rejection test shape

| Option | Description | Selected |
|--------|-------------|----------|
| Inline shadow constant | Test defines local `OLD_DOMAIN` / `OLD_HEADER` consts in a `#[cfg(test)]` module, constructs envelope/header with old values, verifies rejection. Zero production footprint. | ✓ |
| Byte-fixture golden vectors | Committed binary fixtures in `tests/fixtures/` simulate pre-rename on-disk data. More realistic but bloats repo and needs regeneration on format evolution. | |
| Known-answer test on domain strings | Test `hkdf_expand(..., OLD_DOMAIN)` != `hkdf_expand(..., NEW_DOMAIN)` for identical inputs. Clean unit test; doesn't exercise full unseal path. | |
| Both: inline shadow + KAT | Belt-and-suspenders. Inline covers end-to-end rejection; KAT proves domain separation is active. | |

**User's choice:** Inline shadow constant
**Notes:** Zero production footprint, end-to-end coverage, self-contained test. (CONTEXT.md also bakes in a small KAT sanity check as part of the test battery — best of both worlds at low cost.)

---

## Scope: HTML verifier + action.yml

| Option | Description | Selected |
|--------|-------------|----------|
| Both in Phase 84 | Update web/verify/index.html and actions/attest-sbom-action/action.yml in the monorepo NOW. Phase 88 handles the external republish + website content, not a second rename pass. | ✓ |
| Only HTML verifier in Phase 84; action.yml deferred to Phase 88 | Update HTML here (bundled into platform binary). Leave action.yml for Phase 88 republish. Downside: brief monorepo drift. | |
| Neither — both deferred to Phase 88 | Phase 84 touches only core crypto. Everything attestation-file-extension-related moves to Phase 88. Downside: REBRAND-04b only partially done in Phase 84. | |

**User's choice:** Both in Phase 84
**Notes:** Monorepo source-of-truth files are updated here; Phase 88 only re-publishes.

---

## File picker UX + demo script

| Option | Description | Selected |
|--------|-------------|----------|
| Clean rename — .se-attestation.json everywhere | UI label + file-input `accept` filter to `.se-attestation.json` only. Demo script writes `.se-attestation.json`. Consistent with clean-break project philosophy. | ✓ |
| Dual accept — UI accepts both extensions | File picker accepts `.te-attestation.json,.se-attestation.json`. Contradicts clean-break philosophy; no production users with legacy files anyway. | |

**User's choice:** Clean rename everywhere
**Notes:** Matches project-wide clean-break preference (memory: feedback_clean_break_compat).

---

## Claude's Discretion

- HKDF info byte-length change (22 → 21 bytes) has zero cryptographic impact; HKDF-Expand accepts variable-length info
- Magic-header detection function rewrites prefix-only (no legacy fallback)
- CLI default output filename: `attestation.se-attestation.json` (generic "attestation" prefix stays)
- Error messages: keep generic; no "looks like a trustedge file" helpfulness
- Commit granularity: 2-3 atomic plans (crypto constants in core, then CLI/test/script sweep, then HTML + action.yml) — planner decides

## Deferred Ideas

- Helpful error messages for old-format detection (legacy-aware UX) — rejected per clean-break preference
- `scripts/demo-attestation.sh` brand-word prose → Phase 85
- README / CHANGELOG / SECURITY / docs/** prose mentioning the renamed constants → Phase 86
- External republish of attest-sbom-action + Marketplace listing + website content → Phase 88
