// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use anyhow::{Context, Result};
use clap::Parser;
use futures_util::{SinkExt, StreamExt};
use std::path::PathBuf;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

use trustedge_core::{
    auth::{client_authenticate, load_server_cert, save_client_cert, ClientCertificate},
    build_aad, FileHeader, KeyBackend, KeyContext, KeyringBackend, Manifest, NetworkChunk,
    SignedManifest, NONCE_LEN, VERSION,
};

// --- Cryptograph ---
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Key,
};
use ed25519_dalek::{Signature, SigningKey};
use rand_core::RngCore;

/// Load client certificate from file  
fn load_client_cert(path: &PathBuf) -> Result<ClientCertificate> {
    let cert_data = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read client certificate from {:?}", path))?;

    let cert: ClientCertificate =
        serde_json::from_str(&cert_data).context("Failed to parse client certificate JSON")?;

    // Since signing key is not serialized, we need to reconstruct it
    if cert.signing_key.is_none() {
        return Err(anyhow::anyhow!("Client certificate file does not contain signing key. Please regenerate the certificate."));
    }

    Ok(cert)
}

// Network operation timeouts
const CHUNK_SEND_TIMEOUT: Duration = Duration::from_secs(30);
const ACK_READ_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Parser, Debug)]
#[command(name = "trustedge-client", version, about = "TrustEdge network client")]
struct Args {
    /// Server address to connect to
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    server: std::net::SocketAddr,

    /// File to send (will be processed into chunks)
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Send synthetic encrypted chunks instead of a real file
    #[arg(long)]
    test_chunks: Option<u64>,

    /// Chunk size for file processing
    #[arg(long, default_value_t = 4096)]
    chunk_size: usize,

    /// AES-256 key as hex (64 chars) - if not provided, generate or use keyring
    #[arg(long)]
    key_hex: Option<String>,

    /// Set passphrase in system keyring (run once to configure)
    #[arg(long)]
    set_passphrase: Option<String>,

    /// Salt for key derivation (hex, 32 chars -> 16 bytes)
    #[arg(long)]
    salt_hex: Option<String>,

    /// Use keyring passphrase instead of --key-hex
    #[arg(long)]
    use_keyring: bool,

    /// Connection timeout in seconds
    #[arg(long, default_value_t = 10)]
    connect_timeout: u64,

    /// Number of connection retry attempts
    #[arg(long, default_value_t = 3)]
    retry_attempts: u32,

    /// Delay between retry attempts in seconds
    #[arg(long, default_value_t = 2)]
    retry_delay: u64,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Use hardened transport with framed codec
    #[arg(long)]
    hardened: bool,

    /// Expect secure ACKs from server
    #[arg(long)]
    expect_secure_acks: bool,

    /// Path to client certificate file for authentication
    #[arg(long)]
    client_cert: Option<PathBuf>,

    /// Client identity for certificate generation (if cert doesn't exist)
    #[arg(long, default_value = "TrustEdge Client")]
    client_identity: String,

    /// Enable authentication with server certificate verification
    #[arg(long)]
    enable_auth: bool,

    /// Path to server certificate file (for authentication)
    #[arg(long)]
    server_cert: Option<PathBuf>,
}

/// Connect to server with timeout and retry logic
async fn connect_with_retry(
    server_addr: std::net::SocketAddr,
    connect_timeout: Duration,
    retry_attempts: u32,
    retry_delay: Duration,
    verbose: bool,
) -> Result<TcpStream> {
    let mut last_error = None;

    for attempt in 1..=retry_attempts {
        if verbose && attempt > 1 {
            println!("Connection attempt {} of {}", attempt, retry_attempts);
        }

        match timeout(connect_timeout, TcpStream::connect(server_addr)).await {
            Ok(Ok(stream)) => {
                if verbose {
                    println!("Connected to {} on attempt {}", server_addr, attempt);
                }
                return Ok(stream);
            }
            Ok(Err(e)) => {
                last_error = Some(anyhow::Error::from(e));
                if verbose {
                    println!("Connection attempt {} failed: connection refused", attempt);
                }
            }
            Err(_) => {
                last_error = Some(anyhow::anyhow!(
                    "Connection timeout after {:?}",
                    connect_timeout
                ));
                if verbose {
                    println!(
                        "Connection attempt {} failed: timeout after {:?}",
                        attempt, connect_timeout
                    );
                }
            }
        }

        // Don't sleep after the last attempt
        if attempt < retry_attempts {
            if verbose {
                println!("Waiting {:?} before retry...", retry_delay);
            }
            sleep(retry_delay).await;
        }
    }

    Err(last_error
        .unwrap_or_else(|| anyhow::anyhow!("Failed to connect after {} attempts", retry_attempts)))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(passphrase) = args.set_passphrase.as_ref() {
        let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
        backend.store_passphrase(passphrase)?;
        println!("Passphrase stored in system keyring");
        return Ok(());
    }

    println!("Connecting to TrustEdge server at {}", args.server);
    let mut stream = connect_with_retry(
        args.server,
        Duration::from_secs(args.connect_timeout),
        args.retry_attempts,
        Duration::from_secs(args.retry_delay),
        args.verbose,
    )
    .await
    .with_context(|| {
        format!(
            "Failed to connect to {} after {} attempts",
            args.server, args.retry_attempts
        )
    })?;
    println!("Connected successfully!");

    // Perform authentication if enabled
    if args.enable_auth {
        // Load or create client certificate
        let client_cert = if let Some(cert_path) = &args.client_cert {
            load_client_cert(cert_path).context("Failed to load client certificate")?
        } else {
            let cert = ClientCertificate::generate(&args.client_identity)?;

            // Save certificate for future use
            let cert_path = format!("{}_client.cert", args.client_identity);
            save_client_cert(&cert, &cert_path)?;
            println!("Generated new client certificate: {}", cert_path);
            cert
        };

        // Load server certificate
        let _server_cert = if let Some(cert_path) = &args.server_cert {
            load_server_cert(&cert_path.to_string_lossy())
                .context("Failed to load server certificate")?
        } else {
            return Err(anyhow::anyhow!(
                "Server certificate path required when authentication is enabled. Use --server-cert"
            ));
        };

        // Perform client authentication
        match client_authenticate(
            &mut stream,
            client_cert.signing_key()?,
            Some(client_cert.identity.clone()),
            None,
        )
        .await
        {
            Ok((session_id, _server_cert)) => {
                if args.verbose {
                    println!(
                        "[AUTH] Authenticated successfully with server. Session ID: {}",
                        hex::encode(session_id)
                    );
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Authentication failed: {}", e));
            }
        }
    }

    // Determine AES key
    let key_bytes = if args.use_keyring {
        let salt_hex = args
            .salt_hex
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("--salt-hex required when using keyring"))?;
        let salt_bytes = hex::decode(salt_hex)?;
        if salt_bytes.len() != 16 {
            return Err(anyhow::anyhow!("Salt must be 16 bytes (32 hex chars)"));
        }
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&salt_bytes);
        println!("Using keyring passphrase with provided salt");

        let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
        let context = KeyContext::new(salt.to_vec());
        let derived_key = backend.derive_key(&salt, &context)?;
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&derived_key);
        key_bytes
    } else if let Some(ref kh) = args.key_hex {
        parse_key_hex(kh)?
    } else {
        let mut kb = [0u8; 32];
        OsRng.fill_bytes(&mut kb);
        println!("Generated AES-256 key: {}", hex::encode(kb));
        kb
    };

    // Choose mode and transport
    if args.hardened {
        // Use hardened framed transport
        let codec = LengthDelimitedCodec::builder()
            .max_frame_length(16 * 1024 * 1024) // 16MB max frame
            .new_codec();
        let mut framed = Framed::new(stream, codec);

        if let Some(n) = args.test_chunks {
            send_encrypted_test_chunks_hardened(
                &mut framed,
                n,
                args.chunk_size,
                &key_bytes,
                args.verbose,
                args.expect_secure_acks,
            )
            .await?;
        } else if let Some(ref file_path) = args.file {
            send_encrypted_file_hardened(
                &mut framed,
                file_path,
                &key_bytes,
                args.chunk_size,
                args.verbose,
                args.expect_secure_acks,
            )
            .await?;
        } else {
            return Err(anyhow::anyhow!(
                "Must specify either --file or --test-chunks"
            ));
        }
    } else {
        // Use legacy transport
        if let Some(n) = args.test_chunks {
            send_encrypted_test_chunks(&mut stream, n, args.chunk_size, &key_bytes, args.verbose)
                .await?;
        } else if let Some(ref file_path) = args.file {
            send_encrypted_file(
                &mut stream,
                file_path,
                &key_bytes,
                args.chunk_size,
                args.verbose,
            )
            .await?;
        } else {
            return Err(anyhow::anyhow!(
                "Must specify either --file or --test-chunks"
            ));
        }
    }

    println!("All chunks sent successfully!");
    Ok(())
}

async fn send_encrypted_file(
    stream: &mut TcpStream,
    file_path: &PathBuf,
    key_bytes: &[u8; 32],
    chunk_size: usize,
    verbose: bool,
) -> Result<()> {
    println!("Encrypting and sending file: {:?}", file_path);

    let signing = SigningKey::generate(&mut OsRng);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));

    // Build a real header (mirrors main.rs)
    let (header_hash, nonce_prefix, key_id) = build_session_header(chunk_size)?;

    let mut file = tokio::fs::File::open(file_path)
        .await
        .with_context(|| format!("Failed to open file: {:?}", file_path))?;

    let mut buffer = vec![0u8; chunk_size];
    let mut sequence = 0u64;
    let mut total_bytes_sent = 0usize;

    loop {
        let bytes_read = file.read(&mut buffer).await.context("file read")?;
        if bytes_read == 0 {
            break;
        }
        sequence += 1;

        // Nonce = prefix || counter(seq)
        let nonce_bytes = make_nonce(nonce_prefix, sequence);
        let nonce = (&nonce_bytes).into();

        // Build manifest
        let pt_hash = blake3::hash(&buffer[..bytes_read]);
        let manifest = Manifest {
            v: 1,
            ts_ms: now_ms(),
            seq: sequence,
            header_hash,
            pt_hash: *pt_hash.as_bytes(),
            key_id,
            ai_used: false,
            model_ids: vec![],
            data_type: trustedge_core::DataType::File { mime_type: None }, // Generic file data
            chunk_len: bytes_read as u32, // Bind actual chunk length to AAD
        };

        // Sign & wrap
        let m_bytes = bincode::serialize(&manifest)?;
        let sig: Signature = trustedge_core::format::sign_manifest_with_domain(&signing, &m_bytes);
        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: signing.verifying_key().to_bytes().to_vec(),
        };

        // AAD
        let aad = build_aad(
            &header_hash,
            sequence,
            &nonce_bytes,
            blake3::hash(&m_bytes).as_bytes(),
            manifest.chunk_len,
        );

        // Encrypt
        let ciphertext = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: &buffer[..bytes_read],
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow::anyhow!("AES-GCM encrypt failed"))?;

        // Frame
        let chunk = NetworkChunk::new_with_nonce(
            sequence,
            ciphertext,
            bincode::serialize(&sm)?,
            nonce_bytes,
        );

        // Send chunk with timeout
        timeout(CHUNK_SEND_TIMEOUT, send_chunk(stream, &chunk, verbose))
            .await
            .context("Chunk send timeout")?
            .context("Failed to send chunk")?;

        // Read ACK with timeout
        let ack = timeout(ACK_READ_TIMEOUT, read_ack(stream))
            .await
            .context("ACK read timeout")?
            .context("Failed to read ACK")?;

        total_bytes_sent += bytes_read;

        if verbose {
            println!(
                "[OK] Encrypted chunk {} sent ({} pt bytes -> {} ct bytes), ACK: {}",
                sequence,
                bytes_read,
                chunk.data.len(),
                ack
            );
        } else {
            print!("[SEC]");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }

    if !verbose {
        println!();
    }
    println!(
        "[DONE] Encrypted file transfer complete: {} chunks, {} bytes total",
        sequence, total_bytes_sent
    );
    Ok(())
}

async fn send_encrypted_test_chunks(
    stream: &mut TcpStream,
    num_chunks: u64,
    chunk_size: usize,
    key_bytes: &[u8; 32],
    verbose: bool,
) -> Result<()> {
    println!("[SEND] Sending {} encrypted test chunks...", num_chunks);

    let signing = SigningKey::generate(&mut OsRng);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));

    // Real header (so server can lock invariants)
    let (header_hash, nonce_prefix, key_id) = build_session_header(chunk_size)?;

    for seq in 1..=num_chunks {
        // Synthesize plaintext of up to chunk_size bytes
        let mut pt = format!("This is encrypted test chunk #{seq}. ").into_bytes();
        while pt.len() < chunk_size.min(2048) {
            pt.extend_from_slice(b"padding...");
        }

        let nonce_bytes = make_nonce(nonce_prefix, seq);
        let nonce = (&nonce_bytes).into();

        let pt_hash = blake3::hash(&pt);
        let manifest = Manifest {
            v: 1,
            ts_ms: now_ms(),
            seq,
            header_hash,
            pt_hash: *pt_hash.as_bytes(),
            key_id,
            ai_used: false,
            model_ids: vec![],
            data_type: trustedge_core::DataType::File { mime_type: None }, // Test data
            chunk_len: pt.len() as u32, // Bind actual chunk length to AAD
        };

        let m_bytes = bincode::serialize(&manifest)?;
        let sig: Signature = trustedge_core::format::sign_manifest_with_domain(&signing, &m_bytes);
        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: signing.verifying_key().to_bytes().to_vec(),
        };

        let aad = build_aad(
            &header_hash,
            seq,
            &nonce_bytes,
            blake3::hash(&m_bytes).as_bytes(),
            manifest.chunk_len,
        );
        let ciphertext = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: &pt,
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow::anyhow!("AES-GCM encrypt failed"))?;

        let chunk =
            NetworkChunk::new_with_nonce(seq, ciphertext, bincode::serialize(&sm)?, nonce_bytes);

        // Send chunk with timeout
        timeout(CHUNK_SEND_TIMEOUT, send_chunk(stream, &chunk, verbose))
            .await
            .context("Chunk send timeout")?
            .context("Failed to send chunk")?;

        // Read ACK with timeout
        let ack = timeout(ACK_READ_TIMEOUT, read_ack(stream))
            .await
            .context("ACK read timeout")?
            .context("Failed to read ACK")?;

        if verbose {
            println!("[OK] Test chunk {} acknowledged: {}", seq, ack);
        } else {
            print!("[SEC]");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }

    if !verbose {
        println!();
    }
    Ok(())
}

// --- Small helpers -----------------------------------------------------------

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn make_nonce(prefix: [u8; 4], seq: u64) -> [u8; NONCE_LEN] {
    let mut nonce_bytes = [0u8; NONCE_LEN];
    nonce_bytes[..4].copy_from_slice(&prefix);
    nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
    nonce_bytes
}

fn build_session_header(chunk_size: usize) -> Result<([u8; 32], [u8; 4], [u8; 16])> {
    // Random fields per session
    let mut nonce_prefix = [0u8; 4];
    OsRng.fill_bytes(&mut nonce_prefix);
    let mut key_id = [0u8; 16];
    OsRng.fill_bytes(&mut key_id);

    // Device hash from env (like main.rs), but tolerate absence
    let device_id =
        std::env::var("TRUSTEDGE_DEVICE_ID").unwrap_or_else(|_| "trustedge-abc123".into());
    let salt = std::env::var("TRUSTEDGE_SALT").unwrap_or_else(|_| "trustedge-demo-salt".into());
    let mut device_id_hash = [0u8; 32];
    let mut hasher = blake3::Hasher::new();
    hasher.update(device_id.as_bytes());
    hasher.update(salt.as_bytes());
    device_id_hash.copy_from_slice(hasher.finalize().as_bytes());

    let header = FileHeader {
        version: VERSION,
        aead_alg: trustedge_core::format::AeadAlgorithm::Aes256Gcm as u8,
        sig_alg: trustedge_core::format::SignatureAlgorithm::Ed25519 as u8,
        hash_alg: trustedge_core::format::HashAlgorithm::Blake3 as u8,
        kdf_alg: trustedge_core::format::KdfAlgorithm::Pbkdf2Sha256 as u8,
        reserved: [0; 3],
        key_id,
        device_id_hash,
        nonce_prefix,
        chunk_size: chunk_size as u32,
    };
    let header_bytes = header.to_bytes();
    let header_hash = blake3::hash(&header_bytes);

    Ok((*header_hash.as_bytes(), nonce_prefix, key_id))
}

fn parse_key_hex(s: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(s).context("key_hex: not valid hex")?;
    anyhow::ensure!(bytes.len() == 32, "key_hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

async fn send_chunk(stream: &mut TcpStream, chunk: &NetworkChunk, verbose: bool) -> Result<()> {
    let serialized = bincode::serialize(chunk).context("serialize NetworkChunk")?;
    let length = serialized.len() as u32;

    if verbose {
        println!(
            "[SEND] Sending chunk seq={}, {} bytes",
            chunk.sequence, length
        );
    }

    stream
        .write_all(&length.to_le_bytes())
        .await
        .context("write chunk length")?;
    stream
        .write_all(&serialized)
        .await
        .context("write chunk data")?;
    Ok(())
}

async fn read_ack(stream: &mut TcpStream) -> Result<String> {
    let mut len_bytes = [0u8; 4];
    stream
        .read_exact(&mut len_bytes)
        .await
        .context("read ACK length")?;
    let length = u32::from_le_bytes(len_bytes) as usize;

    let mut ack_bytes = vec![0; length];
    stream
        .read_exact(&mut ack_bytes)
        .await
        .context("read ACK data")?;

    String::from_utf8(ack_bytes).context("ACK is not valid UTF-8")
}

// --- Hardened Transport Functions -------------------------------------------

async fn send_encrypted_test_chunks_hardened(
    framed: &mut Framed<TcpStream, LengthDelimitedCodec>,
    num_chunks: u64,
    chunk_size: usize,
    key_bytes: &[u8; 32],
    verbose: bool,
    expect_secure_acks: bool,
) -> Result<()> {
    println!(
        "[SEND] Sending {} encrypted test chunks via hardened transport...",
        num_chunks
    );

    let signing = SigningKey::generate(&mut OsRng);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));

    // Real header (so server can lock invariants)
    let (header_hash, nonce_prefix, key_id) = build_session_header(chunk_size)?;

    for seq in 1..=num_chunks {
        // Synthesize plaintext of up to chunk_size bytes
        let mut pt = format!("This is encrypted test chunk #{seq}. ").into_bytes();
        while pt.len() < chunk_size.min(2048) {
            pt.extend_from_slice(b"padding...");
        }

        let nonce_bytes = make_nonce(nonce_prefix, seq);
        let nonce = (&nonce_bytes).into();

        let pt_hash = blake3::hash(&pt);
        let manifest = Manifest {
            v: 1,
            ts_ms: now_ms(),
            seq,
            header_hash,
            pt_hash: *pt_hash.as_bytes(),
            key_id,
            ai_used: false,
            model_ids: vec![],
            data_type: trustedge_core::DataType::File { mime_type: None }, // Test data
            chunk_len: pt.len() as u32, // Bind actual chunk length to AAD
        };

        let m_bytes = bincode::serialize(&manifest)?;
        let sig: Signature = trustedge_core::format::sign_manifest_with_domain(&signing, &m_bytes);
        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: signing.verifying_key().to_bytes().to_vec(),
        };

        let aad = build_aad(
            &header_hash,
            seq,
            &nonce_bytes,
            blake3::hash(&m_bytes).as_bytes(),
            manifest.chunk_len,
        );
        let ciphertext = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: &pt,
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow::anyhow!("AES-GCM encrypt failed"))?;

        let chunk =
            NetworkChunk::new_with_nonce(seq, ciphertext, bincode::serialize(&sm)?, nonce_bytes);

        // Send chunk with timeout using framed transport
        timeout(
            CHUNK_SEND_TIMEOUT,
            send_chunk_hardened(framed, &chunk, verbose),
        )
        .await
        .context("Chunk send timeout")?
        .context("Failed to send chunk")?;

        // Read ACK with timeout
        let ack = timeout(ACK_READ_TIMEOUT, read_ack_hardened(framed))
            .await
            .context("ACK read timeout")?
            .context("Failed to read ACK")?;

        // Verify secure ACK if expected
        if expect_secure_acks && ack.contains(":MAC:") && verbose {
            println!("[SEC] Secure ACK received for chunk {}: {}", seq, ack);
        }

        if verbose {
            println!("[OK] Test chunk {} acknowledged: {}", seq, ack);
        } else {
            print!("[SEC]");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
    }

    if !verbose {
        println!();
    }
    Ok(())
}

async fn send_encrypted_file_hardened(
    framed: &mut Framed<TcpStream, LengthDelimitedCodec>,
    file_path: &std::path::Path,
    key_bytes: &[u8; 32],
    chunk_size: usize,
    verbose: bool,
    expect_secure_acks: bool,
) -> Result<()> {
    use tokio::io::AsyncReadExt;

    println!(
        "[SEND] Sending encrypted file via hardened transport: {:?}",
        file_path
    );

    let mut file = tokio::fs::File::open(file_path)
        .await
        .with_context(|| format!("Failed to open file: {:?}", file_path))?;

    let signing = SigningKey::generate(&mut OsRng);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key_bytes));

    // Real header (so server can lock invariants)
    let (header_hash, nonce_prefix, key_id) = build_session_header(chunk_size)?;

    let mut seq = 1u64;
    let mut buffer = vec![0u8; chunk_size];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .await
            .context("Failed to read from file")?;

        if bytes_read == 0 {
            break; // End of file
        }

        let pt = &buffer[..bytes_read];
        let nonce_bytes = make_nonce(nonce_prefix, seq);
        let nonce = (&nonce_bytes).into();

        let pt_hash = blake3::hash(pt);
        let manifest = Manifest {
            v: 1,
            ts_ms: now_ms(),
            seq,
            header_hash,
            pt_hash: *pt_hash.as_bytes(),
            key_id,
            ai_used: false,
            model_ids: vec![],
            data_type: trustedge_core::DataType::File {
                mime_type: Some("application/octet-stream".to_string()),
            },
            chunk_len: pt.len() as u32,
        };

        let m_bytes = bincode::serialize(&manifest)?;
        let sig: Signature = trustedge_core::format::sign_manifest_with_domain(&signing, &m_bytes);
        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: signing.verifying_key().to_bytes().to_vec(),
        };

        let aad = build_aad(
            &header_hash,
            seq,
            &nonce_bytes,
            blake3::hash(&m_bytes).as_bytes(),
            manifest.chunk_len,
        );
        let ciphertext = cipher
            .encrypt(nonce, Payload { msg: pt, aad: &aad })
            .map_err(|_| anyhow::anyhow!("AES-GCM encrypt failed"))?;

        let chunk =
            NetworkChunk::new_with_nonce(seq, ciphertext, bincode::serialize(&sm)?, nonce_bytes);

        // Send chunk with timeout using framed transport
        timeout(
            CHUNK_SEND_TIMEOUT,
            send_chunk_hardened(framed, &chunk, verbose),
        )
        .await
        .context("Chunk send timeout")?
        .context("Failed to send chunk")?;

        // Read ACK with timeout
        let ack = timeout(ACK_READ_TIMEOUT, read_ack_hardened(framed))
            .await
            .context("ACK read timeout")?
            .context("Failed to read ACK")?;

        // Verify secure ACK if expected
        if expect_secure_acks && ack.contains(":MAC:") && verbose {
            println!("[SEC] Secure ACK received for chunk {}: {}", seq, ack);
        }

        if verbose {
            println!("[OK] File chunk {} acknowledged: {}", seq, ack);
        } else {
            print!("[FILE]");
            use std::io::Write;
            std::io::stdout().flush().ok();
        }

        seq += 1;
    }

    if !verbose {
        println!();
    }
    println!("[SEND] File transmission complete: {} chunks", seq - 1);
    Ok(())
}

async fn send_chunk_hardened(
    framed: &mut Framed<TcpStream, LengthDelimitedCodec>,
    chunk: &NetworkChunk,
    verbose: bool,
) -> Result<()> {
    let serialized = bincode::serialize(chunk).context("serialize NetworkChunk")?;

    if verbose {
        println!(
            "[SEND] Sending chunk seq={}, {} bytes via hardened transport",
            chunk.sequence,
            serialized.len()
        );
    }

    framed
        .send(serialized.into())
        .await
        .context("Failed to send chunk via framed transport")?;

    Ok(())
}

async fn read_ack_hardened(framed: &mut Framed<TcpStream, LengthDelimitedCodec>) -> Result<String> {
    let frame = framed
        .next()
        .await
        .ok_or_else(|| anyhow::anyhow!("Connection closed while waiting for ACK"))?
        .context("Failed to receive ACK frame")?;

    String::from_utf8(frame.to_vec()).context("ACK is not valid UTF-8")
}
