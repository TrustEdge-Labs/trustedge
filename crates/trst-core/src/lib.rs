//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Archive Core
//!
//! Canonical cam.video manifest types for .trst archives.
//!
//! This crate provides the single source of truth for manifest types used
//! by both the browser WASM verifier and the main trustedge-core library.
//! It has minimal dependencies suitable for WASM compilation.
//!
//! ## Usage
//!
//! ```rust
//! use trustedge_trst_core::{CamVideoManifest, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo};
//!
//! let manifest = CamVideoManifest::new();
//! ```

pub mod manifest;

// Re-export canonical manifest types at crate root for convenience
pub use manifest::{
    CamVideoManifest, CaptureInfo, ChunkInfo, DeviceInfo, ManifestError, SegmentInfo,
};
