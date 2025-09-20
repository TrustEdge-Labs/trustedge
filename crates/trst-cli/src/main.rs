use std::convert::TryInto;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{Context, Result};
use base64::Engine;
use chrono::{DateTime, SecondsFormat, Utc};
use clap::{Args, Parser, Subcommand};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use trustedge_trst_core::{
    verify_archive, wrap_file, ArchiveError, DeviceInfo, ManifestCapture, WrapConfig,
};

const SECRET_PREFIX: &str = "ed25519-secret";
const PUBLIC_PREFIX: &str = "ed25519";

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
        help = "Input raw stream placeholder"
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
        long,
        value_name = "PATH",
        help = "Path to existing device signing key"
    )]
    device_key: Option<PathBuf>,
    #[arg(long, help = "Optional device identifier")]
    device_id: Option<String>,
    #[arg(long, default_value = "TrustEdgeRefCam")]
    device_model: String,
    #[arg(long, default_value = "1.0.0")]
    device_fw: String,
    #[arg(long, default_value = "1920x1080")]
    resolution: String,
    #[arg(long, default_value = "raw")]
    codec: String,
    #[arg(long, help = "Override capture start timestamp (RFC3339)")]
    started_at: Option<String>,
    #[arg(long, default_value = "UTC")]
    tz: String,
    #[arg(long, help = "Previous archive hash to thread continuity")]
    prev_archive_hash: Option<String>,
}

#[derive(Args, Debug)]
struct VerifyCmd {
    #[arg(value_name = "ARCHIVE", help = "Path to .trst archive directory")]
    archive: PathBuf,
    #[arg(
        long = "device-pub",
        value_name = "KEY",
        help = "Device public key (ed25519:<base64>|hex)"
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
    let (signing_key, public_key, secret_path, public_path, generated) =
        load_or_generate_key(args.device_key.as_deref())?;

    let started_at = match &args.started_at {
        Some(ts) => ts.clone(),
        None => current_timestamp()?,
    };

    let claims = serde_json::json!({
        "location": {
            "lat": 0.0,
            "lon": 0.0,
            "source": "unknown"
        }
    });

    let config = WrapConfig {
        profile: args.profile,
        device: DeviceInfo {
            id: args
                .device_id
                .unwrap_or_else(|| format!("te:cam:{}", &public_key[..12.min(public_key.len())])),
            fw: args.device_fw,
            model: args.device_model,
            public_key: public_key.clone(),
        },
        capture: ManifestCapture {
            started_at,
            tz: args.tz,
            fps: args.fps,
            resolution: args.resolution,
            codec: args.codec,
        },
        chunk_bytes: args.chunk_size,
        chunk_seconds: args.chunk_seconds,
        claims,
        prev_archive_hash: args.prev_archive_hash,
    };

    let result = wrap_file(&args.input, &args.output, &signing_key, config)?;

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
    match verify_archive(&args.archive, &args.device_pub) {
        Ok(report) => {
            println!("Signature: PASS");
            println!("Continuity: PASS");
            println!(
                "Segments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
                report.segment_count,
                report.duration_seconds,
                if report.segment_count > 0 {
                    report.duration_seconds / report.segment_count as f64
                } else {
                    0.0
                }
            );
            Ok(())
        }
        Err(err) => {
            match &err {
                ArchiveError::Signature(_) => {
                    println!("Signature: FAIL");
                    println!("Continuity: SKIP");
                }
                ArchiveError::Continuity(_) => {
                    println!("Signature: PASS");
                    println!("Continuity: FAIL");
                }
                _ => {}
            }
            Err(err.into())
        }
    }
}

fn load_or_generate_key(
    path: Option<&Path>,
) -> Result<(SigningKey, String, PathBuf, PathBuf, bool)> {
    match path {
        Some(existing) => {
            let key_bytes = fs::read(existing)
                .with_context(|| format!("failed to read device key '{}'", existing.display()))?;
            let contents = String::from_utf8_lossy(&key_bytes).trim().to_string();
            let decoded = decode_prefixed(SECRET_PREFIX, &contents)?;
            let array: [u8; 32] = decoded
                .as_slice()
                .try_into()
                .map_err(|_| anyhow::anyhow!("invalid signing key length"))?;
            let signing_key = SigningKey::from_bytes(&array);
            let verifying_key: VerifyingKey = signing_key.verifying_key();
            let public_path = existing.with_extension("pub");
            let public_key_string = encode_prefixed(PUBLIC_PREFIX, verifying_key.as_bytes());
            Ok((
                signing_key,
                public_key_string,
                existing.to_path_buf(),
                public_path,
                false,
            ))
        }
        None => {
            let mut rng = OsRng;
            let signing_key = SigningKey::generate(&mut rng);
            let verifying_key: VerifyingKey = signing_key.verifying_key();
            let secret_path = PathBuf::from("device.key");
            let public_path = PathBuf::from("device.pub");
            let secret_string = encode_prefixed(SECRET_PREFIX, &signing_key.to_bytes());
            let public_string = encode_prefixed(PUBLIC_PREFIX, verifying_key.as_bytes());
            fs::write(&secret_path, format!("{secret_string}\n"))?;
            fs::write(&public_path, format!("{public_string}\n"))?;
            Ok((signing_key, public_string, secret_path, public_path, true))
        }
    }
}

fn current_timestamp() -> Result<String> {
    let now: DateTime<Utc> = Utc::now();
    Ok(now.to_rfc3339_opts(SecondsFormat::Secs, true))
}

fn encode_prefixed(prefix: &str, bytes: &[u8]) -> String {
    format!(
        "{}:{}",
        prefix,
        base64::engine::general_purpose::STANDARD.encode(bytes)
    )
}

fn decode_prefixed(expected: &str, value: &str) -> Result<Vec<u8>> {
    let (prefix, rest) = value
        .split_once(':')
        .ok_or_else(|| anyhow::anyhow!("invalid key encoding"))?;
    if prefix != expected {
        anyhow::bail!("expected prefix '{expected}'");
    }
    base64::engine::general_purpose::STANDARD
        .decode(rest.trim())
        .map_err(|err| anyhow::anyhow!("invalid base64: {err}"))
}
