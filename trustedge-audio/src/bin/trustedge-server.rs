// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast;

// network payload type from lib.rs
use trustedge_audio::{
    build_aad, server_authenticate, KeyBackend, KeyContext, KeyringBackend, Manifest, NetworkChunk,
    SessionManager, SignedManifest, NONCE_LEN,
};

// ---- Crypto bits ------------------------------------------------------------

use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes256Gcm, Key, Nonce,
};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

// ---- CLI --------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "trustedge-server",
    version,
    about = "TrustEdge network processing server"
)]
struct Args {
    /// Address to listen on
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,

    /// Directory to save received chunks (optional)
    #[arg(short, long)]
    output_dir: Option<std::path::PathBuf>,

    /// AES-256 key as hex (64 chars) - required if --decrypt and not using keyring
    #[arg(long)]
    key_hex: Option<String>,

    /// Set passphrase in system keyring (run once to configure)
    #[arg(long)]
    set_passphrase: Option<String>,

    /// Salt for key derivation (hex string, 32 hex chars => 16 bytes)
    #[arg(long)]
    salt_hex: Option<String>,

    /// Use keyring passphrase instead of --key-hex
    #[arg(long)]
    use_keyring: bool,

    /// Decrypt received chunks and save plaintext
    #[arg(long)]
    decrypt: bool,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Enable mutual authentication (requires client certificates)
    #[arg(long)]
    require_auth: bool,

    /// Server identity for certificate generation
    #[arg(long, default_value = "TrustEdge Server")]
    server_identity: String,

    /// Path to server signing key file (optional, generates if not found)
    #[arg(long)]
    server_key: Option<std::path::PathBuf>,
}

// ---- Per-connection state ---------------------------------------------------

struct ProcessingSession {
    connection_id: u64,
    #[allow(dead_code)]
    chunks: HashMap<u64, (Vec<u8>, SignedManifest)>, // available if you later buffer
    cipher: Option<Aes256Gcm>,
    output_file: Option<tokio::fs::File>,

    // stream invariants (locked by first valid chunk)
    expected_seq_next: u64,
    stream_header_hash: Option<[u8; 32]>,
    stream_nonce_prefix: Option<[u8; 4]>,

    // authentication info
    authenticated: bool,
    session_id: Option<[u8; 16]>,
    client_identity: Option<String>,
}

// ---- Main -------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(passphrase) = args.set_passphrase {
        let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
        backend.store_passphrase(&passphrase)?;
        println!("Passphrase stored in system keyring");
        return Ok(());
    }

    // Build cipher if decrypting
    let cipher = if args.decrypt {
        let key_bytes = if args.use_keyring {
            let salt_hex = args
                .salt_hex
                .as_ref()
                .ok_or_else(|| anyhow!("--salt-hex required when using keyring"))?;
            let salt_bytes = hex::decode(salt_hex)?;
            anyhow::ensure!(
                salt_bytes.len() == 16,
                "Salt must be 16 bytes (32 hex chars)"
            );
            let mut salt = [0u8; 16];
            salt.copy_from_slice(&salt_bytes);
            println!("Using keyring passphrase with provided salt");

            let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
            let context = KeyContext::new(salt.to_vec());
            let derived_key = backend.derive_key(&salt, &context)?;
            let mut key_bytes = [0u8; 32];
            key_bytes.copy_from_slice(&derived_key);
            key_bytes
        } else if let Some(ref key_hex) = args.key_hex {
            parse_key_hex(key_hex)?
        } else {
            return Err(anyhow!(
                "Either --key-hex or --use-keyring is required for --decrypt"
            ));
        };
        Some(Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes)))
    } else {
        None
    };

    // Ensure output dir exists
    if let Some(ref dir) = args.output_dir {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create output directory: {:?}", dir))?;
    }

    let listener = TcpListener::bind(args.listen)
        .await
        .with_context(|| format!("Failed to bind to {}", args.listen))?;

    println!("[SRV] TrustEdge server listening on {}", args.listen);
    println!(
        "[DIR] Output directory: {:?}",
        args.output_dir
            .as_deref()
            .unwrap_or(std::path::Path::new("(none)"))
    );
    println!(
        "[SEC] Decryption: {}",
        if args.decrypt { "ENABLED" } else { "disabled" }
    );
    println!(
        "[AUTH] Mutual authentication: {}",
        if args.require_auth {
            "REQUIRED"
        } else {
            "disabled"
        }
    );

    // Initialize session manager if authentication is required
    let session_manager = if args.require_auth {
        Some(SessionManager::new(args.server_identity.clone())?)
    } else {
        None
    };

    if let Some(ref manager) = session_manager {
        println!(
            "[AUTH] Server certificate: {}",
            manager.server_certificate().identity
        );
    }

    // Create shutdown signal handler
    let (shutdown_tx, _) = broadcast::channel::<()>(1);
    let shutdown_listener = shutdown_tx.clone();

    // Spawn shutdown signal handler
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
        println!("\n[SRV] Shutdown signal received, stopping server...");
        let _ = shutdown_listener.send(());
    });

    let mut connection_id = 0u64;
    let mut active_connections = Vec::new();
    let mut shutdown_rx = shutdown_tx.subscribe();

    loop {
        tokio::select! {
            // Accept new connections
            accept_result = listener.accept() => {
                match accept_result {
                    Ok((stream, peer_addr)) => {
                        connection_id += 1;
                        println!(
                            "[CONN] New connection #{} from {}",
                            connection_id, peer_addr
                        );

                        let session = ProcessingSession {
                            connection_id,
                            chunks: HashMap::new(),
                            cipher: cipher.clone(),
                            output_file: None,
                            expected_seq_next: 1, // first chunk must be seq=1
                            stream_header_hash: None,
                            stream_nonce_prefix: None,
                            authenticated: !args.require_auth, // authenticated if auth not required
                            session_id: None,
                            client_identity: None,
                        };

                        let output_dir = args.output_dir.clone();
                        let verbose = args.verbose;
                        let decrypt = args.decrypt;
                        let require_auth = args.require_auth;
                        let conn_shutdown_rx = shutdown_tx.subscribe();

                        // Clone session manager for this connection if needed
                        let session_mgr = if require_auth {
                            session_manager.as_ref().map(|mgr| SessionManager::new(mgr.server_certificate().identity.clone()).unwrap())
                        } else {
                            None
                        };

                        let handle = tokio::spawn(async move {
                            let config = ConnectionConfig {
                                output_dir,
                                decrypt,
                                verbose,
                                require_auth,
                                session_manager: session_mgr,
                            };
                            
                            if let Err(e) = handle_connection_with_shutdown(
                                stream, session, config, conn_shutdown_rx
                            ).await {
                                eprintln!("[ERR] Connection #{} error: {:#}", connection_id, e);
                            } else {
                                println!("[OK] Connection #{} completed", connection_id);
                            }
                        });

                        active_connections.push(handle);
                    }
                    Err(e) => {
                        eprintln!("[ERR] Failed to accept connection: {}", e);
                    }
                }
            }

            // Handle shutdown signal
            _ = shutdown_rx.recv() => {
                println!("[SRV] Graceful shutdown initiated...");
                break;
            }
        }
    }

    // Wait for active connections to complete
    println!(
        "[SRV] Waiting for {} active connections to complete...",
        active_connections.len()
    );
    for handle in active_connections {
        let _ = handle.await;
    }

    println!("[SRV] Server shutdown complete");
    Ok(())
}

// ---- Connection handler with shutdown support ------------------------------

struct ConnectionConfig {
    output_dir: Option<std::path::PathBuf>,
    decrypt: bool,
    verbose: bool,
    require_auth: bool,
    session_manager: Option<SessionManager>,
}

async fn handle_connection_with_shutdown(
    stream: TcpStream,
    session: ProcessingSession,
    config: ConnectionConfig,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    let connection_id = session.connection_id;

    tokio::select! {
        result = handle_connection(stream, session, config.output_dir, config.decrypt, config.verbose, config.require_auth, config.session_manager) => {
            result
        }
        _ = shutdown_rx.recv() => {
            if config.verbose {
                println!("[CONN] Connection #{} interrupted by shutdown", connection_id);
            }
            Ok(())
        }
    }
}

// ---- Connection handler -----------------------------------------------------

async fn handle_connection(
    mut stream: TcpStream,
    mut session: ProcessingSession,
    output_dir: Option<std::path::PathBuf>,
    decrypt: bool,
    verbose: bool,
    require_auth: bool,
    mut session_manager: Option<SessionManager>,
) -> Result<()> {
    let peer_addr = stream.peer_addr().context("Failed to get peer address")?;

    // Perform authentication if required
    if require_auth {
        if let Some(ref mut mgr) = session_manager {
            match server_authenticate(&mut stream, mgr).await {
                Ok(auth_session) => {
                    session.authenticated = true;
                    session.session_id = Some(auth_session.session_id);
                    session.client_identity = auth_session.client_identity.clone();

                    if verbose {
                        println!(
                            "[AUTH] Connection #{} authenticated successfully. Client: {}",
                            session.connection_id,
                            auth_session
                                .client_identity
                                .as_deref()
                                .unwrap_or("(anonymous)")
                        );
                    }
                }
                Err(e) => {
                    eprintln!(
                        "[AUTH] Connection #{} authentication failed: {}",
                        session.connection_id, e
                    );
                    return Err(e.context("Authentication failed"));
                }
            }
        } else {
            return Err(anyhow!(
                "Authentication required but no session manager available"
            ));
        }
    }

    let mut chunks_received = 0u64;
    let mut total_enc_bytes = 0usize;
    let mut total_pt_bytes = 0usize;

    // If decrypting, create output file
    if decrypt {
        if let Some(ref dir) = output_dir {
            let filename = format!("conn{}_decrypted.bin", session.connection_id);
            let filepath = dir.join(filename);
            session.output_file = Some(
                tokio::fs::File::create(&filepath)
                    .await
                    .with_context(|| format!("Failed to create output file: {:?}", filepath))?,
            );
            println!(
                "[WRITE] Connection #{}: Writing decrypted data to {:?}",
                session.connection_id, filepath
            );
        }
    }

    loop {
        // 4-byte little-endian length prefix (as in the client)
        let mut len_bytes = [0u8; 4];
        match stream.read_exact(&mut len_bytes).await {
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e).context("Failed to read chunk length"),
        }
        let length = u32::from_le_bytes(len_bytes) as usize;
        anyhow::ensure!(
            length <= 100 * 1024 * 1024,
            "Chunk too large: {} bytes",
            length
        );

        if verbose {
            println!(
                "[READ] Connection #{}: Reading frame of {} bytes",
                session.connection_id, length
            );
        }

        let mut chunk_bytes = vec![0; length];
        stream
            .read_exact(&mut chunk_bytes)
            .await
            .context("Failed to read chunk")?;

        let chunk: NetworkChunk =
            bincode::deserialize(&chunk_bytes).context("Failed to deserialize NetworkChunk")?;
        chunk.validate().context("Chunk validation failed")?;

        chunks_received += 1;
        total_enc_bytes += chunk.data.len();

        if verbose {
            println!(
                "📨 Conn #{}: got chunk #{}, seq={}, nonce_len={}, ct_len={}, manifest_len={}",
                session.connection_id,
                chunks_received,
                chunk.sequence,
                chunk.nonce.len(),
                chunk.data.len(),
                chunk.manifest.len()
            );
        }

        // Enforce basic ordering (simplest: strictly increasing)
        anyhow::ensure!(
            chunk.sequence == session.expected_seq_next,
            "Out-of-order chunk: got seq={}, expected {}",
            chunk.sequence,
            session.expected_seq_next
        );
        session.expected_seq_next += 1;

        // Decrypt path
        if decrypt && session.cipher.is_some() {
            let pt_len = process_and_decrypt_chunk(&chunk, &mut session, verbose).await?;
            total_pt_bytes += pt_len;
        }

        // Save encrypted payload to disk (optional)
        if let Some(ref dir) = output_dir {
            save_chunk_to_disk(dir, session.connection_id, &chunk)
                .await
                .context("Failed to save chunk to disk")?;
        }

        // ACK
        let ack = format!("ACK:{}", chunk.sequence);
        stream
            .write_all(&(ack.len() as u32).to_le_bytes())
            .await
            .context("Failed to write ACK length")?;
        stream
            .write_all(ack.as_bytes())
            .await
            .context("Failed to write ACK")?;
    }

    if let Some(ref mut f) = session.output_file {
        f.flush().await.context("flush output file")?;
    }

    println!(
        "📊 Connection #{} from {} finished: {} chunks, {} encrypted bytes, {} plaintext bytes",
        session.connection_id, peer_addr, chunks_received, total_enc_bytes, total_pt_bytes
    );

    Ok(())
}

// ---- Chunk processing -------------------------------------------------------

async fn process_and_decrypt_chunk(
    chunk: &NetworkChunk,
    session: &mut ProcessingSession,
    verbose: bool,
) -> Result<usize> {
    let cipher = session
        .cipher
        .as_ref()
        .ok_or_else(|| anyhow!("cipher missing in session"))?;

    // Deserialize SignedManifest
    let sm: SignedManifest =
        bincode::deserialize(&chunk.manifest).context("SignedManifest decode")?;

    // Verify signature
    let pubkey_arr: [u8; 32] = sm
        .pubkey
        .as_slice()
        .try_into()
        .context("pubkey length != 32")?;
    let sig_arr: [u8; 64] = sm.sig.as_slice().try_into().context("sig length != 64")?;
    let vk = VerifyingKey::from_bytes(&pubkey_arr).context("bad pubkey")?;
    vk.verify(&sm.manifest, &Signature::from_bytes(&sig_arr))
        .context("manifest signature verify failed")?;

    // Decode manifest
    let m: Manifest = bincode::deserialize(&sm.manifest).context("manifest decode")?;

    // Lock stream invariants on first chunk
    if session.stream_header_hash.is_none() {
        session.stream_header_hash = Some(m.header_hash);
        anyhow::ensure!(
            chunk.nonce.len() == NONCE_LEN,
            "nonce must be {} bytes",
            NONCE_LEN
        );
        let mut prefix = [0u8; 4];
        prefix.copy_from_slice(&chunk.nonce[..4]);
        session.stream_nonce_prefix = Some(prefix);
        if verbose {
            println!(
                "[LOCKED] Conn #{}: locked header_hash and nonce_prefix",
                session.connection_id
            );
        }
    }

    // Enforce invariants
    let shh = session.stream_header_hash.unwrap();
    anyhow::ensure!(
        m.header_hash == shh,
        "manifest.header_hash changed mid-stream"
    );

    let snp = session.stream_nonce_prefix.unwrap();
    anyhow::ensure!(
        chunk.nonce[..4] == snp,
        "record nonce prefix != stream nonce_prefix"
    );

    // Build AAD and decrypt
    let mh = blake3::hash(&sm.manifest);
    let aad = build_aad(&m.header_hash, chunk.sequence, &chunk.nonce, mh.as_bytes());
    let nonce = Nonce::from_slice(&chunk.nonce);

    let pt = cipher
        .decrypt(
            nonce,
            Payload {
                msg: &chunk.data,
                aad: &aad,
            },
        )
        .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

    // Verify plaintext hash
    let pt_hash_rx = blake3::hash(&pt);
    anyhow::ensure!(pt_hash_rx.as_bytes() == &m.pt_hash, "pt hash mismatch");

    if verbose {
        println!(
            "[UNLOCKED] Conn #{}: decrypted seq {} ({} bytes)",
            session.connection_id,
            chunk.sequence,
            pt.len()
        );
    }

    // Write plaintext if requested
    if let Some(ref mut f) = session.output_file {
        use tokio::io::AsyncWriteExt;
        f.write_all(&pt).await.context("write plaintext")?;
    }

    Ok(pt.len())
}

// ---- Helpers ----------------------------------------------------------------

fn parse_key_hex(s: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(s).context("key_hex not valid hex")?;
    anyhow::ensure!(bytes.len() == 32, "key_hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

async fn save_chunk_to_disk(
    output_dir: &std::path::Path,
    connection_id: u64,
    chunk: &NetworkChunk,
) -> Result<()> {
    let chunk_filename = format!("conn{}_seq{}.bin", connection_id, chunk.sequence);
    let chunk_path = output_dir.join(&chunk_filename);

    tokio::fs::write(&chunk_path, &chunk.data)
        .await
        .with_context(|| format!("Failed to write chunk to {:?}", chunk_path))?;

    let meta_filename = format!("conn{}_seq{}.meta.json", connection_id, chunk.sequence);
    let meta_path = output_dir.join(&meta_filename);

    let metadata = serde_json::json!({
        "sequence": chunk.sequence,
        "timestamp": chunk.timestamp,
        "data_size": chunk.data.len(),
        "manifest_size": chunk.manifest.len(),
        "nonce_hex": hex::encode(chunk.nonce),
        "received_at": chrono::Utc::now().to_rfc3339(),
    });

    tokio::fs::write(&meta_path, metadata.to_string())
        .await
        .with_context(|| format!("Failed to write metadata to {:?}", meta_path))?;

    Ok(())
}
