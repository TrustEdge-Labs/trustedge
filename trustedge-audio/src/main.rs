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

/// Simple local demo: reads an input file in chunks, encrypts each chunk with AES-256-GCM,
/// then immediately decrypts and verifies equality. Writes a copy of the plaintext to --out
/// to prove round-trip integrity.
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

fn main() -> Result<()> {
    let args = Args::parse();

    // DEMO ONLY: random 256-bit key per run. (Later: load from KMS/TPM or .env)
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));

    let mut fin = File::open(&args.input).context("open input")?;
    let mut fout = File::create(&args.out).context("create output")?;

    let mut buf = vec![0u8; args.chunk];
    let mut total_in = 0usize;
    let mut total_out = 0usize;
    let mut chunk_idx: u64 = 0;

    loop {
        let n = fin.read(&mut buf).context("read chunk")?;
        if n == 0 {
            break;
        }
        chunk_idx += 1;

        // Nonce strategy for demo: 96-bit (12 bytes) random per chunk.
        // In production use a counter scheme to guarantee uniqueness under the same key.
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Optional: bind metadata as AAD (authenticated, not encrypted)
        // For now we’ll include chunk index to demonstrate AAD usage.
        let aad = chunk_idx.to_be_bytes();
        let ct = cipher
            .encrypt(nonce, Payload { msg: &buf[..n], aad: &aad })
            .map_err(|_| anyhow!("AES-GCM encrypt failed"))?;

        // Immediately decrypt to verify integrity (simulating the consumer)
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
