//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # Layer 4: Application Logic
//!
//! High-level business logic APIs built on primitives, backends, and protocols.
//! This layer provides self-contained domain features that compose the lower layers
//! into complete application functionality.
//!
//! ## Layer Contract
//!
//! **What belongs here:**
//! - Digital receipt system with ownership chains
//! - Software attestation and provenance tracking
//! - Archive manifest types (cam.video format)
//! - Authentication and session management
//! - High-level APIs that compose primitives, backends, and protocols
//! - Domain-specific business logic
//!
//! **CAN import:**
//! - `primitives` (Layer 1) - for direct crypto operations
//! - `backends` (Layer 2) - for key management
//! - `protocols` (Layer 3) - for envelope and chain operations
//!
//! **NEVER imports:**
//! - `transport` (Layer 5) - network operations are orthogonal to business logic
//! - `io` (Layer 6) - I/O adapters should not be in application logic
//!
//! ## Post-Consolidation Contents
//!
//! After Phase 2-8 migration, this module will contain:
//!
//! - `receipts/` (from `crates/receipts/` - 1,281 LOC, 23 tests)
//!   - `Receipt` type - digital receipt with cryptographic ownership
//!   - `ReceiptChain` - ownership transfer chain
//!   - `ReceiptPolicy` - validation policies
//!   - Receipt issuance, verification, and transfer operations
//!
//! - `attestation/` (from `crates/attestation/`)
//!   - `Attestation` type - software attestation records
//!   - `ProvenanceChain` - provenance tracking
//!   - Build artifact signing and verification
//!
//! - `archives/` (from `crates/trst-core/src/types.rs` - manifest types)
//!   - `ArchiveManifest` - cam.video manifest types
//!   - WASM-compatible manifest serialization
//!   - Archive metadata structures
//!
//! - `auth.rs` (from `crates/core/src/auth.rs`)
//!   - `client_authenticate()` - Ed25519 mutual authentication
//!   - `server_authenticate()` - server-side auth
//!   - `SessionManager` - session lifecycle management
//!   - Challenge-response protocols
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use trustedge_core::applications::{Receipt, Attestation, SessionManager};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! // Create a digital receipt
//! let issuer_key = SigningKey::generate(&mut OsRng);
//! let receipt = Receipt::issue("item-id", "owner-id", &issuer_key)?;
//!
//! // Verify receipt ownership
//! receipt.verify_ownership("owner-id")?;
//!
//! // Authenticate a client session
//! let session_mgr = SessionManager::new();
//! let session = session_mgr.create_session(client_id)?;
//! ```
//!
//! ## Status
//!
//! **Phase 4 migration - receipts live**
//!
//! This module structure was created in Phase 1 to establish the layer hierarchy.
//! Actual code migration happens in Phase 2-8 of the consolidation roadmap.
//!
//! - ✔ `receipts/` - Digital receipt system with cryptographic ownership (1,281 LOC, 23 tests)

pub mod receipts;

