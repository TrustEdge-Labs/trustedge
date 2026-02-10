//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Receipts
//!
//! **DEPRECATED:** Receipt functionality has moved to `trustedge_core::applications::receipts`.
//! This crate re-exports from core for backward compatibility.
//! It will be fully deprecated in Phase 7.

pub use trustedge_core::{Receipt, create_receipt, assign_receipt, extract_receipt, verify_receipt_chain};
