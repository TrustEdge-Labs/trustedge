//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use clap::{Args, Parser, Subcommand};
use trustedge_core::{
    write_archive, read_archive, validate_archive, CamVideoManifest, CaptureInfo, ChunkInfo,
    DeviceInfo, SegmentInfo, DeviceKeypair, encrypt_segment, chain_next, genesis, segment_hash,
    sign_manifest, verify_manifest, generate_nonce24, generate_aad
};
use chacha20poly1305::Key;

#[derive(Debug)]
struct WrapResult {
    output_dir: PathBuf,
    signature: String,
    chunk_count: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "TrustEdge .trst archival tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[allow(clippy::large_enum_variant)]
enum Commands {
    Wrap(WrapCmd),
    Verify(VerifyCmd),
}

#[derive(Args, Debug)]
struct WrapCmd {
    #[arg(
        long = "in",
        value_name = "PATH",
        help = "Input file to wrap"
    )]
    input: PathBuf,
    #[arg(long = "out", value_name = "PATH", help = "Output .trst directory")]
    output: PathBuf,
    #[arg(long, default_value = "cam.video")]
    profile: String,
    #[arg(long, default_value_t = 1_048_576)]
    chunk_size: usize,
    #[arg(long, default_value_t = 2.0)]
    chunk_seconds: f64,
    #[arg(long, default_value_t = 30)]
    fps: u32,
    #[arg(
        long = "device-key",
        value_name = "PATH",
        help = "Path to device signing key file"
    )]
    device_key: Option<PathBuf>,
    #[arg(
        long = "device-pub",
        value_name = "PATH",
        help = "Path to device public key file"
    )]
    device_pub: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct VerifyCmd {
    #[arg(value_name = "ARCHIVE", help = "Path to .trst archive directory")]
    archive: PathBuf,
    #[arg(
        long = "device-pub",
        value_name = "KEY",
        help = "Device public key (ed25519:<base64>)"
    )]
    device_pub: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Wrap(args) => handle_wrap(args),
        Commands::Verify(args) => handle_verify(args),
    }
}

fn handle_wrap(args: WrapCmd) -> Result<()> {
    let (device_keypair, secret_path, public_path, generated) =
        load_or_generate_keypair(args.device_key.as_deref())?;

    // Read input file
    let input_data = fs::read(&args.input)
        .with_context(|| format!("Failed to read input file: {}", args.input.display()))?;

    if input_data.is_empty() {
        anyhow::bail!("Input file is empty");
    }

    // Create output directory
    let archive_name = args.output.file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?
        .to_string_lossy();

    if !archive_name.ends_with(".trst") {
        anyhow::bail!("Output directory must end with .trst");
    }

    fs::create_dir_all(&args.output)?;
    fs::create_dir_all(args.output.join("chunks"))?;
    fs::create_dir_all(args.output.join("signatures"))?;

    // Process chunks
    let chunks = input_data.chunks(args.chunk_size).collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut chain_state = genesis();
    let mut encrypted_chunks = Vec::new();

    // Generate a symmetric key for encryption (simplified for P0)
    let encryption_key = Key::from_slice(b"0123456789abcdef0123456789abcdef"); // 32 bytes for demo

    // Create timestamp for all operations
    let started_at = current_timestamp()?;
    let device_id = format!("te:cam:{}", hex::encode(&device_keypair.public.as_bytes()[9..15])); // Skip "ed25519:" prefix

    for (i, chunk_data) in chunks.iter().enumerate() {
        let chunk_id = i as u32;

        // Generate nonce and encrypt
        let nonce = generate_nonce24();
        let aad = generate_aad("0.1.0", &args.profile, &device_id, &started_at);
        let encrypted_data = encrypt_segment(&encryption_key, &nonce, chunk_data, &aad)?;
        encrypted_chunks.push(encrypted_data.clone());

        // Calculate hash and update chain (hash the encrypted data)
        let hash = segment_hash(&encrypted_data);
        let next_state = chain_next(&chain_state, &hash);

        // Create segment info
        let start_time = format!("{:.3}s", i as f64 * args.chunk_seconds);
        let chunk_filename = format!("{:05}.bin", chunk_id);

        let segment = SegmentInfo {
            chunk_file: chunk_filename,
            blake3_hash: hex::encode(&hash),
            start_time,
            duration_seconds: args.chunk_seconds,
            continuity_hash: hex::encode(&next_state),
        };

        segments.push(segment);
        chain_state = next_state;
    }

    // Create manifest
    let capture_end_time = if chunks.len() > 0 {
        let last_chunk_start = (chunks.len() - 1) as f64 * args.chunk_seconds;
        let end_timestamp = chrono::DateTime::parse_from_rfc3339(&started_at)?
            + chrono::Duration::milliseconds((last_chunk_start * 1000.0) as i64)
            + chrono::Duration::milliseconds((args.chunk_seconds * 1000.0) as i64);
        end_timestamp.to_rfc3339_opts(SecondsFormat::Secs, true)
    } else {
        started_at.clone()
    };

    let manifest = CamVideoManifest {
        trst_version: "0.1.0".to_string(),
        profile: args.profile,
        device: DeviceInfo {
            id: device_id,
            model: "TrustEdgeRefCam".to_string(),
            firmware_version: "1.0.0".to_string(),
            public_key: device_keypair.public.clone(),
        },
        capture: CaptureInfo {
            started_at,
            ended_at: capture_end_time,
            timezone: "UTC".to_string(),
            fps: args.fps as f64,
            resolution: "1920x1080".to_string(),
            codec: "raw".to_string(),
        },
        chunk: ChunkInfo {
            size_bytes: args.chunk_size as u64,
            duration_seconds: args.chunk_seconds,
        },
        segments,
        claims: vec!["location:unknown".to_string()], // Simple claims
        prev_archive_hash: None,
        signature: None,
    };

    // Sign manifest
    let canonical_bytes = manifest.to_canonical_bytes()?;
    let signature = sign_manifest(&device_keypair, &canonical_bytes)?;
    let signed_manifest = CamVideoManifest {
        signature: Some(signature.clone()),
        ..manifest
    };

    // Write archive
    let detached_sig = signature.as_bytes();
    write_archive(&args.output, &signed_manifest, encrypted_chunks, detached_sig)?;

    let result = WrapResult {
        output_dir: args.output,
        signature,
        chunk_count: chunks.len(),
    };

    println!("Archive: {}", result.output_dir.display());
    println!("Signature: {}", result.signature);
    println!("Segments: {}", result.chunk_count);
    if generated {
        println!("Generated device key: {}", secret_path.display());
        println!("Generated device pub: {}", public_path.display());
    }

    Ok(())
}

fn handle_verify(args: VerifyCmd) -> Result<()> {
    // Read and validate archive
    let (manifest, _chunks) = read_archive(&args.archive)?;

    // Parse device public key
    let device_pub_key = if args.device_pub.starts_with("ed25519:") {
        args.device_pub
    } else {
        format!("ed25519:{}", args.device_pub)
    };

    // Get signature and canonical bytes
    let signature = manifest.signature.as_ref()
        .ok_or_else(|| anyhow::anyhow!("Manifest has no signature"))?;

    let canonical_bytes = manifest.to_canonical_bytes()?;

    // Verify signature
    match verify_manifest(&device_pub_key, &canonical_bytes, signature) {
        Ok(true) => {
            println!("Signature: PASS");

            // Validate archive structure and continuity
            match validate_archive(&args.archive) {
                Ok(()) => {
                    println!("Continuity: PASS");

                    let segment_count = manifest.segments.len();
                    let duration_seconds = if segment_count > 0 {
                        manifest.segments.iter().map(|s| s.duration_seconds).sum()
                    } else {
                        0.0
                    };

                    println!(
                        "Segments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
                        segment_count,
                        duration_seconds,
                        if segment_count > 0 {
                            duration_seconds / segment_count as f64
                        } else {
                            0.0
                        }
                    );
                    Ok(())
                }
                Err(err) => {
                    println!("Continuity: FAIL");
                    anyhow::bail!("Archive validation failed: {}", err);
                }
            }
        }
        Ok(false) => {
            println!("Signature: FAIL");
            println!("Continuity: SKIP");
            anyhow::bail!("Signature verification failed");
        }
        Err(err) => {
            println!("Signature: FAIL");
            println!("Continuity: SKIP");
            anyhow::bail!("Signature verification error: {}", err);
        }
    }
}

fn load_or_generate_keypair(
    path: Option<&Path>,
) -> Result<(DeviceKeypair, PathBuf, PathBuf, bool)> {
    match path {
        Some(existing) => {
            let key_bytes = fs::read(existing)
                .with_context(|| format!("failed to read device key '{}'", existing.display()))?;
            let contents = String::from_utf8_lossy(&key_bytes).trim().to_string();
            let device_keypair = DeviceKeypair::import_secret(&contents)?;
            let public_path = existing.with_extension("pub");
            Ok((
                device_keypair,
                existing.to_path_buf(),
                public_path,
                false,
            ))
        }
        None => {
            let device_keypair = DeviceKeypair::generate()?;
            let secret_path = PathBuf::from("device.key");
            let public_path = PathBuf::from("device.pub");
            let secret_string = device_keypair.export_secret();
            let public_string = device_keypair.public.clone();
            fs::write(&secret_path, format!("{secret_string}\n"))?;
            fs::write(&public_path, format!("{public_string}\n"))?;
            Ok((device_keypair, secret_path, public_path, true))
        }
    }
}

fn current_timestamp() -> Result<String> {
    let now: DateTime<Utc> = Utc::now();
    Ok(now.to_rfc3339_opts(SecondsFormat::Secs, true))
}