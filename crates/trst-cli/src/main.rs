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
use chacha20poly1305::Key;
use chrono::{DateTime, SecondsFormat, Utc};
use clap::{Args, Parser, Subcommand};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use serde::Serialize;
use std::time::Instant;
use trustedge_core::{
    chain_next, encrypt_segment, generate_aad, genesis, read_archive, segment_hash, sign_manifest,
    validate_archive, verify_manifest, write_archive, CamVideoManifest, CaptureInfo, ChunkInfo,
    DeviceInfo, DeviceKeypair, SegmentInfo,
};

#[derive(Debug)]
struct WrapResult {
    output_dir: PathBuf,
    signature: String,
    chunk_count: usize,
}

#[derive(Serialize, Default)]
struct VerifyReport {
    signature: String,  // "pass" | "fail" | "unknown"
    continuity: String, // "pass" | "fail" | "skip" | "unknown"
    segments: u32,
    duration_s: f32,
    profile: String,
    device_id: String,
    verify_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>, // Error description for failures
    #[serde(skip_serializing_if = "Option::is_none")]
    first_gap_index: Option<u32>, // Index of first continuity gap
    #[serde(skip_serializing_if = "Option::is_none")]
    out_of_order: Option<bool>, // Whether segments are out of order
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
    #[arg(long = "in", value_name = "PATH", help = "Input file to wrap")]
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
    #[arg(
        long = "seed",
        value_name = "U64",
        help = "Seed RNG for deterministic output (for testing/CI, not cryptographically secure)"
    )]
    seed: Option<u64>,
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
    #[arg(long, help = "Output results as JSON")]
    json: bool,
    #[arg(
        long = "emit-receipt",
        value_name = "PATH",
        help = "Write JSON verification receipt to file"
    )]
    emit_receipt: Option<PathBuf>,
}

fn generate_seeded_nonce24(rng: &mut dyn RngCore) -> [u8; 24] {
    let mut nonce = [0u8; 24];
    rng.fill_bytes(&mut nonce);
    nonce
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
    let archive_name = args
        .output
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid output path"))?
        .to_string_lossy();

    if !archive_name.ends_with(".trst") {
        anyhow::bail!("Output directory must end with .trst");
    }

    fs::create_dir_all(&args.output)?;
    fs::create_dir_all(args.output.join("chunks"))?;
    fs::create_dir_all(args.output.join("signatures"))?;

    // Initialize RNG - seeded if provided, otherwise use default
    let mut rng: Box<dyn RngCore> = match args.seed {
        Some(seed) => Box::new(ChaCha20Rng::seed_from_u64(seed)),
        None => Box::new(rand::thread_rng()),
    };

    // Process chunks
    let chunks = input_data.chunks(args.chunk_size).collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut chain_state = genesis();
    let mut encrypted_chunks = Vec::new();

    // Generate a symmetric key for encryption (simplified for P0)
    let encryption_key = Key::from_slice(b"0123456789abcdef0123456789abcdef"); // 32 bytes for demo

    // Create timestamp for all operations - deterministic if seeded
    let started_at = if args.seed.is_some() {
        // Use deterministic timestamp for seeded runs
        "2025-01-01T00:00:00Z".to_string()
    } else {
        current_timestamp()?
    };
    let device_id = format!(
        "te:cam:{}",
        hex::encode(&device_keypair.public.as_bytes()[9..15])
    ); // Skip "ed25519:" prefix

    for (i, chunk_data) in chunks.iter().enumerate() {
        let chunk_id = i as u32;

        // Generate nonce - seeded if provided
        let nonce = generate_seeded_nonce24(&mut *rng);
        let aad = generate_aad("0.1.0", &args.profile, &device_id, &started_at);
        let encrypted_data = encrypt_segment(encryption_key, &nonce, chunk_data, &aad)?;
        encrypted_chunks.push(encrypted_data.clone());

        // Calculate hash and update chain (hash the encrypted data)
        let hash = segment_hash(&encrypted_data);
        let next_state = chain_next(&chain_state, &hash);

        // Create segment info
        let start_time = format!("{:.3}s", i as f64 * args.chunk_seconds);
        let chunk_filename = format!("{:05}.bin", chunk_id);

        let segment = SegmentInfo {
            chunk_file: chunk_filename,
            blake3_hash: hex::encode(hash),
            start_time,
            duration_seconds: args.chunk_seconds,
            continuity_hash: hex::encode(next_state),
        };

        segments.push(segment);
        chain_state = next_state;
    }

    // Create manifest
    let capture_end_time = if !chunks.is_empty() {
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
    write_archive(
        &args.output,
        &signed_manifest,
        encrypted_chunks,
        detached_sig,
    )?;

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
    let start_time = Instant::now();

    // Initialize report with defaults
    let mut report = VerifyReport::default();

    // Handle IO/Schema errors (exit 12)
    let (manifest, _chunks) = match read_archive(&args.archive) {
        Ok(data) => data,
        Err(e) => {
            report.error = Some(format!("Archive read failed: {}", e));
            report.verify_time_ms = start_time.elapsed().as_millis() as u64;

            // Map error types to human messages
            let first_line = match e {
                trustedge_core::archive::ArchiveError::MissingChunk(_) => "Missing chunk file",
                trustedge_core::archive::ArchiveError::InvalidChunkIndex { .. } => {
                    "Missing chunk file"
                }
                trustedge_core::archive::ArchiveError::Json(_) => "Invalid manifest format",
                trustedge_core::archive::ArchiveError::SignatureMismatch => {
                    "Signature verification failed"
                }
                trustedge_core::archive::ArchiveError::Io(_) => "Archive read error",
                trustedge_core::archive::ArchiveError::SchemaMismatch(_) => "Schema error",
                trustedge_core::archive::ArchiveError::Manifest(_) => "Manifest error",
                trustedge_core::archive::ArchiveError::Chain(_) => "Continuity chain error",
                trustedge_core::archive::ArchiveError::ValidationFailed(_) => "Validation error",
            };

            output_error(&args, &report, first_line)?;
            process::exit(12);
        }
    };

    // Parse device public key (invalid args would be caught by clap)
    let device_pub_key = if args.device_pub.starts_with("ed25519:") {
        args.device_pub.clone()
    } else {
        format!("ed25519:{}", args.device_pub)
    };

    // Populate report with manifest data
    report.profile = manifest.profile.clone();
    report.device_id = manifest.device.id.clone();
    report.segments = manifest.segments.len() as u32;
    report.duration_s = manifest
        .segments
        .iter()
        .map(|s| s.duration_seconds as f32)
        .sum();

    // Check for signature presence (schema error)
    let signature = match manifest.signature.as_ref() {
        Some(sig) => sig,
        None => {
            report.signature = "fail".to_string();
            report.continuity = "skip".to_string();
            report.error = Some("Manifest missing signature".to_string());
            report.verify_time_ms = start_time.elapsed().as_millis() as u64;
            output_error(&args, &report, "Manifest missing signature")?;
            process::exit(12);
        }
    };

    // Get canonical bytes (internal error if this fails)
    let canonical_bytes = match manifest.to_canonical_bytes() {
        Ok(bytes) => bytes,
        Err(e) => {
            report.signature = "fail".to_string();
            report.continuity = "skip".to_string();
            report.error = Some(format!("Canonical serialization failed: {}", e));
            report.verify_time_ms = start_time.elapsed().as_millis() as u64;
            output_error(&args, &report, "Internal canonicalization error")?;
            process::exit(14);
        }
    };

    // Verify signature (exit 10 on failure)
    match verify_manifest(&device_pub_key, &canonical_bytes, signature) {
        Ok(true) => {
            report.signature = "pass".to_string();

            // Validate archive structure and continuity (exit 11 on failure)
            match validate_archive(&args.archive) {
                Ok(()) => {
                    report.continuity = "pass".to_string();
                }
                Err(e) => {
                    report.continuity = "fail".to_string();
                    let error_msg = format!("{}", e);
                    report.error = Some(error_msg.clone());

                    // Extract structured information from chain errors
                    if let trustedge_core::archive::ArchiveError::Chain(chain_err) = &e {
                        match chain_err {
                            trustedge_core::chain::ChainError::Gap(index) => {
                                report.first_gap_index = Some(*index as u32);
                            }
                            trustedge_core::chain::ChainError::OutOfOrder { .. } => {
                                report.out_of_order = Some(true);
                            }
                            _ => {} // Other chain errors don't have specific structured data
                        }
                    }

                    report.verify_time_ms = start_time.elapsed().as_millis() as u64;
                    output_continuity_error(&args, &report)?;
                    process::exit(11);
                }
            }
        }
        Ok(false) => {
            report.signature = "fail".to_string();
            report.continuity = "skip".to_string();
            report.error = Some("Signature verification failed".to_string());
            report.verify_time_ms = start_time.elapsed().as_millis() as u64;
            output_error(&args, &report, "Signature verification failed")?;
            process::exit(10);
        }
        Err(e) => {
            report.signature = "fail".to_string();
            report.continuity = "skip".to_string();
            report.error = Some(format!("Signature verification error: {}", e));
            report.verify_time_ms = start_time.elapsed().as_millis() as u64;
            output_error(&args, &report, "Signature verification failed")?;
            process::exit(10);
        }
    }

    // Success case
    report.verify_time_ms = start_time.elapsed().as_millis() as u64;
    output_success(&args, &report)?;
    Ok(())
}

fn output_success(args: &VerifyCmd, report: &VerifyReport) -> Result<()> {
    if args.json {
        let json_output = serde_json::to_string(report)?;
        println!("{}", json_output);
    } else {
        println!("Signature: PASS");
        println!("Continuity: PASS");
        println!(
            "Segments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
            report.segments,
            report.duration_s,
            if report.segments > 0 {
                report.duration_s / report.segments as f32
            } else {
                0.0
            }
        );
    }

    // Emit receipt if requested
    if let Some(receipt_path) = &args.emit_receipt {
        let json_output = serde_json::to_string_pretty(report)?;
        fs::write(receipt_path, json_output)?;
    }

    Ok(())
}

fn output_error(args: &VerifyCmd, report: &VerifyReport, first_line: &str) -> Result<()> {
    if args.json {
        let json_output = serde_json::to_string(report)?;
        println!("{}", json_output);
    } else {
        eprintln!("{}", first_line);
    }

    // Emit receipt if requested
    if let Some(receipt_path) = &args.emit_receipt {
        let json_output = serde_json::to_string_pretty(report)?;
        fs::write(receipt_path, json_output)?;
    }

    Ok(())
}

fn output_continuity_error(args: &VerifyCmd, report: &VerifyReport) -> Result<()> {
    if args.json {
        let json_output = serde_json::to_string(report)?;
        println!("{}", json_output);
    } else {
        // Check if error message contains hash mismatch for legacy compatibility
        if let Some(error) = &report.error {
            if error.contains("hash mismatch") {
                eprintln!("hash mismatch");
                return Ok(());
            }
        }

        // Extract concise first line for continuity errors
        if let Some(gap_idx) = report.first_gap_index {
            eprintln!("Continuity: FAIL (gap at index {})", gap_idx);
        } else if report.out_of_order == Some(true) {
            eprintln!("Continuity: FAIL (segments out of order)");
        } else {
            eprintln!("Continuity: FAIL");
        }
    }

    // Emit receipt if requested
    if let Some(receipt_path) = &args.emit_receipt {
        let json_output = serde_json::to_string_pretty(report)?;
        fs::write(receipt_path, json_output)?;
    }

    Ok(())
}

// Removed: extract_gap_index() function eliminated string parsing
// Gap index information should come from structured error types, not string parsing

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
            Ok((device_keypair, existing.to_path_buf(), public_path, false))
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
