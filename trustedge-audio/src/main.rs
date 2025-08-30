#![forbid(unsafe_code)]

//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
/// Project: trustedge — Privacy and trust at the edge.
///
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Key, Nonce,
};

use anyhow::{anyhow, Context, Result};
use bincode::{deserialize_from, serialize_into};
use clap::Parser;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::RngCore;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
#[cfg(feature = "audio")]
use trustedge_audio::AudioCapture;
#[cfg(feature = "audio")]
use trustedge_audio::AudioConfig;
use trustedge_audio::{BackendRegistry, KeyBackend, KeyContext, KeyringBackend};
use zeroize::Zeroize;

use trustedge_audio::{
    // helpers
    build_aad,
    write_stream_header,
    // Types
    AudioFormat,
    DataType,
    FileHeader,
    Manifest,
    Record,
    SignedManifest,
    StreamHeader,
    ALG_AES_256_GCM,
    HEADER_LEN,
    MAGIC,
    // Constants
    NONCE_LEN,
    VERSION,
};

/// Input source for the trustedge application
#[derive(Debug)]
enum InputSource {
    File(PathBuf),
    LiveAudio,
}

/// Trait for unified input reading
trait InputReader {
    fn read_chunk(&mut self, buf: &mut [u8]) -> Result<usize>;
}

/// File-based input reader
struct FileInputReader {
    reader: BufReader<File>,
}

impl FileInputReader {
    fn new(reader: BufReader<File>) -> Self {
        Self { reader }
    }
}

impl InputReader for FileInputReader {
    fn read_chunk(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reader.read(buf).context("read chunk")
    }
}

/// Audio-based input reader
#[cfg(feature = "audio")]
struct AudioInputReader {
    capture: AudioCapture,
    started: bool,
}

#[cfg(feature = "audio")]
impl AudioInputReader {
    fn new(mut capture: AudioCapture) -> Result<Self> {
        capture.initialize()?;
        Ok(Self {
            capture,
            started: false,
        })
    }
}

#[cfg(feature = "audio")]
impl InputReader for AudioInputReader {
    fn read_chunk(&mut self, buf: &mut [u8]) -> Result<usize> {
        if !self.started {
            self.capture.start()?;
            self.started = true;
            println!("🎙️  Live audio capture started");
        }

        // Wait for audio chunk - keep trying until we get data
        loop {
            match self.capture.try_next_chunk()? {
                Some(audio_chunk) => {
                    println!("📦 Audio chunk: {} samples", audio_chunk.data.len());
                    let audio_bytes = audio_chunk.to_bytes();
                    let bytes_to_copy = std::cmp::min(audio_bytes.len(), buf.len());
                    buf[..bytes_to_copy].copy_from_slice(&audio_bytes[..bytes_to_copy]);
                    return Ok(bytes_to_copy);
                }
                None => {
                    // Brief pause and try again - don't timeout here
                    std::thread::sleep(Duration::from_millis(10));
                    // Let the main loop handle timeouts via max_duration
                }
            }
        }
    }
}

/// CLI Arguments
#[derive(Parser, Debug)]
#[command(name = "trustedge-audio", version, about)]
struct Args {
    /// Input file (opaque bytes)
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// Output file for round-tripped plaintext (encrypt mode) or decrypt target (decrypt mode)
    #[arg(short, long)]
    out: Option<PathBuf>,

    /// Chunk size in bytes
    #[arg(long, default_value_t = 4096)]
    chunk: usize,

    /// Optional: write envelope (header + records) to this .trst file
    #[arg(long)]
    envelope: Option<PathBuf>,

    /// Skip writing plaintext during encrypt (still verifies+envelopes)
    #[arg(long, default_value_t = false)]
    no_plaintext: bool,

    /// Decrypt mode: read .trst from --input and write plaintext to --out
    #[arg(long, default_value_t = false)]
    decrypt: bool,

    /// 64 hex chars (32 bytes) AES-256 key
    #[arg(long)]
    key_hex: Option<String>,

    /// Where to store generated key (encrypt mode) as hex
    #[arg(long)]
    key_out: Option<PathBuf>,

    /// Store passphrase in OS keyring (one-time setup)
    #[arg(long)]
    set_passphrase: Option<String>,

    /// Salt for key derivation (32 hex chars = 16 bytes)
    #[arg(long)]
    salt_hex: Option<String>,

    /// Use key derived from keyring passphrase + salt instead of --key-hex
    #[arg(long)]
    use_keyring: bool,

    /// Key management backend to use (keyring, tpm, hsm, matter)
    #[arg(long, default_value = "keyring")]
    backend: String,

    /// List available key management backends
    #[arg(long)]
    list_backends: bool,

    /// Backend-specific configuration (format: key=value)
    #[arg(long)]
    backend_config: Vec<String>,

    // === Live Audio Capture Options ===
    /// Enable live audio capture from microphone
    #[arg(long)]
    live_capture: bool,

    /// Audio device name (use --list-audio-devices to see options)
    #[arg(long)]
    audio_device: Option<String>,

    /// List available audio input devices
    #[arg(long)]
    list_audio_devices: bool,

    /// Audio sample rate in Hz
    #[arg(long, default_value_t = 44100)]
    sample_rate: u32,

    /// Number of audio channels (1=mono, 2=stereo)
    #[arg(long, default_value_t = 1)]
    channels: u16,

    /// Duration of each audio chunk in milliseconds
    #[arg(long, default_value_t = 1000)]
    chunk_duration_ms: u64,

    /// Stream live chunks to server (requires --live-capture)
    #[arg(long)]
    stream_to_server: Option<String>,

    /// Maximum capture duration in seconds (0 = unlimited)
    #[arg(long, default_value_t = 0)]
    max_duration: u64,
}

/// Helpers
enum Mode {
    Encrypt,
    Decrypt,
}

/// List available key management backends
fn list_backends() -> Result<()> {
    let registry = BackendRegistry::new();
    let available = registry.list_available_backends();

    println!("Available key management backends:");
    for backend_name in available {
        // Create backend to get info
        if let Ok(backend) = registry.create_backend(backend_name) {
            let info = backend.backend_info();
            let status = if info.available { "✓" } else { "✗" };
            println!("  {} {} - {}", status, info.name, info.description);

            if !info.config_requirements.is_empty() {
                println!(
                    "    Required config: {}",
                    info.config_requirements.join(", ")
                );
            }
        }
    }

    println!("\nUsage examples:");
    println!("  --backend keyring --use-keyring --salt-hex <salt>");
    println!("  --backend tpm --backend-config device_path=/dev/tpm0");
    println!("  --backend hsm --backend-config pkcs11_lib=/usr/lib/libpkcs11.so");

    Ok(())
}

/// List available audio input devices
#[cfg(feature = "audio")]
fn list_audio_devices() -> Result<()> {
    let config = AudioConfig::default();
    let capture = AudioCapture::new(config).context("Failed to create audio capture")?;

    match capture.list_devices() {
        Ok(devices) => {
            if devices.is_empty() {
                println!("No audio input devices found.");
            } else {
                println!("Available audio input devices:");
                for (i, device) in devices.iter().enumerate() {
                    println!("  {}: {}", i + 1, device);
                }
            }
        }
        Err(e) => {
            println!("❌ Error listing audio devices: {}", e);
            println!("💡 This might happen if no audio system is available or permissions are insufficient.");
        }
    }

    Ok(())
}

/// List available audio input devices (stub when audio not available)
#[cfg(not(feature = "audio"))]
fn list_audio_devices() -> Result<()> {
    println!("❌ Audio support not available in this build");
    println!("💡 To enable audio support:");
    println!("   1. Install audio libraries: sudo apt install libasound2-dev pkg-config");
    println!("   2. Rebuild with: cargo build --features audio");
    println!("   3. Or use default build (audio enabled): cargo build");
    Ok(())
}

/// Create a backend from CLI arguments
fn create_backend_from_args(args: &Args) -> Result<Box<dyn KeyBackend>> {
    // For now, only keyring is supported
    match args.backend.as_str() {
        "keyring" => {
            let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
            Ok(Box::new(backend))
        }
        other => {
            anyhow::bail!(
                "Backend '{}' not yet implemented. Available: keyring\n\
                Future backends: tpm, hsm, matter\n\
                Use --list-backends to see all options",
                other
            );
        }
    }
}

/// Parse a hex string into a 32-byte array
fn parse_key_hex(s: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(s).context("key_hex: not valid hex")?;
    anyhow::ensure!(bytes.len() == 32, "key_hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

/// Select the AES key to use for encryption/decryption using the new backend system
fn select_aes_key_with_backend(args: &Args, mode: Mode) -> Result<[u8; 32]> {
    // Check for explicit key first (highest priority)
    if let Some(kh) = &args.key_hex {
        return parse_key_hex(kh);
    }

    // Use backend system for key derivation (if salt provided or use_keyring flag set)
    if args.use_keyring || args.salt_hex.is_some() {
        let backend = create_backend_from_args(args)?;

        let salt_hex = args
            .salt_hex
            .as_ref()
            .ok_or_else(|| anyhow!("--salt-hex required for backend key derivation"))?;
        let salt_bytes = hex::decode(salt_hex).context("salt_hex decode")?;
        anyhow::ensure!(
            salt_bytes.len() == 16,
            "salt must be 16 bytes (32 hex chars)"
        );

        let key_id = [0u8; 16]; // Default key ID for now
        let context = KeyContext::new(salt_bytes);
        return backend.derive_key(&key_id, &context);
    }

    // Fall back to random key generation for encrypt mode
    match mode {
        Mode::Decrypt => anyhow::bail!(
            "Decrypt mode requires key material. Use one of:\n\
            --key-hex <64-char-hex>   # Explicit key\n\
            --use-keyring --salt-hex <salt>  # Keyring backend\n\
            --backend <type> --salt-hex <salt>  # Specific backend"
        ),
        Mode::Encrypt => {
            let mut kb = [0u8; 32];
            OsRng.fill_bytes(&mut kb);
            if let Some(p) = &args.key_out {
                std::fs::write(p, hex::encode(kb)).context("write key_out")?;
            } else {
                eprintln!("NOTE (demo): AES-256 key (hex) = {}", hex::encode(kb));
            }
            Ok(kb)
        }
    }
}

/// Decrypt the envelope (header + records)
fn decrypt_envelope(args: &Args) -> Result<()> {
    // key
    let mut key_bytes = select_aes_key_with_backend(args, Mode::Decrypt)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));

    // io
    let input = args
        .input
        .as_ref()
        .ok_or_else(|| anyhow!("--input is required for --decrypt"))?;
    let out = args
        .out
        .as_ref()
        .ok_or_else(|| anyhow!("--out is required for --decrypt"))?;
    let mut r = BufReader::new(File::open(input).context("open envelope")?);
    let mut w = BufWriter::new(File::create(out).context("create output")?);

    // preamble
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic).context("read magic")?;
    anyhow::ensure!(&magic == MAGIC, "bad magic");
    let mut ver = [0u8; 1];
    r.read_exact(&mut ver).context("read version")?;
    anyhow::ensure!(ver[0] == VERSION, "unsupported version");

    // stream header
    let sh: StreamHeader = deserialize_from(&mut r).context("read stream header")?;
    anyhow::ensure!(sh.header.len() == HEADER_LEN, "bad stream header length");
    let stream_nonce_prefix: [u8; 4] = sh.header[50..54].try_into().unwrap();

    // turn Vec<u8> into the fixed array
    let header_arr: [u8; trustedge_audio::HEADER_LEN] = sh
        .header
        .as_slice()
        .try_into()
        .context("stream header length != 58")?;

    // parse the 58-byte header into a FileHeader
    let fh = trustedge_audio::FileHeader::from_bytes(&header_arr);

    // verify stored header hash matches recompute
    let hh = blake3::hash(&sh.header);
    anyhow::ensure!(hh.as_bytes() == &sh.header_hash, "header_hash mismatch");

    // records
    let mut total_out = 0usize;
    let mut expected_seq: u64 = 1;

    // record loop
    loop {
        let rec: Record = match deserialize_from(&mut r) {
            Ok(x) => x,
            Err(err) => {
                if let bincode::ErrorKind::Io(ref e) = *err {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                }
                return Err(err).context("read record");
            }
        };

        // envelope invariants
        anyhow::ensure!(
            rec.nonce[..4] == stream_nonce_prefix,
            "record nonce prefix != stream header nonce_prefix"
        );

        // ensure nonce counter == seq
        let seq_bytes = rec.seq.to_be_bytes();
        anyhow::ensure!(
            rec.nonce[4..] == seq_bytes,
            "record nonce counter != record seq"
        );

        anyhow::ensure!(
            rec.seq == expected_seq,
            "non-contiguous sequence: got {}, expected {}",
            rec.seq,
            expected_seq
        );
        expected_seq = expected_seq
            .checked_add(1)
            .ok_or_else(|| anyhow!("seq overflow"))?;

        // manifest signature
        let pubkey_arr: [u8; 32] = rec
            .sm
            .pubkey
            .as_slice()
            .try_into()
            .context("pubkey length != 32")?;
        let sig_arr: [u8; 64] = rec.sm.sig.as_slice().try_into().context("sig len != 64")?;
        VerifyingKey::from_bytes(&pubkey_arr)
            .context("bad pubkey")?
            .verify(&rec.sm.manifest, &Signature::from_bytes(&sig_arr))
            .context("manifest signature verify failed")?;

        // manifest contents - deserialize first so we can use it for verification
        let m: Manifest = bincode::deserialize(&rec.sm.manifest).context("manifest decode")?;

        // verify invariants
        anyhow::ensure!(
            rec.nonce[..4] == fh.nonce_prefix,
            "record nonce prefix != stream header nonce_prefix"
        );

        anyhow::ensure!(
            m.header_hash == sh.header_hash,
            "manifest.header_hash != stream header_hash"
        );

        anyhow::ensure!(m.key_id == fh.key_id, "manifest.key_id != header.key_id");

        // ensure manifest seq matches record seq
        anyhow::ensure!(m.seq == rec.seq, "manifest.seq != record.seq");

        // decrypt
        let mh = blake3::hash(&rec.sm.manifest);
        let aad = build_aad(&sh.header_hash, rec.seq, &rec.nonce, mh.as_bytes());
        let pt = cipher
            .decrypt(
                Nonce::from_slice(&rec.nonce),
                Payload {
                    msg: &rec.ct,
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

        // pt hash
        let pt_hash_rx = blake3::hash(&pt);
        anyhow::ensure!(pt_hash_rx.as_bytes() == &m.pt_hash, "pt hash mismatch");

        // write
        w.write_all(&pt).context("write plaintext")?;
        total_out += pt.len();
    }

    w.flush().context("flush plaintext")?;
    key_bytes.zeroize();

    eprintln!("Decrypt complete. Wrote {} bytes.", total_out);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle --list-backends option
    if args.list_backends {
        return list_backends();
    }

    // Handle --list-audio-devices option
    if args.list_audio_devices {
        return list_audio_devices();
    }

    // one-time keyring setup
    if let Some(passphrase) = &args.set_passphrase {
        let backend = KeyringBackend::new().context("Failed to create keyring backend")?;
        backend.store_passphrase(passphrase)?;
        println!("Passphrase stored in system keyring");
        return Ok(());
    }

    if args.decrypt {
        return decrypt_envelope(&args);
    }

    // encrypt mode
    anyhow::ensure!(args.chunk > 0, "chunk must be > 0");
    anyhow::ensure!(
        args.chunk as u64 <= u32::MAX as u64,
        "chunk too large for header"
    );

    // Determine input source: file or live audio
    let input_source = if args.live_capture {
        InputSource::LiveAudio
    } else {
        let input = args
            .input
            .as_ref()
            .ok_or_else(|| anyhow!("--input is required when not using --live-capture"))?;
        InputSource::File(input.clone())
    };

    // outputs
    let out = args
        .out
        .as_ref()
        .ok_or_else(|| anyhow!("--out is required"))?;
    let mut fout = BufWriter::new(File::create(out).context("create output")?);

    // keys
    let mut key_bytes = select_aes_key_with_backend(&args, Mode::Encrypt)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let signing = SigningKey::generate(&mut OsRng); // demo only
    let verify: VerifyingKey = signing.verifying_key();

    // header fields (demo placeholders as needed)
    let mut nonce_prefix = [0u8; 4];
    OsRng.fill_bytes(&mut nonce_prefix);
    let mut key_id = [0u8; 16];
    OsRng.fill_bytes(&mut key_id);

    // device hash (demo)
    let device_id =
        std::env::var("TRUSTEDGE_DEVICE_ID").unwrap_or_else(|_| "trustedge-abc123".into());
    let salt = std::env::var("TRUSTEDGE_SALT").unwrap_or_else(|_| "trustedge-demo-salt".into());
    let mut device_id_hash = [0u8; 32];
    let mut hasher = blake3::Hasher::new();
    hasher.update(device_id.as_bytes());
    hasher.update(salt.as_bytes());
    device_id_hash.copy_from_slice(hasher.finalize().as_bytes());

    let header = FileHeader {
        version: 1,
        alg: ALG_AES_256_GCM,
        key_id,
        device_id_hash,
        nonce_prefix,
        chunk_size: args.chunk as u32,
    };
    let header_bytes = header.to_bytes();
    let header_hash = blake3::hash(&header_bytes);

    // optional envelope writer
    let mut env_out = if let Some(path) = &args.envelope {
        Some(BufWriter::new(
            File::create(path).context("create envelope")?,
        ))
    } else {
        None
    };

    if let Some(w) = env_out.as_mut() {
        let sh = StreamHeader {
            v: VERSION,
            header: header_bytes.to_vec(),
            header_hash: *header_hash.as_bytes(),
        };
        write_stream_header(w, &sh)?;
    }

    // loop
    let mut buf = vec![0u8; args.chunk];
    let mut total_in = 0usize;
    let mut total_out = 0usize;
    let mut seq: u64 = 0;
    let mut nonce_bytes = [0u8; NONCE_LEN];

    // Initialize input source
    let mut input_reader: Box<dyn InputReader> = match &input_source {
        InputSource::File(path) => {
            let fin = BufReader::new(File::open(path).context("open input")?);
            Box::new(FileInputReader::new(fin))
        }
        InputSource::LiveAudio => {
            #[cfg(feature = "audio")]
            {
                let audio_config = AudioConfig {
                    device_name: args.audio_device.clone(),
                    sample_rate: args.sample_rate,
                    channels: args.channels,
                    chunk_duration_ms: args.chunk_duration_ms,
                    buffer_size: 8192,
                };
                let capture = AudioCapture::new(audio_config)?;
                Box::new(AudioInputReader::new(capture)?)
            }
            #[cfg(not(feature = "audio"))]
            {
                return Err(anyhow!(
                    "Audio capture not available - rebuild with --features audio"
                ));
            }
        }
    };

    // loop to process input chunks
    let start_time = std::time::Instant::now();
    let max_duration = if args.max_duration > 0 {
        Some(Duration::from_secs(args.max_duration))
    } else {
        None
    };

    loop {
        // Check time limit for live audio
        if let Some(max_dur) = max_duration {
            if start_time.elapsed() >= max_dur {
                println!("⏱️  Maximum duration reached, stopping capture");
                break;
            }
        }

        let n = input_reader.read_chunk(&mut buf)?;
        if n == 0 {
            // For live audio, continue if within time limit
            if matches!(input_source, InputSource::LiveAudio) {
                if max_duration.is_some() {
                    continue; // Keep trying until time limit
                } else {
                    break; // No time limit, exit on no data
                }
            } else {
                // File EOF
                break;
            }
        }

        seq = seq.checked_add(1).ok_or_else(|| anyhow!("seq overflow"))?;
        nonce_bytes[..4].copy_from_slice(&header.nonce_prefix);
        nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        let pt_hash = blake3::hash(&buf[..n]);
        let ts_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // Determine data type for manifest
        let data_type = match &input_source {
            InputSource::File(_) => DataType::File { mime_type: None },
            InputSource::LiveAudio => DataType::Audio {
                sample_rate: args.sample_rate,
                channels: args.channels,
                format: AudioFormat::F32Le, // Current implementation uses f32 samples
            },
        };

        let m = Manifest {
            v: 1,
            ts_ms,
            seq,
            header_hash: *header_hash.as_bytes(),
            pt_hash: *pt_hash.as_bytes(),
            key_id: header.key_id,
            ai_used: false,
            model_ids: vec![],
            data_type,
        };

        let m_bytes = bincode::serialize(&m).expect("manifest serialize");
        let sig: Signature = signing.sign(&m_bytes);
        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: verify.to_bytes().to_vec(),
        };

        let mhash = blake3::hash(&m_bytes);
        let aad = build_aad(header_hash.as_bytes(), seq, &nonce_bytes, mhash.as_bytes());

        let ct = cipher
            .encrypt(
                nonce,
                Payload {
                    msg: &buf[..n],
                    aad: &aad,
                },
            )
            .map_err(|_| anyhow!("AES-GCM encrypt failed"))?;

        // debug-only tamper check
        #[cfg(debug_assertions)]
        {
            if !ct.is_empty() {
                let mut ct_bad = ct.clone();
                ct_bad[0] ^= 0x01;
                debug_assert!(
                    cipher
                        .decrypt(
                            nonce,
                            Payload {
                                msg: &ct_bad,
                                aad: &aad
                            }
                        )
                        .is_err(),
                    "tamper test should fail"
                );
            }
        }

        // verify manifest + round-trip decrypt (sanity)
        let _m2: Manifest = bincode::deserialize(&sm.manifest).context("manifest decode")?;
        let pubkey_arr: [u8; 32] = sm
            .pubkey
            .as_slice()
            .try_into()
            .context("pubkey length != 32")?;
        let sig_arr: [u8; 64] = sm.sig.as_slice().try_into().context("sig len != 64")?;
        VerifyingKey::from_bytes(&pubkey_arr)
            .context("bad pubkey")?
            .verify(&sm.manifest, &Signature::from_bytes(&sig_arr))
            .context("manifest signature verify failed")?;

        let aad_rx = build_aad(header_hash.as_bytes(), seq, &nonce_bytes, mhash.as_bytes());
        let pt = cipher
            .decrypt(
                Nonce::from_slice(&nonce_bytes),
                Payload {
                    msg: &ct,
                    aad: &aad_rx,
                },
            )
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;
        let pt_hash_rx = blake3::hash(&pt);
        anyhow::ensure!(pt_hash_rx.as_bytes() == &m.pt_hash, "pt hash mismatch");

        if !args.no_plaintext {
            fout.write_all(&pt).context("write out")?;
        }

        if let Some(w) = env_out.as_mut() {
            let rec = Record {
                seq,
                nonce: nonce_bytes,
                sm,
                ct,
            };
            serialize_into(w, &rec).context("write envelope record")?;
        }

        total_in += n;
        total_out += pt.len();
    }

    key_bytes.zeroize();
    if !args.no_plaintext {
        fout.flush().context("flush plaintext")?;
    }
    if let Some(w) = env_out.as_mut() {
        w.flush().context("flush envelope")?;
    }

    // status and exit
    eprintln!(
        "Round-trip complete. Read {} bytes, wrote {} bytes.",
        total_in, total_out
    );
    Ok(())
}
