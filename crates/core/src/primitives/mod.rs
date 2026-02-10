//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # Layer 1: Cryptographic Primitives
//!
//! Pure cryptographic primitives with no external dependencies beyond standard crypto libraries.
//! This layer provides the foundational building blocks for all cryptographic operations in TrustEdge.
//!
//! ## Layer Contract
//!
//! **What belongs here:**
//! - Raw encryption/decryption operations (AES-256-GCM, XChaCha20-Poly1305)
//! - Digital signature operations (Ed25519, P256)
//! - Key derivation (PBKDF2, HKDF)
//! - Hashing primitives (BLAKE3)
//! - Key exchange protocols (X25519 ECDH)
//! - RSA hybrid encryption operations
//! - No business logic, no I/O, no key management
//!
//! **CAN import:**
//! - Standard library (`std::*`)
//! - Crypto crates: `aes-gcm`, `ed25519-dalek`, `blake3`, `chacha20poly1305`, `p256`, `x25519-dalek`, `pbkdf2`, `hkdf`, `rsa`
//!
//! **NEVER imports:**
//! - `backends` (Layer 2) - key management is higher level
//! - `protocols` (Layer 3) - wire formats and envelopes are built on primitives
//! - `applications` (Layer 4) - business logic depends on primitives, not vice versa
//! - `transport` (Layer 5) - network operations are I/O
//! - `io` (Layer 6) - I/O adapters are the highest layer
//!
//! ## Post-Consolidation Contents
//!
//! After Phase 2-8 migration, this module will contain:
//!
//! - `crypto.rs` (from `crates/core/src/crypto.rs`)
//!   - `encrypt_segment()` - AES-256-GCM encryption
//!   - `decrypt_segment()` - AES-256-GCM decryption
//!   - `sign_manifest()` - Ed25519 signing
//!   - `verify_manifest()` - Ed25519 verification
//!   - Nonce generation and parsing
//!
//! - `asymmetric.rs` (from `crates/core/src/asymmetric.rs`)
//!   - `encrypt_key_asymmetric()` - RSA encryption for key wrapping
//!   - `decrypt_key_asymmetric()` - RSA decryption for key unwrapping
//!   - `key_exchange()` - X25519 ECDH key agreement
//!   - Key pair generation (Ed25519, P256, RSA)
//!
//! - `chain.rs` (from `crates/core/src/chain.rs` - primitives only)
//!   - `blake3_hex_or_b64()` - BLAKE3 hashing
//!   - `segment_hash()` - Hash computation for chain segments
//!   - `genesis()` - Genesis seed generation
//!
//! - `hybrid.rs` (from `crates/core/src/hybrid.rs` - crypto ops only)
//!   - X25519 ECDH operations
//!   - RSA hybrid encryption internals
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use trustedge_core::primitives::{encrypt_segment, sign_manifest, DeviceKeypair};
//!
//! // Encrypt a data segment
//! let key = [0u8; 32];
//! let plaintext = b"Secret data";
//! let (ciphertext, nonce) = encrypt_segment(&key, plaintext)?;
//!
//! // Sign a manifest
//! let keypair = DeviceKeypair::generate();
//! let manifest_bytes = b"manifest data";
//! let signature = sign_manifest(&keypair, manifest_bytes)?;
//! ```
//!
//! ## Status
//!
//! **Phase 1 scaffolding - no code yet**
//!
//! This module structure was created in Phase 1 to establish the layer hierarchy.
//! Actual code migration happens in Phase 2-8 of the consolidation roadmap.

