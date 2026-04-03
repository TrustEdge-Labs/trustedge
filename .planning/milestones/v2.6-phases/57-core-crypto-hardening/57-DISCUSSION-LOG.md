<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 57: Core Crypto Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-23
**Phase:** 57-core-crypto-hardening
**Areas discussed:** Zeroize approach, PBKDF2 import minimum

---

## Zeroize Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Derive Zeroize + manual Drop (Recommended) | Add #[derive(Zeroize)] and implement Drop to call .zeroize(). ZeroizeOnDrop derive conflicts with Clone. Matches envelope.rs pattern. | ✓ |
| Wrap sensitive fields in Secret<T> | Use existing Secret<T> wrapper. Larger API change since callers need .expose(). | |
| Derive ZeroizeOnDrop directly | Simplest but incompatible with Clone derive. | |

**User's choice:** Derive Zeroize + manual Drop
**Notes:** None.

---

## PBKDF2 Import Minimum

| Option | Description | Selected |
|--------|-------------|----------|
| 300,000 (from requirement) | Matches existing enforcement at builder + backend levels from v2.2. | |
| 600,000 (match export) | Match OWASP 2023. Stricter but rejects older key files. | ✓ |
| 300,000 with warning at <600k | Accept >= 300k, warn < 600k. | |

**User's choice:** 600,000 — "older key files are irrelevant, solo builder, no requirement to maintain legacy compatibility"
**Notes:** Explicit statement that no legacy compat needed.

---

## Claude's Discretion

- Low-iteration rejection test
- `#[zeroize(drop)]` vs manual `Drop` impl
- Debug impl adjustments

## Deferred Ideas

None.
