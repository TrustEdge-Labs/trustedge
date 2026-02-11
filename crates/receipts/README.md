<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Receipts

## ⚠️ DEPRECATION NOTICE

**This crate has been deprecated as of version 0.3.0.**

All receipt functionality has been consolidated into [`trustedge-core`](https://docs.rs/trustedge-core).

### Timeline

- **0.3.0** (February 2026): Deprecated - warnings issued
- **0.4.0** (August 2026): Removal - crate will be deleted from workspace

### Migration

**Before (deprecated):**
```rust
use trustedge_receipts::{Receipt, create_receipt, assign_receipt};
```

**After (recommended):**
```rust
use trustedge_core::{Receipt, create_receipt, assign_receipt};
```

All APIs remain identical - only import paths change.

See [MIGRATION.md](../../MIGRATION.md) for detailed upgrade instructions.

---

## What are Digital Receipts?

Digital receipts provide cryptographically secure ownership chains for digital assets. Each receipt contains:

- Unique receipt ID
- Ed25519 signature from the issuer
- Asset metadata
- Ownership chain with transfer history

Receipts use TrustEdge's envelope encryption system for tamper-proof provenance tracking.

### Key Features

- **🔐 Real Cryptography**: AES-256-GCM encryption with PBKDF2-HMAC-SHA256 key derivation
- **📋 Transferable Receipts**: Create and assign receipts with cryptographic ownership chains
- **🔗 Chain Integrity**: Each assignment links to previous receipt with cryptographic hash binding
- **🛡️ Attack Resistance**: Comprehensive security against tampering, replay, and forgery attacks
- **🧪 Battle-Tested**: 23 security tests covering cryptographic isolation and attack scenarios

## License

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0).
See [LICENSE](../../LICENSE) for details.
