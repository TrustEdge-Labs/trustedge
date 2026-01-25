<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### 🏗️ Architecture Improvements

#### Workspace Reorganization
- **CLI Extraction**: Extracted CLI from trustedge-core into dedicated trustedge-cli crate for cleaner separation of concerns
- **Manifest Consolidation**: Unified CamVideoManifest types in trustedge-trst-core as canonical source; trustedge-trst-wasm now imports from trst-core
- **Pubky Marked Experimental**: trustedge-pubky and trustedge-pubky-advanced marked as community/experimental crates

#### Code Quality
- **Security Fix**: Removed unmaintained wee_alloc dependency
- **Deprecation Fixes**: Updated all GenericArray::from_slice calls to use array conversion
- **Version Coordination**: Bumped core platform crates to 0.2.0, keeping Pubky at 0.1.0
- **TODO Cleanup**: Converted pending TODOs to documented NOTEs for post-P0 work

### 🔧 YubiKey Improvements
- **GetPublicKey Operation**: Added support for retrieving public keys from YubiKey
- **Slot Validation**: Fixed yubikey_demo slot validation and custom PIN support
- **Documentation**: Fixed incorrect YubiKey CLI examples in backends.md

### 📚 Documentation
- **CLAUDE.md Updates**: Refreshed architecture overview and crate descriptions
- **README Updates**: Added trustedge-cli, updated binary references
- **Secure Node MVP**: Added hardware MVP specifications

---

## [0.3.0] - 2025-01-12

### 🎉 P0 Release: cam.video + verify

#### .trst Archive System
- **Locked Specification**: Finalized .trst archive format for cam.video profile
- **Ed25519 Signatures**: Device identity with detached manifest signatures
- **BLAKE3 Continuity Chains**: Cryptographic linking between archive segments
- **XChaCha20-Poly1305**: Chunk encryption with authenticated encryption

#### trst CLI Tool
- **wrap Command**: Create .trst archives from input files
- **verify Command**: Validate archives against device public keys
- **JSON Output**: Structured verification results with `--json` flag
- **Receipt Emission**: Export verification receipts with `--emit-receipt`

#### Browser Verification
- **WASM Verifier**: Browser-based archive verification (web/demo/)
- **trustedge-trst-wasm**: WebAssembly bindings for verification operations

#### Production Cryptography
- **AES-256-GCM Encryption**: Real chunk encryption replacing placeholders
- **PBKDF2 Key Derivation**: 100,000 iterations with HMAC-SHA256
- **Memory-Safe Key Handling**: All key material properly zeroized
- **Context-Bound Encryption**: Envelope context prevents key reuse

#### Digital Receipt System
- **Cryptographically Secure Receipts**: Production-ready with real encryption
- **Ownership Transfer Chains**: Multi-party assignment with verification
- **Amount Preservation**: Cryptographically protected through chains

#### Test Coverage
- **150+ Tests**: Comprehensive coverage across all crates
- **Security Attack Scenarios**: 23 tests for receipts including forgery, replay, tampering
- **Acceptance Tests**: End-to-end verification in crates/trst-cli/tests/acceptance.rs

---

## [0.2.0] - 2025-09-10

### 🎉 Major Features Added

#### YubiKey Hardware Integration
- **Real YubiKey PKCS#11 Support**: Full integration with YubiKey PIV applets for hardware-backed cryptographic operations
- **Hardware Signing Operations**: Actual signing operations using YubiKey hardware with ECDSA P-256
- **PIV Slot Management**: Support for all standard PIV slots (9a, 9c, 9d, 9e) with proper slot enumeration
- **Hardware Detection Framework**: Intelligent hardware detection with CI-safe fallbacks
- **Certificate Generation**: X.509 certificate generation with YubiKey public keys
- **Hardware Attestation**: Cryptographic proof of hardware-backed operations

#### Universal Backend Architecture
- **Pluggable Crypto Backends**: Capability-based backend system supporting multiple crypto providers
- **Backend Registry**: Runtime backend selection with preference-based routing
- **Software HSM Backend**: File-based HSM simulation with persistent key storage
- **Keyring Integration**: OS keyring support for secure key derivation
- **Operation Dispatch**: Type-safe crypto operation routing with comprehensive error handling

#### Transport Layer Implementation
- **Real TCP Transport**: Full TCP client-server implementation with actual network operations
- **Concurrent Connections**: Multi-client support with proper connection management
- **Large Data Transfer**: Support for multi-megabyte transfers with chunking
- **Connection Management**: Proper timeout handling, error recovery, and resource cleanup
- **Message Size Limits**: Configurable limits with enforcement and validation
- **Bidirectional Communication**: Full duplex communication support

### 🔧 Major Improvements

#### Test Suite Overhaul
- **204 Automated Tests**: Comprehensive test coverage across all components
- **Real Functional Testing**: Eliminated fake/stub tests in favor of actual operations
- **Hardware Test Separation**: Proper CI-safe vs hardware-required test categorization
- **Integration Test Coverage**: End-to-end validation of complete workflows
- **Network Integration Tests**: Real client-server testing with data transfer validation

#### Security Enhancements
- **Domain Separation**: Cryptographic domain separation for signature security
- **Resource Bounds**: DoS protection with comprehensive limits and validation
- **Hardware Root of Trust**: YubiKey integration provides hardware security foundation
- **Session Management**: Secure session handling with timeout controls

#### Developer Experience
- **Comprehensive Documentation**: 10,000+ lines of documentation across 27 files
- **CLI Tool Integration**: Full command-line interface for all operations
- **Example Workflows**: Complete examples for all major use cases
- **Error Handling**: Detailed error messages with recovery guidance

### 🐛 Bug Fixes
- Fixed transport layer configuration validation
- Resolved YubiKey hardware detection edge cases
- Corrected test isolation issues in concurrent scenarios
- Fixed memory management in large data transfers

### 📚 Documentation
- Added comprehensive YubiKey integration guide
- Updated CLI reference with all new options
- Enhanced troubleshooting documentation
- Added performance benchmarking guide

### 🔄 Breaking Changes
- Transport configuration API has been updated for better type safety
- YubiKey backend requires explicit feature flag (`--features yubikey`)
- Some test utilities have been moved to support real testing infrastructure

### 📦 Dependencies
- Added `yubikey` crate for hardware integration
- Added `pkcs11` crate for PKCS#11 operations
- Added `x509-cert` for certificate generation
- Updated `tokio-util` for transport layer improvements

### 🎯 Migration Guide
- Update `Cargo.toml` to version `0.2.0`
- Enable YubiKey support with `--features yubikey` if needed
- Review transport configuration for any custom implementations
- Update test dependencies if using TrustEdge test utilities

---

## [0.1.7] - 2025-09-08
### Fixed
- Resolved test infrastructure issues
- Updated CI workflows

## [0.1.0] - 2025-09-02
### Added
- Initial release with core encryption functionality
- Basic CLI tools
- Roundtrip encryption/decryption
- Ed25519 authentication system

---

[Unreleased]: https://github.com/TrustEdge-Labs/trustedge/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/TrustEdge-Labs/trustedge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/TrustEdge-Labs/trustedge/compare/v0.1.7...v0.2.0
[0.1.7]: https://github.com/TrustEdge-Labs/trustedge/compare/v0.1.0...v0.1.7
[0.1.0]: https://github.com/TrustEdge-Labs/trustedge/releases/tag/v0.1.0