//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

#![deprecated(
    since = "0.3.0",
    note = "This crate has been merged into trustedge-core. \
            Import from `trustedge_core` instead. \
            This facade will be removed in 0.4.0 (August 2026). \
            See https://github.com/TrustEdge-Labs/trustedge/blob/main/MIGRATION.md"
)]

//! # TrustEdge Attestation
//!
//! ## ⚠️ DEPRECATION NOTICE
//!
//! **This crate has been deprecated as of version 0.3.0.**
//!
//! All attestation functionality has been consolidated into [`trustedge-core`](https://docs.rs/trustedge-core).
//!
//! ### Timeline
//!
//! - **0.3.0** (February 2026): Deprecated - warnings issued
//! - **0.4.0** (August 2026): Removal - crate will be deleted from workspace
//!
//! ### Migration
//!
//! **Before (deprecated):**
//! ```rust,ignore
//! use trustedge_attestation::{Attestation, create_signed_attestation};
//! ```
//!
//! **After (recommended):**
//! ```rust,ignore
//! use trustedge_core::{Attestation, create_signed_attestation};
//! ```
//!
//! All APIs remain identical - only import paths change.
//!
//! See [MIGRATION.md](https://github.com/TrustEdge-Labs/trustedge/blob/main/MIGRATION.md) for detailed upgrade instructions.

pub use trustedge_core::{
    create_signed_attestation, verify_attestation, Attestation, AttestationConfig,
    AttestationResult, KeySource, OutputFormat, VerificationConfig, VerificationDetails,
    VerificationInfo, VerificationResult,
};
