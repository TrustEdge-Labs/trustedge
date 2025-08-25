// Copyright (c) 2025 John Turner
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

// trustedge_audio/src/format.rs
use anyhow::Context;
use bincode::serialize_into;
use serde::{Serialize, Deserialize};

/// Stream preamble (so decoders can fast-fail & version-gate)
pub const MAGIC: &[u8; 4] = b"TRST";
pub const VERSION: u8 = 1;

/// AEAD sizes used across the project
pub const NONCE_LEN: usize = 12; // AES-GCM 96-bit nonce
pub const AAD_LEN: usize = 32 /* header_hash */
    + 8 /* seq */
    + NONCE_LEN /* nonce */
    + 32 /* manifest_hash */;

/// Envelope types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamHeader {
    pub v: u8,                 // stream format version
    pub header: Vec<u8>,       // 58 bytes (Vec so serde is happy)
    pub header_hash: [u8; 32], // BLAKE3(header)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedManifest {
    pub manifest: Vec<u8>, // bincode(manifest)
    pub sig: Vec<u8>,      // Ed25519 signature (64 bytes)
    pub pubkey: Vec<u8>,   // Ed25519 public key (32 bytes)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub v: u8,                 // manifest version
    pub ts_ms: u64,            // capture/encode time
    pub seq: u64,              // chunk sequence
    pub header_hash: [u8; 32], // binds to stream header
    pub pt_hash: [u8; 32],     // BLAKE3(plaintext chunk)
    pub ai_used: bool,         // placeholder
    pub model_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub seq: u64,
    pub nonce: [u8; NONCE_LEN],
    pub sm: SignedManifest,
    pub ct: Vec<u8>, // AES-GCM ciphertext (+tag)
}

/// Build AEAD AAD buffer consistently across all binaries.
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

/// Write MAGIC + VERSION + StreamHeader to a writer.
pub fn write_stream_header<W: std::io::Write>(
    w: &mut W,
    stream_header: &StreamHeader,
) -> anyhow::Result<()> {
    // preamble
    w.write_all(MAGIC).context("write magic")?;
    w.write_all(&[VERSION]).context("write version")?;
    // header payload
    serialize_into(w, stream_header).context("write stream header")?;
    Ok(())
}

