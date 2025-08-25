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

---

## 3. Protocol Flow

1. **Connection Establishment:**
   - Client connects to server (TCP, future: QUIC/TLS).
2. **Chunk Transfer:**
   - Client sends a sequence of length-prefixed `NetworkChunk` messages.
   - Server receives, validates, and (optionally) decrypts each chunk.
3. **Validation:**
   - Server checks manifest signature, nonce, sequence, and timestamp.
   - Decrypts chunk using provided nonce and key.
   - Verifies plaintext hash matches manifest.
4. **Acknowledgment (Future):**
   - Protocol may be extended to include ACKs, error reporting, or flow control.

---

## 4. Security Considerations

- **Confidentiality:** AES-256-GCM per chunk
- **Integrity:** Ed25519 signatures on manifests, AES-GCM tags
- **Replay Protection:** Sequence numbers and timestamps
- **Extensibility:** Protocol is versioned and designed for future upgrades (e.g., QUIC, mutual TLS, chunk reordering, error handling)

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

**See also:**
- `src/lib.rs` for struct definitions
- `trustedge-client` and `trustedge-server` for protocol usage
- `THREAT_MODEL.md` for security rationale
