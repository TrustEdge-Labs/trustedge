<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Roadmap: TrustEdge

## Milestones

- ✅ **v1.0 Consolidation** - Phases 1-8 (shipped 2026-02-11)
- ✅ **v1.1 YubiKey Integration Overhaul** - Phases 9-12 (shipped 2026-02-11)
- ✅ **v1.2 Scope Reduction** - Phases 13-14 (shipped 2026-02-12)
- ✅ **v1.3 Dependency Audit** - Phases 15-18 (shipped 2026-02-13)
- ✅ **v1.4 Placeholder Elimination** - Phases 19-23 (shipped 2026-02-13)
- ✅ **v1.5 Platform Consolidation** - Phases 24-27 (shipped 2026-02-22)
- ✅ **v1.6 Final Consolidation** - Phases 28-30 (shipped 2026-02-22)
- ✅ **v1.7 Security & Quality Hardening** - Phases 31-34 (shipped 2026-02-23)
- ✅ **v1.8 KDF Architecture Fix** - Phases 35-37 (shipped 2026-02-24)
- ✅ **v2.0 End-to-End Demo** - Phases 38-41 (shipped 2026-03-16)
- ✅ **v2.1 Data Lifecycle & Hardware Integration** - Phases 42-44 (shipped 2026-03-18)
- ✅ **v2.2 Security Remediation** - Phases 45-47 (shipped 2026-03-19)

## Phases

<details>
<summary>v1.0-v1.8 (Phases 1-37) - See milestone archives</summary>

See `.planning/milestones/v1.0-ROADMAP.md` through `.planning/milestones/v1.8-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.0 End-to-End Demo (Phases 38-41) - SHIPPED 2026-03-16</summary>

Delivered working end-to-end demonstration of TrustEdge's full value proposition. Generic archive profiles, one-command Docker stack, demo script, and README rewrite. 4 phases, 8 plans, 17/17 requirements complete.

**See:** `.planning/milestones/v2.0-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.1 Data Lifecycle & Hardware Integration (Phases 42-44) - SHIPPED 2026-03-18</summary>

Completed the data lifecycle with decryption capability, exposed YubiKey hardware signing in the CLI, and added named archive profiles. 3 phases, 6 plans, 12/12 requirements complete.

**See:** `.planning/milestones/v2.1-ROADMAP.md` for full phase details.

</details>

<details>
<summary>v2.2 Security Remediation (Phases 45-47) - SHIPPED 2026-03-19</summary>

Fixed critical cryptographic flaws. RSA OAEP-SHA256 replaces PKCS#1 v1.5, v1 envelope format removed entirely, PBKDF2 minimum 300k iterations enforced, device keys encrypted at rest with passphrase protection. RUSTSEC-2023-0071 fully resolved. 3 phases, 5 plans, 8/8 requirements complete, 23 commits.

**See:** `.planning/milestones/v2.2-ROADMAP.md` for full phase details.

</details>

---
*Last updated: 2026-03-19 after v2.2 milestone shipped*
