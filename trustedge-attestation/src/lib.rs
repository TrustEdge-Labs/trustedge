// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Software Attestation Library
//!
//! Core attestation functionality for creating software "birth certificates".

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Simple software attestation - the "birth certificate" payload
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attestation {
    /// The SHA-256 hash of the artifact being attested to.
    pub artifact_hash: String,
    /// The name of the artifact file.
    pub artifact_name: String,
    /// The Git commit hash from which the artifact was built.
    pub source_commit_hash: String,
    /// An identifier for the entity that created the attestation.
    pub builder_id: String,
    /// The ISO 8601 timestamp of when the attestation was created.
    pub timestamp: String,
}

/// Create a software attestation from an artifact file
pub fn create_attestation_data(
    artifact_path: &Path,
    builder_id: &str,
) -> Result<Attestation> {
    use sha2::{Digest, Sha256};

    // 1. Hash the artifact
    let artifact_data = std::fs::read(artifact_path)
        .with_context(|| format!("Failed to read artifact: {}", artifact_path.display()))?;
    
    let artifact_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // 2. Get commit hash (simplified - just use a placeholder if not in git repo)
    let source_commit_hash = get_git_commit_hash().unwrap_or_else(|_| "unknown".to_string());

    // 3. Create attestation
    let attestation = Attestation {
        artifact_hash,
        artifact_name: artifact_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_commit_hash,
        builder_id: builder_id.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(attestation)
}

/// Get the current Git commit hash
fn get_git_commit_hash() -> Result<String> {
    use git2::Repository;

    let repo = Repository::discover(".")
        .context("Failed to find Git repository")?;

    let head = repo.head()
        .context("Failed to get HEAD reference")?;

    let commit = head.peel_to_commit()
        .context("Failed to get commit from HEAD")?;

    Ok(commit.id().to_string())
}
