// Copyright (c) 2025 John Turner
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

// trustedge_audio/src/format.rs
// src/format.rs
use anyhow::{Context, Result};
use serde::{Serialize, Deserialize};

pub const NONCE_LEN: usize = 12;
pub const AAD_LEN: usize = 32 + 8 + NONCE_LEN + 32;

pub const MAGIC: &[u8; 4] = b"TRST";
pub const VERSION: u8 = 1;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub v: u8,
    pub ts_ms: u64,
    pub seq: u64,
    pub header_hash: [u8; 32],
    pub pt_hash: [u8; 32],
    pub ai_used: bool,
    pub model_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SignedManifest {
    pub manifest: Vec<u8>,
    pub sig: Vec<u8>,
    pub pubkey: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamHeader {
    pub v: u8,
    pub header: Vec<u8>,       // 58 bytes in practice
    pub header_hash: [u8; 32],
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub seq: u64,
    pub nonce: [u8; NONCE_LEN],
    pub sm: SignedManifest,
    pub ct: Vec<u8>,
}

pub fn build_aad(
    header_hash: &[u8; 32],
    seq: u64,
    nonce: &[u8; NONCE_LEN],
    manifest_hash: &[u8; 32],
) -> [u8; AAD_LEN] {
    let mut aad = [0u8; AAD_LEN];
    let mut off = 0;
    aad[off..off+32].copy_from_slice(header_hash); off += 32;
    aad[off..off+8].copy_from_slice(&seq.to_be_bytes()); off += 8;
    aad[off..off+NONCE_LEN].copy_from_slice(nonce); off += NONCE_LEN;
    aad[off..off+32].copy_from_slice(manifest_hash);
    aad
}

pub fn write_stream_header<W: std::io::Write>(w: &mut W, sh: &StreamHeader) -> Result<()> {
    w.write_all(MAGIC).context("write magic")?;
    w.write_all(&[VERSION]).context("write version")?;
    bincode::serialize_into(w, sh).context("write stream header")?;
    Ok(())
}

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
