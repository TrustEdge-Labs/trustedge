//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Chunk structure types and validation for .trst archives.

use thiserror::Error;

/// Errors related to chunk format validation
#[derive(Error, Debug)]
pub enum ChunkFormatError {
    #[error("Invalid chunk header: {0}")]
    InvalidHeader(String),
    #[error("Chunk sequence gap at index {0}")]
    SequenceGap(usize),
}
