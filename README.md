<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# TrustEdge — Trustable Edge AI (Rust)

> Not another CRUD app. Learning Rust through **Trustable Edge AI** — privacy-preserving edge pipelines.

---

## Why this project?

Most people learning Rust start with CRUD web apps. I’m taking a different route that
aligns with my background in IoT product development, security/PKI and edge systems:

* **Privacy by design**: encrypt at the edge, not just TLS in transit
* **Rust at the edge**: safety + performance for streaming workloads
* **Learning in public**: small, honest milestones → real, reviewable code

**TrustEdge** is a Rust prototype for privacy-preserving, provenance-aware edge audio.

- **Private by default:** audio chunks are encrypted with AES-256-GCM before leaving the device.
- **Provenance by design:** each chunk carries a signed manifest (C2PA-inspired) whose hash is bound into AEAD AAD; tampering breaks decryption.
- **Streaming-friendly:** fixed nonce discipline (prefix||counter) and per-chunk records.

**Non-goals (for now):**

- No key management (KMS/TPM) or device identity lifecycle.
- Not C2PA compliant yet (just “C2PA-inspired”).
- Not production crypto config (demo keys; no rotation or revocation).

If you’re into Rust, IoT, ML at the edge, or security and have ideas or
suggestions, I’d love your feedback.


See the included threat model ([`THREAT_MODEL.md`](./THREAT_MODEL.md)) for a detailed breakdown of security goals, risks, and mitigations.

For details on the wire format and network protocol, see [`PROTOCOL.md`](./PROTOCOL.md).

- Language: Rust (stable)
- Crypto: `aes-gcm` (AEAD), 256-bit keys, 96-bit nonces
- Goal of Phase 1: a clean, verifiable round-trip on real audio bytes

---


## Quick start

```bash
# Install Rust (if needed)
# https://rustup.rs

# Clone
git clone git@github.com:johnzilla/trustedge.git
cd trustedge/trustedge-audio

# Build all binaries
cargo build --release

# Three binaries are available:
# - trustedge-audio: CLI for file encryption/decryption 
# - trustedge-server: Network server for chunk processing
# - trustedge-client: Network client for streaming encrypted chunks

# Encrypt and write envelope (with hex key)
./target/release/trustedge-audio \
  --input ./sample.wav \
  --envelope ./sample.trst \
  --key-out ./aeskey.hex

# Decrypt envelope to plaintext (with hex key)
./target/release/trustedge-audio \
  --decrypt \
  --input ./sample.trst \
  --out ./roundtrip.wav \
  --key-hex $(cat ./aeskey.hex)

# Set a passphrase in the system keyring (run once)
./target/release/trustedge-audio --set-passphrase "my secret passphrase"

# Encrypt using keyring-derived key
./target/release/trustedge-audio \
  --input ./sample.wav \
  --envelope ./sample.trst \
  --use-keyring \
  --salt-hex <32-hex-chars>

# Decrypt using keyring-derived key
./target/release/trustedge-audio \
  --decrypt \
  --input ./sample.trst \
  --out ./roundtrip.wav \
  --use-keyring \
  --salt-hex <32-hex-chars>

# Or, for a simple round-trip (no envelope):
./target/release/trustedge-audio \
  --input ./sample.wav \
  --out ./roundtrip.wav \
  --chunk 8192


# Verify byte-for-byte round trip
sha256sum ./sample.wav ./roundtrip.wav
# hashes should match
```

---

## Network Mode Example

TrustEdge includes a complete client-server network stack for streaming encrypted chunks:

### 1. Start the server

```bash
# Start server with decryption and local storage
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --output-dir ./received_chunks \
  --key-hex <64-char-hex-key> \
  --decrypt --verbose

# Or use keyring-based key derivation
./target/release/trustedge-server \
  --listen 127.0.0.1:8080 \
  --use-keyring \
  --salt-hex <32-char-hex-salt> \
  --decrypt
```

### 2. Run the client

```bash
# Stream a file to the server
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --file ./sample.wav \
  --key-hex <64-char-hex-key> \
  --verbose

# Or send synthetic test chunks
./target/release/trustedge-client \
  --server 127.0.0.1:8080 \
  --test-chunks 10 \
  --use-keyring \
  --salt-hex <32-char-hex-salt>
```

**Protocol Features:**
- Each chunk includes encrypted data, signed manifest, nonce, and timestamp
- Server validates signatures, sequence numbers, and cryptographic integrity
- Real-time processing with ACK/response flow
- Comprehensive validation prevents tampering, replay, and out-of-order attacks

See [`PROTOCOL.md`](./PROTOCOL.md) for complete protocol specification.


**Heads-up:** A matching hash doesn’t “prove” encryption occurred — it proves the **encrypt→decrypt** pipeline is lossless. The code actually performs AES-GCM per chunk and immediately verifies the tag before writing plaintext out.

---



### Documentation

* [`FORMAT.md`](./FORMAT.md) — Binary format specification: structures, byte orders, validation rules, and security invariants
* [`PROTOCOL.md`](./PROTOCOL.md) — Network protocol specification for client-server chunk streaming and validation
* [`THREAT_MODEL.md`](./THREAT_MODEL.md) — Security goals, threat analysis, attack vectors, and mitigations
* [`SECURITY.md`](./SECURITY.md) — Security policy, vulnerability reporting, and best practices
* [`ROADMAP.md`](./ROADMAP.md) — Project direction, milestones, completed features, and planned enhancements
* `src/format.rs` — Centralized format definitions: types, constants, validation helpers
* `src/main.rs` — CLI tool: chunked processing, AES-256-GCM encryption, signed manifests, envelope format
* `src/lib.rs` — Core library with network types, key management, and validation
* `src/bin/trustedge-server.rs` — Network server for chunk processing and validation
* `src/bin/trustedge-client.rs` — Network client for streaming encrypted chunks
* `Cargo.toml` — Dependencies: crypto, serialization, network, and testing frameworks

### CLI options

| Flag               | Description                                                      | Mode(s)           |
|--------------------|------------------------------------------------------------------|-------------------|
| `--input`          | Input file (audio or any bytes)                                  | Encrypt/Decrypt   |
| `--out`            | Output file (decrypted/plaintext)                                | Encrypt/Decrypt   |
| `--chunk`          | Chunk size in bytes (default: 4096)                              | Encrypt/Decrypt   |
| `--envelope`       | Write envelope file (.trst) with header + records                | Encrypt           |
| `--no-plaintext`   | Skip writing round-tripped plaintext                             | Encrypt           |
| `--decrypt`        | Decrypt envelope to plaintext                                    | Decrypt           |
| `--key-hex`        | 64-char hex AES-256 key (for encrypt/decrypt)                    | Encrypt/Decrypt   |
| `--key-out`        | Save generated key to file (encrypt mode)                        | Encrypt           |
| `--set-passphrase` | Store a passphrase in the system keyring (run once)              | Key management    |
| `--use-keyring`    | Use keyring passphrase for key derivation (PBKDF2)               | Encrypt/Decrypt   |
| `--salt-hex`       | 32-char hex salt for PBKDF2 key derivation (with keyring)        | Encrypt/Decrypt   |

### Network-Specific CLI Options

**trustedge-server:**
| Flag               | Description                                                      |
|--------------------|------------------------------------------------------------------|
| `--listen`         | Address to listen on (default: 127.0.0.1:8080)                 |
| `--output-dir`     | Directory to save received chunks (optional)                    |
| `--decrypt`        | Decrypt received chunks and save plaintext                      |
| `--verbose`        | Enable detailed logging and validation reporting                |

**trustedge-client:**
| Flag               | Description                                                      |
|--------------------|------------------------------------------------------------------|
| `--server`         | Server address to connect to (default: 127.0.0.1:8080)         |
| `--file`           | File to send (will be processed into chunks)                    |
| `--test-chunks`    | Send N synthetic encrypted chunks instead of a real file        |
| `--chunk-size`     | Chunk size for file processing (default: 4096)                  |
| `--verbose`        | Enable detailed logging and progress reporting                   |

### How it works

- Reads the input file in user-defined chunks.
- For each chunk:
  - Constructs a unique nonce: 4-byte random prefix + 8-byte counter.
  - Builds AAD (Additional Authenticated Data): `[header_hash][seq][nonce][manifest_hash]`.
  - Creates a signed manifest (Ed25519 signature and public key as bytes) with provenance and integrity info.
  - Encrypts the chunk with AES-256-GCM and the AAD.
  - Immediately verifies the manifest signature, re-derives AAD, decrypts, and checks plaintext integrity.
  - Writes the verified plaintext to the output file (unless `--no-plaintext`).
  - Optionally writes each record to an envelope file (`--envelope`).
- For round-trip testing, the output file does **not** include a header, so its hash matches the input. Envelope files contain all metadata for real-world use.

### What is AAD?

AAD (Additional Authenticated Data) is extra data that is authenticated (integrity-checked) but not encrypted. Here, AAD binds each chunk to the file/session context and the signed manifest, preventing tampering and replay. Layout: `[header_hash][seq][nonce][manifest_hash]`.

### What is a manifest?

Each chunk includes a signed manifest (bincode-encoded struct) containing:
- Manifest version, timestamp, sequence number
- Hash of the file header and plaintext chunk
- Key ID for key identification and rotation support
- AI/model provenance fields (placeholders)
- Ed25519 signature and public key (as bytes)
This allows for strong provenance, integrity, key management, and future extensibility.


### Envelope file format and integrity

The `.trst` envelope file is a binary format containing:

- **StreamHeader**: version, header bytes (58 bytes), header hash (BLAKE3)
- **Record(s)**: sequence number, nonce (12 bytes: 4-byte prefix + 8-byte counter), signed manifest (with Ed25519 signature), ciphertext (AES-GCM)

All fields are bincode-encoded for compactness and speed.

### Enhanced Validation & Security Invariants

Recent improvements include comprehensive validation during decryption to prevent tampering and ensure data integrity:

- **Header Consistency**: Manifest header hash must match stream header hash
- **Key Rotation Support**: Manifest key ID must match file header key ID
- **Strict Sequencing**: Sequence numbers must be contiguous with no gaps
- **Nonce Integrity**: Record nonce prefix must match stream header prefix
- **Cryptographic Binding**: All hashes, signatures, and encrypted data verified
- **Fail-Safe Design**: Any validation failure immediately aborts processing

These invariants ensure that encrypted streams cannot be tampered with, reordered, or substituted without detection.

---

## Testing and Validation

### Test Vectors

TrustEdge includes comprehensive deterministic test vectors for format validation:

```bash
# Run format compliance test with golden hash verification
cargo test vectors::tests::golden_trst_digest_is_stable

# Run integration tests (round-trip, tamper detection)
cargo test --test vectors

# Run all tests
cargo test
```

**Golden Test Vector:**
- **Input**: 32KB deterministic pseudo-random data
- **Chunk Size**: 4KB chunks  
- **Expected Hash**: `8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c`
- **Purpose**: Ensures format stability and enables external validation

### Integration Testing

The test suite validates:
- ✅ **Format compliance**: Deterministic envelope generation with known cryptographic material
- ✅ **Round-trip integrity**: Encrypt → envelope → decrypt cycle verification
- ✅ **Tamper detection**: Corrupted envelopes correctly rejected
- ✅ **CLI functionality**: End-to-end testing via command-line interface
- ✅ **Network protocol**: Client/server chunk transfer validation

### Manual Verification

```bash
# Quick smoke test
echo "test data" > input.txt
./target/release/trustedge-audio 
  --input input.txt --out output.txt --envelope test.trst 
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

./target/release/trustedge-audio 
  --decrypt --input test.trst --out decrypted.txt 
  --key-hex 0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef

diff input.txt decrypted.txt  # Should be identical
```

---

**Envelope Integrity Invariants:**
- Each record's nonce prefix (first 4 bytes) must match the stream header's nonce prefix.
- The nonce counter (last 8 bytes) must equal the record's sequence number.
- The manifest's `seq` field must match the record's `seq` field.
These invariants are strictly enforced during decryption and help prevent record tampering, replay, or mixing between streams. If any validation fails (e.g., signature, nonce prefix, nonce counter, manifest sequence, hash), the record is rejected and an error is reported.



### Key management

- `--key-hex`: Use a user-supplied 64-char hex key for AES-256 (encrypt/decrypt). **Mutually exclusive** with `--use-keyring`.
- `--key-out`: Save the randomly generated key to a file (encrypt mode).
- `--set-passphrase`: Store a passphrase in the system keyring (run once).
- `--use-keyring`: Use the keyring passphrase for key derivation (PBKDF2). **Mutually exclusive** with `--key-hex`.
- `--salt-hex`: 32-char hex salt for PBKDF2 key derivation (required with `--use-keyring`, must be 16 bytes).
- In decrypt mode, you must provide either `--key-hex` or `--use-keyring` (random key is not allowed).
- In encrypt mode, if neither is provided, a random key is generated and optionally saved with `--key-out`.
- **PBKDF2 parameters:** SHA-256, 100,000 iterations, 16-byte (32 hex char) salt.
### Error handling

If any validation fails during decryption (e.g., manifest signature, nonce prefix, nonce counter, manifest sequence, key ID mismatch, header hash, or plaintext hash), the record is rejected and an error is reported or logged. This ensures that tampered, out-of-sequence, replayed, or incorrectly keyed records cannot be decrypted or accepted.

---

**Protocol Versioning:**
The protocol is versioned (see StreamHeader and file preamble). Future changes will increment the version and document compatibility requirements.



### Current Status & Next Steps

**✅ M1 Milestone (Format v1) - COMPLETED:**
* [x] Complete `.trst` envelope format with comprehensive validation
* [x] Deterministic test vectors with golden hash verification
* [x] Production-ready client-server network stack
* [x] Enhanced security: header consistency, key ID validation, strict sequencing
* [x] Comprehensive testing: unit, integration, CLI, and network protocol tests
* [x] Full documentation: format spec, protocol spec, security analysis

**🚀 M2 Milestone (Key Management) - IN PROGRESS:**
* [x] Key ID fields and rotation foundation
* [x] Keyring-based key derivation with PBKDF2
* [ ] Advanced key versioning and migration tools
* [ ] HSM/TPM integration points for production deployments
* [ ] Comprehensive key lifecycle management

**📋 M3 Milestone (Verification & QA) - PLANNED:**
* [ ] `trustedge-verify` CLI tool with human and JSON output
* [ ] Property-based testing with proptest
* [ ] Fuzzing campaign with cargo-fuzz
* [ ] Security audit and penetration testing
* [ ] Performance benchmarking and optimization

---

## Security and Threat Model

For a detailed analysis of security goals, threat actors, attack vectors, mitigations, and future roadmap, see [`THREAT_MODEL.md`](./THREAT_MODEL.md).

For project direction, milestones, and planned features, see [`ROADMAP.md`](./ROADMAP.md).

- Covers network, application, cryptographic, side-channel, and physical threats
- Describes current and planned mitigations
- Outlines security requirements and risk assessment
- Documents ongoing and future security work

**If you are reviewing, deploying, or contributing to TrustEdge, please read the threat model for context on security assumptions and limitations.**

---

## License

This project is licensed under the **Mozilla Public License 2.0 (MPL-2.0)**.
See [`LICENSE`](./LICENSE) for details.

**Disclaimer:** This project is developed independently, on personal time and equipment, and is **not affiliated with or endorsed by my employer**.

---

## Legal & Attribution

**Copyright** © 2025 John Turner. All rights reserved.

**License**: This documentation is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/johnzilla/trustedge) — Privacy and trust at the edge.

**Third-party Dependencies**: See [`Cargo.toml`](./trustedge-audio/Cargo.toml) for complete dependency information and licenses.
