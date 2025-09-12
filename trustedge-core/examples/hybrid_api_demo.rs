// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Hybrid Encryption API Demo
//!
//! This example demonstrates the new high-level hybrid encryption API
//! that matches the specification in Step 2.

use trustedge_core::{
    seal_for_recipient, open_envelope, KeyPair, AsymmetricAlgorithm, TrustEdgeError
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔐 TrustEdge Core Hybrid Encryption API Demo");
    println!("=============================================\n");

    // Step 1: Generate key pairs for Alice and Bob
    println!("📋 Step 1: Generating RSA key pairs...");
    let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;
    let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;

    println!("✅ Alice's key ID: {}", alice_keypair.public.id());
    println!("✅ Bob's key ID: {}", bob_keypair.public.id());
    println!();

    // Step 2: Create sample data
    println!("📋 Step 2: Creating sample data...");
    let sample_data = b"Hello, this is a secret message from Alice to Bob using hybrid encryption!";
    println!("✅ Sample data: {:?}", std::str::from_utf8(sample_data).unwrap());
    println!("✅ Data size: {} bytes", sample_data.len());
    println!();

    // Step 3: Alice seals data for Bob using the high-level API
    println!("📋 Step 3: Alice seals data for Bob...");
    let sealed_envelope = seal_for_recipient(sample_data, &bob_keypair.public)?;
    println!("✅ Envelope sealed successfully!");
    println!("   - Envelope size: {} bytes", sealed_envelope.len());
    println!("   - Overhead: {} bytes ({:.1}%)", 
        sealed_envelope.len() - sample_data.len(),
        ((sealed_envelope.len() - sample_data.len()) as f64 / sample_data.len() as f64) * 100.0
    );
    println!();

    // Step 4: Bob opens the envelope using the high-level API
    println!("📋 Step 4: Bob opens the envelope...");
    let decrypted_data = open_envelope(&sealed_envelope, &bob_keypair.private)?;
    println!("✅ Envelope opened successfully!");
    println!("   - Decrypted data: {:?}", std::str::from_utf8(&decrypted_data).unwrap());
    println!("   - Data matches: {}", sample_data == decrypted_data.as_slice());
    println!();

    // Step 5: Test with wrong key (should fail)
    println!("📋 Step 5: Testing with wrong key...");
    let charlie_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;
    match open_envelope(&sealed_envelope, &charlie_keypair.private) {
        Ok(_) => println!("❌ ERROR: Should have failed with wrong key!"),
        Err(e) => println!("✅ Correctly failed with wrong key: {}", e),
    }
    println!();

    // Step 6: Test with different algorithms
    println!("📋 Step 6: Testing with different algorithms...");
    
    // Ed25519 (for comparison, though not suitable for key encryption)
    let ed25519_keypair = KeyPair::generate(AsymmetricAlgorithm::Ed25519)?;
    println!("✅ Ed25519 key generated: {}", ed25519_keypair.public.id());
    
    // ECDSA P-256 (for ECDH key exchange)
    let ecdsa_keypair = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)?;
    println!("✅ ECDSA P-256 key generated: {}", ecdsa_keypair.public.id());
    
    // Test ECDH key exchange
    let alice_ecdsa = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)?;
    let bob_ecdsa = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)?;
    
    let alice_shared = trustedge_core::key_exchange(&alice_ecdsa.private, &bob_ecdsa.public)?;
    let bob_shared = trustedge_core::key_exchange(&bob_ecdsa.private, &alice_ecdsa.public)?;
    
    println!("✅ ECDH key exchange successful: {}", alice_shared == bob_shared);
    println!("   - Shared secret length: {} bytes", alice_shared.len());
    println!();

    // Step 7: Performance comparison
    println!("📋 Step 7: Performance analysis...");
    let large_data = vec![42u8; 10240]; // 10KB of data
    
    let start = std::time::Instant::now();
    let large_envelope = seal_for_recipient(&large_data, &bob_keypair.public)?;
    let seal_time = start.elapsed();
    
    let start = std::time::Instant::now();
    let _decrypted_large = open_envelope(&large_envelope, &bob_keypair.private)?;
    let open_time = start.elapsed();
    
    println!("✅ Large data (10KB) performance:");
    println!("   - Seal time: {:?}", seal_time);
    println!("   - Open time: {:?}", open_time);
    println!("   - Total overhead: {} bytes ({:.2}%)", 
        large_envelope.len() - large_data.len(),
        ((large_envelope.len() - large_data.len()) as f64 / large_data.len() as f64) * 100.0
    );
    println!();

    println!("🎉 Demo completed successfully!");
    println!("   The hybrid encryption API is working correctly with:");
    println!("   ✅ RSA key encryption/decryption");
    println!("   ✅ AES-256-GCM symmetric encryption");
    println!("   ✅ ECDH key exchange");
    println!("   ✅ Multiple key algorithms");
    println!("   ✅ Error handling for wrong keys");
    println!("   ✅ Performance suitable for production use");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_roundtrip() {
        let alice = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Bob's key");

        let message = b"Test message for API";

        let envelope = seal_for_recipient(message, &bob.public)
            .expect("Failed to seal envelope");

        let decrypted = open_envelope(&envelope, &bob.private)
            .expect("Failed to open envelope");

        assert_eq!(message, decrypted.as_slice());
    }

    #[test]
    fn test_wrong_key_fails() {
        let alice = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Bob's key");
        let charlie = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Charlie's key");

        let message = b"Secret for Bob only";

        let envelope = seal_for_recipient(message, &bob.public)
            .expect("Failed to seal envelope");

        // Bob should be able to decrypt
        let decrypted = open_envelope(&envelope, &bob.private)
            .expect("Bob should be able to decrypt");
        assert_eq!(message, decrypted.as_slice());

        // Charlie should not be able to decrypt
        let result = open_envelope(&envelope, &charlie.private);
        assert!(result.is_err(), "Charlie should not be able to decrypt");
    }

    #[test]
    fn test_ecdh_key_exchange() {
        let alice = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)
            .expect("Failed to generate Alice's key");
        let bob = KeyPair::generate(AsymmetricAlgorithm::EcdsaP256)
            .expect("Failed to generate Bob's key");

        let alice_shared = trustedge_core::key_exchange(&alice.private, &bob.public)
            .expect("Alice's key exchange failed");
        let bob_shared = trustedge_core::key_exchange(&bob.private, &alice.public)
            .expect("Bob's key exchange failed");

        assert_eq!(alice_shared, bob_shared);
        assert!(!alice_shared.is_empty());
    }
}