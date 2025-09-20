//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Software HSM demo and testing tool
//!
//! This tool demonstrates the Software HSM backend capabilities:
//! - Key generation for multiple algorithms
//! - Digital signing and verification
//! - Integration with UniversalBackend architecture

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use trustedge_core::backends::{
    software_hsm::{SoftwareHsmBackend, SoftwareHsmConfig},
    universal::{
        AsymmetricAlgorithm, CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend,
    },
    universal_registry::UniversalBackendRegistry,
};

#[derive(Parser)]
#[command(name = "software-hsm-demo")]
#[command(about = "Software HSM backend demonstration tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to key store directory
    #[arg(long, default_value = "./demo_keys")]
    key_store: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new key pair
    GenerateKey {
        /// Key identifier
        key_id: String,
        /// Key algorithm
        #[arg(value_enum, default_value = "ed25519")]
        algorithm: KeyAlgorithm,
        /// Key description
        #[arg(long)]
        description: Option<String>,
    },
    /// List available keys
    ListKeys,
    /// Sign data with a key
    Sign {
        /// Key identifier
        key_id: String,
        /// Data to sign (text)
        data: String,
        /// Signature algorithm
        #[arg(value_enum, default_value = "ed25519")]
        algorithm: SigAlgorithm,
    },
    /// Verify a signature
    Verify {
        /// Key identifier
        key_id: String,
        /// Original data (text)
        data: String,
        /// Signature (hex-encoded)
        signature: String,
        /// Signature algorithm
        #[arg(value_enum, default_value = "ed25519")]
        algorithm: SigAlgorithm,
    },
    /// Get public key for a key ID
    GetPublicKey {
        /// Key identifier
        key_id: String,
    },
    /// Test UniversalBackend registry integration
    TestRegistry,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum KeyAlgorithm {
    Ed25519,
    EcdsaP256,
}

impl From<KeyAlgorithm> for AsymmetricAlgorithm {
    fn from(ka: KeyAlgorithm) -> Self {
        match ka {
            KeyAlgorithm::Ed25519 => AsymmetricAlgorithm::Ed25519,
            KeyAlgorithm::EcdsaP256 => AsymmetricAlgorithm::EcdsaP256,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SigAlgorithm {
    Ed25519,
    EcdsaP256,
}

impl From<SigAlgorithm> for SignatureAlgorithm {
    fn from(sa: SigAlgorithm) -> Self {
        match sa {
            SigAlgorithm::Ed25519 => SignatureAlgorithm::Ed25519,
            SigAlgorithm::EcdsaP256 => SignatureAlgorithm::EcdsaP256,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Create Software HSM backend with custom config
    let config = SoftwareHsmConfig {
        key_store_path: cli.key_store.clone(),
        default_passphrase: "demo_passphrase_123!".to_string(),
        metadata_file: cli.key_store.join("metadata.json"),
    };

    match cli.command {
        Commands::GenerateKey {
            key_id,
            algorithm,
            description,
        } => {
            let mut backend = SoftwareHsmBackend::with_config(config)?;
            println!("Generating {:?} key pair with ID: {}", algorithm, key_id);

            backend.generate_key_pair(&key_id, algorithm.into(), description)?;

            println!("✔ Key pair generated successfully!");
            println!(
                "   Private key stored in: {}",
                cli.key_store
                    .join(format!("{}_private.key", key_id))
                    .display()
            );
            println!(
                "   Public key stored in: {}",
                cli.key_store
                    .join(format!("{}_public.key", key_id))
                    .display()
            );
        }

        Commands::ListKeys => {
            let backend = SoftwareHsmBackend::with_config(config)?;
            let keys = backend.list_keys()?;

            if keys.is_empty() {
                println!("No keys found. Generate some keys first!");
            } else {
                println!("Available keys:");
                for key in keys {
                    println!("  ● ID: {}", hex::encode(key.key_id));
                    println!("     Description: {}", key.description);
                    println!(
                        "     Created: {}",
                        chrono::DateTime::from_timestamp(key.created_at as i64, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| "Unknown".to_string())
                    );
                    if let Some(last_used) = key.last_used {
                        println!(
                            "     Last used: {}",
                            chrono::DateTime::from_timestamp(last_used as i64, 0)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                                .unwrap_or_else(|| "Unknown".to_string())
                        );
                    }
                    println!();
                }
            }
        }

        Commands::Sign {
            key_id,
            data,
            algorithm,
        } => {
            let backend = SoftwareHsmBackend::with_config(config)?;

            println!("Signing data with key: {}", key_id);
            println!("Data: {}", data);
            println!("Algorithm: {:?}", algorithm);

            let operation = CryptoOperation::Sign {
                data: data.as_bytes().to_vec(),
                algorithm: algorithm.into(),
            };

            match backend.perform_operation(&key_id, operation)? {
                CryptoResult::Signed(signature) => {
                    println!("✔ Signature generated:");
                    println!("   {}", hex::encode(&signature));
                    println!("   Length: {} bytes", signature.len());
                }
                _ => unreachable!("Sign operation should return Signed result"),
            }
        }

        Commands::Verify {
            key_id,
            data,
            signature,
            algorithm,
        } => {
            let backend = SoftwareHsmBackend::with_config(config)?;

            println!("Verifying signature with key: {}", key_id);
            println!("Data: {}", data);
            println!("Signature: {}", signature);

            let signature_bytes = hex::decode(&signature)
                .map_err(|e| anyhow::anyhow!("Invalid hex signature: {}", e))?;

            let operation = CryptoOperation::Verify {
                data: data.as_bytes().to_vec(),
                signature: signature_bytes,
                algorithm: algorithm.into(),
            };

            match backend.perform_operation(&key_id, operation)? {
                CryptoResult::VerificationResult(is_valid) => {
                    if is_valid {
                        println!("✔ Signature is VALID");
                    } else {
                        println!("✖ Signature is INVALID");
                    }
                }
                _ => unreachable!("Verify operation should return VerificationResult"),
            }
        }

        Commands::GetPublicKey { key_id } => {
            let backend = SoftwareHsmBackend::with_config(config)?;

            println!("Getting public key for: {}", key_id);

            let operation = CryptoOperation::GetPublicKey;

            match backend.perform_operation(&key_id, operation)? {
                CryptoResult::PublicKey(public_key) => {
                    println!("✔ Public key (PEM format):");
                    println!("{}", String::from_utf8_lossy(&public_key));
                }
                _ => unreachable!("GetPublicKey operation should return PublicKey result"),
            }
        }

        Commands::TestRegistry => {
            println!("Testing UniversalBackend registry integration...");

            let mut registry = UniversalBackendRegistry::new();
            let backend = SoftwareHsmBackend::with_config(config)?;
            registry.register_backend("software_hsm".to_string(), Box::new(backend));

            println!("✔ Registered Software HSM backend");

            // List backend capabilities
            let capabilities = registry.get_all_capabilities();
            for (name, caps) in capabilities {
                println!("Backend: {}", name);
                println!("  Hardware backed: {}", caps.hardware_backed);
                println!(
                    "  Supports key generation: {}",
                    caps.supports_key_generation
                );
                println!("  Asymmetric algorithms: {:?}", caps.asymmetric_algorithms);
                println!("  Signature algorithms: {:?}", caps.signature_algorithms);
                println!();
            }

            // Test operation routing
            let test_operation = CryptoOperation::GenerateKeyPair {
                algorithm: AsymmetricAlgorithm::Ed25519,
            };

            if let Some(backend_ref) = registry.find_backend_for_operation(&test_operation) {
                println!(
                    "✔ Found backend for Ed25519 key generation: {}",
                    backend_ref.backend_info().name
                );
            } else {
                println!("✖ No backend found for Ed25519 key generation");
            }
        }
    }

    Ok(())
}
