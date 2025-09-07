<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Format Specification v1.0

**Version:** 1.0  
**Date:** August 25, 2025  
**Status:** Draft

> **Related Documentation**: For security considerations and vulnerability reporting, see [`SECURITY.md`](./SECURITY.md)

## Overview

This document defines the binary format for TrustEdge `.trst` envelope files. The format provides authenticated encryption with provenance for chunked data streams, designed for privacy-preserving edge computing.

---

## 1. File Structure

A `.trst` file consists of:

```
[Preamble] [StreamHeader] [Record]* 
```

- **Preamble**: Magic bytes and version (5 bytes)
- **StreamHeader**: Session metadata (variable length, bincode-encoded)  
- **Record**: Zero or more encrypted data records (variable length each, bincode-encoded)

---

## 2. Preamble (5 bytes)

```
Offset | Size | Field   | Description
-------|------|---------|----------------------------------
0      | 4    | MAGIC   | Magic bytes: "TRST" (0x54525354)
4      | 1    | VERSION | Format version: 0x01
```

**Byte Order**: Fixed bytes, no endianness concerns.

**Validation**:
- MAGIC must exactly match `[0x54, 0x52, 0x53, 0x54]`
- VERSION must be `0x01` for this specification

**Failure Modes**:
- **Invalid Magic**: Not a TrustEdge file, abort parsing
- **Unsupported Version**: Future/unknown format, abort parsing

---

## 3. StreamHeader (Variable Length)

The StreamHeader is bincode-encoded with the following structure:

```rust
struct StreamHeader {
    v: u8,                   // Stream format version (0x01)
    header: Vec<u8>,         // File header bytes (58 bytes)
    header_hash: [u8; 32],   // BLAKE3 hash of header bytes
}
```

### 3.1. Embedded FileHeader (58 bytes)

The `header` field contains a 58-byte FileHeader with this layout:

```
Offset | Size | Field         | Description
-------|------|---------------|--------------------------------
0      | 1    | version       | File format version (0x01)
1      | 1    | alg           | Algorithm ID (0x01 = AES-256-GCM)
2      | 16   | key_id        | Key identifier
18     | 32   | device_id_hash| BLAKE3(device_id || salt)
50     | 4    | nonce_prefix  | Random nonce prefix for session
54     | 4    | chunk_size    | Chunk size in bytes (big-endian)
```

**Byte Order**: All multi-byte fields are **big-endian** except where noted.

**Validation**:
- `header` must be exactly 58 bytes
- `header_hash` must equal `BLAKE3(header)`
- FileHeader fields must pass individual validation (see below)

**Failure Modes**:
- **Wrong Header Length**: Not 58 bytes, abort parsing
- **Hash Mismatch**: Header corrupted or tampered, abort parsing

---

## 4. Record (Variable Length)

Each Record is bincode-encoded with this structure:

```rust
struct Record {
    seq: u64,                    // Sequence number (starts at 1)
    nonce: [u8; 12],            // AES-GCM nonce: prefix(4) || counter(8)
    sm: SignedManifest,         // Signed manifest
    ct: Vec<u8>,                // AES-GCM ciphertext + tag
}
```

### 4.1. SignedManifest

```rust
struct SignedManifest {
    manifest: Vec<u8>,          // bincode(Manifest)
    sig: Vec<u8>,              // Ed25519 signature (64 bytes, domain-separated)
    pubkey: Vec<u8>,           // Ed25519 public key (32 bytes)
}
```

**Signature Domain Separation**: The Ed25519 signature is computed with domain separation to prevent cross-context signature reuse:

```
signature_input = b"trustedge.manifest.v1" || manifest_bytes
signature = Ed25519.sign(signature_input)
```

This ensures signatures are cryptographically bound to TrustEdge manifest context and cannot be reused in other protocols or systems.

### 4.2. Manifest

```rust
struct Manifest {
    v: u8,                     // Manifest version (0x01)
    ts_ms: u64,               // Timestamp (milliseconds since UNIX epoch)
    seq: u64,                 // Sequence number (must match Record.seq)
    header_hash: [u8; 32],    // Must match StreamHeader.header_hash
    pt_hash: [u8; 32],        // BLAKE3 hash of plaintext chunk
    key_id: [u8; 16],         // Must match FileHeader.key_id
    ai_used: bool,            // AI processing flag (placeholder)
    model_ids: Vec<String>,   // Model identifiers (placeholder)
}
```

**Byte Order**: 
- `seq`, `ts_ms`: Little-endian (bincode default)
- Nonce counter (last 8 bytes): **Big-endian** to match sequence

---

## 5. Cryptographic Layout

### 5.1. Nonce Construction

```
nonce = nonce_prefix(4) || sequence_be(8)
```

- `nonce_prefix`: From FileHeader, fixed per session
- `sequence_be`: Record sequence as 8-byte big-endian integer

**Constraints**:
- Nonce must be unique per key
- Sequence numbers must be contiguous starting from 1
- All records in a stream must use the same nonce_prefix

### 5.2. AAD (Additional Authenticated Data)

```
AAD = header_hash(32) || seq_be(8) || nonce(12) || manifest_hash(32)
```

Total length: 84 bytes

**Construction**:
1. `header_hash`: StreamHeader.header_hash (32 bytes)
2. `seq_be`: Record sequence as big-endian u64 (8 bytes)  
3. `nonce`: Full 12-byte nonce (4 bytes)
4. `manifest_hash`: BLAKE3(SignedManifest.manifest) (32 bytes)

### 5.3. AES-GCM Encryption

```
ciphertext = AES256-GCM.encrypt(
    key: [u8; 32],
    nonce: [u8; 12], 
    plaintext: chunk_data,
    aad: AAD
)
```

The resulting ciphertext includes the authentication tag (16 bytes appended).

---

## 6. Validation Rules

### 6.1. Stream-Level Invariants

1. **Magic and Version**: Must match expected values
2. **Header Hash**: StreamHeader.header_hash must equal BLAKE3(StreamHeader.header)
3. **Header Length**: FileHeader must be exactly 58 bytes
4. **Sequence Contiguity**: Record sequences must start at 1 and increment by 1

### 6.2. Record-Level Invariants

1. **Nonce Prefix**: Record.nonce[0..4] must equal FileHeader.nonce_prefix
2. **Nonce Counter**: Record.nonce[4..12] must equal Record.seq (big-endian)
3. **Manifest Sequence**: Manifest.seq must equal Record.seq
4. **Header Hash Binding**: Manifest.header_hash must equal StreamHeader.header_hash
5. **Key ID Binding**: Manifest.key_id must equal FileHeader.key_id
6. **Domain-Separated Signature**: Ed25519 signature must verify over domain-separated message:
   ```
   verify_input = b"trustedge.manifest.v1" || manifest_bytes
   Ed25519.verify(verify_input, signature, public_key) == valid
   ```
7. **Public Key**: Must be valid Ed25519 public key (32 bytes)
8. **Plaintext Hash**: After decryption, BLAKE3(plaintext) must equal Manifest.pt_hash

### 6.3. Cryptographic Invariants

1. **AES-GCM**: Must decrypt and authenticate successfully
2. **AAD Construction**: Must be built correctly from components
3. **Nonce Uniqueness**: No nonce reuse within a key's lifetime

---

## 7. Error Handling

### 7.1. Parse Errors

| Error | Cause | Action |
|-------|-------|--------|
| `BadMagic` | MAGIC ≠ "TRST" | Abort, not a TrustEdge file |
| `UnsupportedVersion` | VERSION ≠ 0x01 | Abort, format not supported |
| `HeaderLengthMismatch` | Header ≠ 58 bytes | Abort, corrupted stream |
| `HeaderHashMismatch` | Hash verification failed | Abort, corrupted/tampered |
| `BincodeError` | Deserialization failed | Abort, corrupted data |

### 7.2. Validation Errors

| Error | Cause | Action |
|-------|-------|--------|
| `NoncePrefix Mismatch` | Record prefix ≠ header prefix | Reject record |
| `NonceCounterMismatch` | Nonce counter ≠ sequence | Reject record |
| `SequenceMismatch` | Manifest.seq ≠ Record.seq | Reject record |
| `HeaderHashMismatch` | Manifest binding failed | Reject record |
| `KeyIdMismatch` | Manifest.key_id ≠ FileHeader.key_id | Reject record |
| `SignatureFailure` | Ed25519 verification failed | Reject record |
| `DecryptionFailure` | AES-GCM decrypt/auth failed | Reject record |
| `PlaintextHashMismatch` | Decrypted hash ≠ manifest | Reject record |
| `SequenceGap` | Non-contiguous sequence | Reject record |

### 7.3. Failure Semantics

- **Parse Errors**: Entire file is invalid, stop processing
- **Validation Errors**: Reject individual record, may continue with next
- **Security Policy**: Fail-closed, reject on any cryptographic failure

---

## 8. Test Vectors

### 8.1. Deterministic Test Vector

TrustEdge includes deterministic test vectors to validate format compliance:

**Test Parameters:**
- **AES-256 Key**: `000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` (hex)
- **Ed25519 Seed**: `4242424242424242424242424242424224242424242424242424242424242424` (hex)
- **Nonce Prefix**: `aabbccdd` (hex)
- **Key ID**: `TEST_KEY_ID_16B!` (ASCII)
- **Input Size**: 32,768 bytes (deterministic pseudo-random)
- **Chunk Size**: 4,096 bytes

**Golden Hash:**
```
BLAKE3(.trst) = 8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c
```

**Usage:**
- Run `cargo test vectors::tests::golden_trst_digest_is_stable` to verify format compliance
- Any intentional format changes require updating the golden hash
- External implementations can use these vectors for validation

### 8.2. Integration Testing

The implementation includes comprehensive integration tests:

1. **Round-trip Test**: Encrypt → envelope → decrypt with known keys
2. **Tamper Detection**: Verify corrupted envelopes fail validation  
3. **CLI Testing**: End-to-end testing via command-line interface

**Test Location**: `tests/vectors.rs` and `src/vectors.rs`

---

## 9. Implementation Notes

### 8.1. Bincode Encoding

- Uses bincode's default configuration (little-endian integers)
- Length-prefixed for variable-length fields (strings, vectors)
- No custom serialization logic required

### 8.2. BLAKE3 Hashing

- Standard BLAKE3 with 256-bit (32-byte) output
- No custom parameters or salt
- Used for header hash, manifest hash, and plaintext hash

### 8.3. Ed25519 Signatures

- Standard Ed25519 as per RFC 8032
- 64-byte signatures, 32-byte public keys
- Signs over the raw bincode(Manifest) bytes

### 8.4. Memory Considerations

- Records can be processed streaming (one at a time)
- No requirement to buffer entire file in memory
- Chunk size limits maximum single-record memory usage

---

## 9. Test Vectors

*Note: Test vectors with known keys and expected outputs will be added in a future revision.*

---

## 10. Version History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2025-08-25 | Initial specification |

---

## 11. References

- [BLAKE3 Specification](https://github.com/BLAKE3-team/BLAKE3-specs)
- [Ed25519 - RFC 8032](https://tools.ietf.org/html/rfc8032)
- [AES-GCM - NIST SP 800-38D](https://csrc.nist.gov/publications/detail/sp/800-38d/final)
- [Bincode Documentation](https://docs.rs/bincode/)

---

**See Also:**
- `src/format.rs` - Reference implementation
- `PROTOCOL.md` - Network protocol specification
- `THREAT_MODEL.md` - Security analysis

---

## Legal & Attribution

**Copyright** © 2025 TRUSTEDGE LABS LLC. All rights reserved.

**License**: This specification is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/TrustEdge-Labs/trustedge) — Privacy and trust at the edge.

**Standards Compliance**: This format specification implements [BLAKE3](https://github.com/BLAKE3-team/BLAKE3-specs), [Ed25519 RFC 8032](https://tools.ietf.org/html/rfc8032), [AES-GCM NIST SP 800-38D](https://csrc.nist.gov/publications/detail/sp/800-38d/final), and [Bincode](https://docs.rs/bincode/) serialization.
