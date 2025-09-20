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
use trustedge_attestation::{verify_attestation, VerificationConfig};

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

    // Create verification configuration
    let config = VerificationConfig {
        artifact_path: args.artifact.clone(),
        attestation_path: args.attestation_file.clone(),
        force_json: args.json_input,
    };

    // Perform verification using the centralized library function
    if args.json_input {
        println!("● Reading JSON attestation...");
    } else {
        #[cfg(feature = "envelope")]
        println!("● Reading attestation (trying envelope first, JSON fallback)...");

        #[cfg(not(feature = "envelope"))]
        println!("● Reading JSON attestation...");
    }

    println!("● Computing artifact hash...");

    let result = verify_attestation(config).context("Failed to verify attestation")?;

    // Display results
    if result.is_valid {
        println!("✔ VERIFICATION SUCCESSFUL");
        println!();
        println!("● Artifact Details:");
        println!("   • Name: {}", result.attestation.artifact_name);
        println!("   • Hash: {}...", &result.attestation.artifact_hash[..16]);
        println!(
            "   • Size: {} bytes",
            result.verification_details.artifact_size
        );
        println!();
        println!("● Provenance Information:");
        println!(
            "   • Source Commit: {}",
            result.attestation.source_commit_hash
        );
        println!("   • Builder ID: {}", result.attestation.builder_id);
        println!("   • Created: {}", result.attestation.timestamp);

        if args.verbose {
            println!();
            println!("● Cryptographic Verification:");
            println!("   • Hash Algorithm: SHA-256");
            println!("   • Full Hash: {}", result.attestation.artifact_hash);
            println!(
                "   • Computed Hash: {}",
                result.verification_details.computed_hash
            );
            println!("   • Integrity: ✔ VERIFIED");

            if let Some(envelope_verified) = result.verification_details.envelope_verified {
                println!(
                    "   • Envelope Signature: {}",
                    if envelope_verified {
                        "✔ VERIFIED"
                    } else {
                        "✖ FAILED"
                    }
                );
            }
        }

        println!();
        println!("✔ This software artifact is AUTHENTICATED and VERIFIED");
        println!("   The artifact matches its cryptographic birth certificate.");
    } else {
        println!("✖ VERIFICATION FAILED");
        println!();
        println!("Hash mismatch detected:");
        println!("  Expected: {}", result.verification_details.expected_hash);
        println!("  Computed: {}", result.verification_details.computed_hash);
        println!();
        println!("⚠ WARNING: Artifact may have been tampered with or corrupted!");

        return Err(anyhow::anyhow!(
            "Artifact hash mismatch - integrity check failed"
        ));
    }

    Ok(())
}
