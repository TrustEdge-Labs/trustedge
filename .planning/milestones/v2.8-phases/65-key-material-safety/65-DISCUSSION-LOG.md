<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 65: Key Material Safety - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-25
**Phase:** 65-key-material-safety
**Areas discussed:** PrivateKey approach

---

## PrivateKey Hardening Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Remove serde + pub(crate) field (Recommended) | Remove Serialize/Deserialize, make key_bytes pub(crate), use accessor for external | ✓ |
| Remove serde only | Remove derives but keep key_bytes pub | |
| Custom Serialize that redacts | Keep Serialize but output [REDACTED] for key_bytes | |

**User's choice:** Remove serde + pub(crate) field (Recommended)
**Notes:** Prevents both accidental serialization and direct field access from outside the crate.

---

## Claude's Discretion

- Whether to also restrict algorithm/key_id visibility
- Permission-setting helper extraction
- Non-Unix warning wording

## Deferred Ideas

None.
