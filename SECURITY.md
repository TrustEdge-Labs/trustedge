<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->


# TrustEdge Security Policy

## Supported Versions

TrustEdge is currently in active development. Security updates are provided for:

| Version | Status | Support Level |
| ------- | ------ | ------------- |
| main branch | 🔄 Development | Active security fixes |
| v0.1.x | ⏳ Pre-release | Best effort |
| < v0.1 | ❌ Unsupported | No security support |

**Note**: As TrustEdge is pre-1.0, breaking changes may be introduced to address security issues.

## Security Considerations

### Cryptographic Implementation

TrustEdge implements privacy-preserving audio encryption with the following security properties:

- **Encryption**: AES-256-GCM authenticated encryption
- **Key Derivation**: PBKDF2 with configurable iterations
- **Digital Signatures**: Ed25519 for manifest integrity
- **Hashing**: BLAKE3 for content verification
- **Nonce Management**: Deterministic 12-byte nonces (4-byte random prefix + 8-byte counter)

### Known Limitations

1. **Pre-1.0 Software**: Not recommended for production use
2. **Authentication**: Server and client authentication in development (Phase 3)
3. **Network Security**: TLS authentication being implemented
4. **Side Channel Attacks**: No specific mitigations implemented
5. **Memory Safety**: Relies on Rust's memory safety guarantees

### Current Security Status (Phase 3)

**✅ Implemented Security Features:**
- AES-256-GCM authenticated encryption
- Ed25519 digital signatures for provenance
- PBKDF2 key derivation with keyring integration
- Connection timeouts and retry logic
- Graceful shutdown handling

**🔄 In Development (Phase 3):**
- Server certificate validation and mutual TLS
- Client authentication and authorization
- Enhanced security features (Perfect Forward Secrecy)
- Production deployment security hardening

**📋 Planned Security Features:**
- TPM and HSM key storage backends
- Key rotation and revocation mechanisms
- Rate limiting and DoS protection
- Security audit logging

### Security Assumptions

- Users manage encryption keys securely
- Network transport provides confidentiality (HTTPS/TLS) - being enhanced
- System entropy source is reliable for cryptographic operations
- Dependencies (aes-gcm, ed25519-dalek, etc.) are trustworthy

## Reporting a Vulnerability

We take security vulnerabilities seriously. Please follow responsible disclosure practices.

### 🔒 Private Security Issues

For **sensitive security vulnerabilities** that could be exploited:

1. **GitHub Security Advisories**: [Create Private Advisory](https://github.com/johnzilla/trustedge/security/advisories/new)
2. **Direct Contact**: Email security concerns to security@[PLACEHOLDER] (planned)

**Do not** disclose sensitive vulnerabilities publicly until we've had time to address them.

### 📋 General Security Issues

For **general security improvements** or **questions about security practices**:

1. **Security Issue Template**: [Create Security Issue](./.github/ISSUE_TEMPLATE/security.yml)
2. **GitHub Issues**: Use for non-sensitive security discussions

### 🎯 What to Include

When reporting security issues, please include:

- **Description**: Clear description of the vulnerability
- **Impact**: Potential security impact and attack scenarios
- **Reproduction**: Steps to reproduce the issue
- **Environment**: TrustEdge version, OS, Rust version
- **Mitigation**: Suggested fixes or workarounds (if any)

### ⏱️ Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Progress Updates**: Weekly for active issues
- **Resolution**: Timeline depends on severity and complexity

### 🏆 Recognition

We appreciate security researchers who help improve TrustEdge security:

- **Security Hall of Fame**: Recognition for responsible disclosure
- **Attribution**: Credit in release notes (with permission)
- **Coordination**: Work together on disclosure timeline

### Severity Classification

| Severity | Examples | Response Time |
| -------- | -------- | ------------- |
| **Critical** | Key recovery, authentication bypass | 24-48 hours |
| **High** | Encryption weakness, data corruption | 3-7 days |
| **Medium** | Information disclosure, DoS | 1-2 weeks |
| **Low** | Minor information leaks | 2-4 weeks |

## Security Best Practices

### For Users

1. **Key Management**:
   - Use strong, randomly generated keys
   - Rotate keys regularly
   - Store keys securely (hardware tokens recommended)
   - Never share keys over insecure channels

2. **Network Security**:
   - Always use TLS/HTTPS for network operations
   - Verify server certificates
   - Consider VPN or other transport security

3. **System Security**:
   - Keep TrustEdge updated
   - Use on systems with full disk encryption
   - Monitor for unusual network activity
   - Regular security audits of infrastructure

### For Developers

1. **Code Review**: All security-related changes require review
2. **Testing**: Security features must have comprehensive tests
3. **Dependencies**: Regular updates and vulnerability scanning
4. **Documentation**: Security implications must be documented

## Security Audit Status

| Component | Last Audit | Status | Notes |
| --------- | ---------- | ------ | ----- |
| Cryptographic Implementation | TBD | ⏳ Pending | Awaiting external review |
| Key Management | TBD | ⏳ Pending | Internal review needed |
| Network Protocol | TBD | ⏳ Pending | Protocol design review |
| File Format | TBD | ⏳ Pending | Format specification review |

**Next Audit**: Planned before v1.0 release

## Compliance and Standards

TrustEdge aims to align with:

- **NIST Cryptographic Standards**: AES, SHA-3 family (BLAKE3)
- **RFC Standards**: Ed25519 (RFC 8032), relevant IETF standards
- **Industry Best Practices**: OWASP guidelines, secure coding practices

## Contact Information

**Security Team**: [To be updated with actual contacts]
- **Lead**: John Turner (john@example.com)
- **GPG Key**: [KEY_ID] (for encrypted communications)
- **Security Advisory**: Subscribe to security@trustedge.example.com

---

**Document Version**: 1.0  
**Last Updated**: August 25, 2025  
**Next Review**: November 2025

---

*This document is part of the TrustEdge project documentation.*

*Copyright (c) 2025 John Turner. Licensed under the Mozilla Public License 2.0 (MPL-2.0).*
*See LICENSE file for full license terms.*

*For technical details, see [THREAT_MODEL.md](THREAT_MODEL.md), [PROTOCOL.md](PROTOCOL.md), and [FORMAT.md](FORMAT.md).*
*For contribution guidelines, see [README.md](README.md).*
