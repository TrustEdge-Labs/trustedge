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
use trustedge_core::{
    create_signed_attestation, AttestationConfig, KeySource, OutputFormat,
};

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

    // Determine output format
    let output_format = if args.json_only {
        OutputFormat::JsonOnly
    } else {
        OutputFormat::SealedEnvelope
    };

    // Create attestation configuration
    let config = AttestationConfig {
        artifact_path: args.file.clone(),
        builder_id: args.builder_id.clone(),
        output_format,
        key_source: KeySource::Generate, // Demo mode with ephemeral keys
    };

    println!("● Analyzing artifact and repository...");

    // Create the attestation using the centralized library function
    let result = create_signed_attestation(config).context("Failed to create attestation")?;

    if args.verbose {
        println!("✔ Attestation created:");
        println!("   ● Artifact: {}", result.attestation.artifact_name);
        println!("   ● Hash: {}...", &result.attestation.artifact_hash[..16]);
        println!("   ● Commit: {}", result.attestation.source_commit_hash);
        println!("   ● Timestamp: {}", result.attestation.timestamp);
    }

    // Write output to file
    std::fs::write(&args.output, &result.serialized_output)
        .with_context(|| format!("Failed to write output to {}", args.output.display()))?;

    // Display appropriate success message
    if args.json_only {
        println!("✔ JSON attestation written to: {}", args.output.display());
    } else {
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
            println!("Use verify_attestation example to verify this attestation.");

            if let Some(verification_info) = &result.verification_info {
                println!();
                println!("● Verification Information (demo mode):");
                println!(
                    "   • Public Key: {}...",
                    &verification_info.verification_key[..16]
                );
                if let Some(private_key) = &verification_info.private_key {
                    println!(
                        "   • Private Key: {}... (included for demo)",
                        &private_key[..16]
                    );
                }
            }
        }
    }

    Ok(())
}
