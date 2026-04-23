<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
-->

# v6.0.0 — Sealedge Rebrand

Trademark-driven rename from "trustedge" to "sealedge" — clean break,
no backward-compat decrypt path.

## Breaking changes

- Repo: `TrustEdge-Labs/trustedge` → `TrustEdge-Labs/sealedge` (GitHub 301 redirect in place)
- Crates: `trustedge-*` → `sealedge-*` (workspace + 2 experimental crates)
- Binaries: `trst` → `seal`, `trustedge` → `sealedge`, `trustedge-server` → `sealedge-server`, etc.
- Crypto constants: `TRUSTEDGE-KEY-V1` → `SEALEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1` → `SEALEDGE_ENVELOPE_V1`
- File extensions: `.trst` → `.seal`, `.te-attestation.json` → `.se-attestation.json`
- Env vars: `TRUSTEDGE_*` → `SEALEDGE_*`
- GitHub Action: `TrustEdge-Labs/attest-sbom-action@v1` → `TrustEdge-Labs/sealedge-attest-sbom-action@v2`

TrustEdge Labs (the company brand) is unchanged. `trustedgelabs.com` domain unchanged.

See [MIGRATION.md](MIGRATION.md) for full upgrade guidance.
