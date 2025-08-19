
//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Key, Nonce,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rand_core::RngCore;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use zeroize::Zeroize; // For secure memory handling

/// Simple local demo: reads an input file in chunks, encrypts each chunk with AES-256-GCM,
/// then immediately decrypts and verifies, then writes a copy of the plaintext to --out
/// to show round-trip integrity.
#[derive(Parser, Debug)]
#[command(name = "trustedge-audio", version, about)]
struct Args {
    /// Input file (e.g., raw/wav/mp3 — treated as opaque bytes)
    #[arg(short, long)]
    input: PathBuf,
    /// Output round-tripped file (decrypted copy)
    #[arg(short, long)]
    out: PathBuf,
    /// Chunk size in bytes
    #[arg(long, default_value_t = 4096)]
    chunk: usize,
}

// Set up the file header struct for AAD (58 bytes)
const HEADER_LEN: usize = 58;
const ALG_AES_256_GCM: u8 = 1;

const AAD_LEN: usize = 32 + 8 + 12; // blake3(header) || seq || nonce

#[derive(Clone, Copy, Debug)]
struct FileHeader {
    version: u8,              // 1
    alg: u8,                  // 1
    key_id: [u8; 16],         // 16
    device_id_hash: [u8; 32], // 32
    nonce_prefix: [u8; 4],    // 4
    chunk_size: u32,          // 4 (big-endian)
}

impl FileHeader {
    /// Serialize deterministically to a fixed 58-byte header.
    fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[0] = self.version;
        out[1] = self.alg;
        out[2..18].copy_from_slice(&self.key_id);
        out[18..50].copy_from_slice(&self.device_id_hash);
        out[50..54].copy_from_slice(&self.nonce_prefix);
        out[54..58].copy_from_slice(&self.chunk_size.to_be_bytes()); // BE
        out
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // check the chunk size
    anyhow::ensure!(args.chunk > 0, "chunk must be > 0");
    anyhow::ensure!(args.chunk as u64 <= u32::MAX as u64, "chunk too large");

    // NO PRODUCTION
    // DEMO ONLY: random 256-bit key per run. (Later: load from KMS/TPM or .env)
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));

    let mut fin = File::open(&args.input).context("open input")?;
    let mut fout = File::create(&args.out).context("create output")?;

    let mut buf = vec![0u8; args.chunk];
    let mut total_in = 0usize;
    let mut total_out = 0usize;

    // header fields (DEMO ONLY: replace with real values as needed)
    let mut nonce_prefix = [0u8; 4];
    OsRng.fill_bytes(&mut nonce_prefix);
    let mut key_id = [0u8; 16];
    OsRng.fill_bytes(&mut key_id);

    // Derive a stable device_id_hash = BLAKE3(device_id || salt)
    // NO PRODUCTION
    let device_id = std::env::var("TRUSTEDGE_DEVICE_ID")
        .unwrap_or_else(|_| "trustedge-abc123".to_string());
    let salt = std::env::var("TRUSTEDGE_SALT")
        .unwrap_or_else(|_| "trustedge-demo-salt".to_string());

    let mut device_id_hash = [0u8; 32];
    let mut hasher = blake3::Hasher::new();
    hasher.update(device_id.as_bytes());
    hasher.update(salt.as_bytes());
    let digest = hasher.finalize();
    device_id_hash.copy_from_slice(digest.as_bytes());

    // Create the file header
    let header = FileHeader {
    version: 1,
    alg: ALG_AES_256_GCM,
    key_id,
    device_id_hash,
    nonce_prefix,
    chunk_size: args.chunk as u32,
};
    let header_bytes = header.to_bytes();           // [u8; 58]
    let header_hash = blake3::hash(&header_bytes);  // same as before

    // Don't write the header to the output file, instead write header+enc to a separate file (TBD)
    // fout.write_all(&header_bytes).context("write header")?;

    let mut seq: u64 = 0;
    let mut aad = [0u8; AAD_LEN];  // AAD buffer

    // Nonce: 4-byte prefix || 8-byte counter
    const NONCE_LEN: usize = 12;
    let mut nonce_bytes = [0u8; NONCE_LEN];

    loop {
        let n = fin.read(&mut buf).context("read chunk")?;
        if n == 0 {
            break;
        }
        seq = seq.checked_add(1).ok_or_else(|| anyhow!("seq overflow"))?;

        nonce_bytes[..4].copy_from_slice(&header.nonce_prefix);
        nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Build AAD: [header_hash][seq][nonce]
        aad[..32].copy_from_slice(header_hash.as_bytes());
        aad[32..40].copy_from_slice(&seq.to_be_bytes());
        aad[40..].copy_from_slice(&nonce_bytes);

        let ct = cipher
            .encrypt(nonce, Payload { msg: &buf[..n], aad: &aad })
            .map_err(|_| anyhow!("AES-GCM encrypt failed"))?;

        // quick bad payload check
        let mut ct_bad = ct.clone();
        ct_bad[0] ^= 0x01;
        let should_err = cipher.decrypt(nonce, Payload { msg: &ct_bad, aad: &aad });
        debug_assert!(should_err.is_err(), "tamper test should fail");

        // Immediately decrypt to verify integrity (simulating a stream consumer)
        let pt = cipher
            .decrypt(nonce, Payload { msg: &ct, aad: &aad })
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

        // Write the verified plaintext to output file
        fout.write_all(&pt).context("write out")?;

        total_in += n;
        total_out += pt.len();
    }

    // cleanup 
    key_bytes.zeroize();
    drop(cipher);
    fout.flush().ok();

    eprintln!(
        "Round-trip complete. Read {} bytes, wrote {} bytes.",
        total_in, total_out
    );
    Ok(())
}
