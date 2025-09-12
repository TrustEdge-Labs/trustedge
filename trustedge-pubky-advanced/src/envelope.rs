// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Envelope V2 - Hybrid Encryption with Pubky Integration
//!
//! This module implements the next generation of TrustEdge envelopes with:
//! - Hybrid encryption (X25519 ECDH + AES-256-GCM)
//! - Pubky integration for decentralized key discovery
//! - Dual key architecture (Ed25519 identity + X25519 encryption)

use crate::keys::DualKeyPair;
use anyhow::{Context, Result};
use blake3;
use ed25519_dalek::{Signature, Signer, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use trustedge_core::{format::AeadAlgorithm, NetworkChunk, NONCE_LEN};
use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};
use zeroize::Zeroize;

/// Magic number identifying v2 TrustEdge files
const ENVELOPE_V2_MAGIC: [u8; 4] = *b"TRS2";

/// Version number for v2 envelopes
const ENVELOPE_V2_VERSION: u8 = 2;

/// The chunk size to use when breaking up large payloads
const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64KB chunks

/// Key exchange algorithms supported
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[repr(u8)]
pub enum KeyExchangeAlgorithm {
    X25519Ecdh = 1,
}

/// V2 Envelope with hybrid encryption
///
/// Structure:
/// [ Envelope Header ]
/// [ Encrypted Session Key ]
/// [ Encrypted Payload (NetworkChunks) ]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvelopeV2 {
    /// Header containing metadata and recipient info
    pub header: EnvelopeHeaderV2,
    /// Session key encrypted with recipient's X25519 public key
    pub encrypted_session_key: Vec<u8>,
    /// The encrypted payload chunks
    pub chunks: Vec<NetworkChunk>,
    /// Ed25519 signature over the entire envelope
    pub signature: Vec<u8>,
}

/// Header for V2 envelopes
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvelopeHeaderV2 {
    /// Magic number identifying this as a TrustEdge v2 file
    pub magic: [u8; 4],
    /// Version number (2)
    pub version: u8,
    /// Pubky address or X25519 public key of the recipient
    pub recipient_pubkey_id: String,
    /// Key exchange algorithm used
    pub key_exchange_algorithm: KeyExchangeAlgorithm,
    /// Sender's Ed25519 public key (for signature verification)
    pub sender_ed25519_pubkey: [u8; 32],
    /// Sender's ephemeral X25519 public key (for ECDH)
    pub sender_x25519_pubkey: [u8; 32],
    /// Timestamp when envelope was created
    pub created_at: u64,
    /// Total size of the original payload
    pub payload_size: u64,
    /// Number of chunks in the payload
    pub chunk_count: u32,
    /// Algorithm used for payload encryption (as u8 for serialization)
    pub aead_algorithm: u8,
}

impl EnvelopeV2 {
    /// Seal a payload using hybrid encryption
    ///
    /// This creates a v2 envelope with:
    /// 1. Random AES-256 session key for payload encryption
    /// 2. X25519 ECDH to encrypt the session key
    /// 3. Ed25519 signature for authenticity
    pub fn seal(
        payload: &[u8],
        sender_keys: &DualKeyPair,
        recipient_x25519_pubkey: &X25519PublicKey,
        recipient_pubky_id: &str,
    ) -> Result<Self> {
        use rand::{rngs::OsRng, RngCore};

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("Failed to get current timestamp")?
            .as_secs();

        // Generate ephemeral X25519 key for this envelope
        let ephemeral_secret = EphemeralSecret::random_from_rng(OsRng);
        let ephemeral_public = X25519PublicKey::from(&ephemeral_secret);

        // Generate random session key for AES-256-GCM
        let mut session_key = [0u8; 32];
        OsRng.fill_bytes(&mut session_key);

        // Perform X25519 ECDH to get shared secret
        let shared_secret = ephemeral_secret.diffie_hellman(recipient_x25519_pubkey);

        // Derive encryption key for the session key using HKDF
        let session_key_encryption_key = Self::derive_session_key_encryption_key(
            shared_secret.as_bytes(),
            &ephemeral_public.to_bytes(),
            &recipient_x25519_pubkey.to_bytes(),
        )?;

        // Encrypt the session key
        let encrypted_session_key = Self::encrypt_session_key(
            &session_key,
            &session_key_encryption_key,
        )?;

        // Calculate chunk count
        let chunk_count = payload.len().div_ceil(DEFAULT_CHUNK_SIZE) as u32;

        // Create header
        let header = EnvelopeHeaderV2 {
            magic: ENVELOPE_V2_MAGIC,
            version: ENVELOPE_V2_VERSION,
            recipient_pubkey_id: recipient_pubky_id.to_string(),
            key_exchange_algorithm: KeyExchangeAlgorithm::X25519Ecdh,
            sender_ed25519_pubkey: sender_keys.ed25519_public().to_bytes(),
            sender_x25519_pubkey: ephemeral_public.to_bytes(),
            created_at: timestamp,
            payload_size: payload.len() as u64,
            chunk_count,
            aead_algorithm: AeadAlgorithm::Aes256Gcm as u8,
        };

        // Encrypt payload chunks using the session key
        let chunks = Self::encrypt_payload_chunks(payload, &session_key, &header)?;

        // Create envelope without signature first
        let mut envelope = EnvelopeV2 {
            header,
            encrypted_session_key,
            chunks,
            signature: Vec::new(),
        };

        // Sign the entire envelope (excluding the signature field)
        let envelope_hash = envelope.compute_hash_for_signing()?;
        let signature = sender_keys.ed25519_key.sign(&envelope_hash);
        envelope.signature = signature.to_bytes().to_vec();

        // Clear sensitive data
        session_key.zeroize();

        Ok(envelope)
    }

    /// Verify the envelope's cryptographic integrity
    pub fn verify(&self) -> bool {
        // Verify Ed25519 signature
        if !self.verify_signature() {
            return false;
        }

        // Verify header integrity
        if !self.verify_header() {
            return false;
        }

        // Verify chunk sequence
        if !self.verify_chunk_sequence() {
            return false;
        }

        true
    }

    /// Unseal the envelope to recover the original payload
    pub fn unseal(&self, recipient_keys: &DualKeyPair) -> Result<Vec<u8>> {
        if !self.verify() {
            return Err(anyhow::anyhow!("Envelope verification failed"));
        }

        // Perform X25519 ECDH to recover shared secret
        let sender_ephemeral_pubkey = X25519PublicKey::from(self.header.sender_x25519_pubkey);
        let shared_secret = recipient_keys.x25519_key.diffie_hellman(&sender_ephemeral_pubkey);

        // Derive the same encryption key used for the session key
        let session_key_encryption_key = Self::derive_session_key_encryption_key(
            shared_secret.as_bytes(),
            &self.header.sender_x25519_pubkey,
            &recipient_keys.x25519_public().to_bytes(),
        )?;

        // Decrypt the session key
        let session_key = Self::decrypt_session_key(
            &self.encrypted_session_key,
            &session_key_encryption_key,
        )?;

        // Decrypt and reassemble the payload
        let payload = Self::decrypt_payload_chunks(&self.chunks, &session_key, &self.header)?;

        // Verify payload size
        if payload.len() != self.header.payload_size as usize {
            return Err(anyhow::anyhow!(
                "Payload size mismatch: expected {}, got {}",
                self.header.payload_size,
                payload.len()
            ));
        }

        Ok(payload)
    }

    /// Get the envelope hash for chaining
    pub fn hash(&self) -> [u8; 32] {
        let envelope_bytes = bincode::serialize(self).unwrap_or_default();
        *blake3::hash(&envelope_bytes).as_bytes()
    }

    /// Serialize envelope to bytes for storage/transmission
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
            .context("Failed to serialize envelope")
    }

    /// Deserialize envelope from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        bincode::deserialize(bytes)
            .context("Failed to deserialize envelope")
    }

    // Private helper methods

    /// Derive encryption key for the session key using HKDF
    fn derive_session_key_encryption_key(
        shared_secret: &[u8],
        sender_pubkey: &[u8; 32],
        recipient_pubkey: &[u8; 32],
    ) -> Result<[u8; 32]> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        // Create HKDF instance
        let hk = Hkdf::<Sha256>::new(None, shared_secret);

        // Create info context for key derivation
        let mut info = Vec::new();
        info.extend_from_slice(b"TRUSTEDGE_V2_SESSION_KEY");
        info.extend_from_slice(sender_pubkey);
        info.extend_from_slice(recipient_pubkey);

        // Derive 32-byte key
        let mut key = [0u8; 32];
        hk.expand(&info, &mut key)
            .map_err(|e| anyhow::anyhow!("HKDF expansion failed: {:?}", e))?;

        Ok(key)
    }

    /// Encrypt the session key using AES-256-GCM
    fn encrypt_session_key(session_key: &[u8; 32], encryption_key: &[u8; 32]) -> Result<Vec<u8>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};
        use rand::{rngs::OsRng, RngCore};

        let cipher = Aes256Gcm::new_from_slice(encryption_key)
            .context("Failed to create cipher for session key encryption")?;

        // Generate random nonce
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt session key
        let mut ciphertext = session_key.to_vec();
        cipher
            .encrypt_in_place(nonce, b"", &mut ciphertext)
            .map_err(|e| anyhow::anyhow!("Session key encryption failed: {:?}", e))?;

        // Prepend nonce to ciphertext
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt the session key
    fn decrypt_session_key(encrypted_data: &[u8], encryption_key: &[u8; 32]) -> Result<[u8; 32]> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};

        if encrypted_data.len() < 12 {
            return Err(anyhow::anyhow!("Encrypted session key too short"));
        }

        let cipher = Aes256Gcm::new_from_slice(encryption_key)
            .context("Failed to create cipher for session key decryption")?;

        // Extract nonce and ciphertext
        let nonce = Nonce::from_slice(&encrypted_data[..12]);
        let mut ciphertext = encrypted_data[12..].to_vec();

        // Decrypt
        cipher
            .decrypt_in_place(nonce, b"", &mut ciphertext)
            .map_err(|e| anyhow::anyhow!("Session key decryption failed: {:?}", e))?;

        // Convert to fixed-size array
        if ciphertext.len() != 32 {
            return Err(anyhow::anyhow!("Invalid session key length"));
        }

        let mut session_key = [0u8; 32];
        session_key.copy_from_slice(&ciphertext);

        Ok(session_key)
    }

    /// Encrypt payload into chunks using the session key
    fn encrypt_payload_chunks(
        payload: &[u8],
        session_key: &[u8; 32],
        header: &EnvelopeHeaderV2,
    ) -> Result<Vec<NetworkChunk>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};
        use rand::{rngs::OsRng, RngCore};

        let cipher = Aes256Gcm::new_from_slice(session_key)
            .context("Failed to create cipher for payload encryption")?;

        let mut chunks = Vec::new();

        for (i, chunk_data) in payload.chunks(DEFAULT_CHUNK_SIZE).enumerate() {
            // Generate random nonce for this chunk
            let mut nonce_bytes = [0u8; NONCE_LEN];
            OsRng.fill_bytes(&mut nonce_bytes);
            let nonce = Nonce::from_slice(&nonce_bytes);

            // Create AAD for this chunk
            let aad = Self::build_chunk_aad(header, i as u64, &nonce_bytes, chunk_data.len())?;

            // Encrypt chunk
            let mut ciphertext = chunk_data.to_vec();
            cipher
                .encrypt_in_place(nonce, &aad, &mut ciphertext)
                .map_err(|e| anyhow::anyhow!("Chunk encryption failed: {:?}", e))?;

            // Create minimal manifest for v2 (much simpler than v1)
            let manifest = ChunkManifestV2 {
                sequence: i as u64,
                chunk_size: chunk_data.len() as u32,
            };

            let manifest_bytes = bincode::serialize(&manifest)
                .context("Failed to serialize chunk manifest")?;

            // Create network chunk
            let chunk = NetworkChunk::new_with_nonce(
                i as u64,
                ciphertext,
                manifest_bytes,
                nonce_bytes,
            );

            chunks.push(chunk);
        }

        Ok(chunks)
    }

    /// Decrypt payload chunks using the session key
    fn decrypt_payload_chunks(chunks: &[NetworkChunk], session_key: &[u8; 32], header: &EnvelopeHeaderV2) -> Result<Vec<u8>> {
        use aes_gcm::{AeadInPlace, Aes256Gcm, KeyInit, Nonce};

        let cipher = Aes256Gcm::new_from_slice(session_key)
            .context("Failed to create cipher for payload decryption")?;

        // Sort chunks by sequence
        let mut sorted_chunks = chunks.to_vec();
        sorted_chunks.sort_by_key(|chunk| chunk.sequence);

        let mut payload = Vec::new();

        for chunk in sorted_chunks {
            // Deserialize manifest
            let manifest: ChunkManifestV2 = bincode::deserialize(&chunk.manifest)
                .context("Failed to deserialize chunk manifest")?;

            // Create nonce
            let nonce = Nonce::from_slice(&chunk.nonce);

            // Create AAD using the same method as encryption
            let aad = Self::build_chunk_aad(header, manifest.sequence, &chunk.nonce, manifest.chunk_size as usize)?;

            // Decrypt chunk
            let mut plaintext = chunk.data.clone();
            cipher
                .decrypt_in_place(nonce, &aad, &mut plaintext)
                .map_err(|e| anyhow::anyhow!("Chunk decryption failed: {:?}", e))?;

            payload.extend_from_slice(&plaintext);
        }

        Ok(payload)
    }

    /// Build AAD for chunk encryption
    fn build_chunk_aad(
        header: &EnvelopeHeaderV2,
        sequence: u64,
        nonce: &[u8],
        chunk_size: usize,
    ) -> Result<Vec<u8>> {
        let mut aad = Vec::new();
        aad.extend_from_slice(&header.magic);
        aad.push(header.version);
        aad.extend_from_slice(&sequence.to_le_bytes());
        aad.extend_from_slice(nonce);
        aad.extend_from_slice(&(chunk_size as u32).to_le_bytes());
        Ok(aad)
    }

    /// Compute hash for signing (excludes signature field)
    fn compute_hash_for_signing(&self) -> Result<[u8; 32]> {
        // Create a temporary envelope without signature for hashing
        let temp_envelope = EnvelopeV2 {
            header: self.header.clone(),
            encrypted_session_key: self.encrypted_session_key.clone(),
            chunks: self.chunks.clone(),
            signature: Vec::new(), // Empty signature for hashing
        };

        let envelope_bytes = bincode::serialize(&temp_envelope)
            .context("Failed to serialize envelope for signing")?;

        Ok(*blake3::hash(&envelope_bytes).as_bytes())
    }

    /// Verify the Ed25519 signature
    fn verify_signature(&self) -> bool {
        // Compute hash for verification
        let envelope_hash = match self.compute_hash_for_signing() {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        // Convert signature bytes
        let signature_bytes: [u8; 64] = match self.signature.as_slice().try_into() {
            Ok(bytes) => bytes,
            Err(_) => return false,
        };

        let signature = match Signature::try_from(signature_bytes.as_slice()) {
            Ok(sig) => sig,
            Err(_) => return false,
        };

        // Get sender's public key
        let sender_pubkey = match VerifyingKey::from_bytes(&self.header.sender_ed25519_pubkey) {
            Ok(key) => key,
            Err(_) => return false,
        };

        // Verify signature
        sender_pubkey.verify(&envelope_hash, &signature).is_ok()
    }

    /// Verify header integrity
    fn verify_header(&self) -> bool {
        // Check magic number
        if self.header.magic != ENVELOPE_V2_MAGIC {
            return false;
        }

        // Check version
        if self.header.version != ENVELOPE_V2_VERSION {
            return false;
        }

        // Check that we have the expected number of chunks
        if self.chunks.len() != self.header.chunk_count as usize {
            return false;
        }

        true
    }

    /// Verify chunk sequence integrity
    fn verify_chunk_sequence(&self) -> bool {
        if self.chunks.len() != self.header.chunk_count as usize {
            return false;
        }

        // Check sequence numbers are 0..chunk_count-1
        let mut sequences: Vec<u64> = self.chunks.iter().map(|c| c.sequence).collect();
        sequences.sort();

        for (expected, actual) in (0..self.header.chunk_count as u64).zip(sequences.iter()) {
            if expected != *actual {
                return false;
            }
        }

        true
    }
}

/// Simplified manifest for V2 chunks
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChunkManifestV2 {
    /// Sequence number of this chunk
    sequence: u64,
    /// Size of the chunk data in bytes
    chunk_size: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_envelope_v2_seal_unseal() {
        let sender_keys = DualKeyPair::generate();
        let recipient_keys = DualKeyPair::generate();
        
        let payload = b"Hello, hybrid encryption world!";
        let recipient_pubky_id = recipient_keys.pubky_identity();

        // Seal envelope
        let envelope = EnvelopeV2::seal(
            payload,
            &sender_keys,
            &recipient_keys.x25519_public(),
            &recipient_pubky_id,
        ).expect("Failed to seal envelope");

        // Verify envelope
        assert!(envelope.verify());

        // Unseal envelope
        let unsealed = envelope.unseal(&recipient_keys)
            .expect("Failed to unseal envelope");

        assert_eq!(payload, unsealed.as_slice());
    }

    #[test]
    fn test_large_payload_v2() {
        let sender_keys = DualKeyPair::generate();
        let recipient_keys = DualKeyPair::generate();
        
        // Create large payload
        let large_payload = vec![42u8; DEFAULT_CHUNK_SIZE * 3 + 1000];
        let recipient_pubky_id = recipient_keys.pubky_identity();

        // Seal envelope
        let envelope = EnvelopeV2::seal(
            &large_payload,
            &sender_keys,
            &recipient_keys.x25519_public(),
            &recipient_pubky_id,
        ).expect("Failed to seal large envelope");

        // Verify
        assert!(envelope.verify());
        assert_eq!(envelope.header.chunk_count, 4);

        // Unseal
        let unsealed = envelope.unseal(&recipient_keys)
            .expect("Failed to unseal large envelope");

        assert_eq!(large_payload, unsealed);
    }

    #[test]
    fn test_envelope_serialization() {
        let sender_keys = DualKeyPair::generate();
        let recipient_keys = DualKeyPair::generate();
        
        let payload = b"Test serialization";
        let recipient_pubky_id = recipient_keys.pubky_identity();

        // Create envelope
        let envelope = EnvelopeV2::seal(
            payload,
            &sender_keys,
            &recipient_keys.x25519_public(),
            &recipient_pubky_id,
        ).expect("Failed to seal envelope");

        // Serialize and deserialize
        let bytes = envelope.to_bytes().expect("Failed to serialize");
        let deserialized = EnvelopeV2::from_bytes(&bytes).expect("Failed to deserialize");

        // Verify deserialized envelope
        assert!(deserialized.verify());
        
        let unsealed = deserialized.unseal(&recipient_keys)
            .expect("Failed to unseal deserialized envelope");
        
        assert_eq!(payload, unsealed.as_slice());
    }
}