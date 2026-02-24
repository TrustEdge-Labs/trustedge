// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! High-level Envelope API - The "steering wheel" for TrustEdge cryptography
//!
//! This module provides a clean, simple interface over the complex NetworkChunk/Record system.
//! Think of it as the driver interface that hides the engine complexity.

use crate::format::{build_aad, AeadAlgorithm, HashAlgorithm, SignatureAlgorithm, SignedManifest};
use crate::{NetworkChunk, NONCE_LEN};
use anyhow::{Context, Result};
use blake3;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroize;

/// The chunk size to use when breaking up large payloads
const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

/// Default envelope version for serde deserialization of old v1 envelopes that lack the field.
fn default_envelope_version() -> u8 {
    1
}

/// A high-level envelope that wraps and secures arbitrary payloads
///
/// This is the "steering wheel" - a simple interface that hides the complexity
/// of NetworkChunks, Records, manifests, and cryptographic operations.
///
/// Format versions:
///   v1 — per-chunk random salt + HKDF derivation (one key per chunk)
///   v2 — single HKDF derivation per envelope with deterministic counter nonces
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Envelope {
    /// Envelope format version. v1 envelopes deserialize with default 1.
    #[serde(default = "default_envelope_version")]
    version: u8,
    /// Per-envelope random HKDF salt (32 bytes). v1 envelopes deserialize with [0; 32].
    #[serde(default)]
    hkdf_salt: [u8; 32],
    /// The collection of encrypted chunks that make up this envelope
    chunks: Vec<NetworkChunk>,
    /// The signing key used to create this envelope (for verification) - stored as bytes
    verifying_key_bytes: [u8; 32],
    /// The beneficiary public key (who this envelope is intended for) - stored as bytes
    beneficiary_key_bytes: [u8; 32],
    /// Optional metadata about the envelope
    metadata: EnvelopeMetadata,
}

/// Metadata associated with an envelope
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvelopeMetadata {
    /// Timestamp when the envelope was created
    pub created_at: u64,
    /// Total size of the original payload in bytes
    pub payload_size: u64,
    /// Number of chunks in this envelope
    pub chunk_count: u32,
    /// Algorithm used for encryption
    pub aead_algorithm: u8, // Using u8 for serialization simplicity
    /// Algorithm used for signatures
    pub signature_algorithm: u8,
    /// Algorithm used for hashing
    pub hash_algorithm: u8,
}

/// Derive shared encryption key material via X25519 ECDH key agreement and HKDF-SHA256.
///
/// Converts Ed25519 keys to X25519 using the standard conversion path
/// documented by `ed25519-dalek`: `SigningKey::to_scalar_bytes()` →
/// `x25519_dalek::StaticSecret`, and `VerifyingKey::to_montgomery()` →
/// `x25519_dalek::PublicKey`. The raw ECDH shared secret is fed as IKM into
/// HKDF-Extract (RFC 5869), then HKDF-Expand derives 40 bytes of output key material:
///   - bytes 0..32 → AES-256-GCM encryption key
///   - bytes 32..40 → 8-byte nonce prefix for deterministic per-chunk nonce construction
///
/// DH commutativity guarantees both sides derive the same key:
///   sender_secret.diffie_hellman(recipient_pub) == recipient_secret.diffie_hellman(sender_pub)
///
/// Returns `(encryption_key, nonce_prefix)`.
fn derive_shared_encryption_key(
    my_private_key: &SigningKey,
    their_public_key: &VerifyingKey,
    salt: &[u8; 32],
) -> Result<([u8; 32], [u8; 8])> {
    // Convert Ed25519 keys to X25519 using the standard conversion path
    let x25519_secret = x25519_dalek::StaticSecret::from(my_private_key.to_scalar_bytes());
    let x25519_public = x25519_dalek::PublicKey::from(their_public_key.to_montgomery().to_bytes());

    // Standard X25519 Diffie-Hellman key agreement
    let shared_secret = x25519_secret.diffie_hellman(&x25519_public);

    // Reject low-order points (all-zero shared secret = contributory behavior failure)
    if shared_secret.as_bytes().iter().all(|&b| b == 0) {
        return Err(anyhow::anyhow!("ECDH produced zero shared secret"));
    }

    // HKDF-Extract: extract pseudorandom key from ECDH shared secret
    // Salt provides randomness; IKM is the raw ECDH output (NOT concatenated with other data)
    let hkdf = Hkdf::<Sha256>::new(Some(salt), shared_secret.as_bytes());

    // HKDF-Expand: derive 40 bytes of output key material with domain separation.
    // The info parameter binds the derived key to the TrustEdge envelope v2 context.
    // Layout: bytes 0..32 = AES-256-GCM encryption key, bytes 32..40 = 8-byte nonce prefix.
    let info = b"TRUSTEDGE_ENVELOPE_V1";
    let mut okm = [0u8; 40];
    hkdf.expand(info, &mut okm)
        .map_err(|_| anyhow::anyhow!("HKDF expand failed"))?;

    let mut encryption_key = [0u8; 32];
    let mut nonce_prefix = [0u8; 8];
    encryption_key.copy_from_slice(&okm[0..32]);
    nonce_prefix.copy_from_slice(&okm[32..40]);

    // Zeroize the full OKM buffer before returning split copies
    okm.zeroize();

    Ok((encryption_key, nonce_prefix))
}

impl Envelope {
    /// Seal a payload into a secure envelope (the "gas pedal")
    ///
    /// This is the main entry point for securing data. It takes raw bytes
    /// and returns a cryptographically secure v2 envelope.
    ///
    /// v2 seal flow:
    ///   1. Generate a random 32-byte `hkdf_salt` once for the entire envelope.
    ///   2. Derive `(encryption_key, nonce_prefix)` via a single HKDF call.
    ///   3. Encrypt each chunk with the shared key and a deterministic counter nonce:
    ///      `nonce = nonce_prefix[0..8] || chunk_index[1..4] (BE) || last_flag`
    ///   4. Zeroize the encryption key after the chunk loop.
    pub fn seal(
        payload: &[u8],
        signing_key: &SigningKey,
        beneficiary_key: &VerifyingKey,
    ) -> Result<Self> {
        use rand::RngCore;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("Failed to get current timestamp")?
            .as_secs();

        // Calculate how many chunks we'll need
        let chunk_count = payload.len().div_ceil(DEFAULT_CHUNK_SIZE) as u32;

        let metadata = EnvelopeMetadata {
            created_at: timestamp,
            payload_size: payload.len() as u64,
            chunk_count,
            aead_algorithm: AeadAlgorithm::Aes256Gcm as u8,
            signature_algorithm: SignatureAlgorithm::Ed25519 as u8,
            hash_algorithm: HashAlgorithm::Blake3 as u8,
        };

        // Generate per-envelope random HKDF salt once
        let mut hkdf_salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut hkdf_salt);

        // Derive key material once for the entire envelope (v2 path)
        let (mut encryption_key, nonce_prefix) =
            derive_shared_encryption_key(signing_key, beneficiary_key, &hkdf_salt)?;

        // Chunk count for last-chunk detection
        let total_chunks = payload.chunks(DEFAULT_CHUNK_SIZE).count();

        // Break the payload into chunks and encrypt each one
        let mut chunks = Vec::new();
        for (i, chunk_data) in payload.chunks(DEFAULT_CHUNK_SIZE).enumerate() {
            let is_last = i == total_chunks - 1;
            let chunk = Self::create_encrypted_chunk(
                i as u64,
                chunk_data,
                signing_key,
                &encryption_key,
                &nonce_prefix,
                is_last,
                &metadata,
            )?;
            chunks.push(chunk);
        }

        // Zeroize the envelope-level encryption key after all chunks are sealed
        encryption_key.zeroize();

        Ok(Envelope {
            version: 2,
            hkdf_salt,
            chunks,
            verifying_key_bytes: signing_key.verifying_key().to_bytes(),
            beneficiary_key_bytes: beneficiary_key.to_bytes(),
            metadata,
        })
    }

    /// Verify the envelope's cryptographic integrity (the "security check")
    ///
    /// This validates all signatures and ensures the envelope hasn't been tampered with.
    pub fn verify(&self) -> bool {
        // Verify each chunk's signature
        for chunk in &self.chunks {
            if !self.verify_chunk_signature(chunk) {
                return false;
            }
        }

        // Verify chunk sequence is complete and ordered
        if !self.verify_chunk_sequence() {
            return false;
        }

        true
    }

    /// Unseal the envelope to recover the original payload (the "unlock")
    ///
    /// Attempts v2 decryption first (HKDF-once + deterministic nonce reconstruction).
    /// If any chunk fails v2 decryption, falls back to v1 per-chunk salt path.
    /// This try-then-fallback approach handles both format versions without relying
    /// solely on the version field, making it more resilient to partially-migrated data.
    pub fn unseal(&self, decryption_key: &SigningKey) -> Result<Vec<u8>> {
        if !self.verify() {
            return Err(anyhow::anyhow!("Envelope verification failed"));
        }

        // Sort chunks by sequence number to ensure correct order
        let mut sorted_chunks = self.chunks.clone();
        sorted_chunks.sort_by_key(|chunk| chunk.sequence);

        let total_chunks = sorted_chunks.len();

        // Get the sender's public key from the envelope
        let sender_public_key = VerifyingKey::from_bytes(&self.verifying_key_bytes)
            .context("Invalid sender public key in envelope")?;

        // --- V2 path: derive key material once, reconstruct deterministic nonces ---
        let (mut encryption_key, nonce_prefix) =
            derive_shared_encryption_key(decryption_key, &sender_public_key, &self.hkdf_salt)?;

        let mut v2_result: Result<Vec<u8>> = (|| {
            let mut payload = Vec::new();
            for (i, chunk) in sorted_chunks.iter().enumerate() {
                let is_last = i == total_chunks - 1;
                let decrypted =
                    self.decrypt_chunk_v2(chunk, &encryption_key, &nonce_prefix, is_last)?;
                payload.extend_from_slice(&decrypted);
            }
            Ok(payload)
        })();

        // Zeroize the v2 encryption key regardless of outcome
        encryption_key.zeroize();

        // On v2 success, verify payload size and return
        if let Ok(ref payload) = v2_result {
            if payload.len() != self.metadata.payload_size as usize {
                v2_result = Err(anyhow::anyhow!(
                    "Payload size mismatch: expected {}, got {}",
                    self.metadata.payload_size,
                    payload.len()
                ));
            }
        }

        if v2_result.is_ok() {
            return v2_result;
        }

        // --- V1 fallback path: per-chunk salt derivation ---
        let mut v1_payload = Vec::new();
        for chunk in &sorted_chunks {
            let decrypted = self.decrypt_chunk_v1(chunk, decryption_key)?;
            v1_payload.extend_from_slice(&decrypted);
        }

        // Verify the total size matches metadata
        if v1_payload.len() != self.metadata.payload_size as usize {
            return Err(anyhow::anyhow!(
                "Payload size mismatch: expected {}, got {}",
                self.metadata.payload_size,
                v1_payload.len()
            ));
        }

        Ok(v1_payload)
    }

    /// Get the hash of this envelope for chaining purposes
    pub fn hash(&self) -> [u8; 32] {
        let envelope_bytes = bincode::serialize(self).unwrap_or_default();
        *blake3::hash(&envelope_bytes).as_bytes()
    }

    /// Get the beneficiary public key
    pub fn beneficiary(&self) -> VerifyingKey {
        VerifyingKey::from_bytes(&self.beneficiary_key_bytes)
            .expect("Invalid beneficiary key bytes")
    }

    /// Get the envelope metadata
    pub fn metadata(&self) -> &EnvelopeMetadata {
        &self.metadata
    }

    /// Get the issuer's verifying key
    pub fn issuer(&self) -> VerifyingKey {
        VerifyingKey::from_bytes(&self.verifying_key_bytes).expect("Invalid issuer key bytes")
    }

    // Private helper methods for the complex crypto operations

    /// Create an encrypted chunk from raw data (v2 path — deterministic nonce)
    ///
    /// The encryption key and nonce prefix are derived once at seal level and passed in.
    /// Per-chunk nonce: `nonce_prefix[0..8] || chunk_index[1..4] (BE u32) || last_flag`
    ///
    /// ChunkManifest fields `key_derivation_salt` and `pbkdf2_iterations` are zeroed for
    /// v2 envelopes — kept for serde compatibility, not used for decryption.
    #[allow(clippy::too_many_arguments)]
    fn create_encrypted_chunk(
        sequence: u64,
        chunk_data: &[u8],
        signing_key: &SigningKey,
        encryption_key: &[u8; 32],
        nonce_prefix: &[u8; 8],
        is_last_chunk: bool,
        metadata: &EnvelopeMetadata,
    ) -> Result<NetworkChunk> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};

        // Construct deterministic 12-byte nonce:
        //   bytes 0..8  = nonce_prefix (8 bytes from HKDF output)
        //   bytes 8..11 = low 3 bytes of chunk index as BE u32
        //   byte 11     = last-chunk flag (0xFF if last, 0x00 otherwise)
        let mut nonce = [0u8; NONCE_LEN];
        nonce[0..8].copy_from_slice(nonce_prefix);
        let idx_be = (sequence as u32).to_be_bytes();
        nonce[8..11].copy_from_slice(&idx_be[1..4]); // low 3 bytes of u32 BE
        nonce[11] = if is_last_chunk { 0xFF } else { 0x00 };

        // Create the v2 manifest — key_derivation_salt and pbkdf2_iterations zeroed.
        // Fields are kept for serde compat with ChunkManifest; not used by v2 decrypt path.
        let manifest = ChunkManifest {
            sequence,
            chunk_size: chunk_data.len() as u32,
            timestamp: metadata.created_at,
            format_hint: "application/octet-stream".to_string(),
            key_derivation_salt: [0u8; 32],
            pbkdf2_iterations: 0u32,
        };

        let manifest_bytes =
            bincode::serialize(&manifest).context("Failed to serialize manifest")?;

        // Create signed manifest
        let manifest_hash = blake3::hash(&manifest_bytes);
        let manifest_signature = signing_key.sign(manifest_hash.as_bytes());

        let signed_manifest = SignedManifest {
            manifest: manifest_bytes,
            sig: manifest_signature.to_bytes().to_vec(),
            pubkey: signing_key.verifying_key().to_bytes().to_vec(),
        };

        // Create AAD for authenticated encryption
        let header_hash = blake3::hash(b"ENVELOPE_V1");
        let aad = build_aad(
            header_hash.as_bytes(),
            sequence,
            &nonce,
            manifest_hash.as_bytes(),
            chunk_data.len() as u32,
        );

        // Encrypt the chunk data using the envelope-level key
        let cipher =
            Aes256Gcm::new_from_slice(encryption_key).context("Failed to create cipher")?;

        let mut ciphertext = chunk_data.to_vec();
        let nonce_array: &[u8; 12] = nonce
            .as_slice()
            .try_into()
            .context("Nonce conversion failed")?;

        cipher
            .encrypt_in_place(nonce_array.into(), &aad, &mut ciphertext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

        // Create the network chunk
        let signed_manifest_bytes =
            bincode::serialize(&signed_manifest).context("Failed to serialize signed manifest")?;

        Ok(NetworkChunk::new_with_nonce(
            sequence,
            ciphertext,
            signed_manifest_bytes,
            nonce,
        ))
    }

    /// Verify a chunk's cryptographic signature
    fn verify_chunk_signature(&self, chunk: &NetworkChunk) -> bool {
        // Deserialize the signed manifest
        let signed_manifest: SignedManifest = match bincode::deserialize(&chunk.manifest) {
            Ok(sm) => sm,
            Err(_) => return false,
        };

        // Verify the manifest signature
        let manifest_hash = blake3::hash(&signed_manifest.manifest);

        // Convert signature bytes to Signature
        let signature_bytes: [u8; 64] = match signed_manifest.sig.try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = match Signature::try_from(signature_bytes.as_slice()) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        // Get the verifying key from the envelope (not from manifest for consistency)
        let verifying_key = match VerifyingKey::from_bytes(&self.verifying_key_bytes) {
            Ok(key) => key,
            Err(_) => return false,
        };

        verifying_key
            .verify(manifest_hash.as_bytes(), &signature)
            .is_ok()
    }

    /// Verify that all chunks are present and in sequence
    fn verify_chunk_sequence(&self) -> bool {
        if self.chunks.len() != self.metadata.chunk_count as usize {
            return false;
        }

        // Check that sequence numbers are 0..chunk_count-1
        let mut sequences: Vec<u64> = self.chunks.iter().map(|c| c.sequence).collect();
        sequences.sort();

        for (expected, actual) in (0..self.metadata.chunk_count as u64).zip(sequences.iter()) {
            if expected != *actual {
                return false;
            }
        }

        true
    }

    /// Decrypt a single chunk using the v2 path (HKDF-once + deterministic nonce reconstruction).
    ///
    /// The `encryption_key` and `nonce_prefix` are derived once at the envelope level and passed in.
    /// The deterministic nonce is reconstructed as:
    ///   `nonce_prefix[0..8] || chunk_index[1..4] (BE u32 low 3 bytes) || last_flag`
    fn decrypt_chunk_v2(
        &self,
        chunk: &NetworkChunk,
        encryption_key: &[u8; 32],
        nonce_prefix: &[u8; 8],
        is_last_chunk: bool,
    ) -> Result<Vec<u8>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};

        // Deserialize the signed manifest to get chunk metadata
        let signed_manifest: SignedManifest = bincode::deserialize(&chunk.manifest)
            .context("Failed to deserialize signed manifest")?;

        let manifest: ChunkManifest = bincode::deserialize(&signed_manifest.manifest)
            .context("Failed to deserialize chunk manifest")?;

        // Reconstruct the deterministic 12-byte nonce (must match create_encrypted_chunk)
        let mut nonce = [0u8; NONCE_LEN];
        nonce[0..8].copy_from_slice(nonce_prefix);
        let idx_be = (manifest.sequence as u32).to_be_bytes();
        nonce[8..11].copy_from_slice(&idx_be[1..4]); // low 3 bytes of BE u32
        nonce[11] = if is_last_chunk { 0xFF } else { 0x00 };

        // Recreate the AAD used during encryption
        let header_hash = blake3::hash(b"ENVELOPE_V1");
        let manifest_hash = blake3::hash(&signed_manifest.manifest);
        let aad = build_aad(
            header_hash.as_bytes(),
            manifest.sequence,
            &nonce,
            manifest_hash.as_bytes(),
            manifest.chunk_size,
        );

        // Create the cipher and decrypt
        let cipher = Aes256Gcm::new_from_slice(encryption_key)
            .context("Failed to create cipher for v2 decryption")?;

        let nonce_array: &[u8; 12] = nonce
            .as_slice()
            .try_into()
            .context("Nonce conversion failed")?;

        let mut plaintext = chunk.data.clone();
        cipher
            .decrypt_in_place(nonce_array.into(), &aad, &mut plaintext)
            .map_err(|e| anyhow::anyhow!("V2 decryption failed: {:?}", e))?;

        Ok(plaintext)
    }

    /// Decrypt a single chunk using the v1 path (per-chunk random salt + stored random nonce).
    ///
    /// This is the legacy backward-compatible path. Each chunk stores its own
    /// `key_derivation_salt` in the ChunkManifest, and the nonce is read directly
    /// from the NetworkChunk (not reconstructed deterministically).
    fn decrypt_chunk_v1(
        &self,
        chunk: &NetworkChunk,
        decryption_key: &SigningKey,
    ) -> Result<Vec<u8>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};

        // Deserialize the signed manifest to get chunk metadata
        let signed_manifest: SignedManifest = bincode::deserialize(&chunk.manifest)
            .context("Failed to deserialize signed manifest")?;

        let manifest: ChunkManifest = bincode::deserialize(&signed_manifest.manifest)
            .context("Failed to deserialize chunk manifest")?;

        // Get the sender's public key from the envelope
        let sender_public_key = VerifyingKey::from_bytes(&self.verifying_key_bytes)
            .context("Invalid sender public key in envelope")?;

        // V1 path: derive per-chunk key using the per-chunk salt stored in ChunkManifest.
        // Only the encryption_key (first element) is used; nonce comes from the stored chunk nonce.
        let (mut encryption_key, _nonce_prefix) = derive_shared_encryption_key(
            decryption_key,
            &sender_public_key,
            &manifest.key_derivation_salt,
        )?;

        // Create the cipher
        let cipher = Aes256Gcm::new_from_slice(&encryption_key)
            .context("Failed to create cipher for v1 decryption")?;

        // Get the nonce from the chunk (stored as a random nonce in v1)
        let nonce_array: &[u8; 12] = chunk
            .nonce
            .as_slice()
            .try_into()
            .context("Nonce conversion failed")?;

        // Recreate the AAD used during v1 encryption
        let header_hash = blake3::hash(b"ENVELOPE_V1");
        let manifest_hash = blake3::hash(&signed_manifest.manifest);
        let aad = build_aad(
            header_hash.as_bytes(),
            manifest.sequence,
            &chunk.nonce,
            manifest_hash.as_bytes(),
            manifest.chunk_size,
        );

        // Decrypt the chunk data
        let mut plaintext = chunk.data.clone();
        cipher
            .decrypt_in_place(nonce_array.into(), &aad, &mut plaintext)
            .map_err(|e| anyhow::anyhow!("V1 decryption failed: {:?}", e))?;

        // Clear the encryption key from memory
        encryption_key.zeroize();

        Ok(plaintext)
    }
}

/// Manifest for a single chunk within an envelope
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChunkManifest {
    /// Sequence number of this chunk
    sequence: u64,
    /// Size of the chunk data in bytes
    chunk_size: u32,
    /// Timestamp when this chunk was created
    timestamp: u64,
    /// MIME type hint for the data
    format_hint: String,
    /// Salt used for key derivation (32 bytes)
    key_derivation_salt: [u8; 32],
    /// Number of PBKDF2 iterations used
    pbkdf2_iterations: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_envelope_creation() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        let payload = b"Hello, secure world!";

        let envelope = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to create envelope");

        assert_eq!(envelope.metadata.payload_size, payload.len() as u64);
        assert_eq!(envelope.metadata.chunk_count, 1);
        assert!(envelope.verify());
    }

    #[test]
    fn test_envelope_verification() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        let payload = b"Test data for verification";

        let envelope = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to create envelope");

        assert!(envelope.verify());

        // Test that we can access metadata
        assert_eq!(envelope.beneficiary(), beneficiary_key.verifying_key());
        assert_eq!(envelope.issuer(), signing_key.verifying_key());
    }

    #[test]
    fn test_large_payload_chunking() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // Create a payload larger than default chunk size
        let large_payload = vec![0u8; DEFAULT_CHUNK_SIZE * 3 + 1000];

        let envelope = Envelope::seal(
            &large_payload,
            &signing_key,
            &beneficiary_key.verifying_key(),
        )
        .expect("Failed to create envelope");

        assert_eq!(envelope.metadata.payload_size, large_payload.len() as u64);
        assert_eq!(envelope.metadata.chunk_count, 4); // 3 full chunks + 1 partial
        assert!(envelope.verify());
    }

    #[test]
    fn test_envelope_seal_unseal_roundtrip() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        let original_payload = b"This is a test payload for seal/unseal roundtrip testing";

        // Seal the payload
        let envelope = Envelope::seal(
            original_payload,
            &signing_key,
            &beneficiary_key.verifying_key(),
        )
        .expect("Failed to seal envelope");

        // Verify the envelope
        assert!(envelope.verify());

        // Unseal the payload
        let unsealed_payload = envelope
            .unseal(&beneficiary_key)
            .expect("Failed to unseal envelope");

        // Verify the payload is identical
        assert_eq!(original_payload, unsealed_payload.as_slice());
    }

    #[test]
    fn test_envelope_large_payload_roundtrip() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // Create a large payload with pattern for verification
        let mut large_payload = Vec::new();
        for i in 0..100000 {
            large_payload.push((i % 256) as u8);
        }

        // Seal the payload
        let envelope = Envelope::seal(
            &large_payload,
            &signing_key,
            &beneficiary_key.verifying_key(),
        )
        .expect("Failed to seal large envelope");

        // Verify the envelope
        assert!(envelope.verify());
        assert!(envelope.metadata.chunk_count > 1); // Should be chunked

        // Unseal the payload
        let unsealed_payload = envelope
            .unseal(&beneficiary_key)
            .expect("Failed to unseal large envelope");

        // Verify the payload is identical
        assert_eq!(large_payload, unsealed_payload);
    }

    #[test]
    fn test_envelope_wrong_key_fails() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);
        let wrong_key = SigningKey::generate(&mut OsRng);

        let payload = b"Secret message";

        let envelope = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal envelope");

        // Correct key should work
        assert!(envelope.unseal(&beneficiary_key).is_ok());

        // Wrong key should fail
        assert!(envelope.unseal(&wrong_key).is_err());

        // Signing key should also fail (issuer != beneficiary)
        assert!(envelope.unseal(&signing_key).is_err());
    }

    #[test]
    fn test_third_party_cannot_derive_key() {
        let sender = SigningKey::generate(&mut OsRng);
        let recipient = SigningKey::generate(&mut OsRng);
        let payload = b"confidential data";

        let envelope =
            Envelope::seal(payload, &sender, &recipient.verifying_key()).expect("Failed to seal");

        // A third party who knows both public keys cannot unseal
        let attacker = SigningKey::generate(&mut OsRng);
        assert!(
            envelope.unseal(&attacker).is_err(),
            "Third party should not be able to decrypt"
        );

        // Only the intended recipient can unseal
        let recovered = envelope
            .unseal(&recipient)
            .expect("Recipient should unseal");
        assert_eq!(recovered.as_slice(), payload);
    }

    #[test]
    fn test_envelope_hash_consistency() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        let payload = b"Test payload for hash consistency";

        let envelope1 = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal envelope1");

        let envelope2 = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal envelope2");

        // Different envelopes should have different hashes (due to random nonces)
        assert_ne!(envelope1.hash(), envelope2.hash());

        // Same envelope should have consistent hash
        let hash1 = envelope1.hash();
        let hash2 = envelope1.hash();
        assert_eq!(hash1, hash2);
    }

    // --- New tests for Plan 02: v2 decrypt path and v1 fallback ---

    #[test]
    fn test_v2_multi_chunk_roundtrip() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // Payload larger than DEFAULT_CHUNK_SIZE to produce 4 chunks
        let payload: Vec<u8> = (0..(DEFAULT_CHUNK_SIZE * 3 + 500))
            .map(|i| (i % 251) as u8)
            .collect();

        let envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal multi-chunk v2 envelope");

        // Assert v2 format fields
        assert_eq!(envelope.version, 2, "Sealed envelope must be version 2");
        assert_ne!(
            envelope.hkdf_salt, [0u8; 32],
            "hkdf_salt must be non-zero (randomly generated)"
        );
        assert_eq!(
            envelope.metadata.chunk_count, 4,
            "Expected 4 chunks for payload of 3*DEFAULT_CHUNK_SIZE+500"
        );

        // Verify and unseal
        assert!(envelope.verify(), "Envelope signature verification failed");
        let recovered = envelope
            .unseal(&beneficiary_key)
            .expect("Failed to unseal multi-chunk v2 envelope");

        assert_eq!(
            payload, recovered,
            "Recovered payload must be byte-for-byte identical"
        );
    }

    #[test]
    fn test_v2_single_chunk_roundtrip() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // Small payload that fits in a single chunk
        let payload = b"single chunk v2 test payload";

        let envelope = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal single-chunk v2 envelope");

        assert_eq!(envelope.version, 2);
        assert_eq!(envelope.metadata.chunk_count, 1);

        assert!(envelope.verify());
        let recovered = envelope
            .unseal(&beneficiary_key)
            .expect("Failed to unseal single-chunk v2 envelope");

        assert_eq!(payload.as_ref(), recovered.as_slice());
    }

    #[test]
    fn test_v2_envelope_version_field() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        let payload = b"version field serialization test";

        let envelope = Envelope::seal(payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal envelope");

        // Assert version field is 2
        assert_eq!(envelope.version, 2);

        // Serialize to JSON and check the field is present
        let json = serde_json::to_string(&envelope).expect("Failed to serialize envelope to JSON");
        assert!(
            json.contains("\"version\":2"),
            "JSON must contain \"version\":2, got: {}",
            &json[..json.len().min(200)]
        );

        // Deserialize back and verify version is preserved
        let deserialized: Envelope =
            serde_json::from_str(&json).expect("Failed to deserialize envelope from JSON");
        assert_eq!(
            deserialized.version, 2,
            "Version must be preserved through serialization roundtrip"
        );
    }

    #[test]
    fn test_v2_deterministic_nonces_are_unique() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // At least 3 chunks
        let payload: Vec<u8> = (0..(DEFAULT_CHUNK_SIZE * 3 + 100))
            .map(|i| (i % 127) as u8)
            .collect();

        let envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal multi-chunk envelope");

        assert!(envelope.chunks.len() >= 3, "Need at least 3 chunks");

        // Collect all nonces
        let nonces: Vec<[u8; 12]> = envelope
            .chunks
            .iter()
            .map(|c| {
                let arr: [u8; 12] = c
                    .nonce
                    .as_slice()
                    .try_into()
                    .expect("Nonce must be 12 bytes");
                arr
            })
            .collect();

        // All nonces must be distinct (deterministic counter produces unique values)
        for i in 0..nonces.len() {
            for j in (i + 1)..nonces.len() {
                assert_ne!(
                    nonces[i], nonces[j],
                    "Nonces at positions {} and {} must be distinct",
                    i, j
                );
            }
        }

        // All nonces must share the same 8-byte prefix (deterministic, not random)
        let prefix = &nonces[0][0..8];
        for (idx, nonce) in nonces.iter().enumerate() {
            assert_eq!(
                &nonce[0..8],
                prefix,
                "Nonce at position {} must share the same 8-byte prefix",
                idx
            );
        }
    }

    #[test]
    fn test_v1_legacy_fallback() {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit};
        use rand::RngCore;

        let signing_key = SigningKey::generate(&mut OsRng);
        let beneficiary_key = SigningKey::generate(&mut OsRng);

        // Payload with 2 chunks to exercise multi-chunk v1 fallback
        let payload: Vec<u8> = (0..(DEFAULT_CHUNK_SIZE + 500))
            .map(|i| (i % 199) as u8)
            .collect();

        // First, seal normally to get a v2 envelope (for key/metadata scaffolding)
        let v2_envelope = Envelope::seal(&payload, &signing_key, &beneficiary_key.verifying_key())
            .expect("Failed to seal v2 envelope for mutation");

        // Now manually rebuild as a v1 envelope by re-encrypting each chunk with
        // per-chunk random salts and random nonces, simulating legacy v1 format.
        let mut v1_chunks = Vec::new();
        for chunk_data_slice in payload.chunks(DEFAULT_CHUNK_SIZE) {
            let sequence = v1_chunks.len() as u64;

            // Generate per-chunk random salt (v1 style)
            let mut per_chunk_salt = [0u8; 32];
            OsRng.fill_bytes(&mut per_chunk_salt);

            // Derive key using per-chunk salt (v1 path: recipient_key + sender_pubkey + salt)
            let sender_vk = signing_key.verifying_key();
            let (mut enc_key, _) =
                derive_shared_encryption_key(&beneficiary_key, &sender_vk, &per_chunk_salt)
                    .expect("Failed to derive v1 per-chunk key");

            // Generate random nonce (v1 style)
            let mut nonce = [0u8; NONCE_LEN];
            OsRng.fill_bytes(&mut nonce);

            // Build ChunkManifest with per-chunk salt and nonzero pbkdf2_iterations
            let manifest = ChunkManifest {
                sequence,
                chunk_size: chunk_data_slice.len() as u32,
                timestamp: v2_envelope.metadata.created_at,
                format_hint: "application/octet-stream".to_string(),
                key_derivation_salt: per_chunk_salt,
                pbkdf2_iterations: 1u32, // nonzero signals v1 format
            };

            let manifest_bytes =
                bincode::serialize(&manifest).expect("Failed to serialize v1 manifest");
            let manifest_hash = blake3::hash(&manifest_bytes);
            let manifest_signature = signing_key.sign(manifest_hash.as_bytes());

            let signed_manifest = SignedManifest {
                manifest: manifest_bytes,
                sig: manifest_signature.to_bytes().to_vec(),
                pubkey: signing_key.verifying_key().to_bytes().to_vec(),
            };

            // Build AAD with the random nonce (v1 style — nonce stored, not reconstructed)
            let header_hash = blake3::hash(b"ENVELOPE_V1");
            let aad = build_aad(
                header_hash.as_bytes(),
                sequence,
                &nonce,
                manifest_hash.as_bytes(),
                chunk_data_slice.len() as u32,
            );

            // Encrypt with v1 per-chunk key
            let cipher = Aes256Gcm::new_from_slice(&enc_key).expect("Failed to create v1 cipher");
            let mut ciphertext = chunk_data_slice.to_vec();
            let nonce_arr: &[u8; 12] = nonce.as_slice().try_into().unwrap();
            cipher
                .encrypt_in_place(nonce_arr.into(), &aad, &mut ciphertext)
                .expect("Failed to encrypt v1 chunk");

            enc_key.zeroize();

            let signed_manifest_bytes =
                bincode::serialize(&signed_manifest).expect("Failed to serialize signed manifest");

            let network_chunk =
                NetworkChunk::new_with_nonce(sequence, ciphertext, signed_manifest_bytes, nonce);
            v1_chunks.push(network_chunk);
        }

        // Build v1 Envelope by mutating the v2 one: set version=1, hkdf_salt=[0;32], replace chunks
        let v1_envelope = Envelope {
            version: 1,
            hkdf_salt: [0u8; 32],
            chunks: v1_chunks,
            verifying_key_bytes: v2_envelope.verifying_key_bytes,
            beneficiary_key_bytes: v2_envelope.beneficiary_key_bytes,
            metadata: v2_envelope.metadata.clone(),
        };

        // unseal() must fall back to v1 path and return correct plaintext
        assert!(
            v1_envelope.verify(),
            "V1 envelope must pass signature verification"
        );
        let recovered = v1_envelope
            .unseal(&beneficiary_key)
            .expect("V1 legacy fallback must successfully decrypt");

        assert_eq!(
            payload, recovered,
            "V1 fallback must recover payload byte-for-byte"
        );
    }
}
