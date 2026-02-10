//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Unified error hierarchy for the TrustEdge workspace.

use thiserror::Error;

/// Top-level unified error type for TrustEdge operations
#[derive(Error, Debug)]
pub enum TrustEdgeError {
    #[error("Cryptographic operation failed")]
    Crypto(#[from] CryptoError),

    #[error("Backend operation failed")]
    Backend(#[from] BackendError),

    #[error("Transport layer error")]
    Transport(#[from] TransportError),

    #[error("Archive operation failed")]
    Archive(#[from] ArchiveError),

    #[error("Manifest processing error")]
    Manifest(#[from] ManifestError),

    #[error("Chain validation error")]
    Chain(#[from] ChainError),

    #[error("Asymmetric crypto error")]
    Asymmetric(#[from] AsymmetricError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Errors related to cryptographic operations
#[derive(Error, Debug, Clone)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("Invalid signature format: {0}")]
    InvalidSignatureFormat(String),

    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    #[error("Invalid nonce format: {0}")]
    InvalidNonceFormat(String),
}

/// Errors related to continuity chain validation
#[derive(Error, Debug, Clone)]
pub enum ChainError {
    #[error("Gap in chain at segment index {0}")]
    Gap(usize),

    #[error("Segments out of order: expected continuity hash {expected}, found {found}")]
    OutOfOrder { expected: String, found: String },

    #[error("End of chain truncated")]
    EndOfChainTruncated,
}

/// Errors related to asymmetric cryptography operations
#[derive(Error, Debug)]
pub enum AsymmetricError {
    #[error("Unsupported algorithm: {0:?}")]
    UnsupportedAlgorithm(crate::backends::AsymmetricAlgorithm),

    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),

    #[error("Key exchange failed: {0}")]
    KeyExchangeFailed(String),

    #[error("Backend error: {0}")]
    BackendError(String),
}

/// Errors related to manifest processing
#[derive(Error, Debug)]
pub enum ManifestError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid field value: {0}")]
    InvalidField(String),
}

/// Errors related to archive operations
#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Manifest error: {0}")]
    Manifest(#[from] ManifestError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Schema mismatch: {0}")]
    SchemaMismatch(String),

    #[error("Missing chunk file: {0}")]
    MissingChunk(String),

    #[error("Invalid chunk index: expected {expected}, found {found}")]
    InvalidChunkIndex { expected: usize, found: usize },

    #[error("Signature mismatch: manifest.signature does not match signatures/manifest.sig")]
    SignatureMismatch,

    #[error("Continuity chain error: {0}")]
    Chain(#[from] ChainError),

    #[error("Archive validation failed: {0}")]
    ValidationFailed(String),
}

/// Errors related to backend operations
#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Backend operation not supported: {0}")]
    UnsupportedOperation(String),

    #[error("Key not found: {0}")]
    KeyNotFound(String),

    #[error("Backend initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Hardware backend error: {0}")]
    HardwareError(String),

    #[error("Backend operation failed: {0}")]
    OperationFailed(String),
}

/// Errors related to transport layer operations
#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Send failed: {0}")]
    SendFailed(String),

    #[error("Receive failed: {0}")]
    ReceiveFailed(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),

    #[error("Operation timed out: {0}")]
    Timeout(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
