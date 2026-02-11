<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Architecture

**Analysis Date:** 2026-02-09

## Pattern Overview

**Overall:** Modular Rust workspace with capability-based cryptographic abstraction and pluggable backend system.

**Key Characteristics:**
- Workspace-based organization with 10 specialized crates under `crates/`
- Universal Backend system for pluggable crypto operations (Software HSM, Keyring, YubiKey)
- Envelope abstraction for secure data packaging with Ed25519 signing and AES-256-GCM encryption
- Trait-based transport abstraction (TCP with length-delimited framing, QUIC with TLS)
- Algorithm-agility support with forward compatibility (format version 2 with algorithm negotiation)
- Layered design separating business logic (receipts, attestation) from cryptographic primitives (core)

## Layers

**Core Cryptography Layer (`crates/core/src/`):**
- Purpose: Production cryptographic primitives and network operations
- Location: `crates/core/`
- Contains: Envelope format, crypto operations, backends, transport, manifest handling
- Depends on: Ed25519-Dalek, AES-GCM, ChaCha20-Poly1305, PBKDF2, BLAKE3
- Used by: CLI tools, receipts, attestation, WASM bindings, Pubky integration

**Backend Abstraction Layer (`crates/core/src/backends/`):**
- Purpose: Pluggable key management and crypto operation dispatch
- Location: `crates/core/src/backends/`
- Contains: Universal trait, Software HSM, OS Keyring, YubiKey PKCS#11, registry
- Key types: `UniversalBackend` trait, `CryptoOperation` enum, `BackendCapabilities`
- Depends on: PKCS#11 (optional), YubiKey driver (optional), System keyring
- Pattern: Capability-based dispatch - backends advertise what they support

**Transport Layer (`crates/core/src/transport/`):**
- Purpose: Protocol abstraction for network communication
- Location: `crates/core/src/transport/`
- Contains: TCP with length-delimited codec, QUIC with TLS 1.3
- Abstraction: `Transport` async trait with connect/send_chunk/receive_chunk/close
- Depends on: Tokio, Quinn (QUIC), Rustls
- Used by: Server/client binaries, network features in CLI

**Business Logic Layer:**
- **Receipts** (`crates/receipts/`): Transferable ownership claims with cryptographic chain links
  - Depends on: Core Envelope abstraction
  - Serialization: Serde JSON inside envelopes
- **Attestation** (`crates/attestation/`): Software "birth certificates" with provenance tracking
  - Depends on: Core Envelope abstraction
  - Formats: JSON-only or sealed-envelope output
- **Archive System** (`crates/trst-core/` + `crates/trst-cli/`): .trst format for cam.video manifests
  - Canonical manifest types in trst-core (WASM-compatible)
  - Wrap/verify CLI in trst-cli

**CLI/Application Layer:**
- **Main CLI** (`crates/trustedge-cli/`): Envelope encryption with file or audio input
- **Server/Client Binaries** (`crates/core/src/bin/`): Network operations
  - Server: Listens for chunks, validates, decrypts (optional)
  - Client: Connects to server, sends encrypted chunks
  - Auth support: Ed25519 mutual authentication with sessions
- **Demo Binaries**: Software HSM demo, YubiKey demo, archive inspector

**Integration Layer:**
- **Pubky Integration** (`crates/pubky/`, `crates/pubky-advanced/`): Community-contributed
  - Simple adapter for Pubky network key publishing
  - Hybrid encryption with X25519 ECDH (advanced version)
- **WASM Bindings** (`crates/wasm/`, `crates/trst-wasm/`): Browser integration
  - Compiled to WebAssembly for client-side operations
  - Re-exports public API suitable for JS bindings

## Data Flow

**Encryption/Envelope Creation:**

1. Input → `InputReader` trait (file via `FileInputReader` or audio via `AudioInputReader`)
2. Chunking: Data split into 64KB chunks (configurable)
3. Per-chunk encryption: XChaCha20-Poly1305 or AES-256-GCM per `format::AeadAlgorithm`
4. Manifest creation: Signed metadata with device info, capture info, chunk info
5. Envelope construction: `Envelope` aggregates encrypted chunks + verifying keys + metadata
6. Transport: Chunks sent as `NetworkChunk` (sequence, data, manifest, nonce, timestamp)

**Network Data Flow (Client → Server):**

```
Client StreamHeader (magic "TRST", version, algorithms)
    ↓
Per-chunk NetworkChunk (bincode-serialized):
    ├─ sequence: u64
    ├─ data: Vec<u8> (encrypted payload)
    ├─ manifest: Vec<u8> (signed manifest bincode)
    ├─ nonce: [u8; 12]
    └─ timestamp: u64
    ↓
Server receives and validates:
    ├─ Manifest deserialization + signature verification
    ├─ Nonce uniqueness check
    ├─ Timestamp validation (not >5 min in future)
    └─ Optional decryption (if --decrypt flag)
    ↓
Server ACKs with sequence number (optional session MAC)
```

**Envelope Decryption:**

1. Envelope received (contains all chunks + metadata)
2. For each chunk:
   - Extract manifest and verify signature
   - Derive encryption key using PBKDF2(sender_pubkey || recipient_pubkey || salt || sequence)
   - Decrypt with XChaCha20-Poly1305 using AAD = sender || recipient || nonce || metadata_hash
3. Reassemble plaintext from chunk order
4. Return decrypted payload + metadata

**State Management:**

- **Session State** (`auth.rs`): Server maintains `SessionManager` with 30-min timeout
  - Session ID: 16 random bytes
  - Challenge-response: Ed25519 signature verification
  - Timestamp-based expiry
- **Backend Registry** (`backends/universal_registry.rs`): Application singleton
  - Available backends discovered at startup
  - Capability discovery cached per backend
- **Transport Connection State**: Per-connection limits enforced
  - Max bytes: 1GB default
  - Max chunks: 10k default
  - Idle timeout: 5 minutes

## Key Abstractions

**Envelope (High-level steering wheel):**
- Purpose: Simple interface hiding NetworkChunk/Record complexity
- Location: `crates/core/src/envelope.rs`
- Provides: `seal()` for encryption, `unseal()` for decryption
- Internal: Chunking, key derivation, manifest signing/verification
- Pattern: Builder-like interface with metadata

**NetworkChunk (Transport unit):**
- Purpose: Atomic unit sent over network
- Location: `crates/core/src/lib.rs`
- Fields: sequence, encrypted data, signed manifest, nonce, timestamp
- Validation: Manifest non-empty, data non-empty, timestamp reasonable
- Serialization: Bincode for wire format

**Universal Backend (Capability-based dispatch):**
- Purpose: Pluggable crypto operations without monolithic trait
- Location: `crates/core/src/backends/universal.rs`
- Abstraction: `CryptoOperation` enum with variant per operation
- Backends implement: `supports_operation()` + `perform_operation()`
- Prevents: Backends forced to implement unsupported operations

**Transport Abstraction:**
- Purpose: Swap protocols without changing application logic
- Location: `crates/core/src/transport/mod.rs`
- Trait: `Transport` with async send/receive/connect/close
- Implementations: `TcpTransport`, `QuicTransport`
- Factory: `TransportFactory::create_tcp()`, `TransportFactory::create_quic()`

**Manifest Types (Format):**
- Purpose: Canonical JSON serialization with algorithm agility
- Location: `crates/core/src/manifest.rs`
- Contains: Device info (id, pubkey), capture info (timestamp, format), chunk info (hash, size), segment info
- Signature: Ed25519, stored separately for detached verification
- Format version: 2 (negotiated at stream start)

**Signed Manifest:**
- Purpose: Tamper-proof metadata
- Location: `crates/core/src/format.rs`
- Contains: Serialized manifest + Ed25519 signature
- Verification: Always checked before trusting metadata

**Receipt (Business logic):**
- Purpose: Transferable ownership claim
- Location: `crates/receipts/src/lib.rs`
- Fields: issuer pubkey, beneficiary pubkey, amount, prev_envelope_hash, description, timestamp
- Chain link: References previous receipt via envelope hash
- Serialization: Serde JSON, stored inside envelope

## Entry Points

**CLI Main:**
- Location: `crates/trustedge-cli/src/main.rs`
- Triggers: `trustedge` binary with subcommands
- Responsibilities:
  - Parse arguments (input source, output path, encryption key, backend type)
  - Select input reader (file or audio)
  - Select backend (keyring or pubky)
  - Encrypt or decrypt data
  - Write output to disk

**Server Binary:**
- Location: `crates/core/src/bin/trustedge-server.rs`
- Triggers: `trustedge-server` with listen address
- Responsibilities:
  - Listen for TCP connections
  - Receive NetworkChunk messages
  - Validate manifests and signatures
  - Optionally decrypt and save plaintext
  - Send ACKs with session IDs
  - Enforce resource limits per connection

**Client Binary:**
- Location: `crates/core/src/bin/trustedge-client.rs`
- Triggers: `trustedge-client` with server address and input file
- Responsibilities:
  - Connect to server
  - Read input file in chunks
  - Encrypt with derived key
  - Send NetworkChunk to server
  - Receive ACKs

**Archive CLI:**
- Location: `crates/trst-cli/src/main.rs`
- Triggers: `trst` binary with wrap/verify subcommands
- Responsibilities:
  - Wrap: Create .trst archive with manifest and signatures
  - Verify: Validate archive structure and device signatures

## Error Handling

**Strategy:** Two-tiered error system
- Library crates: `thiserror` for structured error types
- CLI applications: `anyhow` for ad-hoc error handling with context

**Error Types by Module:**
- `CryptoError`: Encryption/decryption failures, key format issues, signature verification
- `ChainError`: Continuity chain validation failures
- `ManifestError`: Manifest serialization/deserialization
- `ArchiveError`: Archive format violations
- `TrustEdgeError`: High-level envelope operations

**Pattern - Result types:**
```rust
// Library crate (thiserror)
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
}

// CLI application (anyhow)
fn main() -> Result<()> {
    let data = std::fs::read(path).context("failed to read input")?;
    Ok(())
}
```

## Cross-Cutting Concerns

**Logging:** Not centralized; uses `RUST_LOG` environment variable with debug output via `eprintln!`
- Pattern: Manual error/warning printing to stderr
- Verbosity controlled by `--verbose` flag in CLI tools

**Validation:** Multi-layered
- Input: Manifest signature verification before trusting metadata
- Network: Timestamp checks, nonce uniqueness
- Stream: Length-delimited codec prevents message fragmentation

**Authentication:** Ed25519-based mutual auth (optional)
- Server generates server certificate with signing key
- Client generates certificate with identity
- Challenge-response: Server sends challenge, client signs and verifies server signature
- Sessions: 30-minute timeout, per-connection sequence numbers

**Key Material Cleanup:** Zeroization on drop
- Pattern: `zeroize::Zeroize` trait on sensitive types
- DeviceKeypair: Secrets zeroized in Drop impl
- PBKDF2 output: Explicitly zeroized after use

**Algorithm Selection:** Explicit negotiation at stream header
- Format fields: `aead_algorithm`, `signature_algorithm`, `hash_algorithm` (u8 enums)
- Deserialization: `TryFrom<u8>` implementation validates supported algorithms
- Forward compatibility: Version 2 header reserves algorithm space (1-127 standard, 128-255 experimental)

---

*Architecture analysis: 2026-02-09*
