//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Archive format types for .trst archives.

pub mod chunks;
pub mod manifest;
pub mod signatures;

// Re-export commonly used items at domain level
pub use chunks::ChunkFormatError;
pub use manifest::{
    CamVideoManifest, CaptureInfo, ChunkInfo, DeviceInfo, ManifestFormatError, SegmentInfo,
};
pub use signatures::SignatureFormatError;
