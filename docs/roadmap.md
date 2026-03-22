<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Roadmap

> **Vision:** Trusted data from the edge. Capture, encrypt, and transfer data with cryptographic provenance and privacy-preserving architecture.

---

## Current Status (v2.4)

TrustEdge v2.4 is a **security-hardened, fully tested platform** with:

- ✅ **Data-Agnostic Archives**: .trst format with generic, cam.video, sensor, audio, and log profiles
- ✅ **Full Data Lifecycle**: `trst keygen` → `trst wrap` → `trst verify` → `trst unwrap`
- ✅ **Strong Cryptography**: AES-256-GCM, Ed25519, ECDSA P-256, BLAKE3, HKDF-SHA256, RSA OAEP-SHA256
- ✅ **Encrypted Keys at Rest**: TRUSTEDGE-KEY-V1 format (PBKDF2-SHA256 600k + AES-256-GCM), 0600 permissions
- ✅ **YubiKey Hardware Signing**: PIV ECDSA P-256 via `trst wrap --backend yubikey`
- ✅ **Platform Service**: Axum HTTP verification server with PostgreSQL multi-tenant backend
- ✅ **Docker Deployment**: One-command `docker compose up` with auto-migration
- ✅ **SvelteKit Dashboard**: Device management and receipt viewing
- ✅ **Browser Verification**: WASM-based .trst archive verification
- ✅ **Security Validated**: 45 security tests across archive integrity, key protection, nonce uniqueness, error paths
- ✅ **406 Tests**: Across 9 workspace crates, 14 milestones shipped (v1.0-v2.4)

---

## Future Directions

### Enhanced Hardware Support

**TPM 2.0 Integration**
- Hardware-backed key storage and attestation
- Platform integrity verification
- Enterprise security compliance

**HSM Support**
- PKCS#11 enterprise HSM integration
- Hardware security module backends
- High-availability key management

### Advanced Cryptography

**Post-Quantum Cryptography**
- Algorithm agility framework ready for PQC algorithms
- Hybrid classical/post-quantum signatures
- Future-proof cryptographic transitions

**Zero-Knowledge Proofs**
- Receipt verification without revealing amounts
- Privacy-preserving audit trails
- Selective disclosure mechanisms

### Ecosystem Integration

**IoT Device Support**
- Embedded device SDKs
- Lightweight protocol variants
- Edge device attestation

**Cloud Integration**
- AWS/Azure/GCP backend adapters
- Serverless function support
- Container orchestration

### Developer Experience

**Language Bindings**
- Python SDK with native performance
- JavaScript/TypeScript bindings
- Go and C++ interfaces

**Tooling & Monitoring**
- Visual receipt chain explorer
- Cryptographic audit tools
- Performance monitoring dashboards

---

## Contributing

TrustEdge welcomes contributions in these areas:

- **Hardware Backend Development**: New crypto backend implementations
- **Protocol Extensions**: Enhanced network protocols and formats
- **Security Research**: Cryptographic analysis and testing
- **Documentation**: Guides, tutorials, and API documentation
- **Testing**: Security test scenarios and fuzzing

For contribution guidelines, see [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## Community & Support

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Architecture discussions and Q&A
- **Security**: Responsible disclosure via [SECURITY.md](../SECURITY.md)
- **Commercial**: Enterprise support at [enterprise@trustedgelabs.com](mailto:enterprise@trustedgelabs.com)

---

*This roadmap reflects current development priorities and may evolve based on community feedback and emerging requirements.*