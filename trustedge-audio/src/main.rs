
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
use blake3;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rand_core::RngCore;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

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
struct FileHeader {
    version: u8,                // 1 byte: Version of the header format
    alg: u8,                    // 1 byte: Encryption algorithm (1 = AES-256-GCM)
    key_id: [u8; 16],           // 16 bytes: Unique identifier for the encryption key
    device_id_hash: [u8; 32],   // 32 bytes: Hash of the device ID
    nonce_prefix: [u8; 4],      // 4 bytes: Prefix for the nonce
    chunk_size: u32,            // 4 bytes: Size of each chunk in bytes
}

impl FileHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.push(self.version);
        v.push(self.alg);
        v.extend_from_slice(&self.key_id);
        v.extend_from_slice(&self.device_id_hash);
        v.extend_from_slice(&self.nonce_prefix);
        v.extend_from_slice(&self.chunk_size.to_be_bytes());
        v
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

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
    let mut device_id_hash = [0u8; 32];
    OsRng.fill_bytes(&mut device_id_hash);
    let header = FileHeader {
        version: 1,
        alg: 1, // 1 = AES-256-GCM
        key_id,
        device_id_hash,
        nonce_prefix,
        chunk_size: args.chunk as u32,
    };
    let header_bytes = header.to_bytes();
    fout.write_all(&header_bytes).context("write header")?;

    // Compute header hash for AAD using blake3
    let header_hash = blake3::hash(&header_bytes);

    let mut seq: u64 = 0;
    loop {
        let n = fin.read(&mut buf).context("read chunk")?;
        if n == 0 {
            break;
        }
        seq += 1;

        // Nonce: 4-byte prefix || 8-byte counter
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[..4].copy_from_slice(&header.nonce_prefix);
        nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Build AAD: [header_hash][seq][nonce]
        let mut aad = Vec::new();
        aad.extend_from_slice(header_hash.as_bytes());
        aad.extend_from_slice(&seq.to_be_bytes());
        aad.extend_from_slice(&nonce_bytes);

        let ct = cipher
            .encrypt(nonce, Payload { msg: &buf[..n], aad: &aad })
            .map_err(|_| anyhow!("AES-GCM encrypt failed"))?;

        // Immediately decrypt to verify integrity (simulating a stream consumer)
        let pt = cipher
            .decrypt(nonce, Payload { msg: &ct, aad: &aad })
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

        // Write the verified plaintext to output file
        fout.write_all(&pt).context("write out")?;

        total_in += n;
        total_out += pt.len();
    }

    fout.flush().ok();
    eprintln!(
        "Round-trip complete. Read {} bytes, wrote {} bytes.",
        total_in, total_out
    );
    Ok(())
}
