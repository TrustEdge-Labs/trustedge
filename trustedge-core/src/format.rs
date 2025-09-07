// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

/// trustedge_core/src/format.rs
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};

pub const NONCE_LEN: usize = 12;
pub const AAD_LEN: usize = 32 + 8 + NONCE_LEN + 32;
pub const HEADER_LEN: usize = 58;

pub const MAGIC: &[u8; 4] = b"TRST";
pub const VERSION: u8 = 1;
pub const ALG_AES_256_GCM: u8 = 1;

/// Domain separation string for manifest signatures
/// Prevents signature reuse across different contexts or protocols
pub const MANIFEST_DOMAIN_SEP: &[u8] = b"trustedge.manifest.v1";

/// FileHeader structure
#[derive(Clone, Copy, Debug)]
pub struct FileHeader {
    pub version: u8,              // 1
    pub alg: u8,                  // 1
    pub key_id: [u8; 16],         // 16
    pub device_id_hash: [u8; 32], // 32
    pub nonce_prefix: [u8; 4],    // 4
    pub chunk_size: u32,          // 4 (BE)
}

/// FileHeader serialization/deserialization
impl FileHeader {
    pub fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[0] = self.version;
        out[1] = self.alg;
        out[2..18].copy_from_slice(&self.key_id);
        out[18..50].copy_from_slice(&self.device_id_hash);
        out[50..54].copy_from_slice(&self.nonce_prefix);
        out[54..58].copy_from_slice(&self.chunk_size.to_be_bytes());
        out
    }

    /// Create a FileHeader from bytes
    pub fn from_bytes(bytes: &[u8; HEADER_LEN]) -> Self {
        let mut key_id = [0u8; 16];
        key_id.copy_from_slice(&bytes[2..18]);
        let mut device_id_hash = [0u8; 32];
        device_id_hash.copy_from_slice(&bytes[18..50]);
        let mut nonce_prefix = [0u8; 4];
        nonce_prefix.copy_from_slice(&bytes[50..54]);
        let chunk_size = u32::from_be_bytes([bytes[54], bytes[55], bytes[56], bytes[57]]);

        FileHeader {
            version: bytes[0],
            alg: bytes[1],
            key_id,
            device_id_hash,
            nonce_prefix,
            chunk_size,
        }
    }
}

/// Data type enumeration for different input sources
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DataType {
    /// Unknown or unspecified data type
    Unknown,
    /// Generic file data (original behavior)
    File { mime_type: Option<String> },
    /// Live audio capture
    Audio {
        sample_rate: u32,
        channels: u16,
        format: AudioFormat,
    },
    /// Video capture (future use)
    Video {
        width: u32,
        height: u32,
        fps: f32,
        format: String,
    },
    /// Raw sensor data (future use)
    Sensor { sensor_type: String },
}

/// Audio format specification
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    /// 32-bit floating point samples
    F32Le,
    /// 16-bit signed integer samples
    I16Le,
    /// 24-bit signed integer samples
    I24Le,
    /// Other/custom format
    Other(String),
}

/// Manifest structure
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub v: u8,
    pub ts_ms: u64,
    pub seq: u64,
    pub header_hash: [u8; 32],
    pub pt_hash: [u8; 32],
    pub key_id: [u8; 16], // Added for key identification/rotation
    pub ai_used: bool,
    pub model_ids: Vec<String>,
    pub data_type: DataType, // NEW: Data type and format metadata
}

/// SignedManifest structure
#[derive(Serialize, Deserialize)]
pub struct SignedManifest {
    pub manifest: Vec<u8>,
    pub sig: Vec<u8>,
    pub pubkey: Vec<u8>,
}

/// StreamHeader structure
#[derive(Serialize, Deserialize)]
pub struct StreamHeader {
    pub v: u8,
    pub header: Vec<u8>, // 58 bytes in practice
    pub header_hash: [u8; 32],
}

/// Record structure
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub seq: u64,
    pub nonce: [u8; NONCE_LEN],
    pub sm: SignedManifest,
    pub ct: Vec<u8>,
}

/// Build Additional Authenticated Data (AAD) for encryption
pub fn build_aad(
    header_hash: &[u8; 32],
    seq: u64,
    nonce: &[u8; NONCE_LEN],
    manifest_hash: &[u8; 32],
) -> [u8; AAD_LEN] {
    let mut aad = [0u8; AAD_LEN];
    let mut off = 0;
    aad[off..off + 32].copy_from_slice(header_hash);
    off += 32;
    aad[off..off + 8].copy_from_slice(&seq.to_be_bytes());
    off += 8;
    aad[off..off + NONCE_LEN].copy_from_slice(nonce);
    off += NONCE_LEN;
    aad[off..off + 32].copy_from_slice(manifest_hash);
    aad
}

/// Write the stream header to the output
pub fn write_stream_header<W: std::io::Write>(w: &mut W, sh: &StreamHeader) -> Result<()> {
    w.write_all(MAGIC).context("write magic")?;
    w.write_all(&[VERSION]).context("write version")?;
    bincode::serialize_into(w, sh).context("write stream header")?;
    Ok(())
}

/// Read the preamble and stream header from the input
pub fn read_preamble_and_header<R: std::io::Read>(r: &mut R) -> Result<StreamHeader> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic).context("read magic")?;
    anyhow::ensure!(&magic == MAGIC, "bad magic");
    let mut ver = [0u8; 1];
    r.read_exact(&mut ver).context("read version")?;
    anyhow::ensure!(ver[0] == VERSION, "unsupported version");
    let sh: StreamHeader = bincode::deserialize_from(r).context("read stream header")?;
    Ok(sh)
}

/// Sign manifest bytes with domain separation
/// This prevents signature reuse across different contexts or protocols
pub fn sign_manifest_with_domain(signing_key: &SigningKey, manifest_bytes: &[u8]) -> Signature {
    let mut message = Vec::with_capacity(MANIFEST_DOMAIN_SEP.len() + manifest_bytes.len());
    message.extend_from_slice(MANIFEST_DOMAIN_SEP);
    message.extend_from_slice(manifest_bytes);
    signing_key.sign(&message)
}

/// Verify manifest signature with domain separation
/// This prevents signature reuse across different contexts or protocols
pub fn verify_manifest_with_domain(
    verifying_key: &VerifyingKey, 
    manifest_bytes: &[u8], 
    signature: &Signature
) -> Result<()> {
    let mut message = Vec::with_capacity(MANIFEST_DOMAIN_SEP.len() + manifest_bytes.len());
    message.extend_from_slice(MANIFEST_DOMAIN_SEP);
    message.extend_from_slice(manifest_bytes);
    verifying_key.verify(&message, signature)
        .map_err(|e| anyhow::anyhow!("Domain-separated manifest signature verification failed: {}", e))
}
