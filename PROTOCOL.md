# TrustEdge Protocol Specification

**Version:** 0.1 (Draft)
**Date:** August 24, 2025

## Overview

This document describes the wire format and protocol for chunk transfer between TrustEdge clients and servers. The protocol is designed for privacy-preserving, authenticated, and integrity-checked streaming of data (e.g., audio) at the edge.

---

## 1. Transport Layer

- **Current:** TCP (with plans for QUIC/TLS in future)
- **Message Framing:** Each message is length-prefixed (u32, big-endian) followed by the message payload.

---

## 2. Message Types

### 2.1. NetworkChunk

Each chunk sent over the wire is serialized as a `NetworkChunk` struct (bincode encoding):

```
struct NetworkChunk {
    sequence: u64,           // Chunk sequence number
    data: Vec<u8>,           // Encrypted chunk data (AES-GCM)
    manifest: Vec<u8>,       // bincode-encoded, signed manifest
    nonce: [u8; 12],         // Nonce used for AES-GCM
    timestamp: u64,          // Seconds since UNIX epoch
}
```

- **Encoding:** bincode
- **Length Prefix:** Each chunk is sent as `[u32 length][bincode(NetworkChunk)]`

### 2.1.1. Envelope File Format

The `.trst` envelope file is a binary format containing:

- **StreamHeader**: version, header bytes (58 bytes), header hash (BLAKE3)
- **Record(s)**: sequence number, nonce (12 bytes: 4-byte prefix + 8-byte counter), signed manifest (with Ed25519 signature), ciphertext (AES-GCM)

All fields are bincode-encoded for compactness and speed.

**Envelope Integrity Invariants:**
- Each record's nonce prefix (first 4 bytes) must match the stream header's nonce prefix.
- The nonce counter (last 8 bytes) must equal the record's sequence number.
- The manifest's `seq` field must match the record's `seq` field.
These invariants are strictly enforced during decryption and help prevent record tampering, replay, or mixing between streams. If any validation fails (e.g., signature, nonce prefix, nonce counter, manifest sequence, hash), the record is rejected and an error is reported.

### 2.2. Manifest

The manifest is a bincode-encoded struct, signed with Ed25519:

```
struct Manifest {
    v: u8,                   // Manifest version
    ts_ms: u64,              // Millisecond timestamp
    seq: u64,                // Chunk sequence
    header_hash: [u8; 32],   // BLAKE3 hash of file/session header
    pt_hash: [u8; 32],       // BLAKE3 hash of plaintext chunk
    ai_used: bool,           // Placeholder for AI usage
    model_ids: Vec<String>,  // Placeholder for model IDs
}

struct SignedManifest {
    manifest: Vec<u8>,       // bincode(Manifest)
    sig: Vec<u8>,            // Ed25519 signature
    pubkey: Vec<u8>,         // Ed25519 public key
}
```


### 2.2. Fields and AAD recipe

- **AAD** = BLAKE3(header) || seq_be(8) || nonce(12) || BLAKE3(manifest_bytes)
- **Nonce** = nonce_prefix(4) || seq_be(8) (unique per key/session)
- **Record** = seq, nonce, signed_manifest { manifest_bytes, ed25519_sig, pubkey }, ct

**Integrity check:** See above: nonce prefix, nonce counter, and manifest sequence integrity are enforced for every record.

---


## 3. Protocol Flow

1. **Connection Establishment:**
   - Client connects to server (TCP, future: QUIC/TLS).
2. **Chunk Transfer:**
   - Client sends a sequence of length-prefixed `NetworkChunk` messages.
   - Server receives, validates, and (optionally) decrypts each chunk.
3. **Validation:**
    - Server checks manifest signature, nonce, sequence, timestamp, and that:
       - The record's nonce prefix matches the stream header's prefix
       - The nonce counter matches the record's sequence number
       - The manifest's `seq` matches the record's sequence number
   - Decrypts chunk using provided nonce and key (see Key Selection below).
   - Verifies plaintext hash matches manifest.
   - If any validation fails (e.g., signature, nonce prefix, hash), the record is rejected and an error is reported/logged.
4. **Acknowledgment (Future):**
   - Protocol may be extended to include ACKs, error reporting, or flow control.

---


## 4. Security Considerations

- **Confidentiality:** AES-256-GCM per chunk
- **Integrity:** Ed25519 signatures on manifests, AES-GCM tags, and nonce prefix integrity
- **Replay Protection:** Sequence numbers and timestamps
- **Extensibility:** Protocol is versioned and designed for future upgrades (e.g., QUIC, mutual TLS, chunk reordering, error handling)

### Key Selection

- The decryption key must be provided via `--key-hex` (64-char hex) or derived from the keyring using `--use-keyring` and `--salt-hex` (32 hex chars, 16 bytes).
- In decrypt mode, one of these must be provided; random key is not allowed.
- In encrypt mode, if neither is provided, a random key is generated and optionally saved with `--key-out`.

**Mutual Exclusivity:** `--key-hex` and `--use-keyring` are mutually exclusive. Only one may be used at a time.

**PBKDF2 Parameters:**
- PBKDF2 with SHA-256
- 100,000 iterations
- 16-byte (32 hex char) salt (required with `--use-keyring`)

---

## 5. Example Message (Hex Dump)

```
[00 00 01 2A] [bincode(NetworkChunk...)]
// 4-byte length prefix, then bincode-encoded struct
```

---

## 6. Future Extensions

- Encrypted transport (QUIC/TLS)
- Chunk acknowledgments and retransmission
- Session management and authentication
- Error reporting and diagnostics
- Support for additional data types (e.g., video, sensor data)

---

## 7. Error Handling

If any validation fails during decryption (e.g., manifest signature, nonce prefix, nonce counter, manifest sequence, header hash, or plaintext hash), the record is rejected and an error is reported or logged. This ensures that tampered, out-of-sequence, or replayed records cannot be decrypted or accepted.

---

**See also:**
- `src/lib.rs` for struct definitions
- `src/main.rs` for CLI and envelope logic
- `trustedge-client` and `trustedge-server` for protocol usage
- `THREAT_MODEL.md` for security rationale

---

**Protocol Versioning:**
The protocol is versioned (see StreamHeader and file preamble). Future changes will increment the version and document compatibility requirements.
