<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 54: Transport Security - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-22
**Phase:** 54-transport-security
**Areas discussed:** Verification approach, Dev mode policy, Test strategy

---

## Verification Approach

| Option | Description | Selected |
|--------|-------------|----------|
| Delegate to rustls provider (Recommended) | Call the default aws-lc-rs provider's verify() on the actual message+cert+signature. The code already looks up the scheme — just needs to use it instead of discarding it. Simplest fix, standard TLS verification. | ✓ |
| Custom hardware-aware verify | Implement custom signature verification that also checks the cert was hardware-generated. More complex, may not be needed since verify_server_cert already checks that. | |

**User's choice:** Delegate to rustls provider
**Notes:** None — straightforward choice.

---

## Dev Mode Policy

| Option | Description | Selected |
|--------|-------------|----------|
| Gate behind insecure-tls (Recommended) | Move accept_any_hardware() behind the existing insecure-tls feature flag. Dev mode still works, but can't accidentally ship in release builds. Consistent with how SkipServerVerification is already gated. | ✓ |
| Remove entirely | Delete accept_any_hardware(). Developers must use real certs even in dev. Strictest but may slow development. | |
| Keep but warn at runtime | Keep it available but log a loud warning on every connection. Less safe than compile-time gating. | |

**User's choice:** Gate behind insecure-tls
**Notes:** Consistent with existing pattern in the codebase.

---

## Test Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Unit + integration (Recommended) | Unit test: construct a bad DigitallySignedStruct and verify it's rejected by verify_tls12/13_signature. Integration test: attempt QUIC connection with wrong cert and verify handshake fails. Covers both layers. | ✓ |
| Unit tests only | Test verify_tls12/13_signature directly with valid and invalid signatures. Faster, no network setup, but doesn't prove end-to-end rejection. | |
| Integration only | Test at QUIC connection level with forged certs. Proves end-to-end but harder to isolate the specific verification. | |

**User's choice:** Unit + integration
**Notes:** None.

---

## Claude's Discretion

- Exact test certificate generation approach
- Whether to refactor common verification logic into a shared helper
- Error message text for verification failures

## Deferred Ideas

None — discussion stayed within phase scope.
