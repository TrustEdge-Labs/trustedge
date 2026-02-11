//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

#![cfg_attr(docsrs, feature(doc_cfg))]

//! # TrustEdge Core
//!
//! Core cryptographic library and CLI tools for privacy-preserving edge computing.
//!
//! TrustEdge Core provides production-ready cryptographic primitives, universal backend systems,
//! and secure network operations for data-agnostic encryption at the edge.
//!
//! ## Key Features
//!
//! - **Production Cryptography**: AES-256-GCM encryption with PBKDF2 key derivation
//! - **Universal Backend System**: Pluggable crypto operations (Software HSM, Keyring, YubiKey)
//! - **Live Audio Capture**: Real-time microphone input with configurable quality
//! - **Network Operations**: Secure client-server communication with mutual authentication
//! - **Hardware Integration**: Full YubiKey PKCS#11 support with real hardware signing
//! - **Algorithm Agility**: Configurable cryptographic algorithms with forward compatibility
//! - **Memory Safety**: Proper key material cleanup with zeroization
//!
//! ## Quick Start
//!
//! ```rust
//! use trustedge_core::Envelope;
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Generate key pairs
//! let sender_key = SigningKey::generate(&mut OsRng);
//! let recipient_key = SigningKey::generate(&mut OsRng);
//!
//! // Encrypt data
//! let data = b"Secret message";
//! let envelope = Envelope::seal(data, &sender_key, &recipient_key.verifying_key())?;
//!
//! // Decrypt data
//! let decrypted = envelope.unseal(&recipient_key)?;
//! assert_eq!(decrypted, data);
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! TrustEdge Core is organized into several key modules:
//!
//! - [`envelope`] - Cryptographic envelope format for secure data packaging
//! - [`backends`] - Universal Backend system for pluggable crypto operations
//! - [`audio`] - Live audio capture functionality (requires `audio` feature)
//! - [`auth`] - Network authentication and session management
//! - [`transport`] - Network transport layer for secure communication
//! - [`asymmetric`] - Public key cryptography operations
//! - [`format`] - Data format handling and MIME type detection
//!
//! ## Feature Flags
//!
//! TrustEdge Core uses `default = []` (no features enabled) for fast CI and maximum portability.
//! Enable features as needed for your deployment:
//!
//! ### Backend Features
//!
//! Hardware and storage integrations:
//!
//! - **`yubikey`** — YubiKey PIV hardware signing and key management via PKCS#11.
//!   Requires PCSC libraries (`libpcsclite-dev` on Linux, built-in on macOS).
//!
//! ### Platform Features
//!
//! I/O and system capabilities:
//!
//! - **`audio`** — Live audio capture from microphones via cpal.
//!   Requires audio libraries (`libasound2-dev` on Linux, CoreAudio on macOS, WASAPI on Windows).
//!
//! ### Usage
//!
//! ```toml
//! [dependencies]
//! trustedge-core = { version = "0.2", features = ["audio"] }
//! ```

use serde::{Deserialize, Serialize};

/// The length of the nonce used for AES-GCM encryption (12 bytes).
pub const NONCE_LEN: usize = 12;

pub mod archive;
pub mod asymmetric;
pub mod audio;
pub mod auth;
pub mod backends;
pub mod chain;
pub mod crypto;
pub mod envelope;
pub mod envelope_v2_bridge;
pub mod error;
pub mod format;
pub mod hybrid;
pub mod transport;
pub mod vectors;

// Layer hierarchy (Phase 1 scaffolding -- populated in later phases)
pub mod primitives;
pub mod protocols;
pub mod applications;
pub mod io;

pub use archive::{archive_dir_name, read_archive, validate_archive, write_archive, ArchiveError};
pub use asymmetric::{
    decrypt_key_asymmetric, encrypt_key_asymmetric, key_exchange, AsymmetricError, KeyPair,
    PrivateKey, PublicKey,
};
#[cfg(feature = "audio")]
#[cfg_attr(docsrs, doc(cfg(feature = "audio")))]
pub use audio::AudioCapture;
pub use audio::{AudioChunk, AudioConfig};
pub use auth::{
    client_authenticate, server_authenticate, AuthChallenge, AuthMessage, AuthMessageType,
    ClientAuthResponse, ServerAuthConfirm, ServerCertificate, SessionInfo, SessionManager,
    SESSION_ID_SIZE, SESSION_TIMEOUT,
};
pub use backends::{
    AsymmetricAlgorithm,
    BackendCapabilities,
    BackendInfo,
    BackendPreferences,
    BackendRegistry,
    CryptoOperation,
    CryptoResult,
    HashAlgorithm,
    KeyBackend,
    KeyContext,
    KeyDerivationContext,
    KeyMetadata,
    KeyringBackend,
    SignatureAlgorithm,
    SymmetricAlgorithm,
    // Universal backend system (new)
    UniversalBackend,
    UniversalBackendRegistry,
    UniversalKeyringBackend,
};
pub use chain::{
    blake3_hex_or_b64, chain_next, genesis, segment_hash, validate_chain, ChainError, ChainSegment,
};
pub use crypto::{
    decrypt_segment, encrypt_segment, format_nonce, generate_aad, generate_nonce24, parse_nonce,
    sign_manifest, verify_manifest, CryptoError, DeviceKeypair,
};
pub use envelope::{Envelope, EnvelopeMetadata};
pub use error::{
    TrustEdgeError,
    BackendError,
    TransportError,
};
pub use envelope_v2_bridge::{
    detect_envelope_format, EnvelopeFormat, EnvelopeInfo, UnifiedEnvelope,
};
pub use format::*;
pub use hybrid::{open_envelope, seal_for_recipient, HybridEncryptionError, SymmetricKey};
pub use trustedge_trst_protocols::archive::manifest::{
    CamVideoManifest, CaptureInfo, ChunkInfo, DeviceInfo, SegmentInfo,
};
pub use error::ManifestError;  // ManifestError is re-exported from error.rs (which aliases ManifestFormatError)
pub use transport::{Transport, TransportConfig, TransportFactory};

// Receipt system re-exports (Layer 4 applications)
pub use applications::receipts::{Receipt, create_receipt, assign_receipt, extract_receipt, verify_receipt_chain};

// Attestation system re-exports (Layer 4 applications)
pub use applications::attestation::{
    Attestation, AttestationConfig, AttestationResult,
    OutputFormat, KeySource, VerificationConfig, VerificationResult,
    VerificationDetails, VerificationInfo,
    create_signed_attestation, verify_attestation,
};

/// Represents a chunk of data sent over the network, including encrypted data,
/// a signed manifest, the nonce used for encryption, and a timestamp.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkChunk {
    /// The sequence number of the chunk in the stream.
    pub sequence: u64,
    /// The encrypted data payload for this chunk.
    pub data: Vec<u8>,
    /// The serialized, signed manifest (bincode-encoded).
    pub manifest: Vec<u8>,
    /// The nonce used for AES-GCM encryption.
    pub nonce: [u8; NONCE_LEN],
    /// The timestamp (seconds since UNIX epoch) when the chunk was created.
    pub timestamp: u64,
}

impl NetworkChunk {
    /// Creates a new `NetworkChunk` with the given sequence number, encrypted data, and manifest.
    /// The nonce is set to zero and should be set explicitly after creation.
    pub fn new(seq: u64, encrypted_data: Vec<u8>, manifest_bytes: Vec<u8>) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            sequence: seq,
            data: encrypted_data,
            manifest: manifest_bytes,
            nonce: [0; NONCE_LEN], // Default nonce - should be set explicitly
            timestamp,
        }
    }

    /// Creates a new `NetworkChunk` with the given sequence number, encrypted data, manifest, and explicit nonce.
    pub fn new_with_nonce(
        seq: u64,
        encrypted_data: Vec<u8>,
        manifest_bytes: Vec<u8>,
        nonce: [u8; NONCE_LEN],
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            sequence: seq,
            data: encrypted_data,
            manifest: manifest_bytes,
            nonce,
            timestamp,
        }
    }

    /// Validates the `NetworkChunk`.
    ///
    /// Checks that the data and manifest are not empty, and that the timestamp is not more than 5 minutes in the future.
    /// Returns `Ok(())` if valid, or an error otherwise.
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.data.is_empty() {
            return Err(anyhow::anyhow!("Chunk data is empty"));
        }
        if self.manifest.is_empty() {
            return Err(anyhow::anyhow!("Manifest is empty"));
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| anyhow::anyhow!("Time error: {}", e))?
            .as_secs();

        if self.timestamp > now + 300 {
            // Not more than 5 minutes in future
            return Err(anyhow::anyhow!("Chunk timestamp is too far in the future"));
        }

        Ok(())
    }
}
