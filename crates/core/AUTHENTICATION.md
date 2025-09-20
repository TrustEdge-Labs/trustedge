<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Core Authentication

> **📖 For complete authentication documentation, see [../AUTHENTICATION_GUIDE.md](../AUTHENTICATION_GUIDE.md)**

This directory contains the core TrustEdge authentication implementation. For comprehensive setup guides, security considerations, and production deployment instructions, please refer to the main authentication guide in the project root.

## Quick Reference

- **Authentication Implementation**: [`src/auth.rs`](src/auth.rs) - Complete Ed25519 mutual authentication system
- **Certificate Management**: 672-line implementation with automatic generation and validation
- **Session Management**: Cryptographically secure sessions with configurable timeouts

## Core Features

✅ **Mutual Authentication**: Ed25519-based client/server authentication  
✅ **Certificate Generation**: Automatic Ed25519 key pair and certificate creation  
✅ **Session Security**: Time-limited sessions with cryptographic session IDs  
✅ **Challenge-Response**: Replay protection with fresh random challenges  

## Documentation Structure

| Document | Purpose |
|----------|---------|
| **[../AUTHENTICATION_GUIDE.md](../AUTHENTICATION_GUIDE.md)** | **Complete authentication setup and usage guide** |
| **[../SECURITY.md](../SECURITY.md)** | Security policies and vulnerability reporting |
| **[../EXAMPLES.md](../EXAMPLES.md)** | Authentication examples and CLI usage |

---

**For detailed authentication setup, troubleshooting, and production deployment, see [../AUTHENTICATION_GUIDE.md](../AUTHENTICATION_GUIDE.md).**