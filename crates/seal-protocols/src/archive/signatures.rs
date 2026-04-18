//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Signature envelope types for .trst archives.

use thiserror::Error;

/// Errors related to signature envelope parsing
#[derive(Error, Debug)]
pub enum SignatureFormatError {
    #[error("Invalid signature format: {0}")]
    InvalidFormat(String),
    #[error("Missing signature field: {0}")]
    MissingField(String),
}
