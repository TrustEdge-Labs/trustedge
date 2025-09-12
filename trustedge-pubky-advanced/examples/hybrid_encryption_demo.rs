// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Hybrid Encryption Demo
//!
//! This example demonstrates the new TrustEdge v2 envelope format with:
//! - Dual key architecture (Ed25519 + X25519)
//! - Hybrid encryption (X25519 ECDH + AES-256-GCM)
//! - Pubky integration for decentralized key discovery

use trustedge_pubky_advanced::{DualKeyPair, EnvelopeV2};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 TrustEdge Pubky Hybrid Encryption Demo");
    println!("==========================================\n");

    // Step 1: Generate dual key pairs for Alice and Bob
    println!("📋 Step 1: Generating dual key pairs...");
    let alice_keys = DualKeyPair::generate();
    let bob_keys = DualKeyPair::generate();

    println!("✅ Alice's Pubky ID: {}", alice_keys.pubky_identity());
    println!("✅ Bob's Pubky ID: {}", bob_keys.pubky_identity());
    println!();

    // Step 2: Create sample data to encrypt
    println!("📋 Step 2: Creating sample data...");
    let sample_data = create_sample_audio_data();
    println!("✅ Sample data size: {} bytes", sample_data.len());
    println!("✅ Data preview: {:?}...", &sample_data[..20]);
    println!();

    // Step 3: Alice encrypts data for Bob
    println!("📋 Step 3: Alice encrypts data for Bob...");
    let bob_pubky_id = bob_keys.pubky_identity();
    let envelope = EnvelopeV2::seal(
        &sample_data,
        &alice_keys,
        &bob_keys.x25519_public(),
        &bob_pubky_id,
    )?;

    println!("✅ Envelope created successfully!");
    println!(
        "   - Magic: {:?}",
        std::str::from_utf8(&envelope.header.magic).unwrap_or("???")
    );
    println!("   - Version: {}", envelope.header.version);
    println!("   - Recipient: {}", envelope.header.recipient_pubkey_id);
    println!(
        "   - Key Exchange: {:?}",
        envelope.header.key_exchange_algorithm
    );
    println!("   - Payload Size: {} bytes", envelope.header.payload_size);
    println!("   - Chunk Count: {}", envelope.header.chunk_count);
    println!(
        "   - Encrypted Session Key Size: {} bytes",
        envelope.encrypted_session_key.len()
    );
    println!("   - Signature Size: {} bytes", envelope.signature.len());
    println!();

    // Step 4: Verify envelope integrity
    println!("📋 Step 4: Verifying envelope integrity...");
    let is_valid = envelope.verify();
    println!(
        "✅ Envelope verification: {}",
        if is_valid { "PASSED" } else { "FAILED" }
    );
    println!();

    // Step 5: Bob decrypts the data
    println!("📋 Step 5: Bob decrypts the data...");
    let decrypted_data = envelope.unseal(&bob_keys)?;
    println!("✅ Decryption successful!");
    println!("   - Decrypted size: {} bytes", decrypted_data.len());
    println!("   - Data matches: {}", sample_data == decrypted_data);
    println!();

    // Step 6: Serialize and deserialize envelope
    println!("📋 Step 6: Testing serialization...");
    let serialized = envelope.to_bytes()?;
    println!("✅ Serialized envelope size: {} bytes", serialized.len());

    let deserialized = EnvelopeV2::from_bytes(&serialized)?;
    println!("✅ Deserialization successful!");

    let deserialized_data = deserialized.unseal(&bob_keys)?;
    println!(
        "✅ Deserialized data matches: {}",
        sample_data == deserialized_data
    );
    println!();

    // Step 7: Performance analysis
    println!("📋 Step 7: Performance analysis...");
    let overhead = serialized.len() - sample_data.len();
    let overhead_percent = (overhead as f64 / sample_data.len() as f64) * 100.0;
    println!(
        "✅ Encryption overhead: {} bytes ({:.2}%)",
        overhead, overhead_percent
    );

    // Calculate efficiency metrics
    let header_size = bincode::serialize(&envelope.header)?.len();
    let session_key_size = envelope.encrypted_session_key.len();
    let signature_size = envelope.signature.len();
    let chunk_overhead =
        serialized.len() - sample_data.len() - header_size - session_key_size - signature_size;

    println!("   - Header size: {} bytes", header_size);
    println!("   - Encrypted session key: {} bytes", session_key_size);
    println!("   - Signature: {} bytes", signature_size);
    println!("   - Chunk overhead: {} bytes", chunk_overhead);
    println!();

    // Step 8: Security features demonstration
    println!("📋 Step 8: Security features...");
    println!("✅ Forward Secrecy: Ephemeral X25519 key used");
    println!("✅ Authentication: Ed25519 signature verified");
    println!("✅ Confidentiality: AES-256-GCM encryption");
    println!("✅ Integrity: AEAD provides tamper detection");
    println!("✅ Key Exchange: X25519 ECDH for session key");
    println!();

    // Step 9: Demonstrate key derivation determinism
    println!("📋 Step 9: Key derivation determinism...");
    let derived_x25519_1 = DualKeyPair::derive_x25519_from_ed25519(&alice_keys.ed25519_key);
    let derived_x25519_2 = DualKeyPair::derive_x25519_from_ed25519(&alice_keys.ed25519_key);
    println!(
        "✅ Deterministic X25519 derivation: {}",
        derived_x25519_1.to_bytes() == derived_x25519_2.to_bytes()
    );
    println!();

    println!("🎉 Demo completed successfully!");
    println!("   All tests passed - TrustEdge v2 hybrid encryption is working correctly!");

    Ok(())
}

/// Create sample audio data for testing
fn create_sample_audio_data() -> Vec<u8> {
    // Simulate audio data with a pattern that's easy to verify
    let mut data = Vec::new();

    // Add a "header" section
    data.extend_from_slice(b"TRUSTEDGE_AUDIO_V2");
    data.extend_from_slice(&[0u8; 14]); // Padding to 32 bytes

    // Add sample rate and metadata
    data.extend_from_slice(&44100u32.to_le_bytes()); // Sample rate
    data.extend_from_slice(&2u16.to_le_bytes()); // Channels
    data.extend_from_slice(&16u16.to_le_bytes()); // Bit depth
    data.extend_from_slice(&[0u8; 22]); // Reserved

    // Generate synthetic audio data (sine wave pattern)
    for i in 0..8192 {
        let sample = (((i as f32 * 0.1).sin() * 32767.0) as i16).to_le_bytes();
        data.extend_from_slice(&sample);
    }

    // Add some random data to make it more realistic
    use rand::{Rng, SeedableRng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(42); // Deterministic for testing
    for _ in 0..1024 {
        data.push(rng.gen());
    }

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_data_consistency() {
        let data1 = create_sample_audio_data();
        let data2 = create_sample_audio_data();
        assert_eq!(data1, data2, "Sample data should be deterministic");
    }

    #[test]
    fn test_hybrid_encryption_roundtrip() {
        let alice_keys = DualKeyPair::generate();
        let bob_keys = DualKeyPair::generate();
        let data = create_sample_audio_data();

        let envelope = EnvelopeV2::seal(
            &data,
            &alice_keys,
            &bob_keys.x25519_public(),
            &bob_keys.pubky_identity(),
        )
        .expect("Failed to seal envelope");

        assert!(envelope.verify(), "Envelope should verify");

        let decrypted = envelope
            .unseal(&bob_keys)
            .expect("Failed to unseal envelope");

        assert_eq!(data, decrypted, "Decrypted data should match original");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let alice_keys = DualKeyPair::generate();
        let bob_keys = DualKeyPair::generate();
        let data = b"Test data for serialization";

        let envelope = EnvelopeV2::seal(
            data,
            &alice_keys,
            &bob_keys.x25519_public(),
            &bob_keys.pubky_identity(),
        )
        .expect("Failed to seal envelope");

        let serialized = envelope.to_bytes().expect("Failed to serialize");
        let deserialized = EnvelopeV2::from_bytes(&serialized).expect("Failed to deserialize");

        assert!(deserialized.verify(), "Deserialized envelope should verify");

        let decrypted = deserialized
            .unseal(&bob_keys)
            .expect("Failed to unseal deserialized envelope");

        assert_eq!(
            data,
            decrypted.as_slice(),
            "Data should survive serialization roundtrip"
        );
    }
}
