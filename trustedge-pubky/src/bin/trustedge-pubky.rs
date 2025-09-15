#![forbid(unsafe_code)]

//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
/// 
/// TrustEdge Pubky CLI - Decentralized key management and hybrid encryption
///

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use trustedge_pubky::{create_pubky_backend_random, create_pubky_backend_from_seed, send_trusted_data, receive_trusted_data, extract_private_key_seed};

#[derive(Parser)]
#[command(name = "trustedge-pubky")]
#[command(about = "TrustEdge Pubky CLI - Decentralized key management and hybrid encryption")]
#[command(long_about = "
TrustEdge Pubky CLI provides decentralized key management and hybrid encryption
using the Pubky network for key resolution. This enables secure communication
without centralized key infrastructure.

Key Features:
• Generate and manage Pubky identities
• Publish public keys to the decentralized Pubky network  
• Resolve recipient keys by Pubky ID
• Hybrid encryption (X25519 ECDH + AES-256-GCM)
• Migration tools for envelope format upgrades

Security Model:
- Private keys are stored locally and never transmitted
- Public keys are published to the Pubky network for discovery
- Hybrid encryption provides forward secrecy and efficiency
- All operations use cryptographically secure random number generation

Examples:
  # Generate a new identity
  trustedge-pubky generate --output my-key.txt
  
  # Encrypt a file for someone
  trustedge-pubky encrypt --input document.pdf --output document.trst --recipient <pubky-id>
  
  # Decrypt a received file
  trustedge-pubky decrypt --input document.trst --output document.pdf --key my-key.txt
")]
#[command(version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new Pubky keypair and identity
    /// 
    /// Creates a new cryptographic identity for use with the Pubky network.
    /// The identity consists of a private key (kept secret) and a Pubky ID 
    /// (derived from the public key, used for identification).
    /// 
    /// SECURITY WARNING: The private key file contains sensitive cryptographic
    /// material. Store it securely and never share it. Loss of the private key
    /// means permanent loss of access to encrypted data.
    Generate {
        /// Output file for the private key (32 bytes as hex)
        /// 
        /// The private key will be saved as 64 hexadecimal characters.
        /// This file is required for decryption and should be kept secure.
        #[arg(short, long, help = "Save private key to file (KEEP SECURE!)")]
        output: Option<PathBuf>,
        
        /// Generate from seed (32 bytes hex) instead of random
        /// 
        /// Use a specific seed for deterministic key generation.
        /// Useful for key recovery or testing. Seed must be exactly
        /// 64 hexadecimal characters (32 bytes).
        #[arg(long, help = "Use specific seed (64 hex chars) instead of random")]
        seed: Option<String>,
        
        /// Show only the Pubky ID (public identifier)
        /// 
        /// Outputs just the Pubky ID without additional information.
        /// Useful for scripting or when you only need the public identifier.
        #[arg(long, help = "Output only the Pubky ID")]
        id_only: bool,
    },
    
    /// Publish your public key to the Pubky network
    Publish {
        /// Private key file (32 bytes hex)
        #[arg(short, long)]
        key: PathBuf,
        
        /// TrustEdge public key to publish (if different from Pubky key)
        #[arg(long)]
        trustedge_key: Option<PathBuf>,
    },
    
    /// Resolve a Pubky ID to get the TrustEdge public key
    Resolve {
        /// Pubky ID to resolve (hex-encoded)
        pubky_id: String,
        
        /// Output file for the resolved public key
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        /// Show key information in human-readable format
        #[arg(long)]
        info: bool,
    },
    
    /// Encrypt data for a Pubky recipient
    /// 
    /// Encrypts a file using hybrid encryption (X25519 ECDH + AES-256-GCM).
    /// The recipient's public key is automatically resolved from the Pubky network
    /// using their Pubky ID. The encrypted envelope can only be decrypted by
    /// the recipient using their private key.
    /// 
    /// The encryption process:
    /// 1. Resolve recipient's public key from Pubky network
    /// 2. Generate ephemeral key pair for this encryption
    /// 3. Perform ECDH key exchange to derive shared secret
    /// 4. Encrypt data with AES-256-GCM using derived key
    /// 5. Package everything into a v2 Pubky envelope
    Encrypt {
        /// Input file to encrypt
        /// 
        /// Any file type is supported. The original file format is preserved
        /// and will be restored exactly upon decryption.
        #[arg(short, long, help = "File to encrypt")]
        input: PathBuf,
        
        /// Output file for encrypted envelope (.trst recommended)
        /// 
        /// The encrypted envelope contains all information needed for decryption
        /// by the recipient. Use .trst extension by convention.
        #[arg(short, long, help = "Encrypted envelope output file")]
        output: PathBuf,
        
        /// Pubky ID of the recipient (64 hex characters)
        /// 
        /// The recipient's public Pubky ID. Their public key will be automatically
        /// resolved from the Pubky network. Get this from the recipient or use
        /// 'trustedge-pubky resolve' to verify it exists.
        #[arg(short, long, help = "Recipient's Pubky ID (64 hex chars)")]
        recipient: String,
        
        /// Your private key file (currently unused - ephemeral keys used)
        /// 
        /// Reserved for future use. Currently, ephemeral keys are generated
        /// for each encryption to provide forward secrecy.
        #[arg(short, long, help = "Your private key file (future use)")]
        key: Option<PathBuf>,
    },
    
    /// Decrypt a Pubky envelope
    /// 
    /// Decrypts a v2 Pubky envelope that was encrypted for you. Requires your
    /// private key file that corresponds to the Pubky ID the sender used.
    /// The original file format and content are restored exactly.
    /// 
    /// The decryption process:
    /// 1. Parse the v2 Pubky envelope
    /// 2. Extract the ephemeral public key from the envelope
    /// 3. Perform ECDH key exchange with your private key
    /// 4. Derive the same shared secret used for encryption
    /// 5. Decrypt the data using AES-256-GCM
    /// 6. Verify integrity and restore original file
    Decrypt {
        /// Input envelope file (.trst)
        /// 
        /// The encrypted envelope file received from the sender.
        /// Must be a valid v2 Pubky envelope format.
        #[arg(short, long, help = "Encrypted envelope file to decrypt")]
        input: PathBuf,
        
        /// Output file for decrypted data
        /// 
        /// The decrypted file will be written here with original content
        /// and format preserved exactly as it was before encryption.
        #[arg(short, long, help = "Output file for decrypted data")]
        output: PathBuf,
        
        /// Your private key file (32 bytes as hex)
        /// 
        /// The private key file created with 'trustedge-pubky generate'.
        /// Must correspond to the Pubky ID the sender used as recipient.
        #[arg(short, long, help = "Your private key file (64 hex chars)")]
        key: PathBuf,
    },
    
    /// Migrate v1 envelopes to v2 Pubky format
    Migrate {
        /// Input v1 envelope file
        #[arg(short, long)]
        input: PathBuf,
        
        /// Output v2 envelope file
        #[arg(short, long)]
        output: PathBuf,
        
        /// Recipient Pubky ID for v2 envelope
        #[arg(short, long)]
        recipient: String,
        
        /// Your private key for v1 decryption
        #[arg(long)]
        v1_key: PathBuf,
        
        /// Your Pubky private key for v2 encryption
        #[arg(long)]
        pubky_key: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    match args.command {
        Commands::Generate { output, seed, id_only } => {
            generate_keypair(output, seed, id_only)
        }
        Commands::Publish { key, trustedge_key } => {
            publish_key(key, trustedge_key)
        }
        Commands::Resolve { pubky_id, output, info } => {
            resolve_key(pubky_id, output, info)
        }
        Commands::Encrypt { input, output, recipient, key } => {
            encrypt_data(input, output, recipient, key)
        }
        Commands::Decrypt { input, output, key } => {
            decrypt_data(input, output, key)
        }
        Commands::Migrate { input, output, recipient, v1_key, pubky_key } => {
            migrate_envelope(input, output, recipient, v1_key, pubky_key)
        }
    }
}

fn generate_keypair(output: Option<PathBuf>, seed: Option<String>, id_only: bool) -> Result<()> {
    let backend = if let Some(seed_hex) = seed {
        // Validate seed format
        let seed_hex = seed_hex.trim();
        if seed_hex.len() != 64 {
            anyhow::bail!(
                "❌ Invalid seed length: {} characters\n\
                Expected: 64 hexadecimal characters (32 bytes)\n\
                Example: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef",
                seed_hex.len()
            );
        }
        
        let seed_bytes = hex::decode(&seed_hex)
            .with_context(|| format!(
                "❌ Invalid seed format: not valid hexadecimal\n\
                Seed must contain only characters 0-9 and a-f\n\
                Provided: {}",
                seed_hex
            ))?;
            
        if seed_bytes.len() != 32 {
            anyhow::bail!(
                "❌ Seed decoded to {} bytes, expected 32 bytes\n\
                This should not happen with 64 hex characters",
                seed_bytes.len()
            );
        }
        
        let mut seed_array = [0u8; 32];
        seed_array.copy_from_slice(&seed_bytes);
        create_pubky_backend_from_seed(&seed_array)
            .context("❌ Failed to create Pubky backend from seed")?
    } else {
        create_pubky_backend_random()
            .context("❌ Failed to create random Pubky backend")?
    };
    
    let pubky_id = backend.our_pubky_id();
    
    if id_only {
        println!("{}", pubky_id);
        return Ok(());
    }
    
    println!("Generated Pubky Identity:");
    println!("  Pubky ID: {}", pubky_id);
    
    if let Some(output_path) = output {
        let private_key_seed = extract_private_key_seed(&backend);
        let private_key_hex = hex::encode(private_key_seed);
        
        std::fs::write(&output_path, &private_key_hex)
            .with_context(|| format!(
                "❌ Failed to write private key to: {}\n\
                Check that:\n\
                • The directory exists\n\
                • You have write permissions\n\
                • There is sufficient disk space",
                output_path.display()
            ))?;
        
        println!("✅ Private key saved to: {}", output_path.display());
        println!("🔐 Key file contains: 64 hexadecimal characters (32 bytes)");
        println!("⚠️  SECURITY WARNING: Keep this file secure!");
        println!("   • Never share this file with anyone");
        println!("   • Store it in a secure location");
        println!("   • Consider encrypting it with a password");
        println!("   • Loss of this file means permanent loss of encrypted data");
    } else {
        println!("💡 Use --output to save the private key to a file");
        println!("⚠️  Without saving, this identity cannot be recovered!");
        println!("   Example: trustedge-pubky generate --output my-key.txt");
    }
    
    Ok(())
}

fn publish_key(_key: PathBuf, _trustedge_key: Option<PathBuf>) -> Result<()> {
    // TODO: Implement key publishing
    anyhow::bail!("Key publishing not yet implemented - requires async Pubky client integration");
}

fn resolve_key(pubky_id: String, output: Option<PathBuf>, info: bool) -> Result<()> {
    // Create a backend to resolve the key
    let backend = create_pubky_backend_random()
        .context("Failed to create Pubky backend")?;
    
    let public_key = backend.resolve_public_key_sync(&pubky_id)
        .context("Failed to resolve Pubky ID")?;
    
    if info {
        println!("Resolved TrustEdge Public Key:");
        println!("  Pubky ID: {}", pubky_id);
        println!("  Algorithm: {:?}", public_key.algorithm);
        println!("  Key Size: {} bytes", public_key.key_bytes.len());
        if let Some(key_id) = &public_key.key_id {
            println!("  Key ID: {}", key_id);
        }
    }
    
    if let Some(output_path) = output {
        // Save the public key in a simple format
        let key_data = serde_json::to_string_pretty(&serde_json::json!({
            "pubky_id": pubky_id,
            "algorithm": format!("{:?}", public_key.algorithm),
            "key_bytes": hex::encode(&public_key.key_bytes),
            "key_id": public_key.key_id
        }))?;
        
        std::fs::write(&output_path, key_data)
            .context("Failed to write public key file")?;
        
        println!("✅ Public key saved to: {}", output_path.display());
    }
    
    Ok(())
}

fn encrypt_data(input: PathBuf, output: PathBuf, recipient: String, _key: Option<PathBuf>) -> Result<()> {
    // Validate recipient Pubky ID format
    let recipient = recipient.trim();
    if recipient.len() != 64 {
        anyhow::bail!(
            "❌ Invalid recipient Pubky ID length: {} characters\n\
            Expected: 64 hexadecimal characters\n\
            Example: 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef\n\
            Provided: {}",
            recipient.len(),
            recipient
        );
    }
    
    if !recipient.chars().all(|c| c.is_ascii_hexdigit()) {
        anyhow::bail!(
            "❌ Invalid recipient Pubky ID format: contains non-hexadecimal characters\n\
            Pubky ID must contain only characters 0-9 and a-f\n\
            Provided: {}",
            recipient
        );
    }
    
    // Read input data
    println!("📖 Reading input file: {}", input.display());
    let data = std::fs::read(&input)
        .with_context(|| format!(
            "❌ Failed to read input file: {}\n\
            Check that:\n\
            • The file exists\n\
            • You have read permissions\n\
            • The file is not locked by another process",
            input.display()
        ))?;
    
    if data.is_empty() {
        println!("⚠️  Warning: Input file is empty");
    }
    
    // Create Pubky backend (ephemeral keys for forward secrecy)
    println!("🔑 Creating ephemeral encryption keys...");
    let backend = create_pubky_backend_random()
        .context("❌ Failed to create Pubky backend for encryption")?;
    
    // Encrypt data (this will resolve the recipient's key from Pubky network)
    println!("🌐 Resolving recipient's public key from Pubky network...");
    println!("🔒 Encrypting data with hybrid encryption (X25519 + AES-256-GCM)...");
    let envelope = send_trusted_data(&data, &recipient, &backend)
        .with_context(|| format!(
            "❌ Failed to encrypt data for recipient: {}\n\
            Possible causes:\n\
            • Recipient's Pubky ID not found on network\n\
            • Network connectivity issues\n\
            • Invalid recipient public key format",
            recipient
        ))?;
    
    // Write envelope
    println!("💾 Writing encrypted envelope...");
    std::fs::write(&output, &envelope)
        .with_context(|| format!(
            "❌ Failed to write envelope to: {}\n\
            Check that:\n\
            • The directory exists\n\
            • You have write permissions\n\
            • There is sufficient disk space ({} bytes needed)",
            output.display(),
            envelope.len()
        ))?;
    
    println!("✅ Encryption completed successfully!");
    println!("📊 Encryption Summary:");
    println!("   📁 Input:     {} ({} bytes)", input.display(), data.len());
    println!("   📦 Envelope:  {} ({} bytes)", output.display(), envelope.len());
    println!("   👤 Recipient: {}", recipient);
    println!("   🔐 Format:    v2 Pubky envelope (X25519 + AES-256-GCM)");
    println!("   📈 Overhead:  {} bytes ({:.1}%)", 
             envelope.len() - data.len(), 
             (envelope.len() - data.len()) as f64 / data.len() as f64 * 100.0);
    
    println!("\n💡 Next steps:");
    println!("   • Send {} to the recipient", output.display());
    println!("   • Recipient can decrypt with: trustedge-pubky decrypt --input {} --output <file> --key <their-key>", output.display());
    
    Ok(())
}

fn decrypt_data(input: PathBuf, output: PathBuf, key: PathBuf) -> Result<()> {
    // Read envelope
    let envelope = std::fs::read(&input)
        .context("Failed to read envelope file")?;
    
    // Read private key
    let key_hex = std::fs::read_to_string(&key)
        .context("Failed to read private key file")?;
    let key_bytes = hex::decode(key_hex.trim())
        .context("Invalid private key hex")?;
    if key_bytes.len() != 32 {
        anyhow::bail!("Private key must be exactly 32 bytes (64 hex characters)");
    }
    
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    
    // Create PrivateKey from the seed
    // We need to determine the algorithm from the envelope or use a default
    use trustedge_core::{PrivateKey, AsymmetricAlgorithm};
    
    // For now, assume Ed25519 - in a real implementation, this should be detected
    let private_key = PrivateKey::new(AsymmetricAlgorithm::Ed25519, key_array.to_vec());
    
    // Decrypt the envelope
    let decrypted_data = receive_trusted_data(&envelope, &private_key)
        .context("Failed to decrypt envelope")?;
    
    // Write decrypted data
    std::fs::write(&output, &decrypted_data)
        .context("Failed to write decrypted data")?;
    
    println!("✅ Decryption complete");
    println!("   Input: {} ({} bytes)", input.display(), envelope.len());
    println!("   Output: {} ({} bytes)", output.display(), decrypted_data.len());
    
    Ok(())
}

fn migrate_envelope(
    input: PathBuf,
    output: PathBuf,
    recipient: String,
    v1_key: PathBuf,
    _pubky_key: PathBuf,
) -> Result<()> {
    println!("🔄 Migrating envelope from v1 to v2 Pubky format...");
    
    // Step 1: Read the v1 envelope
    let v1_envelope_data = std::fs::read(&input)
        .context("Failed to read v1 envelope file")?;
    
    println!("   📖 Read v1 envelope: {} bytes", v1_envelope_data.len());
    
    // Step 2: Decrypt the v1 envelope using the old key
    // For now, this is a placeholder - we need to implement v1 decryption
    println!("   🔓 Decrypting v1 envelope...");
    
    // Read the v1 private key
    let _v1_key_data = std::fs::read_to_string(&v1_key)
        .context("Failed to read v1 private key")?;
    
    // This is a simplified approach - in reality, we'd need to:
    // 1. Detect the v1 envelope format
    // 2. Use the appropriate v1 decryption method
    // 3. Extract the original plaintext
    
    anyhow::bail!(
        "V1 envelope decryption not yet implemented.\n\
        \n\
        To complete migration, you need to:\n\
        1. Decrypt the v1 envelope manually using trustedge-core\n\
        2. Re-encrypt the plaintext using trustedge-pubky encrypt\n\
        \n\
        Example workflow:\n\
        # Decrypt v1 envelope\n\
        trustedge-core --decrypt --input {} --out plaintext.dat --key-hex <v1-key>\n\
        \n\
        # Re-encrypt with Pubky\n\
        trustedge-pubky encrypt --input plaintext.dat --output {} --recipient {}\n\
        \n\
        # Clean up\n\
        rm plaintext.dat",
        input.display(),
        output.display(),
        recipient
    );
}