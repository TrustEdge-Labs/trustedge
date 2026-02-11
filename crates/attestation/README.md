<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Attestation

## ⚠️ DEPRECATION NOTICE

**This crate has been deprecated as of version 0.3.0.**

All attestation functionality has been consolidated into [`trustedge-core`](https://docs.rs/trustedge-core).

### Timeline

- **0.3.0** (February 2026): Deprecated - warnings issued
- **0.4.0** (August 2026): Removal - crate will be deleted from workspace

### Migration

**Before (deprecated):**
```rust
use trustedge_attestation::{Attestation, create_signed_attestation, verify_attestation};
```

**After (recommended):**
```rust
use trustedge_core::{Attestation, create_signed_attestation, verify_attestation};
```

All APIs remain identical - only import paths change.

See [MIGRATION.md](../../MIGRATION.md) for detailed upgrade instructions.

---

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
