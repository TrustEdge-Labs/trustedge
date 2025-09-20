/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Demo - Hardware Root of Trust
//!
//! This demo showcases YubiKey integration with the TrustEdge Universal Backend system.

use anyhow::{Context, Result};
use clap::Parser;
use trustedge_core::backends::{
    universal::{CryptoOperation, SignatureAlgorithm, UniversalBackend},
    YubiKeyBackend, YubiKeyConfig,
};

#[derive(Parser)]
#[command(name = "yubikey-demo")]
#[command(about = "YubiKey hardware root of trust demo")]
struct Args {
    #[arg(short, long, help = "PKCS#11 module path")]
    pkcs11_path: Option<String>,

    #[arg(short = 'P', long, help = "PIN for YubiKey authentication")]
    pin: Option<String>,

    #[arg(short, long, help = "PKCS#11 slot number")]
    slot: Option<u64>,

    #[arg(short, long, help = "Key ID to use for signing")]
    key_id: Option<String>,

    #[arg(short = 'v', long, help = "Enable verbose output")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Test YubiKey connectivity and list available keys
    Test,
    /// List all keys on the YubiKey
    ListKeys,
    /// Sign test data with a specific key
    Sign {
        #[arg(short, long, help = "Data to sign (text)")]
        data: String,
        #[arg(
            short,
            long,
            default_value = "ecdsa-p256",
            help = "Signature algorithm"
        )]
        algorithm: String,
    },
    /// Get hardware attestation proof
    Attest {
        #[arg(short, long, default_value = "test-challenge", help = "Challenge data")]
        challenge: String,
    },
    /// Show YubiKey backend capabilities
    Capabilities,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("■ TrustEdge YubiKey Demo - Hardware Root of Trust");
    println!("═══════════════════════════════════════════════════");

    // Configure YubiKey backend
    let mut config = YubiKeyConfig::default();

    if let Some(path) = args.pkcs11_path {
        config.pkcs11_module_path = path;
    }

    config.pin = args.pin;
    config.slot = args.slot;
    config.verbose = args.verbose;

    if args.verbose {
        println!("📋 Configuration:");
        println!("   PKCS#11 Module: {}", config.pkcs11_module_path);
        println!(
            "   PIN: {}",
            if config.pin.is_some() {
                "[PROVIDED]"
            } else {
                "[NONE]"
            }
        );
        println!("   Slot: {:?}", config.slot);
        println!();
    }

    // Initialize YubiKey backend
    let mut backend =
        YubiKeyBackend::with_config(config).context("Failed to initialize YubiKey backend")?;

    // Execute command
    match args.command {
        Commands::Test => test_connectivity(&backend)?,
        Commands::ListKeys => list_keys(&backend)?,
        Commands::Sign { data, algorithm } => {
            let key_id = args.key_id.unwrap_or_else(|| "default".to_string());
            sign_data(&mut backend, &key_id, &data, &algorithm)?;
        }
        Commands::Attest { challenge } => {
            let key_id = args.key_id.unwrap_or_else(|| "default".to_string());
            attest_hardware(&mut backend, &key_id, &challenge)?;
        }
        Commands::Capabilities => show_capabilities(&backend)?,
    }

    println!("✔ Demo completed successfully!");
    Ok(())
}

fn test_connectivity(backend: &YubiKeyBackend) -> Result<()> {
    println!("● Testing YubiKey connectivity...");

    let info = backend.backend_info();
    println!("   Backend: {} - {}", info.name, info.description);
    println!(
        "   Available: {}",
        if info.available { "✔ YES" } else { "✖ NO" }
    );

    if !info.available {
        return Err(anyhow::anyhow!("YubiKey not available"));
    }

    println!("✔ YubiKey connectivity test passed!");
    Ok(())
}

fn list_keys(backend: &YubiKeyBackend) -> Result<()> {
    println!("● Listing keys on YubiKey...");
    let keys = backend.list_keys().context("Failed to list keys")?;

    if keys.is_empty() {
        println!("   No keys found on YubiKey");
        return Ok(());
    }

    println!("   Found {} key(s):", keys.len());
    for (i, key) in keys.iter().enumerate() {
        println!("   {}. Key ID: {:02x?}", i + 1, key.key_id);
        println!("      Description: {}", key.description);
        println!("      Created: {} (unix timestamp)", key.created_at);
        if let Some(last_used) = key.last_used {
            println!("      Last Used: {} (unix timestamp)", last_used);
        } else {
            println!("      Last Used: Never");
        }
        println!();
    }

    Ok(())
}

fn sign_data(
    backend: &mut YubiKeyBackend,
    key_id: &str,
    data: &str,
    algorithm: &str,
) -> Result<()> {
    println!("● Signing data with YubiKey...");
    println!("   Key ID: {}", key_id);
    println!("   Data: \"{}\"", data);
    println!("   Algorithm: {}", algorithm);

    let sig_alg = match algorithm {
        "ed25519" => SignatureAlgorithm::Ed25519,
        "ecdsa-p256" => SignatureAlgorithm::EcdsaP256,
        "rsa-pkcs1v15" => SignatureAlgorithm::RsaPkcs1v15,
        "rsa-pss" => SignatureAlgorithm::RsaPss,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported algorithm: {}. Supported: ed25519, ecdsa-p256, rsa-pkcs1v15, rsa-pss",
                algorithm
            ))
        }
    };

    let operation = CryptoOperation::Sign {
        data: data.as_bytes().to_vec(),
        algorithm: sig_alg,
    };

    let result = backend
        .perform_operation(key_id, operation)
        .context("Failed to sign data")?;

    match result {
        trustedge_core::backends::universal::CryptoResult::Signed(signature) => {
            println!("✔ Signature generated successfully!");
            println!("   Signature length: {} bytes", signature.len());
            println!(
                "   Signature preview: {:02x?}...",
                &signature[..std::cmp::min(8, signature.len())]
            );
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unexpected result type for signing operation"
            ))
        }
    }

    Ok(())
}

fn attest_hardware(backend: &mut YubiKeyBackend, key_id: &str, challenge: &str) -> Result<()> {
    println!("● Getting hardware attestation...");
    println!("   Key ID: {}", key_id);
    println!("   Challenge: \"{}\"", challenge);

    let operation = CryptoOperation::Attest {
        challenge: challenge.as_bytes().to_vec(),
    };

    let result = backend
        .perform_operation(key_id, operation)
        .context("Failed to get attestation")?;

    match result {
        trustedge_core::backends::universal::CryptoResult::AttestationProof(proof) => {
            println!("✔ Attestation proof generated!");
            println!("   Proof length: {} bytes", proof.len());
            println!(
                "   Proof (hex): {}",
                hex::encode(&proof[..std::cmp::min(32, proof.len())])
            );
            if proof.len() > 32 {
                println!("   ... (truncated, showing first 32 bytes)");
            }
        }
        _ => return Err(anyhow::anyhow!("Unexpected result type")),
    }

    Ok(())
}

fn show_capabilities(backend: &YubiKeyBackend) -> Result<()> {
    println!("● YubiKey Backend Capabilities:");

    let caps = backend.get_capabilities();

    println!(
        "   Hardware Backed: {}",
        if caps.hardware_backed {
            "✔ YES"
        } else {
            "✖ NO"
        }
    );
    println!(
        "   Supports Attestation: {}",
        if caps.supports_attestation {
            "✔ YES"
        } else {
            "✖ NO"
        }
    );
    println!(
        "   Key Derivation: {}",
        if caps.supports_key_derivation {
            "✔ YES"
        } else {
            "✖ NO"
        }
    );
    println!(
        "   Key Generation: {}",
        if caps.supports_key_generation {
            "✔ YES"
        } else {
            "✖ NO"
        }
    );

    if let Some(max_size) = caps.max_key_size {
        println!("   Max Key Size: {} bits", max_size);
    }

    println!("   Asymmetric Algorithms:");
    for alg in &caps.asymmetric_algorithms {
        println!("     • {:?}", alg);
    }

    println!("   Signature Algorithms:");
    for alg in &caps.signature_algorithms {
        println!("     • {:?}", alg);
    }

    println!("   Hash Algorithms:");
    for alg in &caps.hash_algorithms {
        println!("     • {:?}", alg);
    }

    Ok(())
}
