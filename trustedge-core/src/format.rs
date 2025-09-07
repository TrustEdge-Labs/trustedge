// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

/// trustedge_core/src/format.rs
use anyhow::{Context, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

pub const NONCE_LEN: usize = 12;
pub const AAD_LEN: usize = 32 + 8 + NONCE_LEN + 32 + 4; // Added 4 bytes for chunk_len
pub const HEADER_LEN: usize = 66; // Updated from 58 for algorithm agility

pub const MAGIC: &[u8; 4] = b"TRST";
pub const VERSION: u8 = 2; // Updated for algorithm agility
pub const ALG_AES_256_GCM: u8 = 1; // Legacy constant for backward compatibility

/// AEAD (Authenticated Encryption with Associated Data) algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AeadAlgorithm {
    Aes256Gcm = 1,
    ChaCha20Poly1305 = 2,
    Aes256Siv = 3, // For future quantum resistance
                   // Reserve 4-127 for standard algorithms
                   // Reserve 128-255 for experimental/custom algorithms
}

impl TryFrom<u8> for AeadAlgorithm {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(AeadAlgorithm::Aes256Gcm),
            2 => Ok(AeadAlgorithm::ChaCha20Poly1305),
            3 => Ok(AeadAlgorithm::Aes256Siv),
            _ => Err(anyhow::anyhow!("Unsupported AEAD algorithm: {}", value)),
        }
    }
}

/// Signature algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SignatureAlgorithm {
    Ed25519 = 1,
    EcdsaP256 = 2,
    EcdsaP384 = 3,
    RsaPss2048 = 4,
    RsaPss4096 = 5,
    Dilithium3 = 6, // Post-quantum signature
    Falcon512 = 7,  // Post-quantum signature
                    // Reserve 8-127 for standard algorithms
                    // Reserve 128-255 for experimental algorithms
}

impl TryFrom<u8> for SignatureAlgorithm {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SignatureAlgorithm::Ed25519),
            2 => Ok(SignatureAlgorithm::EcdsaP256),
            3 => Ok(SignatureAlgorithm::EcdsaP384),
            4 => Ok(SignatureAlgorithm::RsaPss2048),
            5 => Ok(SignatureAlgorithm::RsaPss4096),
            6 => Ok(SignatureAlgorithm::Dilithium3),
            7 => Ok(SignatureAlgorithm::Falcon512),
            _ => Err(anyhow::anyhow!(
                "Unsupported signature algorithm: {}",
                value
            )),
        }
    }
}

/// Hash algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum HashAlgorithm {
    Blake3 = 1,
    Sha256 = 2,
    Sha384 = 3,
    Sha512 = 4,
    Sha3_256 = 5,
    Sha3_512 = 6,
    // Reserve 7-127 for standard algorithms
    // Reserve 128-255 for experimental algorithms
}

impl TryFrom<u8> for HashAlgorithm {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HashAlgorithm::Blake3),
            2 => Ok(HashAlgorithm::Sha256),
            3 => Ok(HashAlgorithm::Sha384),
            4 => Ok(HashAlgorithm::Sha512),
            5 => Ok(HashAlgorithm::Sha3_256),
            6 => Ok(HashAlgorithm::Sha3_512),
            _ => Err(anyhow::anyhow!("Unsupported hash algorithm: {}", value)),
        }
    }
}

/// Key Derivation Function algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KdfAlgorithm {
    Pbkdf2Sha256 = 1,
    Argon2id = 2,
    Scrypt = 3,
    Hkdf = 4,
    // Reserve 5-127 for standard algorithms
    // Reserve 128-255 for experimental algorithms
}

impl TryFrom<u8> for KdfAlgorithm {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(KdfAlgorithm::Pbkdf2Sha256),
            2 => Ok(KdfAlgorithm::Argon2id),
            3 => Ok(KdfAlgorithm::Scrypt),
            4 => Ok(KdfAlgorithm::Hkdf),
            _ => Err(anyhow::anyhow!("Unsupported KDF algorithm: {}", value)),
        }
    }
}

// Security limits and bounds
pub const MAX_CHUNK_SIZE: u32 = 128 * 1024 * 1024; // 128MB max chunk size
pub const MAX_RECORDS_PER_STREAM: u64 = 1_000_000; // 1M records max per stream
pub const MAX_STREAM_SIZE_BYTES: u64 = 10 * 1024 * 1024 * 1024; // 10GB max stream size
pub const AES_GCM_TAG_SIZE: usize = 16; // AES-GCM authentication tag size

/// Domain separation string for manifest signatures
/// Prevents signature reuse across different contexts or protocols
pub const MANIFEST_DOMAIN_SEP: &[u8] = b"trustedge.manifest.v1";

/// Enhanced FileHeader structure with algorithm agility
#[derive(Clone, Copy, Debug)]
pub struct FileHeader {
    pub version: u8,              // 1 byte  - File format version
    pub aead_alg: u8,             // 1 byte  - AEAD algorithm (was 'alg')
    pub sig_alg: u8,              // 1 byte  - Signature algorithm
    pub hash_alg: u8,             // 1 byte  - Hash algorithm
    pub kdf_alg: u8,              // 1 byte  - KDF algorithm
    pub reserved: [u8; 3],        // 3 bytes - Reserved for future use
    pub key_id: [u8; 16],         // 16 bytes - Key identifier
    pub device_id_hash: [u8; 32], // 32 bytes - Hash of device ID + salt
    pub nonce_prefix: [u8; 4],    // 4 bytes  - Random nonce prefix for session
    pub chunk_size: u32,          // 4 bytes  - Chunk size in bytes (big-endian)
}

/// FileHeader serialization/deserialization
impl FileHeader {
    pub fn to_bytes(&self) -> [u8; HEADER_LEN] {
        let mut out = [0u8; HEADER_LEN];
        out[0] = self.version;
        out[1] = self.aead_alg;
        out[2] = self.sig_alg;
        out[3] = self.hash_alg;
        out[4] = self.kdf_alg;
        out[5..8].copy_from_slice(&self.reserved);
        out[8..24].copy_from_slice(&self.key_id);
        out[24..56].copy_from_slice(&self.device_id_hash);
        out[56..60].copy_from_slice(&self.nonce_prefix);
        out[60..64].copy_from_slice(&self.chunk_size.to_be_bytes());
        out
    }

    /// Create a FileHeader from bytes with validation
    pub fn from_bytes(bytes: &[u8; HEADER_LEN]) -> Result<Self, anyhow::Error> {
        // Validate algorithm IDs before constructing
        let _aead_alg = AeadAlgorithm::try_from(bytes[1]).context("Invalid AEAD algorithm")?;
        let _sig_alg =
            SignatureAlgorithm::try_from(bytes[2]).context("Invalid signature algorithm")?;
        let _hash_alg = HashAlgorithm::try_from(bytes[3]).context("Invalid hash algorithm")?;
        let _kdf_alg = KdfAlgorithm::try_from(bytes[4]).context("Invalid KDF algorithm")?;

        let mut key_id = [0u8; 16];
        key_id.copy_from_slice(&bytes[8..24]);
        let mut device_id_hash = [0u8; 32];
        device_id_hash.copy_from_slice(&bytes[24..56]);
        let mut nonce_prefix = [0u8; 4];
        nonce_prefix.copy_from_slice(&bytes[56..60]);
        let mut reserved = [0u8; 3];
        reserved.copy_from_slice(&bytes[5..8]);
        let chunk_size = u32::from_be_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]);

        Ok(FileHeader {
            version: bytes[0],
            aead_alg: bytes[1],
            sig_alg: bytes[2],
            hash_alg: bytes[3],
            kdf_alg: bytes[4],
            reserved,
            key_id,
            device_id_hash,
            nonce_prefix,
            chunk_size,
        })
    }

    /// Create a FileHeader with default algorithm choices (for backward compatibility)
    pub fn new_with_defaults() -> Self {
        FileHeader {
            version: VERSION,
            aead_alg: AeadAlgorithm::Aes256Gcm as u8,
            sig_alg: SignatureAlgorithm::Ed25519 as u8,
            hash_alg: HashAlgorithm::Blake3 as u8,
            kdf_alg: KdfAlgorithm::Pbkdf2Sha256 as u8,
            reserved: [0; 3],
            key_id: [0; 16],
            device_id_hash: [0; 32],
            nonce_prefix: [0; 4],
            chunk_size: 0,
        }
    }
}

/// Data type enumeration for different input sources
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DataType {
    /// Unknown or unspecified data type
    Unknown,
    /// Generic file data (original behavior)
    File { mime_type: Option<String> },
    /// Live audio capture
    Audio {
        sample_rate: u32,
        channels: u16,
        format: AudioFormat,
    },
    /// Video capture (future use)
    Video {
        width: u32,
        height: u32,
        fps: f32,
        format: String,
    },
    /// Raw sensor data (future use)
    Sensor { sensor_type: String },
}

/// Audio format specification
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AudioFormat {
    /// 32-bit floating point samples
    F32Le,
    /// 16-bit signed integer samples
    I16Le,
    /// 24-bit signed integer samples
    I24Le,
    /// Other/custom format
    Other(String),
}

/// Manifest structure
#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub v: u8,
    pub ts_ms: u64,
    pub seq: u64,
    pub header_hash: [u8; 32],
    pub pt_hash: [u8; 32],
    pub key_id: [u8; 16], // Added for key identification/rotation
    pub ai_used: bool,
    pub model_ids: Vec<String>,
    pub data_type: DataType, // Data type and format metadata
    pub chunk_len: u32,      // NEW: Expected plaintext length of this chunk (bound via AAD)
}

/// SignedManifest structure
#[derive(Serialize, Deserialize)]
pub struct SignedManifest {
    pub manifest: Vec<u8>,
    pub sig: Vec<u8>,
    pub pubkey: Vec<u8>,
}

/// StreamHeader structure
#[derive(Serialize, Deserialize)]
pub struct StreamHeader {
    pub v: u8,
    pub header: Vec<u8>, // 58 bytes in practice
    pub header_hash: [u8; 32],
}

/// Record structure
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub seq: u64,
    pub nonce: [u8; NONCE_LEN],
    pub sm: SignedManifest,
    pub ct: Vec<u8>,
}

/// Build Additional Authenticated Data (AAD) for encryption
/// AAD = header_hash(32) || seq_be(8) || nonce(12) || manifest_hash(32) || chunk_len_be(4)
pub fn build_aad(
    header_hash: &[u8; 32],
    seq: u64,
    nonce: &[u8; NONCE_LEN],
    manifest_hash: &[u8; 32],
    chunk_len: u32,
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
    off += 32;
    aad[off..off + 4].copy_from_slice(&chunk_len.to_be_bytes());
    aad
}

/// Write the stream header to the output
pub fn write_stream_header<W: std::io::Write>(w: &mut W, sh: &StreamHeader) -> Result<()> {
    w.write_all(MAGIC).context("write magic")?;
    w.write_all(&[VERSION]).context("write version")?;
    bincode::serialize_into(w, sh).context("write stream header")?;
    Ok(())
}

/// Legacy FileHeader structure for V1 compatibility (58 bytes)
#[derive(Clone, Copy, Debug)]
pub struct FileHeaderV1 {
    pub version: u8,              // 1
    pub alg: u8,                  // 1
    pub key_id: [u8; 16],         // 16
    pub device_id_hash: [u8; 32], // 32
    pub nonce_prefix: [u8; 4],    // 4
    pub chunk_size: u32,          // 4 (BE)
}

impl FileHeaderV1 {
    pub fn from_bytes(bytes: &[u8; 58]) -> Self {
        let mut key_id = [0u8; 16];
        key_id.copy_from_slice(&bytes[2..18]);
        let mut device_id_hash = [0u8; 32];
        device_id_hash.copy_from_slice(&bytes[18..50]);
        let mut nonce_prefix = [0u8; 4];
        nonce_prefix.copy_from_slice(&bytes[50..54]);
        let chunk_size = u32::from_be_bytes([bytes[54], bytes[55], bytes[56], bytes[57]]);

        FileHeaderV1 {
            version: bytes[0],
            alg: bytes[1],
            key_id,
            device_id_hash,
            nonce_prefix,
            chunk_size,
        }
    }
}

/// Legacy StreamHeader for V1 compatibility
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StreamHeaderV1 {
    pub v: u8,
    pub header: Vec<u8>, // 58 bytes for V1
    pub header_hash: [u8; 32],
}

/// Migrate V1 header to V2 format with default algorithm choices
fn migrate_v1_to_v2(v1: StreamHeaderV1) -> Result<StreamHeader> {
    if v1.header.len() != 58 {
        return Err(anyhow::anyhow!(
            "Invalid V1 header length: {}",
            v1.header.len()
        ));
    }

    let v1_bytes: [u8; 58] = v1
        .header
        .try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert V1 header to array"))?;
    let v1_header = FileHeaderV1::from_bytes(&v1_bytes);

    let v2_header = FileHeader {
        version: 2,
        aead_alg: v1_header.alg, // Copy existing AES-256-GCM
        sig_alg: SignatureAlgorithm::Ed25519 as u8,
        hash_alg: HashAlgorithm::Blake3 as u8,
        kdf_alg: KdfAlgorithm::Pbkdf2Sha256 as u8,
        reserved: [0; 3],
        key_id: v1_header.key_id,
        device_id_hash: v1_header.device_id_hash,
        nonce_prefix: v1_header.nonce_prefix,
        chunk_size: v1_header.chunk_size,
    };

    let v2_bytes = v2_header.to_bytes();
    Ok(StreamHeader {
        v: 2,
        header: v2_bytes.to_vec(),
        header_hash: *blake3::hash(&v2_bytes).as_bytes(),
    })
}

/// Read the preamble and stream header from the input with version migration
pub fn read_preamble_and_header<R: std::io::Read>(r: &mut R) -> Result<StreamHeader> {
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic).context("read magic")?;
    anyhow::ensure!(&magic == MAGIC, "bad magic");

    let mut ver = [0u8; 1];
    r.read_exact(&mut ver).context("read version")?;

    match ver[0] {
        1 => {
            // Legacy 58-byte header format
            let sh_v1: StreamHeaderV1 =
                bincode::deserialize_from(r).context("read V1 stream header")?;
            migrate_v1_to_v2(sh_v1)
        }
        2 => {
            // New 66-byte header format with algorithm agility
            let sh: StreamHeader = bincode::deserialize_from(r).context("read V2 stream header")?;
            Ok(sh)
        }
        _ => Err(anyhow::anyhow!("Unsupported version: {}", ver[0])),
    }
}

/// Sign manifest bytes with domain separation
/// This prevents signature reuse across different contexts or protocols
pub fn sign_manifest_with_domain(signing_key: &SigningKey, manifest_bytes: &[u8]) -> Signature {
    let mut message = Vec::with_capacity(MANIFEST_DOMAIN_SEP.len() + manifest_bytes.len());
    message.extend_from_slice(MANIFEST_DOMAIN_SEP);
    message.extend_from_slice(manifest_bytes);
    signing_key.sign(&message)
}

/// Verify manifest signature with domain separation
/// This prevents signature reuse across different contexts or protocols
pub fn verify_manifest_with_domain(
    verifying_key: &VerifyingKey,
    manifest_bytes: &[u8],
    signature: &Signature,
) -> Result<()> {
    let mut message = Vec::with_capacity(MANIFEST_DOMAIN_SEP.len() + manifest_bytes.len());
    message.extend_from_slice(MANIFEST_DOMAIN_SEP);
    message.extend_from_slice(manifest_bytes);
    verifying_key.verify(&message, signature).map_err(|e| {
        anyhow::anyhow!(
            "Domain-separated manifest signature verification failed: {}",
            e
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm_enum_roundtrip() {
        // Test AEAD algorithms round-trip through u8
        for alg in [
            AeadAlgorithm::Aes256Gcm,
            AeadAlgorithm::ChaCha20Poly1305,
            AeadAlgorithm::Aes256Siv,
        ] {
            let byte_val = alg as u8;
            let parsed = AeadAlgorithm::try_from(byte_val).unwrap();
            assert_eq!(alg, parsed);
        }

        // Test signature algorithms round-trip through u8
        for alg in [
            SignatureAlgorithm::Ed25519,
            SignatureAlgorithm::EcdsaP256,
            SignatureAlgorithm::EcdsaP384,
        ] {
            let byte_val = alg as u8;
            let parsed = SignatureAlgorithm::try_from(byte_val).unwrap();
            assert_eq!(alg, parsed);
        }

        // Test hash algorithms round-trip through u8
        for alg in [
            HashAlgorithm::Blake3,
            HashAlgorithm::Sha256,
            HashAlgorithm::Sha384,
        ] {
            let byte_val = alg as u8;
            let parsed = HashAlgorithm::try_from(byte_val).unwrap();
            assert_eq!(alg, parsed);
        }

        // Test KDF algorithms round-trip through u8
        for alg in [
            KdfAlgorithm::Pbkdf2Sha256,
            KdfAlgorithm::Argon2id,
            KdfAlgorithm::Scrypt,
        ] {
            let byte_val = alg as u8;
            let parsed = KdfAlgorithm::try_from(byte_val).unwrap();
            assert_eq!(alg, parsed);
        }
    }

    #[test]
    fn test_unsupported_algorithm_rejection() {
        // Test that unknown algorithm IDs are rejected
        assert!(AeadAlgorithm::try_from(99).is_err());
        assert!(SignatureAlgorithm::try_from(99).is_err());
        assert!(HashAlgorithm::try_from(99).is_err());
        assert!(KdfAlgorithm::try_from(99).is_err());

        // Test boundary values
        assert!(AeadAlgorithm::try_from(0).is_err());
        assert!(AeadAlgorithm::try_from(255).is_err());
    }

    #[test]
    fn test_fileheader_v2_roundtrip() {
        let header = FileHeader {
            version: 2,
            aead_alg: AeadAlgorithm::Aes256Gcm as u8,
            sig_alg: SignatureAlgorithm::Ed25519 as u8,
            hash_alg: HashAlgorithm::Blake3 as u8,
            kdf_alg: KdfAlgorithm::Pbkdf2Sha256 as u8,
            reserved: [0; 3],
            key_id: [1; 16],
            device_id_hash: [2; 32],
            nonce_prefix: [3, 4, 5, 6],
            chunk_size: 4096,
        };

        // Test serialization/deserialization roundtrip
        let bytes = header.to_bytes();
        assert_eq!(bytes.len(), HEADER_LEN);

        let parsed = FileHeader::from_bytes(&bytes).unwrap();
        assert_eq!(header.version, parsed.version);
        assert_eq!(header.aead_alg, parsed.aead_alg);
        assert_eq!(header.sig_alg, parsed.sig_alg);
        assert_eq!(header.hash_alg, parsed.hash_alg);
        assert_eq!(header.kdf_alg, parsed.kdf_alg);
        assert_eq!(header.key_id, parsed.key_id);
        assert_eq!(header.device_id_hash, parsed.device_id_hash);
        assert_eq!(header.nonce_prefix, parsed.nonce_prefix);
        assert_eq!(header.chunk_size, parsed.chunk_size);
    }

    #[test]
    fn test_non_default_algorithms() {
        let header = FileHeader {
            version: 2,
            aead_alg: AeadAlgorithm::ChaCha20Poly1305 as u8,
            sig_alg: SignatureAlgorithm::EcdsaP256 as u8,
            hash_alg: HashAlgorithm::Sha256 as u8,
            kdf_alg: KdfAlgorithm::Argon2id as u8,
            reserved: [0; 3],
            key_id: [0x42; 16],
            device_id_hash: [0x33; 32],
            nonce_prefix: [0x11, 0x22, 0x33, 0x44],
            chunk_size: 8192,
        };

        // Test serialization/deserialization roundtrip with non-default algorithms
        let bytes = header.to_bytes();
        let parsed = FileHeader::from_bytes(&bytes).unwrap();

        assert_eq!(parsed.aead_alg, AeadAlgorithm::ChaCha20Poly1305 as u8);
        assert_eq!(parsed.sig_alg, SignatureAlgorithm::EcdsaP256 as u8);
        assert_eq!(parsed.hash_alg, HashAlgorithm::Sha256 as u8);
        assert_eq!(parsed.kdf_alg, KdfAlgorithm::Argon2id as u8);
    }

    #[test]
    fn test_invalid_algorithm_parsing() {
        let mut bytes = [0u8; HEADER_LEN];
        bytes[0] = 2; // version
        bytes[1] = 99; // invalid AEAD algorithm
        bytes[2] = SignatureAlgorithm::Ed25519 as u8;
        bytes[3] = HashAlgorithm::Blake3 as u8;
        bytes[4] = KdfAlgorithm::Pbkdf2Sha256 as u8;

        // Should fail due to invalid AEAD algorithm
        assert!(FileHeader::from_bytes(&bytes).is_err());

        // Test invalid signature algorithm
        bytes[1] = AeadAlgorithm::Aes256Gcm as u8;
        bytes[2] = 99; // invalid signature algorithm
        assert!(FileHeader::from_bytes(&bytes).is_err());

        // Test invalid hash algorithm
        bytes[2] = SignatureAlgorithm::Ed25519 as u8;
        bytes[3] = 99; // invalid hash algorithm
        assert!(FileHeader::from_bytes(&bytes).is_err());

        // Test invalid KDF algorithm
        bytes[3] = HashAlgorithm::Blake3 as u8;
        bytes[4] = 99; // invalid KDF algorithm
        assert!(FileHeader::from_bytes(&bytes).is_err());
    }

    #[test]
    fn test_default_header_creation() {
        let header = FileHeader::new_with_defaults();

        assert_eq!(header.version, VERSION);
        assert_eq!(header.aead_alg, AeadAlgorithm::Aes256Gcm as u8);
        assert_eq!(header.sig_alg, SignatureAlgorithm::Ed25519 as u8);
        assert_eq!(header.hash_alg, HashAlgorithm::Blake3 as u8);
        assert_eq!(header.kdf_alg, KdfAlgorithm::Pbkdf2Sha256 as u8);
        assert_eq!(header.reserved, [0; 3]);
    }

    #[test]
    fn test_v1_to_v2_migration() {
        // Create a V1 header
        let v1_header_bytes = {
            let mut bytes = [0u8; 58];
            bytes[0] = 1; // version
            bytes[1] = ALG_AES_256_GCM; // AES-256-GCM
            bytes[2..18].copy_from_slice(&[0x42; 16]); // key_id
            bytes[18..50].copy_from_slice(&[0x33; 32]); // device_id_hash
            bytes[50..54].copy_from_slice(&[0x11, 0x22, 0x33, 0x44]); // nonce_prefix
            bytes[54..58].copy_from_slice(&4096u32.to_be_bytes()); // chunk_size
            bytes
        };

        let v1_stream_header = StreamHeaderV1 {
            v: 1,
            header: v1_header_bytes.to_vec(),
            header_hash: *blake3::hash(&v1_header_bytes).as_bytes(),
        };

        // Test migration
        let v2_stream_header = migrate_v1_to_v2(v1_stream_header).unwrap();

        assert_eq!(v2_stream_header.v, 2);
        assert_eq!(v2_stream_header.header.len(), HEADER_LEN);

        let v2_header_bytes: [u8; HEADER_LEN] = v2_stream_header.header.try_into().unwrap();
        let v2_header = FileHeader::from_bytes(&v2_header_bytes).unwrap();

        // Verify algorithms are correctly mapped
        assert_eq!(v2_header.version, 2);
        assert_eq!(v2_header.aead_alg, ALG_AES_256_GCM);
        assert_eq!(v2_header.sig_alg, SignatureAlgorithm::Ed25519 as u8);
        assert_eq!(v2_header.hash_alg, HashAlgorithm::Blake3 as u8);
        assert_eq!(v2_header.kdf_alg, KdfAlgorithm::Pbkdf2Sha256 as u8);

        // Verify other fields are preserved
        assert_eq!(v2_header.key_id, [0x42; 16]);
        assert_eq!(v2_header.device_id_hash, [0x33; 32]);
        assert_eq!(v2_header.nonce_prefix, [0x11, 0x22, 0x33, 0x44]);
        assert_eq!(v2_header.chunk_size, 4096);
    }
}
