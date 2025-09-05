//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
//
/// lib.rs - Core library for privacy-preserving edge data processing
//
// Provides audio capture, encryption, authentication, and key management.
use serde::{Deserialize, Serialize};

/// The length of the nonce used for AES-GCM encryption (12 bytes).
pub const NONCE_LEN: usize = 12;

pub mod audio;
pub mod auth;
pub mod backends;
pub mod format;
pub mod transport;
pub mod vectors;

#[cfg(feature = "audio")]
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
pub use format::*;
pub use transport::{Transport, TransportConfig, TransportFactory};

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
