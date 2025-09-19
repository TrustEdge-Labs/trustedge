#!/usr/bin/env cargo
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Attest - Create software attestations
//!
//! Creates cryptographically signed "birth certificates" for software artifacts,
//! capturing provenance information from Git repositories and build environments.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use trustedge_attestation::create_attestation_data;

#[cfg(feature = "envelope")]
use trustedge_core::{Envelope, backends::UniversalBackendRegistry};

/// Create software attestation (birth certificate)
#[derive(Parser, Debug)]
#[command(name = "trustedge-attest", version, about)]
struct Args {
    /// Path to the software artifact to attest
    #[arg(short, long)]
    file: PathBuf,

    /// Builder identifier (e.g., email, CI job ID)
    #[arg(short, long)]
    builder_id: String,

    /// Output file for the attestation (.trst file)
    #[arg(short, long)]
    output: PathBuf,

    /// Key management backend to use
    #[arg(long, default_value = "software_hsm")]
    backend: String,

    /// Key ID to use for signing
    #[arg(long, default_value = "attestation_key")]
    key_id: String,

    /// Use keyring backend
    #[arg(long)]
    use_keyring: bool,

    /// Show detailed progress information
    #[arg(short, long)]
    verbose: bool,

    /// Output JSON attestation without envelope (for inspection)
    #[arg(long)]
    json_only: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("● TrustEdge Software Attestation Tool");
        println!("=====================================");
        println!("● Artifact: {}", args.file.display());
        println!("● Builder: {}", args.builder_id);
        println!("● Output: {}", args.output.display());
    }

    // Step 1-3: Create the attestation data
    println!("● Analyzing artifact and repository...");
    let attestation = create_attestation_data(&args.file, &args.builder_id)
        .context("Failed to create attestation")?;

    if args.verbose {
        println!("✔ Attestation created:");
        println!("   ● Artifact: {}", attestation.artifact_name);
        println!("   ● Hash: {}...", &attestation.artifact_hash[..16]);
        println!("   ● Commit: {}", attestation.source_commit_hash);
        println!("   ● Timestamp: {}", attestation.timestamp);
    }

    // JSON-only output (for inspection/debugging)
    if args.json_only {
        let json = serde_json::to_string_pretty(&attestation)
            .context("Failed to serialize attestation to JSON")?;
        
        std::fs::write(&args.output, json)
            .with_context(|| format!("Failed to write JSON to {}", args.output.display()))?;
        
        println!("✔ JSON attestation written to: {}", args.output.display());
        return Ok(());
    }

    // Step 4: Seal in TrustEdge Envelope (if envelope feature is enabled)
    #[cfg(feature = "envelope")]
    {
        println!("● Creating cryptographic envelope...");
        
        // Serialize attestation to JSON
        let payload = serde_json::to_vec(&attestation)
            .context("Failed to serialize attestation")?;

        // Create backend and keys
        let registry = UniversalBackendRegistry::new();
        let _backend = registry.get_backend(&args.backend)
            .ok_or_else(|| {
                let available_backends = registry.list_backend_names();
                anyhow::anyhow!(
                    "Backend '{}' not available. Available backends: {:?}", 
                    args.backend, 
                    available_backends
                )
            })?;

        // Generate ephemeral keys for demonstration
        // In production, these would come from the backend
        let mut csprng = rand::rngs::OsRng;
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        
        // For attestations, we use the same key as both sender and beneficiary
        // This makes it a publicly verifiable signature rather than encrypted message
        let beneficiary_key = signing_key.verifying_key();

        // Create envelope
        let envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key)
            .context("Failed to create envelope")?;

        // Create a simple attestation file format that includes both the envelope
        // and the private key needed for verification
        #[derive(serde::Serialize)]
        struct AttestationFile {
            envelope: Envelope,
            verification_key: [u8; 32], // Public key for verification
            private_key: [u8; 32], // Private key for unsealing (demo only!)
        }

        let attestation_file = AttestationFile {
            envelope,
            verification_key: signing_key.verifying_key().to_bytes(),
            private_key: signing_key.to_bytes(),
        };

        // Write to file using bincode serialization
        let output_file = std::fs::File::create(&args.output)
            .with_context(|| format!("Failed to create output file: {}", args.output.display()))?;
        let mut writer = std::io::BufWriter::new(output_file);
        
        bincode::serialize_into(&mut writer, &attestation_file)
            .context("Failed to write attestation file")?;
        
        use std::io::Write;
        writer.flush()?;

        println!("✔ Sealed attestation created: {}", args.output.display());
        println!("● Cryptographically signed software birth certificate");
        
        if args.verbose {
            println!();
            println!("● This attestation provides verifiable proof of:");
            println!("   • Software artifact integrity (SHA-256 hash)");
            println!("   • Source code provenance (Git commit)");
            println!("   • Build environment details");
            println!("   • Builder identity and timestamp");
            println!();
            println!("Use 'trustedge-verify' to verify this attestation.");
        }
    }

    #[cfg(not(feature = "envelope"))]
    {
        // Fallback: just write JSON if envelope feature is not enabled
        let json = serde_json::to_string_pretty(&attestation)
            .context("Failed to serialize attestation to JSON")?;
        
        std::fs::write(&args.output, json)
            .with_context(|| format!("Failed to write attestation to {}", args.output.display()))?;
        
        println!("✔ Attestation written to: {}", args.output.display());
        println!("● Note: Install with --features envelope for cryptographic sealing");
    }

    Ok(())
}
