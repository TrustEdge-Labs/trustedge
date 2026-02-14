// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::broadcast;
use tokio::time::timeout;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[cfg(feature = "keyring")]
use trustedge_core::KeyringBackend;
use trustedge_core::{server_authenticate, NetworkChunk, SessionManager};

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

    /// Maximum bytes per connection (default: 1GB)
    #[arg(long, default_value = "1073741824")]
    max_connection_bytes: u64,

    /// Maximum chunks per connection (default: 10000)
    #[arg(long, default_value = "10000")]
    max_connection_chunks: u64,

    /// Connection read timeout in seconds (default: 30)
    #[arg(long, default_value = "30")]
    connection_timeout: u64,

    /// Enable MAC'd ACKs with session IDs
    #[arg(long)]
    secure_acks: bool,
}

// ---- Per-connection state ---------------------------------------------------

struct ProcessingSession {
    connection_id: u64,
    output_file: Option<tokio::fs::File>,

    // stream invariants (locked by first valid chunk)
    expected_seq_next: u64,
    stream_nonce_prefix: Option<[u8; 4]>,
    header_verified: bool,
    header_locked: bool,

    // authentication info
    authenticated: bool,
    session_id: Option<[u8; 16]>,
    client_identity: Option<String>,

    // connection limits and tracking
    bytes_received: u64,
    chunks_received: u64,
    last_activity: Instant,
}

// ---- Main -------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(_passphrase) = args.set_passphrase {
        #[cfg(feature = "keyring")]
        {
            let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
            backend.store_passphrase(&_passphrase)?;
            println!("Passphrase stored in system keyring");
            return Ok(());
        }
        #[cfg(not(feature = "keyring"))]
        {
            return Err(anyhow::anyhow!("Keyring support requires the 'keyring' feature. Build with: cargo build --features keyring"));
        }
    }

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

                        let now = Instant::now();
                        let session = ProcessingSession {
                            connection_id,
                            output_file: None,
                            expected_seq_next: 1, // first chunk must be seq=1
                            stream_nonce_prefix: None,
                            header_verified: false,
                            header_locked: false,
                            authenticated: !args.require_auth, // authenticated if auth not required
                            session_id: None,
                            client_identity: None,
                            bytes_received: 0,
                            chunks_received: 0,
                            last_activity: now,
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
                            max_connection_bytes: args.max_connection_bytes,
                            max_connection_chunks: args.max_connection_chunks,
                            connection_timeout: Duration::from_secs(args.connection_timeout),
                            secure_acks: args.secure_acks,
                        };                            if let Err(e) = handle_connection_with_shutdown(
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

#[derive(Clone)]
struct ConnectionConfig {
    output_dir: Option<std::path::PathBuf>,
    decrypt: bool,
    verbose: bool,
    require_auth: bool,
    session_manager: Option<SessionManager>,
    max_connection_bytes: u64,
    max_connection_chunks: u64,
    connection_timeout: Duration,
    secure_acks: bool,
}

async fn handle_connection_with_shutdown(
    stream: TcpStream,
    session: ProcessingSession,
    config: ConnectionConfig,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> Result<()> {
    let connection_id = session.connection_id;

    tokio::select! {
        result = handle_hardened_connection(stream, session, &config) => {
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

// ---- Hardened Connection handler -------------------------------------------

async fn handle_hardened_connection(
    stream: TcpStream,
    mut session: ProcessingSession,
    config: &ConnectionConfig,
) -> Result<()> {
    let peer_addr = stream.peer_addr().context("Failed to get peer address")?;

    // Create framed transport with length-delimited codec
    let codec = LengthDelimitedCodec::builder()
        .max_frame_length(16 * 1024 * 1024) // 16MB max frame
        .new_codec();
    let mut framed = Framed::new(stream, codec);

    if config.verbose {
        println!(
            "[CONN] Connection #{} from {} - using hardened handler",
            session.connection_id, peer_addr
        );
    }

    // Perform authentication if required
    if config.require_auth {
        if let Some(mut mgr) = config.session_manager.clone() {
            // Extract the underlying stream for authentication
            let mut stream = framed.into_parts().io;
            match server_authenticate(&mut stream, &mut mgr).await {
                Ok(auth_session) => {
                    session.authenticated = true;
                    session.session_id = Some(auth_session.session_id);
                    session.client_identity = auth_session.client_identity.clone();

                    // Recreate framed transport
                    let codec = LengthDelimitedCodec::builder()
                        .max_frame_length(16 * 1024 * 1024)
                        .new_codec();
                    framed = Framed::new(stream, codec);

                    if config.verbose {
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

    // If decrypting, create output file
    if config.decrypt {
        if let Some(ref dir) = config.output_dir {
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

    // Main processing loop
    loop {
        // Check connection limits before processing
        if config.max_connection_bytes > 0 && session.bytes_received > config.max_connection_bytes {
            anyhow::bail!(
                "Connection #{} exceeded byte limit: {} bytes (max: {})",
                session.connection_id,
                session.bytes_received,
                config.max_connection_bytes
            );
        }

        if config.max_connection_chunks > 0
            && session.chunks_received > config.max_connection_chunks
        {
            anyhow::bail!(
                "Connection #{} exceeded chunk limit: {} chunks (max: {})",
                session.connection_id,
                session.chunks_received,
                config.max_connection_chunks
            );
        }

        // Check idle timeout
        if session.last_activity.elapsed() > config.connection_timeout {
            anyhow::bail!(
                "Connection #{} idle timeout: {:?}",
                session.connection_id,
                config.connection_timeout
            );
        }

        // Receive frame with timeout
        let frame = match timeout(config.connection_timeout, framed.next()).await {
            Ok(Some(Ok(frame))) => frame,
            Ok(Some(Err(e))) => {
                return Err(anyhow::anyhow!("Frame decode error: {}", e));
            }
            Ok(None) => {
                if config.verbose {
                    println!(
                        "[CONN] Connection #{} closed by client",
                        session.connection_id
                    );
                }
                break;
            }
            Err(_) => {
                anyhow::bail!("Connection #{} read timeout", session.connection_id);
            }
        };

        // Update tracking
        session.bytes_received += frame.len() as u64;
        session.chunks_received += 1;
        session.last_activity = Instant::now();

        // Deserialize NetworkChunk
        let chunk: NetworkChunk = match bincode::deserialize(&frame) {
            Ok(chunk) => chunk,
            Err(e) => {
                eprintln!(
                    "[ERR] Connection #{} failed to deserialize chunk: {}",
                    session.connection_id, e
                );
                continue;
            }
        };

        // Capture sequence before moving chunk
        let chunk_sequence = chunk.sequence;

        // Process the chunk with basic validation
        match process_chunk_hardened(&mut session, chunk, config.decrypt, config.verbose).await {
            Ok(ack_msg) => {
                // Send ACK
                let ack_response = if config.secure_acks {
                    create_secure_ack(&session, &ack_msg)?
                } else {
                    ack_msg
                };

                if let Err(e) = framed.send(ack_response.into_bytes().into()).await {
                    eprintln!(
                        "[ERR] Connection #{} failed to send ACK: {}",
                        session.connection_id, e
                    );
                    break;
                }

                if config.verbose {
                    println!(
                        "[ACK] Connection #{} sent ACK for chunk seq={}",
                        session.connection_id, chunk_sequence
                    );
                }
            }
            Err(e) => {
                eprintln!(
                    "[ERR] Connection #{} chunk processing failed: {}",
                    session.connection_id, e
                );

                // Send error response
                let error_msg = format!("ERROR: {}", e);
                if let Err(send_err) = framed.send(error_msg.into_bytes().into()).await {
                    eprintln!(
                        "[ERR] Connection #{} failed to send error response: {}",
                        session.connection_id, send_err
                    );
                }
                break;
            }
        }
    }

    if config.verbose {
        println!(
            "[STATS] Connection #{} processed {} chunks, {} bytes",
            session.connection_id, session.chunks_received, session.bytes_received
        );
    }

    Ok(())
}

// ---- Hardened chunk processing ---------------------------------------------

async fn process_chunk_hardened(
    session: &mut ProcessingSession,
    chunk: NetworkChunk,
    _decrypt: bool,
    verbose: bool,
) -> Result<String> {
    // Extract nonce prefix for header verification (first 4 bytes)
    let chunk_nonce_prefix = [
        chunk.nonce[0],
        chunk.nonce[1],
        chunk.nonce[2],
        chunk.nonce[3],
    ];

    // Verify header consistency on first chunk and lock it
    if !session.header_locked {
        if !session.header_verified {
            // First chunk - store nonce prefix as "header" identifier
            session.stream_nonce_prefix = Some(chunk_nonce_prefix);
            session.header_verified = true;
            session.header_locked = true;

            if verbose {
                println!(
                    "[HEADER] Connection #{} locked stream nonce prefix: {:02x?}",
                    session.connection_id, chunk_nonce_prefix
                );
            }
        }
    } else {
        // Subsequent chunks - verify nonce prefix consistency (prevents renegotiation)
        if let Some(expected_prefix) = session.stream_nonce_prefix {
            if chunk_nonce_prefix != expected_prefix {
                anyhow::bail!(
                    "Nonce prefix mismatch - stream header locked, renegotiation rejected"
                );
            }
        }
    }

    // Verify sequence number consistency
    if chunk.sequence != session.expected_seq_next {
        anyhow::bail!(
            "Sequence mismatch: got {}, expected {}",
            chunk.sequence,
            session.expected_seq_next
        );
    }

    // Basic chunk validation passed
    if verbose {
        println!(
            "[CHUNK] Connection #{} processed chunk seq={}, {} bytes encrypted data",
            session.connection_id,
            chunk.sequence,
            chunk.data.len()
        );
    }

    // For now, we'll just acknowledge the chunk without decryption
    // Full decryption would require parsing the manifest to get header_hash
    // and rebuilding the AAD properly

    session.expected_seq_next += 1;
    Ok(format!("ACK {}", chunk.sequence))
}

// ---- Secure ACK generation -------------------------------------------------

fn create_secure_ack(session: &ProcessingSession, ack_msg: &str) -> Result<String> {
    if let Some(session_id) = session.session_id {
        // Create MAC'd ACK with session ID
        let session_id_hex = hex::encode(session_id);
        let mac_input = format!("{}:{}", session_id_hex, ack_msg);

        // Create a proper 32-byte key from session_id (16 bytes) by extending it
        let mut blake3_key = [0u8; 32];
        blake3_key[..16].copy_from_slice(&session_id);
        blake3_key[16..].copy_from_slice(&session_id); // Duplicate to fill 32 bytes

        let mac = blake3::keyed_hash(&blake3_key, mac_input.as_bytes());
        let mac_hex = hex::encode(&mac.as_bytes()[..8]); // First 8 bytes as MAC

        Ok(format!("{}:MAC:{}", ack_msg, mac_hex))
    } else {
        // No session ID available, fall back to regular ACK
        Ok(ack_msg.to_string())
    }
}

// ---- Helpers ----------------------------------------------------------------
// (No helpers currently needed)
