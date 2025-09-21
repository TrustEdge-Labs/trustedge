<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# cam.video P0 Implementation Demo

This directory contains a complete end-to-end demonstration of the TrustEdge P0 `.trst` archive format using the `cam.video` profile. The P0 implementation provides the locked specification with cryptographic signatures, continuity chains, and encrypted storage.

## 🚀 5-Minute Quick Start

### 1) Build the workspace
```bash
cargo build --workspace
```

### 2) Create sample input data

**On Linux/macOS:**
```bash
head -c 32M </dev/urandom > examples/cam.video/sample.bin
```

**On Windows (PowerShell):**
```powershell
$bytes = New-Object byte[] (32 * 1024 * 1024)
(New-Object System.Random).NextBytes($bytes)
[System.IO.File]::WriteAllBytes("examples/cam.video/sample.bin", $bytes)
```

**Alternative (cross-platform with openssl):**
```bash
openssl rand 33554432 > examples/cam.video/sample.bin
```

### 3) Wrap using CLI
```bash
cargo run -p trustedge-trst-cli -- wrap --profile cam.video --in examples/cam.video/sample.bin --out ./clip.trst
```

### 4) Verify using CLI
```bash
cargo run -p trustedge-trst-cli -- verify ./clip.trst --device-pub $(cat device.pub)
```

## 📋 Expected Output

### Wrap Command Output:
```
Archive: ./clip.trst
Signature: ed25519:A1B2C3D4E5F6...
Segments: 32
Generated device key: device.key
Generated device pub: device.pub
```

### Verify Command Output:
```
Signature: PASS
Continuity: PASS
Segments: 32  Duration(s): 64.0  Chunk(s): 2.0
```

## 🔧 Library Examples

This directory also includes two Rust examples that demonstrate direct use of the P0 core library APIs:

### `record_and_wrap.rs` - Programmatic Archive Creation
```bash
cargo run -p trustedge-cam-video-examples --bin record_and_wrap
```

This example shows how to:
- Generate device keypairs using the core crypto module
- Read input data and split into fixed-size chunks
- Encrypt each chunk with XChaCha20-Poly1305
- Build BLAKE3 continuity chains
- Create and sign cam.video manifests
- Write complete .trst archive structures

### `verify_cli.rs` - Programmatic Archive Verification
```bash
cargo run -p trustedge-cam-video-examples --bin verify_cli [archive_path] [device_pub_path]
```

This example demonstrates:
- Reading .trst archive structures
- Verifying Ed25519 signatures against canonical manifest bytes
- Validating BLAKE3 continuity chain integrity
- Checking chunk file hash consistency
- Comprehensive verification reporting

## 📁 Archive Structure

The generated `.trst` archives follow this structure:
```
clip.trst/
├── manifest.json          # Signed cam.video manifest
├── signatures/
│   └── manifest.sig        # Detached signature
└── chunks/
    ├── 00000.bin           # Encrypted chunk 0
    ├── 00001.bin           # Encrypted chunk 1
    └── ...                 # Additional chunks
```

## 🔐 P0 Security Features

- **Ed25519 Signatures**: Each manifest is cryptographically signed with device keys
- **XChaCha20-Poly1305 Encryption**: All data chunks are encrypted with unique nonces
- **BLAKE3 Continuity Chains**: Tamper-evident chain linking all segments
- **Canonical Serialization**: Deterministic manifest ordering for consistent signatures
- **Comprehensive Validation**: Multi-layer verification of signatures, hashes, and structure

## 🎯 Profile Specification

The `cam.video` profile implements:
- **Chunk-based storage**: Fixed-size segments with timing metadata
- **Device identity**: Cryptographic device fingerprinting
- **Capture metadata**: Timestamp, resolution, codec, and frame rate information
- **Claims system**: Extensible metadata for location, source verification, etc.
- **Chain continuity**: Genesis-rooted hash chain for segment ordering

## 🧪 Testing & Validation

Run the integration tests to verify P0 compliance:
```bash
cargo test -p trustedge-trst-cli
```

## 📚 Further Documentation

- **[P0 Implementation Status](../../P0_IMPLEMENTATION.md)** - Complete P0 checklist and progress
- **[Core Module Documentation](../../crates/core/src/)** - Low-level API reference
- **[CLI Documentation](../../crates/trst-cli/)** - Command-line interface guide

---

*This example demonstrates the P0 implementation of the TrustEdge .trst specification, locked for the cam.video golden profile.*