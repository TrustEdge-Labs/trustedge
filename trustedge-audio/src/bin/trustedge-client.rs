//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::{Result, Context};
use trustedge_audio::NetworkChunk;
use clap::Parser;
use std::path::PathBuf;

// Import the cryptography stuff from main.rs
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Key, Nonce,
};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer};
use rand_core::RngCore;
use serde::{Serialize, Deserialize};

use trustedge_audio::NONCE_LEN;

// structures from main.rs
// const NONCE_LEN: usize = 12;
const AAD_LEN: usize = 84;

#[derive(Serialize, Deserialize)]
struct Manifest {
    v: u8,
    ts_ms: u64,
    seq: u64,
    header_hash: [u8; 32],
    pt_hash: [u8; 32],
    ai_used: bool,
    model_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SignedManifest {
    manifest: Vec<u8>,
    sig: Vec<u8>,
    pubkey: Vec<u8>,
}

#[derive(Parser, Debug)]
#[command(name = "trustedge-client", version, about = "TrustEdge network client")]
struct Args {
    /// Server address to connect to
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    server: std::net::SocketAddr,
    
    /// File to encrypt and send
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Send test chunks instead of a real file
    #[arg(long)]
    test_chunks: Option<u64>,
    
    /// Chunk size for file processing
    #[arg(long, default_value_t = 4096)]
    chunk_size: usize,
    
    /// AES-256 key as hex (64 chars) - if not provided, generates random
    #[arg(long)]
    key_hex: Option<String>,
    
    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🔗 Connecting to TrustEdge server at {}", args.server);
    
    let mut stream = TcpStream::connect(args.server).await
        .with_context(|| format!("Failed to connect to {}", args.server))?;
    
    println!("✅ Connected successfully!");
    
    if let Some(num_chunks) = args.test_chunks {
        send_test_chunks(&mut stream, num_chunks, args.verbose).await?;
    } else if let Some(ref file_path) = args.file {
        send_encrypted_file(&mut stream, &file_path, &args).await?;
    } else {
        return Err(anyhow::anyhow!("Must specify either --file or --test-chunks"));
    }
    
    println!("🎉 All chunks sent successfully!");
    Ok(())
}

async fn send_encrypted_file(
    stream: &mut TcpStream, 
    file_path: &PathBuf,
    args: &Args
) -> Result<()> {
    println!("🔐 Encrypting and sending file: {:?}", file_path);
    
    // Set up encryption (copied from main.rs)
    let signing = SigningKey::generate(&mut OsRng);
    let _verify: VerifyingKey = signing.verifying_key();
    
    // Key setup
    let key_bytes = if let Some(ref kh) = args.key_hex {
        parse_key_hex(kh)?
    } else {
        let mut kb = [0u8; 32];
        OsRng.fill_bytes(&mut kb);
        println!("🔑 Generated AES-256 key: {}", hex::encode(kb));
        kb
    };
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    
    // Create dummy header hash for now - IRL use FileHeader
    let mut header_hash = [0u8; 32];
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"dummy-file-header");
    header_hash.copy_from_slice(hasher.finalize().as_bytes());
    
    // Read and encrypt file
    use tokio::io::AsyncReadExt as _;
    let mut file = tokio::fs::File::open(file_path).await
        .with_context(|| format!("Failed to open file: {:?}", file_path))?;
    
    let mut buffer = vec![0u8; args.chunk_size];
    let mut sequence = 0u64;
    let mut total_bytes_sent = 0usize;
    
    // Nonce setup - use random prefix for real security
    let mut nonce_prefix = [0u8; 4];
    OsRng.fill_bytes(&mut nonce_prefix);  // Back to random for proper security
    
    loop {
        let bytes_read = file.read(&mut buffer).await
            .context("Failed to read from file")?;
            
        if bytes_read == 0 {
            break; // End of file
        }
        
        sequence += 1;
        
        // Create nonce
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes[..4].copy_from_slice(&nonce_prefix);
        nonce_bytes[4..].copy_from_slice(&sequence.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Hash plaintext
        let pt_hash = blake3::hash(&buffer[..bytes_read]);
        
        // Create manifest
        let manifest = Manifest {
            v: 1,
            ts_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            seq: sequence,
            header_hash,
            pt_hash: *pt_hash.as_bytes(),
            ai_used: false,
            model_ids: vec![],
        };
        
        // Sign manifest
        let m_bytes = bincode::serialize(&manifest)?;
        let sig: Signature = signing.sign(&m_bytes);
        
        let signed_manifest = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: signing.verifying_key().to_bytes().to_vec(),
        };
        
        // Create AAD
        let manifest_hash = blake3::hash(&m_bytes);
        let aad = build_aad(&header_hash, sequence, &nonce_bytes, manifest_hash.as_bytes());
        
        // Encrypt chunk
        let ciphertext = cipher
            .encrypt(nonce, Payload { msg: &buffer[..bytes_read], aad: &aad })
            .map_err(|_| anyhow::anyhow!("AES-GCM encrypt failed"))?;
        
        // Create NetworkChunk with the nonce
        let network_chunk = NetworkChunk::new_with_nonce(
            sequence,
            ciphertext,
            bincode::serialize(&signed_manifest)?,
            nonce_bytes,  // Include the actual nonce used
        );
        
        // Send to server
        send_chunk(stream, &network_chunk, args.verbose).await
            .with_context(|| format!("Failed to send encrypted chunk {}", sequence))?;
        
        // Read acknowledgment
        let ack = read_ack(stream).await
            .with_context(|| format!("Failed to read ACK for chunk {}", sequence))?;
        
        total_bytes_sent += bytes_read;
        
        if args.verbose {
            println!("✅ Encrypted chunk {} sent ({} bytes plaintext -> {} bytes encrypted), ACK: {}", 
                     sequence, bytes_read, network_chunk.data.len(), ack);
        } else {
            print!("🔐");
            use std::io::Write;
            std::io::stdout().flush().unwrap();
        }
    }
    
    if !args.verbose {
        println!(); // New line after progress
    }
    
    println!("📊 Encrypted file transfer complete: {} chunks, {} bytes total", sequence, total_bytes_sent);
    Ok(())
}

// Helper functions copied from main.rs
fn parse_key_hex(s: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(s).context("key_hex: not valid hex")?;
    anyhow::ensure!(bytes.len() == 32, "key_hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

fn build_aad(header_hash: &[u8; 32], seq: u64, nonce: &[u8; NONCE_LEN], manifest_hash: &[u8; 32]) -> [u8; AAD_LEN] {
    let mut aad = [0u8; AAD_LEN];
    let mut off = 0;
    aad[off..off+32].copy_from_slice(header_hash); off += 32;
    aad[off..off+8].copy_from_slice(&seq.to_be_bytes()); off += 8;
    aad[off..off+NONCE_LEN].copy_from_slice(nonce); off += NONCE_LEN;
    aad[off..off+32].copy_from_slice(manifest_hash);
    aad
}

// test_chunks and helper functions
async fn send_test_chunks(stream: &mut TcpStream, num_chunks: u64, verbose: bool) -> Result<()> {
    println!("📦 Sending {} test chunks to TrustEdge server...", num_chunks);
    
    for i in 1..=num_chunks {
        let test_data = format!("This is test chunk number {}", i).repeat(10);
        let test_manifest = format!("Test manifest for chunk {}", i);
        
        let mut test_nonce = [0u8; NONCE_LEN];
        rand_core::RngCore::fill_bytes(&mut rand_core::OsRng, &mut test_nonce);

        let chunk = NetworkChunk::new_with_nonce(i, test_data.into_bytes(), test_manifest.into_bytes(), test_nonce);
        chunk.validate().context("Test chunk validation failed")?;
        
        send_chunk(stream, &chunk, verbose).await
            .with_context(|| format!("Failed to send test chunk {}", i))?;
        
        let ack = read_ack(stream).await
            .with_context(|| format!("Failed to read ACK for chunk {}", i))?;
            
        if verbose {
            println!("✅ Chunk {} acknowledged: {}", i, ack);
        }
    }
    
    Ok(())
}

async fn send_chunk(stream: &mut TcpStream, chunk: &NetworkChunk, verbose: bool) -> Result<()> {
    let serialized = bincode::serialize(chunk)
        .context("Failed to serialize NetworkChunk")?;
    
    let length = serialized.len() as u32;
    
    if verbose {
        println!("📤 Sending chunk seq={}, {} bytes", chunk.sequence, length);
    }
    
    stream.write_all(&length.to_le_bytes()).await
        .context("Failed to write chunk length")?;
    stream.write_all(&serialized).await
        .context("Failed to write chunk data")?;
    
    Ok(())
}

async fn read_ack(stream: &mut TcpStream) -> Result<String> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).await
        .context("Failed to read ACK length")?;
        
    let length = u32::from_le_bytes(len_bytes) as usize;
    
    let mut ack_bytes = vec![0; length];
    stream.read_exact(&mut ack_bytes).await
        .context("Failed to read ACK data")?;
    
    String::from_utf8(ack_bytes)
        .context("ACK is not valid UTF-8")
}
