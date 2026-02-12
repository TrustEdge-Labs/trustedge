<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Attestation

> **EXPERIMENTAL** -- This crate is Tier 2 (experimental). Re-export facade for `trustedge-core` attestation. No maintenance commitment. Depend on `trustedge-core` directly for production use.

## What is Software Attestation?

Software attestation provides cryptographically signed "birth certificates" for software artifacts, proving their integrity, provenance, and build context.

Each attestation contains:

- SHA-256 hash of the artifact
- Git commit hash from source
- Builder identity
- Timestamp
- Ed25519 digital signature

Attestations use TrustEdge's envelope encryption system for tamper-evident software supply chain security.

### Key Features

- **🔐 Cryptographic Signatures**: Ed25519 digital signatures with hardware-backed keys
- **📦 TrustEdge Envelope Integration**: Sealed attestations using the envelope system
- **🌐 Git Integration**: Captures source commit hash and repository information
- **🏗️ Build Provenance**: Records builder identity and timestamp
- **🛡️ Tamper Evidence**: SHA-256 artifact hashing for integrity verification

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0).
See [LICENSE](../../LICENSE) for details.
