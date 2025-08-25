//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
#![forbid(unsafe_code)]

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, Payload},
    Aes256Gcm, Key, Nonce,
};

use anyhow::{anyhow, Context, Result};
use bincode::{serialize_into, deserialize_from};
use clap::Parser;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier}; // demo key for now NO PRODUCTION
use hex;
use rand_core::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use trustedge_audio::KeyManager;
use zeroize::Zeroize;

// algorithm ID
const ALG_AES_256_GCM: u8 = 1;

// File header length for AAD (58 bytes)
const HEADER_LEN: usize = 58;

const NONCE_LEN: usize = 12; // AES-GCM uses a 96-bit (12-byte) nonce.

// AAD length for AES-256-GCM (32 + 8 + 12 + 32 = 84 bytes)
const AAD_LEN: usize = 32    /* header_hash */ 
                        + 8  /* seq */ 
                        + NONCE_LEN /* nonce */ 
                        + 32 /* manifest_hash */;

/// Simple local demo: reads an input file in chunks, encrypts each chunk with AES-256-GCM,
/// then immediately decrypts and verifies, then writes a copy of the plaintext to --out
/// to show round-trip integrity.
#[derive(Parser, Debug)]
#[command(name = "trustedge-audio", version, about)]
struct Args {
    /// Input file (e.g., raw/wav/mp3 — treated as opaque bytes)
    #[arg(short, long)]
    input: Option<PathBuf>,
    /// Output round-tripped file (decrypted copy)
    #[arg(short, long)]
    out: Option<PathBuf>,
    /// Chunk size in bytes
    #[arg(long, default_value_t = 4096)]
    chunk: usize,
    // write a .trst with header + records
    #[arg(long)]
    envelope: Option<PathBuf>,
    // skip writing round-tripped plaintext        
    #[arg(long, default_value_t = false)]
    no_plaintext: bool,
    // when true, read .trst and decrypt to --out   
    #[arg(long, default_value_t = false)]
    decrypt: bool,                 
    // 64-hex-char AES-256 key for both modes
    #[arg(long)]
    key_hex: Option<String>,       
    // optional: where to dump generated key during encrypt
    #[arg(long)]
    key_out: Option<PathBuf>,  

    /// Set passphrase in system keyring (run once to configure)
    #[arg(long)]
    set_passphrase: Option<String>,

    /// Salt for key derivation (hex string)
    #[arg(long)]
    salt_hex: Option<String>,

    /// Use keyring passphrase instead of --key-hex
    #[arg(long)]
    use_keyring: bool,                
}

// set up the file header
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
        out[54..58].copy_from_slice(&self.chunk_size.to_be_bytes()); 
        out
    }
}

// set up the manifest, signed and unsigned
#[derive(Serialize, Deserialize)]
struct Manifest {
    v: u8,                // manifest version (1)
    ts_ms: u64,           // capture/encode time
    seq: u64,             // chunk sequence
    header_hash: [u8; 32],// binds to your file/session header
    pt_hash: [u8; 32],    // BLAKE3 of plaintext chunk
    ai_used: bool,        // placeholder for "was AI used?" flag
    model_ids: Vec<String>, // maybe? for model(s) used at edge
}

// note: serde only auto-derives for arrays up to [u8; 32] so try using Vec<u8> for both fields
#[derive(Serialize, Deserialize)]
pub struct SignedManifest {
    manifest: Vec<u8>,    // bincode(manifest)
    sig: Vec<u8>,         // ed25519 signature over manifest bytes
    pubkey: Vec<u8>,      // Ed25519 public key (or a key_id)
}

// set up a StreamHeader to be used once
#[derive(serde::Serialize, serde::Deserialize)]
pub struct StreamHeader {
    v: u8,                          // stream format version
    header: Vec<u8>,                // 58-byte header bytes, serde won't allow HEADER_LEN
    header_hash: [u8; 32],          // BLAKE3(header)
}

// set up a Record to be used per chunk
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Record {
    seq: u64,
    nonce: [u8; NONCE_LEN],
    sm: SignedManifest,             // Signed manifest (bytes + sig + pubkey)
    ct: Vec<u8>,                    // AES-GCM ciphertext (+tag)
}

// helper function to parse and validate a basic hex-encoded key
fn parse_key_hex(s: &str) -> Result<[u8; 32]> {
    let bytes = hex::decode(s).context("key_hex: not valid hex")?;
    anyhow::ensure!(bytes.len() == 32, "key_hex must be 32 bytes (64 hex chars)");
    let mut out = [0u8; 32];
    out.copy_from_slice(&bytes);
    Ok(out)
}

// helper function to build AAD, use offsets to avoid magic numbers
fn build_aad(header_hash: &[u8; 32], seq: u64, nonce: &[u8; NONCE_LEN], manifest_hash: &[u8; 32]) -> [u8; AAD_LEN] {
    let mut aad = [0u8; AAD_LEN];
    let mut off = 0;
    aad[off..off+32].copy_from_slice(header_hash); off += 32;
    aad[off..off+8].copy_from_slice(&seq.to_be_bytes()); off += 8;
    aad[off..off+NONCE_LEN].copy_from_slice(nonce); off += NONCE_LEN;
    aad[off..off+32].copy_from_slice(manifest_hash);
    aad
}

// helper function to decrypt the envelope
fn decrypt_envelope(args: &Args) -> Result<()> {
    // Require key for decrypt
    let key_hex = args.key_hex.as_ref().ok_or_else(|| anyhow!("--key-hex is required in --decrypt mode"))?;
    let key_bytes = parse_key_hex(key_hex)?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
  
    // Open envelope and output
    let input = args.input.as_ref().ok_or_else(|| anyhow::anyhow!("--input is required for decrypt mode"))?;
    let out = args.out.as_ref().ok_or_else(|| anyhow::anyhow!("--out is required for decrypt mode"))?;

    // Use the passed parameters directly
    let mut r = BufReader::new(File::open(input).context("open envelope")?);
    let mut w = BufWriter::new(File::create(out).context("create output")?);

    // Read stream header
    let sh: StreamHeader = deserialize_from(&mut r).context("read stream header")?;

    // Trust but verify: recompute header_hash
    let hh = blake3::hash(&sh.header);
    anyhow::ensure!(hh.as_bytes() == &sh.header_hash, "header_hash mismatch in stream header");

    // Consume records until EOF
    let mut total_out = 0usize;
    loop {
        // Try to read next record
        let rec: Record = match deserialize_from(&mut r) {
            Ok(x) => x,
            Err(err) => {
                // bincode EOF handling
                if let bincode::ErrorKind::Io(ref e) = *err {
                    if e.kind() == std::io::ErrorKind::UnexpectedEof {
                        break;
                    }
                }
                return Err(err).context("read record");
            }
        };

        // Verify the signed manifest
        let pubkey_arr: [u8; 32] = rec.sm.pubkey.as_slice().try_into().context("pubkey length != 32")?;
        let sig_arr: [u8; 64] = rec.sm.sig.as_slice().try_into().context("signature length != 64")?;
        let vk = VerifyingKey::from_bytes(&pubkey_arr).context("bad pubkey")?;
        vk.verify(&rec.sm.manifest, &Signature::from_bytes(&sig_arr))
            .context("manifest signature verify failed")?;

        // Decode the manifest & check the header hash
        let m: Manifest = bincode::deserialize(&rec.sm.manifest).context("manifest decode")?;
        anyhow::ensure!(m.header_hash == sh.header_hash, "manifest.header_hash != stream header_hash");

        // Bind AAD and decrypt
        let mh = blake3::hash(&rec.sm.manifest);
        let aad = build_aad(&sh.header_hash, rec.seq, &rec.nonce, mh.as_bytes());
        let pt = cipher
            .decrypt(Nonce::from_slice(&rec.nonce), Payload { msg: &rec.ct, aad: &aad })
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

        // Verify the plaintext hash from the manifest
        let pt_hash_rx = blake3::hash(&pt);
        anyhow::ensure!(pt_hash_rx.as_bytes() == &m.pt_hash, "pt hash mismatch");

        // Write out the plaintext
        w.write_all(&pt).context("write plaintext")?;
        total_out += pt.len();

    }

    // flush output
    w.flush().context("flush plaintext")?;

    // status and exit
    eprintln!("Decrypt complete. Wrote {} bytes.", total_out);
    Ok(())
}

// streaming header helper function
fn write_stream_header<W: std::io::Write>(
    w: &mut W,
    header_bytes: &[u8; HEADER_LEN],
    header_hash: &[u8; 32],
) -> anyhow::Result<()> {
    let sh = StreamHeader {
        v: 1,
        header: header_bytes.to_vec(),   // Vec<u8> so Serde is happy
        header_hash: *header_hash,       // copy into [u8; 32]
    };
    serialize_into(w, &sh).context("write stream header")?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // check if we're decrypting
    if args.decrypt {
        return decrypt_envelope(&args);
    }

    // check the chunk size
    anyhow::ensure!(args.chunk > 0, "chunk must be > 0");
    anyhow::ensure!(args.chunk as u64 <= u32::MAX as u64, "chunk too large");

    // set up keys
    let signing = SigningKey::generate(&mut OsRng);
    let verify: VerifyingKey = signing.verifying_key();

    let key_manager = KeyManager::new();
    
    // Handle setting passphrase
    if let Some(passphrase) = args.set_passphrase {
        key_manager.store_passphrase(&passphrase)?;
        println!("Passphrase stored in system keyring");
        return Ok(());
    }
    
    // check for input and output args
    let input = args.input.as_ref().ok_or_else(|| anyhow::anyhow!("--input is required"))?;
    let out = args.out.as_ref().ok_or_else(|| anyhow::anyhow!("--out is required"))?;

    // Determine encryption key
    let key_bytes = if args.use_keyring {
        let salt_hex = args.salt_hex.as_ref()
            .ok_or_else(|| anyhow::anyhow!("--salt-hex required when using keyring"))?;
        let salt_bytes = hex::decode(salt_hex)?;
        if salt_bytes.len() != 16 {
            return Err(anyhow::anyhow!("Salt must be 16 bytes (32 hex chars)"));
        }
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&salt_bytes);
        key_manager.derive_key(&salt)?
    } else if let Some(ref kh) = args.key_hex {
        parse_key_hex(kh)?
    } else {
        // Your existing random key generation
        let mut kb = [0u8; 32];
        OsRng.fill_bytes(&mut kb);
        kb
    };   

    // key selection for ENCRYPT mode
    let mut key_bytes = if let Some(ref kh) = args.key_hex {
        parse_key_hex(kh)?
    } else {
        let mut kb = [0u8; 32];
        OsRng.fill_bytes(&mut kb);
        if let Some(ref p) = args.key_out {
            std::fs::write(p, hex::encode(kb)).context("write key_out")?;
        } else {
            eprintln!("NOTE (demo): AES-256 key (hex) = {}", hex::encode(kb));
        }
        kb
    };

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));

    // use buffers for less syscalls
    let mut fin  = BufReader::new(File::open(&input).context("open input")?);
    let mut fout = BufWriter::new(File::create(&out).context("create output")?);

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
    let header_hash = blake3::hash(&header_bytes);  // BLAKE3(header)

    // if arg use, set up the optional envelope writer
    let mut env_out = if let Some(path) = &args.envelope {
        Some(BufWriter::new(File::create(path).context("create envelope")?))
    } else {
        None
    };

    // if writing an envelope, then write a StreamHeader once
    if let Some(w) = env_out.as_mut() {
        write_stream_header(w, &header_bytes, header_hash.as_bytes())?;
    }

    // Initialize sequence number and nonce
    let mut seq: u64 = 0;
    let mut nonce_bytes = [0u8; NONCE_LEN];

    // loop through the chunks
    loop {
        let n = fin.read(&mut buf).context("read chunk")?;

        if n == 0 {
            break;
        }

        // check for a sequence overflow
        seq = seq.checked_add(1).ok_or_else(|| anyhow!("seq overflow"))?;

        nonce_bytes[..4].copy_from_slice(&header.nonce_prefix);
        nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Hash the plaintext chunk BEFORE encryption
        let pt_hash = blake3::hash(&buf[..n]);

        // Get a millisecond timestamp since epoch; default to 0 (figure out what happens then)
        let ts_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        // build the manifest
        let m = Manifest {
            v: 1,
            ts_ms,
            seq,
            header_hash: *header_hash.as_bytes(),
            pt_hash: *pt_hash.as_bytes(),
            ai_used: false,                         // placeholder for AI usage flag
            model_ids: vec![],                      // placeholder for AI model IDs
        };

        // serialize the manifest then sign it
        let m_bytes = bincode::serialize(&m).expect("manifest serialize");
        let sig: Signature = signing.sign(&m_bytes);

        let sm = SignedManifest {
            manifest: m_bytes.clone(),
            sig: sig.to_bytes().to_vec(),
            pubkey: verify.to_bytes().to_vec(),
        };

        // link the manifest to the ciphertext via AAD using a hash of the manifest bytes
        let mhash = blake3::hash(&m_bytes);
        
        // Build AAD: [header_hash][seq][nonce][manifest_hash]
        let aad = build_aad(header_hash.as_bytes(), seq, &nonce_bytes, mhash.as_bytes());

        // encrypt a chunk
        let ct = cipher
            .encrypt(nonce, Payload { msg: &buf[..n], aad: &aad })
            .map_err(|_| anyhow!("AES-GCM encrypt failed"))?;

        // quick bad payload check
        let mut ct_bad = ct.clone();
        ct_bad[0] ^= 0x01;
        let should_err = cipher.decrypt(nonce, Payload { msg: &ct_bad, aad: &aad });
        debug_assert!(should_err.is_err(), "tamper test should fail");

        // before decrypting, verify the signed manifest (origin & integrity)
        let m2: Manifest = bincode::deserialize(&sm.manifest).context("manifest decode")?;

        // check pubkey and signature lengths
        let pubkey_arr: [u8; 32] = sm.pubkey.as_slice()
            .try_into()
            .context("pubkey length != 32")?;
        let sig_arr: [u8; 64] = sm.sig.as_slice()
            .try_into()
            .context("signature length != 64")?;

        // verify the manifest signature
        let vk = VerifyingKey::from_bytes(&pubkey_arr).context("bad pubkey")?;
        vk.verify(&sm.manifest, &Signature::from_bytes(&sig_arr))
            .context("manifest signature verify failed")?;

        // redo AAD from the received manifest bytes to validate the GCM tag check
        let mhash_rx = blake3::hash(&sm.manifest);

        let aad_rx = build_aad(header_hash.as_bytes(), seq, &nonce_bytes, mhash_rx.as_bytes());

        // now decrypt & authenticate the payload with the AAD bound to the manifest
        let pt = cipher
            .decrypt(Nonce::from_slice(&nonce_bytes), Payload { msg: &ct, aad: &aad_rx })
            .map_err(|_| anyhow!("AES-GCM decrypt/verify failed"))?;

        // verify the plaintext hash matches the pt_hash from the manifest (trust but verify)
        let pt_hash_rx = blake3::hash(&pt);
        anyhow::ensure!(pt_hash_rx.as_bytes() == &m2.pt_hash, "pt hash mismatch");

        // check args for no plaintext, otherwise do the write
        if !args.no_plaintext {
            fout.write_all(&pt).context("write out")?;
        }

        // after a successful decrypt + (optional) plaintext write, log a record
        if let Some(w) = env_out.as_mut() {
            let rec = Record {
                        seq,
                        nonce: nonce_bytes,  // [u8; NONCE_LEN]
                        sm,                  // move SignedManifest into the record
                        ct,                  // move ciphertext (no longer needed)
            };
            serialize_into(w, &rec).context("write envelope record")?;
        }

        // update byte counters
        total_in += n;        total_out += pt.len();

    }

    // cleanup 
    key_bytes.zeroize();
    drop(cipher);
    
    // flush output streams
    if !args.no_plaintext {
        fout.flush().context("flush plaintext output")?;
    }
    if let Some(w) = env_out.as_mut() {
        w.flush().context("flush envelope")?;
    }
    fout.flush().ok();

    // status then exit
    eprintln!(
        "Round-trip complete. Read {} bytes, wrote {} bytes.",
        total_in, total_out
    );
    Ok(())
}
