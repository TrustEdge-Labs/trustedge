// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Pubky Integration
//!
//! This crate provides hybrid encryption capabilities for TrustEdge using the Pubky protocol
//! for decentralized key discovery. It implements:
//!
//! - Dual key architecture (Ed25519 identity + X25519 encryption)
//! - Hybrid encryption (X25519 ECDH + AES-256-GCM)
//! - Pubky integration for censorship-resistant key discovery
//! - V2 envelope format with improved security and usability

pub mod envelope;
pub mod keys;
pub mod pubky_client;

pub use envelope::{EnvelopeV2, EnvelopeHeaderV2, KeyExchangeAlgorithm};
pub use keys::{DualKeyPair, PubkyIdentity};
pub use pubky_client::{PubkyClient, PubkyError};

// Re-export commonly used types
pub use ed25519_dalek::{SigningKey, VerifyingKey};
pub use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};