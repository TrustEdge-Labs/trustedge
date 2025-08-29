<!--
Copyright (c) 2025 John Turner
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
-->

# TrustEdge Protocol Specification

**Version:** 1.0 (Stable)
**Date:** August 28, 2025

> **Related Documentation**: For security policies and vulnerability reporting, see [`SECURITY.md`](./SECURITY.md)

## Overview

This document describes the wire format and network protocol for chunk transfer between TrustEdge clients and servers. The protocol provides privacy-preserving, authenticated, and integrity-checked streaming of data with comprehensive validation and tamper detection.

**Key Features:**
- **TCP-based transport** with plans for QUIC/TLS
- **Length-prefixed message framing** for reliable parsing
- **Comprehensive validation** with strict security invariants
- **Real-time processing** with ACK/response flow
- **Complete tamper detection** preventing replay and reordering attacks

---

## 1. Transport Layer

- **Current:** TCP with comprehensive validation and error handling
- **Message Framing:** Length-prefixed (u32, little-endian) followed by bincode-encoded payload
- **Future:** QUIC/TLS for enhanced security and performance
- **Flow Control:** ACK/response protocol with error reporting

---

## 2. Message Types

### 2.1. NetworkChunk

Each chunk sent over the wire is serialized as a `NetworkChunk` struct (bincode encoding):

```rust
struct NetworkChunk {
    sequence: u64,           // Chunk sequence number (starting from 1)
    data: Vec<u8>,           // Encrypted chunk data (AES-GCM)
    manifest: Vec<u8>,       // bincode-encoded, signed manifest
    nonce: [u8; 12],         // Nonce used for AES-GCM (prefix + counter)
    timestamp: u64,          // Seconds since UNIX epoch
}
```

**Transport Encoding:**
- **Serialization:** bincode (efficient binary format)
- **Frame Format:** `[u32 length][bincode(NetworkChunk)]`
- **Validation:** Each chunk undergoes comprehensive validation on receipt

**Chunk Validation:**
- Data and manifest must not be empty
- Timestamp must not be more than 5 minutes in the future
- Sequence numbers must be contiguous (no gaps or duplicates)
- Nonce prefix must match session header prefix
- All cryptographic validation must pass

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
    key_id: [u8; 16],        // Key identifier for rotation support
    ai_used: bool,           // Placeholder for AI usage
    model_ids: Vec<String>,  // Placeholder for model IDs
}

struct SignedManifest {
    manifest: Vec<u8>,       // bincode(Manifest)
    sig: Vec<u8>,            // Ed25519 signature
    pubkey: Vec<u8>,         // Ed25519 public key
}

struct FileHeader {
    version: u8,             // File format version
    alg: u8,                 // Algorithm identifier (1 = AES-256-GCM)
    key_id: [u8; 16],        // Key identifier (matches manifest.key_id)
    device_id_hash: [u8; 32], // BLAKE3 hash of device ID + salt
    nonce_prefix: [u8; 4],   // Random nonce prefix for session
    chunk_size: u32,         // Chunk size in bytes (big-endian)
}
```


### 2.2. Fields and AAD recipe

- **AAD** = BLAKE3(header) || seq_be(8) || nonce(12) || BLAKE3(manifest_bytes)
- **Nonce** = nonce_prefix(4) || seq_be(8) (unique per key/session)
- **Record** = seq, nonce, signed_manifest { manifest_bytes, ed25519_sig, pubkey }, ct

**Integrity check:** See above: nonce prefix, nonce counter, and manifest sequence integrity are enforced for every record.

---


## 3. Protocol Flow

### 3.1. Connection and Session Setup

1. **Connection Establishment:**
   - Client connects to server via TCP
   - Server allocates per-connection state and session tracking
   - Connection ID assigned for logging and debugging

2. **Session Initialization:**
   - First valid chunk establishes session parameters
   - Header hash, nonce prefix, and key ID locked for session
   - Sequence tracking initialized (expecting sequence 1)

### 3.2. Chunk Processing Pipeline

1. **Chunk Reception:**
   - Server reads length-prefixed NetworkChunk from socket
   - Basic structure validation (non-empty data/manifest, timestamp bounds)

2. **Cryptographic Validation:**
   - Manifest signature verification (Ed25519)
   - Key ID validation against configured keys
   - Nonce prefix and counter validation
   - Sequence number continuity check

3. **Decryption and Integrity:**
   - AES-GCM decryption with AAD binding
   - Plaintext hash verification against manifest
   - Header consistency validation

4. **Processing and Storage:**
   - Optional plaintext storage to output directory
   - Session state update (next expected sequence)
   - ACK response to client (future enhancement)

### 3.3. Validation Enforcement

**Critical Security Invariants (strictly enforced):**
- Record nonce prefix must match stream header nonce prefix
- Nonce counter (last 8 bytes) must equal record sequence number
- Manifest sequence must match record sequence number
- Manifest header hash must match stream header hash
- Manifest key ID must match file header key ID
- Sequence numbers must be strictly contiguous (no gaps or reuse)

**Failure Response:**
- Any validation failure immediately rejects the chunk
- Detailed error logging for debugging and monitoring
- Connection may be terminated for security violations

---


## 4. Security Considerations

- **Confidentiality:** AES-256-GCM per chunk
- **Integrity:** Ed25519 signatures on manifests, AES-GCM tags, and nonce prefix integrity
- **Replay Protection:** Sequence numbers and timestamps
- **Extensibility:** Protocol is versioned and designed for future upgrades (e.g., QUIC, mutual TLS, chunk reordering, error handling)

### Key Selection & Backend Architecture

**Current Implementation:**
- The decryption key must be provided via `--key-hex` (64-char hex) or derived from the keyring using `--use-keyring` and `--salt-hex` (32 hex chars, 16 bytes).
- In decrypt mode, one of these must be provided; random key is not allowed.
- In encrypt mode, if neither is provided, a random key is generated and optionally saved with `--key-out`.

**Planned Modular Backend System (Phase 2):**
The key management system will be refactored to support pluggable backends:

```rust
trait KeyBackend {
    fn derive_key(&self, key_id: &[u8; 16], context: &KeyContext) -> Result<[u8; 32]>;
    fn store_key(&self, key_id: &[u8; 16], key_data: &[u8; 32]) -> Result<()>;
    fn rotate_key(&self, old_id: &[u8; 16], new_id: &[u8; 16]) -> Result<()>;
    fn list_keys(&self) -> Result<Vec<KeyMetadata>>;
}

// Backend implementations:
// - KeyringBackend (current PBKDF2 implementation)
// - TpmBackend (TPM 2.0 integration)
// - HsmBackend (Hardware Security Module)
// - MatterBackend (Matter certificate-based keys)
```

**Backend Selection CLI (Planned):**
```bash
# Use keyring backend (current default)
--backend keyring --salt-hex <salt>

# Use TPM backend
--backend tpm --device-path /dev/tpm0 --key-handle <handle>

# Use HSM backend  
--backend hsm --pkcs11-lib /usr/lib/libpkcs11.so --slot-id 0

# Use Matter certificate backend
--backend matter --fabric-id <id> --device-cert <path>
```

**Migration Between Backends (Planned):**
```bash
# Migrate from keyring to TPM
trustedge-audio --migrate-backend \
  --from keyring --salt-hex <salt> \
  --to tpm --device-path /dev/tpm0 \
  --key-id <key-id>
```

**Mutual Exclusivity:** Backend selection flags are mutually exclusive. Only one backend may be specified per operation.

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

## 6. Future Extensions & Planned Features

### 6.1 Live Audio Streaming Protocol
**Status:** Planned for Phase 3

- **Real-time chunk processing** with configurable latency targets
- **Audio-specific metadata** in manifest (sample rate, channels, format)
- **Temporal synchronization** for live playback scenarios
- **Buffer management** for network jitter and packet loss recovery

**Planned Message Extensions:**
```rust
struct AudioChunk {
    chunk: NetworkChunk,         // Standard encrypted chunk
    sample_rate: u32,            // Audio sample rate (Hz)
    channels: u8,                // Number of audio channels
    format: AudioFormat,         // PCM format specification
    duration_ms: u32,            // Chunk duration in milliseconds
}

struct LiveStreamMetadata {
    session_id: [u8; 16],        // Unique session identifier
    device_id: Vec<u8>,          // Audio capture device identifier
    start_timestamp: u64,        // Session start time (Unix epoch)
    expected_duration: Option<u64>, // Expected session duration (ms)
}
```

### 6.2 Matter Device Integration Protocol
**Status:** Planned for Phase 4

- **Certificate-based device authentication** using Matter credentials
- **Device onboarding workflow** with local test CA simulation
- **Matter device ID mapping** to TrustEdge envelope manifests
- **Commissioning protocol integration** for seamless device addition

**Planned Extensions:**
```rust
struct MatterDeviceManifest {
    matter_device_id: [u8; 8],   // Matter 64-bit device identifier
    fabric_id: [u8; 8],          // Matter fabric identifier
    certificate_chain: Vec<Vec<u8>>, // X.509 certificate chain
    commissioning_data: Vec<u8>, // Device commissioning information
}

struct DeviceAttestationData {
    attestation_nonce: [u8; 32], // Attestation challenge nonce
    device_signature: Vec<u8>,   // Device-signed attestation
    timestamp: u64,              // Attestation timestamp
}
```

### 6.3 Enhanced Key Management Protocol
**Status:** Planned for Phase 2

- **Multi-backend key derivation** (keyring, TPM, HSM)
- **Key rotation protocol** with backward compatibility
- **Hardware security module integration** for enterprise deployments
- **Distributed key management** for multi-device scenarios

**Key Rotation Message Flow:**
```
Client → Server: KeyRotationRequest {
    old_key_id: [u8; 16],
    new_key_id: [u8; 16], 
    rotation_signature: Vec<u8>,
    effective_timestamp: u64
}

Server → Client: KeyRotationResponse {
    status: RotationStatus,
    confirmed_key_id: [u8; 16],
    migration_required: bool
}
```

### 6.4 Advanced Transport Features
**Status:** Planned for Phase 4

- **QUIC transport layer** for improved performance and security
- **Mutual TLS authentication** for production deployments
- **Connection pooling** and multiplexing for high-throughput scenarios
- **Chunk retransmission** and error recovery protocols

### 6.5 Audit and Compliance Extensions
**Status:** Planned for Phase 5

- **Comprehensive audit logging** with structured JSON output
- **Compliance report generation** for regulatory requirements
- **Chain of custody tracking** for forensic applications
- **Tamper evidence reporting** with detailed failure analysis

---

## 8. Test Vectors and Validation

### 8.1. Deterministic Test Vectors

TrustEdge provides deterministic test vectors to verify protocol compliance:

**Test Configuration:**
- **32KB input data**: Deterministic pseudo-random bytes via LCG
- **4KB chunks**: Standard chunking for realistic testing
- **Fixed cryptographic material**: Known AES-256 and Ed25519 keys
- **Golden envelope hash**: `8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c`

**Validation Tests:**
1. **Format compliance**: `cargo test vectors::tests::golden_trst_digest_is_stable`
2. **Round-trip integrity**: `cargo test golden_envelope_roundtrip`  
3. **Tamper detection**: `cargo test tamper_fails_on_manifest_change`

### 8.2. Integration Testing

The implementation includes end-to-end protocol testing:

- **CLI-based testing**: Real binary execution via command line
- **File-based round-trips**: Input → encrypt → envelope → decrypt → verify
- **Network protocol testing**: Client/server chunk transfer validation
- **Error injection**: Corruption detection and failure modes

**Test Execution:**
```bash
cargo test                     # All tests
cargo test --test vectors      # Integration tests only
cargo test golden_trst         # Format compliance only
```

---

## 9. Error Handling

If any validation fails during decryption (e.g., manifest signature, nonce prefix, nonce counter, manifest sequence, key ID mismatch, header hash, or plaintext hash), the record is rejected and an error is reported or logged. This ensures that tampered, out-of-sequence, replayed, or incorrectly keyed records cannot be decrypted or accepted.

---

**See also:**
- `src/format.rs` — Centralized format definitions and constants
- `src/lib.rs` for struct definitions
- `src/main.rs` for CLI and envelope logic
- `trustedge-client` and `trustedge-server` for protocol usage
- `THREAT_MODEL.md` for security rationale

---

**Protocol Versioning:**
The protocol is versioned (see StreamHeader and file preamble). Future changes will increment the version and document compatibility requirements.

---

## Legal & Attribution

**Copyright** © 2025 John Turner. All rights reserved.

**License**: This specification is licensed under the [Mozilla Public License 2.0 (MPL-2.0)](https://mozilla.org/MPL/2.0/).

**Project**: [TrustEdge](https://github.com/johnzilla/trustedge) — Privacy and trust at the edge.

**Standards**: This specification references [BLAKE3](https://github.com/BLAKE3-team/BLAKE3-specs), [Ed25519 RFC 8032](https://tools.ietf.org/html/rfc8032), and [AES-GCM NIST SP 800-38D](https://csrc.nist.gov/publications/detail/sp/800-38d/final).
