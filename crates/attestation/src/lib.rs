//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Attestation
//!
//! **DEPRECATED:** Attestation functionality has moved to `trustedge_core::applications::attestation`.
//! This crate re-exports from core for backward compatibility.
//! It will be fully deprecated in Phase 7.

pub use trustedge_core::{
    Attestation, AttestationConfig, AttestationResult,
    OutputFormat, KeySource, VerificationConfig, VerificationResult,
    VerificationDetails, VerificationInfo,
    create_signed_attestation, verify_attestation,
};
