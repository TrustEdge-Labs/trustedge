//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Protocols
//!
//! Protocol and format definitions for TrustEdge archives and capture profiles.
//!
//! This crate provides the single source of truth for protocol types used
//! by both the browser WASM verifier and the main trustedge-core library.
//! It has minimal dependencies suitable for WASM compilation.
//!
//! ## Modules
//!
//! - `archive` - Archive format types (manifest, chunks, signatures)
//! - `capture` - Capture profile types (cam.video and future profiles)
//!
//! ## Usage
//!
//! ```rust
//! use trustedge_trst_protocols::{CamVideoManifest, DeviceInfo, CaptureInfo, ChunkInfo, SegmentInfo};
//!
//! let manifest = CamVideoManifest::new();
//! ```

pub mod archive;
pub mod capture;

// Re-export commonly used items at crate root for convenience
pub use archive::manifest::{
    CamVideoManifest, CaptureInfo, ChunkInfo, DeviceInfo, ManifestFormatError, SegmentInfo,
};

// Backward compatibility alias for trst-wasm (temporary)
#[doc(hidden)]
pub use ManifestFormatError as ManifestError;
