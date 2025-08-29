#![allow(dead_code)]
//
// Copyright (c) 2025 John Turner
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//
/// Deterministic test vectors for the TrustEdge envelope format.
///
/// Fix *all* randomness (AES key, signing key, nonce prefix, header fields,
/// timestamp) so `.trst` bytes are **identical** on every run under test.
///
/// First run: the test will print a BLAKE3 digest of the generated .trst buffer.
/// Copy that digest into `GOLDEN_TRST_BLAKE3` below and commit. When the format/crypto
/// changes intentionally, re-run, copy the new digest (rebase the golden), and commit.
///
/// Test vectors for the TrustEdge envelope format.
#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::{
        build_aad, write_stream_header, FileHeader, Manifest, Record, SignedManifest, StreamHeader,
        ALG_AES_256_GCM, HEADER_LEN, MAGIC, NONCE_LEN, VERSION,
    };

    use bincode::serialize_into;
    use blake3;
    use hex;

    /// crypto (test-only)
    use aes_gcm::{
        aead::{Aead, KeyInit, Payload},
        Aes256Gcm, Key,
    };
    use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

    // ----------------------------
    // Fixed, test-only constants
    // ----------------------------

    /// 32-byte AES-256 key (hex = 000102...1f). TEST USE ONLY.
    const TEST_AES_KEY: [u8; 32] = [
        0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
        0x0F, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D,
        0x1E, 0x1F,
    ];

    /// 32-byte Ed25519 signing seed (NOT a production key). TEST USE ONLY.
    const TEST_SIGNING_SEED: [u8; 32] = [
        0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42,
        0x42, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24, 0x24,
        0x24, 0x24,
    ];

    /// 4-byte deterministic nonce prefix for tests.
    const TEST_NONCE_PREFIX: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];

    /// 16-byte key id embedded in header + manifest for tests.
    const TEST_KEY_ID: [u8; 16] = *b"TEST_KEY_ID_16B!";

    /// Deterministic device id hash (we derive at runtime from fixed inputs).
    const TEST_DEVICE_ID: &[u8] = b"trustedge-test-device";
    const TEST_SALT: &[u8] = b"trustedge-test-salt";

    /// Deterministic timestamp used in manifests for tests.
    const TEST_TS_MS: u64 = 1_700_000_000_000;

    /// Replace this after first run (see test output).
    /// const GOLDEN_TRST_BLAKE3: &str = "<fill-me-after-first-run>";
    /// after first run =
    const GOLDEN_TRST_BLAKE3: &str =
        "8ecc3b2fcb0887dfd6ff3513c0caa3febb2150a920213fa5b622243ad530f34c";

    // ----------------------------
    // Helpers
    // ----------------------------

    /// Create a file header for the given chunk size.
    fn make_file_header(chunk_size: usize) -> (FileHeader, [u8; HEADER_LEN], [u8; 32]) {
        assert!(chunk_size as u64 <= u32::MAX as u64, "chunk too large");

        // device_id_hash = BLAKE3(device_id || salt)
        let mut hasher = blake3::Hasher::new();
        hasher.update(TEST_DEVICE_ID);
        hasher.update(TEST_SALT);
        let device_hash = *hasher.finalize().as_bytes();

        let fh = FileHeader {
            version: 1,
            alg: ALG_AES_256_GCM,
            key_id: TEST_KEY_ID,
            device_id_hash: device_hash,
            nonce_prefix: TEST_NONCE_PREFIX,
            chunk_size: chunk_size as u32,
        };

        let header_bytes = fh.to_bytes();
        let header_hash = *blake3::hash(&header_bytes).as_bytes();
        (fh, header_bytes, header_hash)
    }

    /// Create a nonce for the given prefix and sequence number.
    fn make_nonce(prefix: [u8; 4], seq: u64) -> [u8; NONCE_LEN] {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        nonce_bytes[..4].copy_from_slice(&prefix);
        nonce_bytes[4..].copy_from_slice(&seq.to_be_bytes());
        nonce_bytes
    }

    /// Produce a deterministic `.trst` envelope for `input`, chunked with `chunk_size`.
    fn deterministic_trst(input: &[u8], chunk_size: usize) -> Vec<u8> {
        // Keys / crypto (fixed for tests)
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&TEST_AES_KEY));
        let signing = SigningKey::from_bytes(&TEST_SIGNING_SEED);
        let verify: VerifyingKey = signing.verifying_key();

        // Header & hash
        let (fh, header_bytes, header_hash) = make_file_header(chunk_size);
        let nonce_prefix = fh.nonce_prefix;

        // Build the .trst in memory
        let mut out = Vec::<u8>::new();

        // Preamble
        out.write_all(MAGIC).unwrap();
        out.write_all(&[VERSION]).unwrap();

        // Stream header
        let sh = StreamHeader {
            v: VERSION,
            header: header_bytes.to_vec(),
            header_hash,
        };
        write_stream_header(&mut out, &sh).unwrap();

        // Chunk & record loop
        let mut seq: u64 = 0;
        let mut offset: usize = 0;

        while offset < input.len() {
            let end = usize::min(offset + chunk_size, input.len());
            let pt = &input[offset..end];

            seq = seq.checked_add(1).expect("seq overflow");

            let nonce_bytes = make_nonce(nonce_prefix, seq);
            let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);

            // Plaintext hash
            let pt_hash = blake3::hash(pt);

            // Manifest (use fixed timestamp + key_id)
            let m = Manifest {
                v: 1,
                ts_ms: TEST_TS_MS,
                seq,
                header_hash,
                pt_hash: *pt_hash.as_bytes(),
                ai_used: false,
                model_ids: vec![],
                key_id: TEST_KEY_ID,
            };

            let m_bytes = bincode::serialize(&m).expect("manifest serialize");
            let sig: Signature = signing.sign(&m_bytes);
            let sm = SignedManifest {
                manifest: m_bytes.clone(),
                sig: sig.to_bytes().to_vec(),
                pubkey: verify.to_bytes().to_vec(),
            };

            // AAD = [header_hash || seq || nonce || blake3(manifest)]
            let mh = blake3::hash(&m_bytes);
            let aad = build_aad(&header_hash, seq, &nonce_bytes, mh.as_bytes());

            // Encrypt
            let ct = cipher
                .encrypt(nonce, Payload { msg: pt, aad: &aad })
                .expect("AES-GCM encrypt failed");

            // Record
            let rec = Record {
                seq,
                nonce: nonce_bytes,
                sm,
                ct,
            };

            serialize_into(&mut out, &rec).expect("write record");
            offset = end;
        }

        out
    }

    /// Deterministic generator for a stable pseudo-random test input buffer.
    fn make_golden_input(len: usize) -> Vec<u8> {
        // Simple LCG to create repeatable "random-looking" bytes.
        let mut v = Vec::with_capacity(len);
        let (mut x, a, c, m) = (
            0x1234_5678u64,
            6364136223846793005u64,
            1442695040888963407u64,
            1u64 << 63,
        );
        for _ in 0..len {
            x = (a.wrapping_mul(x).wrapping_add(c)) % m;
            v.push(((x >> 24) & 0xFF) as u8);
        }
        v
    }

    // ----------------------------
    // The golden test
    // ----------------------------

    /// Test that the golden TRST digest is stable across runs.
    #[test]
    fn golden_trst_digest_is_stable() {
        // Choose a deterministic input and chunk size
        let input = make_golden_input(32_768); // 32 KB
        let chunk_size = 4096;

        // Build deterministic .trst bytes
        let trst = deterministic_trst(&input, chunk_size);

        // Compute digest and print it for first-time copy
        let digest_hex = hex::encode(blake3::hash(&trst).as_bytes());
        eprintln!("\n[GOLDEN] BLAKE3(trst) = {}\n", digest_hex);

        // If first run, this will fail: copy printed digest into GOLDEN_TRST_BLAKE3 and re-run.
        assert_eq!(
            digest_hex,
            GOLDEN_TRST_BLAKE3,
            "Golden mismatch.\n\
             NOTE: First time you run, copy the printed digest above into GOLDEN_TRST_BLAKE3 and commit.\n\
             If you intentionally changed format/crypto, update (rebase) the golden digest."
        );
    }
}
