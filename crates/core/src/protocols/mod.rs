//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # Layer 3: Wire Formats and Protocols
//!
//! Wire formats, envelope construction, chain validation, and manifest serialization.
//! This layer defines the on-disk and over-the-wire data structures that TrustEdge uses
//! to exchange encrypted data, maintain continuity chains, and serialize metadata.
//!
//! ## Layer Contract
//!
//! **What belongs here:**
//! - Envelope format definition and seal/unseal operations
//! - Continuity chain validation and verification
//! - Manifest serialization (CamVideoManifest, canonical JSON)
//! - Format version negotiation and algorithm enums
//! - Envelope format detection and bridging (V1/V2)
//! - Test vectors for protocol compliance
//!
//! **CAN import:**
//! - `primitives` (Layer 1) - for crypto operations (sign, verify, hash)
//! - `backends` (Layer 2) - for key management during envelope operations
//!
//! **NEVER imports:**
//! - `applications` (Layer 4) - business logic is built on protocols, not vice versa
//! - `transport` (Layer 5) - network operations use protocols, not vice versa
//! - `io` (Layer 6) - I/O adapters consume protocols, not vice versa
//!
//! ## Post-Consolidation Contents
//!
//! After Phase 2-8 migration, this module will contain:
//!
//! - `envelope.rs` (from `crates/core/src/envelope.rs`)
//!   - `Envelope` type - core envelope structure
//!   - `Envelope::seal()` - create signed, encrypted envelope
//!   - `Envelope::unseal()` - verify and decrypt envelope
//!   - `EnvelopeMetadata` - envelope header information
//!
//! - `chain.rs` (from `crates/core/src/chain.rs` - validation logic)
//!   - `validate_chain()` - continuity chain verification
//!   - `chain_next()` - chain advancement
//!   - `ChainSegment` - chain segment structure
//!
//! - `manifest.rs` (from `crates/core/src/manifest.rs`)
//!   - `CamVideoManifest` - canonical cam.video manifest type
//!   - `CaptureInfo`, `DeviceInfo`, `ChunkInfo`, `SegmentInfo` - manifest components
//!   - Canonical JSON serialization
//!
//! - `format.rs` (from `crates/core/src/format.rs`)
//!   - `SignedManifest` - signed manifest wrapper
//!   - Algorithm enums (signature, symmetric, asymmetric, hash)
//!   - Format version constants
//!
//! - `vectors.rs` (from `crates/core/src/vectors.rs`)
//!   - Test vectors for envelope format
//!   - Reference implementations for compliance testing
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use trustedge_core::protocols::{Envelope, ChainSegment, CamVideoManifest};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! // Create and seal an envelope
//! let sender_key = SigningKey::generate(&mut OsRng);
//! let recipient_key = SigningKey::generate(&mut OsRng);
//! let data = b"Secret message";
//! let envelope = Envelope::seal(data, &sender_key, &recipient_key.verifying_key())?;
//!
//! // Validate a continuity chain
//! let genesis_hash = genesis(b"seed");
//! let segment = ChainSegment { prev: genesis_hash, current: segment_hash(b"data") };
//! validate_chain(&[segment])?;
//! ```
//!
//! ## Status
//!
//! **Phase 1 scaffolding - no code yet**
//!
//! This module structure was created in Phase 1 to establish the layer hierarchy.
//! Actual code migration happens in Phase 2-8 of the consolidation roadmap.
