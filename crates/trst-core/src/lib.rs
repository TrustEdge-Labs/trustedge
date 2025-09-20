//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Archive Core
//!
//! Core primitives for creating and verifying .trst archives.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub mod manifest;
pub mod verify;
pub mod wrap;

pub use manifest::*;
pub use verify::*;
pub use wrap::*;

#[derive(Error, Debug)]
pub enum ArchiveError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Generic error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Signature error: {0}")]
    Signature(String),
    #[error("Archive format error: {0}")]
    Format(String),
    #[error("Missing chunk: {0}")]
    MissingChunk(String),
    #[error("Hash mismatch: {0}")]
    HashMismatch(String),
    #[error("Unexpected end: {0}")]
    UnexpectedEnd(String),
    #[error("Continuity error: {0}")]
    Continuity(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub fw: String,
    pub model: String,
    pub public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestCapture {
    pub started_at: String,
    pub tz: String,
    pub fps: u32,
    pub resolution: String,
    pub codec: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrapConfig {
    pub profile: String,
    pub device: DeviceInfo,
    pub capture: ManifestCapture,
    pub chunk_bytes: usize,
    pub chunk_seconds: f64,
    pub claims: serde_json::Value,
    pub prev_archive_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyOutcome {
    pub signature: bool,
    pub continuity: bool,
    pub segment_count: usize,
    pub duration_seconds: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrapResult {
    pub output_dir: std::path::PathBuf,
    pub signature: String,
    pub chunk_count: usize,
}

/// Wrap a file into a .trst archive
pub fn wrap_file<P: AsRef<Path>>(
    input_path: P,
    output_dir: P,
    signing_key: &ed25519_dalek::SigningKey,
    config: WrapConfig,
) -> Result<WrapResult, ArchiveError> {
    wrap::wrap_file_impl(input_path, output_dir, signing_key, config)
}

/// Verify a .trst archive
pub fn verify_archive<P: AsRef<Path>>(
    archive_path: P,
    device_pub: &str,
) -> Result<VerifyOutcome, ArchiveError> {
    verify::verify_archive_impl(archive_path, device_pub)
}

/// Verify manifest bytes directly
pub fn verify_manifest_bytes(
    manifest_bytes: &[u8],
    device_pub: &str,
) -> Result<VerifyOutcome, ArchiveError> {
    verify::verify_manifest_bytes_impl(manifest_bytes, device_pub)
}
