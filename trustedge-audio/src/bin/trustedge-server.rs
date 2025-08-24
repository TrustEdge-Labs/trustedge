//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use anyhow::{Result, Context};
use trustedge_audio::NetworkChunk;
use clap::Parser;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt as _;

// Import cryptography for decryption
use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use ed25519_dalek::{VerifyingKey, Signature, Verifier};
use serde::{Serialize, Deserialize};

// Copy structures from client
const NONCE_LEN: usize = 12;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SignedManifest {
    manifest: Vec<u8>,
    sig: Vec<u8>,
    pubkey: Vec<u8>,
}

#[derive(Parser, Debug)]
#[command(name = "trustedge-server", version, about = "TrustEdge network processing server")]
struct Args {
    /// Address to listen on
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,
    
    /// Directory to save received chunks (optional)
    #[arg(short, long)]
    output_dir: Option<std::path::PathBuf>,
    
    /// AES-256 key as hex (64 chars) - required for decryption
    #[arg(long)]
    key_hex: Option<String>,
    
    /// Decrypt received chunks and save plaintext
    #[arg(long)]
    decrypt: bool,
    
    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

struct ProcessingSession {
    connection_id: u64,
    chunks: HashMap<u64, (Vec<u8>, SignedManifest)>, // seq -> (ciphertext, signed_manifest)
    cipher: Option<Aes256Gcm>,
    output_file: Option<tokio::fs::File>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Validate key if decryption is enabled
    let cipher = if args.decrypt {
        let key_hex = args.key_hex.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--key-hex is required when --decrypt is enabled"))?;
        let key_bytes = parse_key_hex(key_hex)?;
        Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes)))
    } else {
        None
    };
    
    // Create output directory if specified
    if let Some(ref dir) = args.output_dir {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create output directory: {:?}", dir))?;
    }
    
    let listener = TcpListener::bind(args.listen).await
        .with_context(|| format!("Failed to bind to {}", args.listen))?;
    
    println!("🚀 TrustEdge server listening on {}", args.listen);
    println!("📁 Output directory: {:?}", args.output_dir.as_deref().unwrap_or(std::path::Path::new("(none)")));
    println!("🔐 Decryption: {}", if args.decrypt { "ENABLED" } else { "disabled" });
    
    let mut connection_id = 0u64;
    
    while let Ok((stream, peer_addr)) = listener.accept().await {
        connection_id += 1;
        println!("🔗 New connection #{} from {}", connection_id, peer_addr);
        
        let session = ProcessingSession {
            connection_id,
            chunks: HashMap::new(),
            cipher: cipher.clone(),
            output_file: None,
        };
        
        let output_dir = args.output_dir.clone();
        let verbose = args.verbose;
        let decrypt = args.decrypt;
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, session, output_dir, decrypt, verbose).await {
                eprintln!("❌ Connection #{} error: {}", connection_id, e);
            } else {
                println!("✅ Connection #{} completed successfully", connection_id);
            }
        });
    }
    
    Ok(())
}

async fn handle_connection(
    mut stream: TcpStream, 
    mut session: ProcessingSession,
    output_dir: Option<std::path::PathBuf>,
    decrypt: bool,
    verbose: bool
) -> Result<()> {
    let peer_addr = stream.peer_addr().context("Failed to get peer address")?;
    
    let mut chunks_received = 0u64;
    let mut total_bytes = 0usize;
    let mut total_plaintext_bytes = 0usize;
    
    // If decrypting, create output file
    if decrypt {
        if let Some(ref dir) = output_dir {
            let filename = format!("conn{}_decrypted.bin", session.connection_id);
            let filepath = dir.join(filename);
            session.output_file = Some(
                tokio::fs::File::create(&filepath).await
                    .with_context(|| format!("Failed to create output file: {:?}", filepath))?
            );
            println!("📝 Connection #{}: Writing decrypted data to {:?}", session.connection_id, filepath);
        }
    }
    
    loop {
        // Read length prefix (4 bytes, little-endian)
        let mut len_bytes = [0u8; 4];
        match stream.read_exact(&mut len_bytes).await {
            Ok(_) => {},
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                break; // Client closed connection gracefully
            }
            Err(e) => return Err(e).context("Failed to read chunk length"),
        }
        
        let length = u32::from_le_bytes(len_bytes) as usize;
        
        // Sanity check on chunk size
        if length > 100 * 1024 * 1024 { // 100MB max
            return Err(anyhow::anyhow!("Chunk too large: {} bytes", length));
        }
        
        if verbose {
            println!("📦 Connection #{}: Reading chunk of {} bytes", session.connection_id, length);
        }
        
        // Read chunk data
        let mut chunk_bytes = vec![0; length];
        stream.read_exact(&mut chunk_bytes).await
            .context("Failed to read chunk data")?;
        
        // Deserialize the NetworkChunk
        let chunk: NetworkChunk = bincode::deserialize(&chunk_bytes)
            .context("Failed to deserialize NetworkChunk")?;
            
        // Validate the chunk
        chunk.validate().context("Chunk validation failed")?;
        
        chunks_received += 1;
        total_bytes += chunk.data.len();
        
        println!("📨 Connection #{}: Received chunk #{} (seq={}, encrypted={} bytes, manifest={} bytes)", 
                 session.connection_id, chunks_received, chunk.sequence, chunk.data.len(), chunk.manifest.len());
        
        // Process the chunk (decrypt if enabled)
        if decrypt && session.cipher.is_some() {
            match process_encrypted_chunk(&chunk, &mut session, verbose).await {
                Ok(plaintext_size) => {
                    total_plaintext_bytes += plaintext_size;
                    if verbose {
                        println!("🔓 Connection #{}: Decrypted chunk {} ({} bytes plaintext)", 
                                 session.connection_id, chunk.sequence, plaintext_size);
                    }
                }
                Err(e) => {
                    eprintln!("⚠️  Connection #{}: Failed to decrypt chunk {}: {}", 
                              session.connection_id, chunk.sequence, e);
                    // Continue processing other chunks
                }
            }
        }
        
        // Save raw chunk to disk if requested
        if let Some(ref dir) = output_dir {
            save_chunk_to_disk(dir, session.connection_id, &chunk).await
                .context("Failed to save chunk to disk")?;
        }
        
        // Send acknowledgment
        let ack_msg = format!("ACK:{}", chunk.sequence);
        let ack_bytes = ack_msg.as_bytes();
        let ack_len = ack_bytes.len() as u32;
        
        stream.write_all(&ack_len.to_le_bytes()).await
            .context("Failed to write ack length")?;
        stream.write_all(ack_bytes).await
            .context("Failed to write ack data")?;
            
        if verbose {
            println!("✉️  Connection #{}: Sent ACK for chunk {}", session.connection_id, chunk.sequence);
        }
    }
    
    // Flush and close output file
    if let Some(ref mut file) = session.output_file {
        file.flush().await.context("Failed to flush output file")?;
    }
    
    println!("📊 Connection #{} from {} finished: {} chunks, {} encrypted bytes, {} plaintext bytes", 
             session.connection_id, peer_addr, chunks_received, total_bytes, total_plaintext_bytes);
    
    Ok(())
}

async fn process_encrypted_chunk(
    chunk: &NetworkChunk,
    session: &mut ProcessingSession,
    verbose: bool
) -> Result<usize> {
    let cipher = session.cipher.as_ref().unwrap();
    
    // Deserialize the signed manifest
    let signed_manifest: SignedManifest = bincode::deserialize(&chunk.manifest)
        .context("Failed to deserialize SignedManifest")?;
    
    // Verify the signature
    let pubkey_arr: [u8; 32] = signed_manifest.pubkey.as_slice()
        .try_into()
        .context("Invalid pubkey length")?;
    let sig_arr: [u8; 64] = signed_manifest.sig.as_slice()
        .try_into()
        .context("Invalid signature length")?;
        
    let vk = VerifyingKey::from_bytes(&pubkey_arr)
        .context("Invalid public key")?;
    vk.verify(&signed_manifest.manifest, &Signature::from_bytes(&sig_arr))
        .context("Manifest signature verification failed")?;

    // Deserialize the manifest
    let manifest: Manifest = bincode::deserialize(&signed_manifest.manifest)
        .context("Failed to deserialize manifest")?;
    
    if verbose {
        println!("✅ Connection #{}: Signature verified for chunk {}", session.connection_id, chunk.sequence);
    }
    
    // Use the nonce that was sent with the chunk
    let nonce = Nonce::from_slice(&chunk.nonce);
    
    // Build AAD using the received nonce
    let manifest_hash = blake3::hash(&signed_manifest.manifest);
    let aad = build_aad(&manifest.header_hash, chunk.sequence, &chunk.nonce, manifest_hash.as_bytes());
    
    // Decrypt using the correct nonce
    let plaintext = cipher
        .decrypt(nonce, Payload { msg: &chunk.data, aad: &aad })
        .map_err(|_| anyhow::anyhow!("AES-GCM decrypt/verify failed"))?;
    
    // Verify plaintext hash
    let pt_hash = blake3::hash(&plaintext);
    if pt_hash.as_bytes() != &manifest.pt_hash {
        return Err(anyhow::anyhow!("Plaintext hash mismatch"));
    }
    
    // Write to output file if available
    if let Some(ref mut file) = session.output_file {
        use tokio::io::AsyncWriteExt;
        file.write_all(&plaintext).await
            .context("Failed to write plaintext to output file")?;
    }
    
    Ok(plaintext.len())
}

// Helper functions
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

async fn save_chunk_to_disk(
    output_dir: &std::path::Path, 
    connection_id: u64, 
    chunk: &NetworkChunk
) -> Result<()> {
    let chunk_filename = format!("conn{}_chunk{}_seq{}.bin", 
                                 connection_id, chunk.sequence, chunk.sequence);
    let chunk_path = output_dir.join(&chunk_filename);
    
    tokio::fs::write(&chunk_path, &chunk.data).await
        .with_context(|| format!("Failed to write chunk to {:?}", chunk_path))?;
    
    let meta_filename = format!("conn{}_chunk{}_seq{}.meta.json", 
                                connection_id, chunk.sequence, chunk.sequence);
    let meta_path = output_dir.join(&meta_filename);
    
    let metadata = serde_json::json!({
        "sequence": chunk.sequence,
        "timestamp": chunk.timestamp,
        "data_size": chunk.data.len(),
        "manifest_size": chunk.manifest.len(),
        "received_at": chrono::Utc::now().to_rfc3339(),
    });
    
    tokio::fs::write(&meta_path, metadata.to_string()).await
        .with_context(|| format!("Failed to write metadata to {:?}", meta_path))?;
    
    Ok(())
}
