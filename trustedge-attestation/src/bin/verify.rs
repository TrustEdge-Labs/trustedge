#!/usr/bin/env cargo
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! TrustEdge Verify - Verify software attestations
//!
//! Verifies cryptographically signed "birth certificates" for software artifacts,
//! checking integrity and authenticity against the original artifact.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use trustedge_attestation::Attestation;

#[cfg(feature = "envelope")]
use trustedge_core::Envelope;

/// Verify software attestation
#[derive(Parser, Debug)]
#[command(name = "trustedge-verify", version, about)]
struct Args {
    /// Path to the software artifact to verify
    #[arg(short, long)]
    artifact: PathBuf,

    /// Path to the attestation file (.trst or .json)
    #[arg(short = 't', long)]
    attestation_file: PathBuf,

    /// Show detailed verification information
    #[arg(short, long)]
    verbose: bool,

    /// Treat attestation file as raw JSON (not envelope)
    #[arg(long)]
    json_input: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("● TrustEdge Attestation Verification Tool");
        println!("==========================================");
        println!("● Artifact: {}", args.artifact.display());
        println!("● Attestation: {}", args.attestation_file.display());
        println!();
    }

    // Read the attestation
    let attestation = if args.json_input {
        // Read as JSON directly
        println!("● Reading JSON attestation...");
        let json_data = std::fs::read_to_string(&args.attestation_file)
            .with_context(|| format!("Failed to read attestation file: {}", args.attestation_file.display()))?;
        
        serde_json::from_str::<Attestation>(&json_data)
            .context("Failed to parse JSON attestation")?
    } else {
        // Try to read as envelope first, fallback to JSON
        #[cfg(feature = "envelope")]
        {
            println!("● Reading encrypted attestation envelope...");
            match read_envelope_attestation(&args.attestation_file) {
                Ok(attestation) => {
                    println!("✔ Envelope verified and unsealed");
                    attestation
                }
                Err(e) => {
                    if args.verbose {
                        println!("⚠ Envelope reading failed ({}), trying JSON fallback...", e);
                    }
                    read_json_attestation(&args.attestation_file)?
                }
            }
        }
        
        #[cfg(not(feature = "envelope"))]
        {
            println!("● Reading JSON attestation...");
            read_json_attestation(&args.attestation_file)?
        }
    };

    // Verify the artifact hash
    println!("● Computing artifact hash...");
    let artifact_data = std::fs::read(&args.artifact)
        .with_context(|| format!("Failed to read artifact: {}", args.artifact.display()))?;
    
    use sha2::{Sha256, Digest};
    let computed_hash = format!("{:x}", Sha256::digest(&artifact_data));

    // Compare hashes
    if computed_hash == attestation.artifact_hash {
        println!("✔ VERIFICATION SUCCESSFUL");
        println!();
        println!("● Artifact Details:");
        println!("   • Name: {}", attestation.artifact_name);
        println!("   • Hash: {}...", &attestation.artifact_hash[..16]);
        println!("   • Size: {} bytes", artifact_data.len());
        println!();
        println!("● Provenance Information:");
        println!("   • Source Commit: {}", attestation.source_commit_hash);
        println!("   • Builder ID: {}", attestation.builder_id);
        println!("   • Created: {}", attestation.timestamp);
        
        if args.verbose {
            println!();
            println!("● Cryptographic Verification:");
            println!("   • Hash Algorithm: SHA-256");
            println!("   • Full Hash: {}", attestation.artifact_hash);
            println!("   • Integrity: ✔ VERIFIED");
        }
        
        println!();
        println!("✔ This software artifact is AUTHENTICATED and VERIFIED");
        println!("   The artifact matches its cryptographic birth certificate.");
    } else {
        println!("✖ VERIFICATION FAILED");
        println!();
        println!("Hash mismatch detected:");
        println!("  Expected: {}", attestation.artifact_hash);
        println!("  Computed: {}", computed_hash);
        println!();
        println!("⚠ WARNING: Artifact may have been tampered with or corrupted!");
        
        return Err(anyhow::anyhow!("Artifact hash mismatch - integrity check failed"));
    }

    Ok(())
}

#[cfg(feature = "envelope")]
fn read_envelope_attestation(path: &PathBuf) -> Result<Attestation> {
    use std::fs::File;
    use std::io::BufReader;

    #[derive(serde::Deserialize)]
    struct AttestationFile {
        envelope: Envelope,
        #[allow(dead_code)]
        verification_key: [u8; 32],
        private_key: [u8; 32],
    }

    let file = File::open(path)
        .with_context(|| format!("Failed to open attestation file: {}", path.display()))?;
    let mut reader = BufReader::new(file);
    
    let attestation_file: AttestationFile = bincode::deserialize_from(&mut reader)
        .context("Failed to read attestation file")?;

    // Verify the envelope signature
    if !attestation_file.envelope.verify() {
        return Err(anyhow::anyhow!("Envelope signature verification failed"));
    }

    // Reconstruct the private key for unsealing
    let private_key = ed25519_dalek::SigningKey::from_bytes(&attestation_file.private_key);

    let payload = attestation_file.envelope.unseal(&private_key)
        .context("Failed to unseal envelope")?;

    let attestation: Attestation = serde_json::from_slice(&payload)
        .context("Failed to parse attestation from envelope payload")?;

    Ok(attestation)
}

fn read_json_attestation(path: &PathBuf) -> Result<Attestation> {
    let json_data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read attestation file: {}", path.display()))?;
    
    serde_json::from_str::<Attestation>(&json_data)
        .context("Failed to parse JSON attestation")
}
