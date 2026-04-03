<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 59: CLI & Deploy Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-24
**Phase:** 59-cli-deploy-hardening
**Areas discussed:** CLI key suppression, nginx TLS

---

## CLI Key Suppression

| Option | Description | Selected |
|--------|-------------|----------|
| Error out (Recommended) | Require --key-out or --show-key. Prevents silent key loss. | ✓ |
| Silent generation | Generate but don't display. Data unrecoverable. | |
| Print redacted hint | No key material, just a message. | |

**User's choice:** Error out

---

## nginx TLS

| Option | Description | Selected |
|--------|-------------|----------|
| Conditional TLS via env vars (Recommended) | HTTPS activates with SSL_CERT_PATH + SSL_KEY_PATH. HTTP stays. envsubst template. | ✓ |
| TLS always-on with self-signed | Generate cert on first boot. Noisy for local dev. | |
| HTTP redirect to HTTPS | Breaks local dev without certs. | |

**User's choice:** Conditional TLS via env vars

---

## Claude's Discretion

- nginx template mechanism
- HTTPS health check
- Error message wording

## Deferred Ideas

None.
