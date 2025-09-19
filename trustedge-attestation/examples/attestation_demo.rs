// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

use std::fs::File;
use std::io::Write;
use tempfile::tempdir;
use trustedge_attestation::{create_attestation_data, Attestation};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("● TrustEdge Software Attestation Demo");
    println!("=====================================");

    // Create a temporary artifact
    let temp_dir = tempdir()?;
    let artifact_path = temp_dir.path().join("demo-software.bin");
    let mut file = File::create(&artifact_path)?;
    file.write_all(b"Demo software binary content")?;
    file.flush()?;
    drop(file);

    println!("● Created demo artifact: {}", artifact_path.display());

    // Method 1: Use the create_attestation_data function (recommended)
    println!("\n● Creating attestation using create_attestation_data()...");
    let attestation = create_attestation_data(&artifact_path, "demo-builder@example.com")?;

    println!("✔ Created software birth certificate:");
    println!("● Artifact: {}", attestation.artifact_name);
    println!("● Hash: {}...", &attestation.artifact_hash[..16]);
    println!("● Commit: {}", attestation.source_commit_hash);
    println!("● Builder: {}", attestation.builder_id);
    println!("● Timestamp: {}", attestation.timestamp);

    // Method 2: Manual construction for advanced use cases
    println!("\n● Manual construction example...");
    use sha2::{Digest, Sha256};
    let manual_hash = format!("{:x}", Sha256::digest(std::fs::read(&artifact_path)?));
    let manual_attestation = Attestation {
        artifact_hash: manual_hash,
        artifact_name: "manually-created-demo.bin".to_string(),
        source_commit_hash: "manual123def456789".to_string(),
        builder_id: "manual-builder@example.com".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    println!("✔ Manual attestation created:");
    println!("● Hash: {}...", &manual_attestation.artifact_hash[..16]);

    println!("● This attestation provides cryptographic proof of:");
    println!("  • Software artifact integrity (hash verification)");
    println!("  • Source code provenance (Git commit)");
    println!("  • Build environment details");
    println!("  • Builder identity and timestamp");

    println!("\n✔ TrustEdge attestation demo complete!");
    println!("Use the CLI tools (trustedge-attest/trustedge-verify) for production use.");

    Ok(())
}
