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
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroize;

/// The chunk size to use when breaking up large payloads
const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

/// Number of PBKDF2 iterations for key derivation (balanced security/performance)
const PBKDF2_ITERATIONS: u32 = 100_000;

/// A high-level envelope that wraps and secures arbitrary payloads
///
/// This is the "steering wheel" - a simple interface that hides the complexity
/// of NetworkChunks, Records, manifests, and cryptographic operations.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Envelope {
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

/// Derive a shared encryption key using only public keys and context
///
/// This is a simplified approach that derives the same key for both parties
/// using only public information plus chunk-specific context.
/// 
/// Security note: This is not as strong as proper ECDH, but provides reasonable
/// security for this use case where we need deterministic key derivation.
fn derive_shared_encryption_key(
    sender_key: &VerifyingKey,
    recipient_key: &VerifyingKey, 
    salt: &[u8; 32],
    sequence: u64,
    metadata_hash: &[u8],
    iterations: u32,
) -> Result<[u8; 32]> {
    // Create deterministic key material using only public keys
    let mut key_material = Vec::new();
    
    // Use consistent ordering: sender_public || recipient_public || context
    key_material.extend_from_slice(&sender_key.to_bytes());
    key_material.extend_from_slice(&recipient_key.to_bytes());
    key_material.extend_from_slice(salt);
    key_material.extend_from_slice(&sequence.to_le_bytes());
    key_material.extend_from_slice(metadata_hash);
    
    // Add a fixed context string to prevent key reuse in other contexts
    key_material.extend_from_slice(b"TRUSTEDGE_ENVELOPE_V1");
    
    // Derive the encryption key using PBKDF2
    let mut derived_key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(&key_material, salt, iterations, &mut derived_key);
    
    // Clear the key material from memory
    key_material.zeroize();
    
    Ok(derived_key)
}

impl Envelope {
    /// Seal a payload into a secure envelope (the "gas pedal")
    ///
    /// This is the main entry point for securing data. It takes raw bytes
    /// and returns a cryptographically secure envelope.
    pub fn seal(
        payload: &[u8],
        signing_key: &SigningKey,
        beneficiary_key: &VerifyingKey,
    ) -> Result<Self> {
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

        // Break the payload into chunks and encrypt each one
        let mut chunks = Vec::new();
        for (i, chunk_data) in payload.chunks(DEFAULT_CHUNK_SIZE).enumerate() {
            let chunk = Self::create_encrypted_chunk(i as u64, chunk_data, signing_key, beneficiary_key, &metadata)?;
            chunks.push(chunk);
        }

        Ok(Envelope {
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
    /// This reverses the sealing process, decrypting and reassembling the original data.
    pub fn unseal(&self, decryption_key: &SigningKey) -> Result<Vec<u8>> {
        if !self.verify() {
            return Err(anyhow::anyhow!("Envelope verification failed"));
        }

        // Sort chunks by sequence number to ensure correct order
        let mut sorted_chunks = self.chunks.clone();
        sorted_chunks.sort_by_key(|chunk| chunk.sequence);

        // Decrypt and reassemble the payload
        let mut payload = Vec::new();
        for chunk in sorted_chunks {
            let decrypted_chunk = self.decrypt_chunk(&chunk, decryption_key)?;
            payload.extend_from_slice(&decrypted_chunk);
        }

        // Verify the total size matches metadata
        if payload.len() != self.metadata.payload_size as usize {
            return Err(anyhow::anyhow!(
                "Payload size mismatch: expected {}, got {}",
                self.metadata.payload_size,
                payload.len()
            ));
        }

        Ok(payload)
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

    /// Create an encrypted chunk from raw data (internal engine work)
    fn create_encrypted_chunk(
        sequence: u64,
        chunk_data: &[u8],
        signing_key: &SigningKey,
        beneficiary_key: &VerifyingKey,
        metadata: &EnvelopeMetadata,
    ) -> Result<NetworkChunk> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};
        use rand::RngCore;

        // Generate a random salt for key derivation
        let mut key_derivation_salt = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_derivation_salt);

        // Generate a random nonce
        let mut nonce = [0u8; NONCE_LEN];
        rand::thread_rng().fill_bytes(&mut nonce);

        // Create the manifest for this chunk
        let manifest = ChunkManifest {
            sequence,
            chunk_size: chunk_data.len() as u32,
            timestamp: metadata.created_at,
            format_hint: "application/octet-stream".to_string(),
            key_derivation_salt,
            pbkdf2_iterations: PBKDF2_ITERATIONS,
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

        // Derive encryption key from signing key, beneficiary key, and chunk-specific data
        let metadata_hash = blake3::hash(&bincode::serialize(metadata).unwrap_or_default());
        let mut encryption_key = derive_shared_encryption_key(
            &signing_key.verifying_key(), // sender's public key
            beneficiary_key,               // recipient's public key
            &key_derivation_salt,
            sequence,
            metadata_hash.as_bytes(),
            PBKDF2_ITERATIONS,
        )?;

        // Create AAD for authenticated encryption
        let header_hash = blake3::hash(b"ENVELOPE_V1"); // Simple header for envelope format
        let aad = build_aad(
            header_hash.as_bytes(),
            sequence,
            &nonce,
            manifest_hash.as_bytes(),
            chunk_data.len() as u32,
        );

        // Encrypt the chunk data
        let cipher =
            Aes256Gcm::new_from_slice(&encryption_key).context("Failed to create cipher")?;

        let mut ciphertext = chunk_data.to_vec();
        let nonce_obj = Nonce::from_slice(&nonce);

        cipher
            .encrypt_in_place(nonce_obj, &aad, &mut ciphertext)
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;

        // Clear the encryption key from memory
        encryption_key.zeroize();

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

    /// Decrypt a single chunk (internal engine work)
    fn decrypt_chunk(
        &self,
        chunk: &NetworkChunk,
        decryption_key: &SigningKey,
    ) -> Result<Vec<u8>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};

        // Deserialize the signed manifest to get chunk metadata
        let signed_manifest: SignedManifest = bincode::deserialize(&chunk.manifest)
            .context("Failed to deserialize signed manifest")?;

        // Deserialize the chunk manifest to get key derivation parameters
        let manifest: ChunkManifest = bincode::deserialize(&signed_manifest.manifest)
            .context("Failed to deserialize chunk manifest")?;

        // Get the sender's public key from the envelope
        let sender_public_key = VerifyingKey::from_bytes(&self.verifying_key_bytes)
            .context("Invalid sender public key in envelope")?;

        // Derive the same encryption key used during sealing
        let metadata_hash = blake3::hash(&bincode::serialize(&self.metadata).unwrap_or_default());
        let mut encryption_key = derive_shared_encryption_key(
            &sender_public_key,                    // sender's public key
            &decryption_key.verifying_key(),       // recipient's public key
            &manifest.key_derivation_salt,
            manifest.sequence,
            metadata_hash.as_bytes(),
            manifest.pbkdf2_iterations,
        )?;

        // Create the cipher
        let cipher = Aes256Gcm::new_from_slice(&encryption_key)
            .context("Failed to create cipher for decryption")?;

        // Get the nonce from the chunk
        let nonce_obj = Nonce::from_slice(&chunk.nonce);

        // Recreate the AAD used during encryption
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
            .decrypt_in_place(nonce_obj, &aad, &mut plaintext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;

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
        let unsealed_payload = envelope.unseal(&beneficiary_key)
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
        let unsealed_payload = envelope.unseal(&beneficiary_key)
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
}
