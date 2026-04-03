<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 72: Core Crypto Hygiene - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-27
**Phase:** 72-core-crypto-hygiene
**Areas discussed:** Envelope::hash() error strategy

---

## Envelope::hash() Error Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Return Result<[u8; 32]> | Proper error propagation. 1 production + ~15 test callers to update. Matches no-unwrap convention. | ✓ |
| .expect() with message | Simpler, bincode serialize practically infallible. Zero caller changes. | |
| You decide | Claude picks based on conventions and blast radius | |

**User's choice:** Return Result<[u8; 32]>
**Notes:** Follows project convention of no unwrap in production code

---

## Claude's Discretion

- Error type for Envelope::hash() (anyhow::Result vs specific type)

## Deferred Ideas

None
