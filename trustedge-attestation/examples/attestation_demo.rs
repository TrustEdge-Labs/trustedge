// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;
use trustedge_attestation::{
    create_signed_attestation, AttestationConfig, KeySource, OutputFormat,
};

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

    // Method 1: Create JSON attestation (recommended for inspection)
    println!("\n● Creating JSON attestation...");
    let json_config = AttestationConfig {
        artifact_path: artifact_path.clone(),
        builder_id: "demo-builder@example.com".to_string(),
        output_format: OutputFormat::JsonOnly,
        key_source: KeySource::Generate,
    };

    let json_result = create_signed_attestation(json_config)?;

    println!("✔ Created software birth certificate:");
    println!("● Artifact: {}", json_result.attestation.artifact_name);
    println!("● Hash: {}...", &json_result.attestation.artifact_hash[..16]);
    println!("● Commit: {}", json_result.attestation.source_commit_hash);
    println!("● Builder: {}", json_result.attestation.builder_id);
    println!("● Timestamp: {}", json_result.attestation.timestamp);

    // Method 2: Create sealed envelope attestation (recommended for production)
    #[cfg(feature = "envelope")]
    {
        println!("\n● Creating sealed envelope attestation...");
        let envelope_config = AttestationConfig {
            artifact_path: PathBuf::from(artifact_path),
            builder_id: "demo-builder@example.com".to_string(),
            output_format: OutputFormat::SealedEnvelope,
            key_source: KeySource::Generate,
        };

        let envelope_result = create_signed_attestation(envelope_config)?;

        println!("✔ Created sealed attestation:");
        println!("● Size: {} bytes", envelope_result.serialized_output.len());
        if let Some(verification_info) = &envelope_result.verification_info {
            println!("● Public Key: {}...", &verification_info.verification_key[..16]);
        }
    }

    #[cfg(not(feature = "envelope"))]
    {
        println!("\n● Envelope feature not enabled - only JSON attestations available");
        println!("● Enable with: cargo run --example attestation_demo --features envelope");
    }

    println!("\n● This attestation provides cryptographic proof of:");
    println!("  • Software artifact integrity (SHA-256 hash verification)");
    println!("  • Source code provenance (Git commit hash)");
    println!("  • Build environment details");
    println!("  • Builder identity and timestamp");

    println!("\n✔ TrustEdge attestation demo complete!");
    println!("Use the CLI tools (trustedge-attest/trustedge-verify) for production use.");

    Ok(())
}
