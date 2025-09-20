// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Clean Pubky Adapter Demo
//!
//! This example demonstrates the clean, simple adapter between TrustEdge core
//! and the Pubky network as specified in Step 3.

use trustedge_core::{AsymmetricAlgorithm, KeyPair};
use trustedge_pubky::{
    create_pubky_backend_random, receive_trusted_data, send_trusted_data, PubkyBackend,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 TrustEdge Pubky Clean Adapter Demo");
    println!("====================================\n");

    // Step 1: Create Pubky adapters for Alice and Bob
    println!("📋 Step 1: Creating Pubky adapters...");
    let alice_adapter = create_pubky_backend_random()?;
    let bob_adapter = create_pubky_backend_random()?;

    println!("✅ Alice's Pubky ID: {}", alice_adapter.our_pubky_id());
    println!("✅ Bob's Pubky ID: {}", bob_adapter.our_pubky_id());
    println!();

    // Step 2: Generate TrustEdge key pairs
    println!("📋 Step 2: Generating TrustEdge key pairs...");
    let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;
    let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;

    println!("✅ Alice's TrustEdge key ID: {}", alice_keypair.public.id());
    println!("✅ Bob's TrustEdge key ID: {}", bob_keypair.public.id());
    println!();

    // Step 3: Publish public keys to Pubky network
    println!("📋 Step 3: Publishing public keys to Pubky network...");
    let alice_pubky_id = alice_adapter.publish_public_key(&alice_keypair.public)?;
    let bob_pubky_id = bob_adapter.publish_public_key(&bob_keypair.public)?;

    println!("✅ Alice published key with Pubky ID: {}", alice_pubky_id);
    println!("✅ Bob published key with Pubky ID: {}", bob_pubky_id);
    println!();

    // Step 4: Demonstrate the clean API - Alice sends data to Bob
    println!("📋 Step 4: Alice sends trusted data to Bob...");
    let secret_message = b"Hello Bob! This is a secret message sent via Pubky network resolution!";

    println!(
        "   Original message: {:?}",
        std::str::from_utf8(secret_message).unwrap()
    );
    println!("   Message size: {} bytes", secret_message.len());

    // This is your exact API specification!
    let sealed_envelope = send_trusted_data(
        secret_message,
        &bob_pubky_id,  // Pubky ID for resolution
        &alice_adapter, // Alice's adapter for network access
    )?;

    println!("✅ Message sealed successfully!");
    println!("   Envelope size: {} bytes", sealed_envelope.len());
    println!(
        "   Overhead: {} bytes ({:.1}%)",
        sealed_envelope.len() - secret_message.len(),
        ((sealed_envelope.len() - secret_message.len()) as f64 / secret_message.len() as f64)
            * 100.0
    );
    println!();

    // Step 5: Bob receives and decrypts the data
    println!("📋 Step 5: Bob receives trusted data...");
    let decrypted_message = receive_trusted_data(&sealed_envelope, &bob_keypair.private)?;

    println!("✅ Message decrypted successfully!");
    println!(
        "   Decrypted message: {:?}",
        std::str::from_utf8(&decrypted_message).unwrap()
    );
    println!(
        "   Data matches: {}",
        secret_message == decrypted_message.as_slice()
    );
    println!();

    // Step 6: Demonstrate key resolution
    println!("📋 Step 6: Testing key resolution...");
    let resolved_bob_key = alice_adapter.resolve_public_key(&bob_pubky_id)?;

    println!("✅ Key resolution successful!");
    println!("   Resolved key ID: {}", resolved_bob_key.id());
    println!("   Original key ID: {}", bob_keypair.public.id());
    println!(
        "   Keys match: {}",
        resolved_bob_key.id() == bob_keypair.public.id()
    );
    println!();

    // Step 7: Test with different key types
    println!("📋 Step 7: Testing with Ed25519 keys...");
    let alice_ed25519 = KeyPair::generate(AsymmetricAlgorithm::Ed25519)?;
    let alice_ed25519_pubky_id = alice_adapter.publish_public_key(&alice_ed25519.public)?;

    println!("✅ Ed25519 key published: {}", alice_ed25519_pubky_id);

    let resolved_ed25519 = bob_adapter.resolve_public_key(&alice_ed25519_pubky_id)?;
    println!("✅ Ed25519 key resolved: {}", resolved_ed25519.id());
    println!();

    // Step 8: Performance test
    println!("📋 Step 8: Performance test with larger data...");
    let large_data = vec![42u8; 10240]; // 10KB

    let start = std::time::Instant::now();
    let large_envelope = send_trusted_data(&large_data, &bob_pubky_id, &alice_adapter)?;
    let send_time = start.elapsed();

    let start = std::time::Instant::now();
    let _decrypted_large = receive_trusted_data(&large_envelope, &bob_keypair.private)?;
    let receive_time = start.elapsed();

    println!("✅ Large data performance:");
    println!("   Send time: {:?}", send_time);
    println!("   Receive time: {:?}", receive_time);
    println!(
        "   Overhead: {} bytes ({:.2}%)",
        large_envelope.len() - large_data.len(),
        ((large_envelope.len() - large_data.len()) as f64 / large_data.len() as f64) * 100.0
    );
    println!();

    println!("🎉 Clean Adapter Demo completed successfully!");
    println!("   The adapter provides exactly the API you specified:");
    println!("   ✅ send_trusted_data(data, recipient_id, adapter)");
    println!("   ✅ receive_trusted_data(envelope, private_key)");
    println!("   ✅ Clean separation between core crypto and Pubky network");
    println!("   ✅ Automatic key resolution via Pubky network");
    println!("   ✅ Support for multiple key algorithms");
    println!("   ✅ Production-ready performance");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clean_adapter_api() {
        // Create adapters
        let alice_adapter =
            create_pubky_backend_random().expect("Failed to create Alice's adapter");
        let bob_adapter = create_pubky_backend_random().expect("Failed to create Bob's adapter");

        // Generate keys
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)
            .expect("Failed to generate Alice's key");
        let bob_keypair =
            KeyPair::generate(AsymmetricAlgorithm::Rsa2048).expect("Failed to generate Bob's key");

        // Publish Bob's key
        let bob_pubky_id = bob_adapter
            .publish_public_key(&bob_keypair.public)
            .expect("Failed to publish Bob's key");

        // Test the clean API
        let message = b"Test message for clean API";

        let envelope = send_trusted_data(message, &bob_pubky_id, &alice_adapter)
            .expect("Failed to send trusted data");

        let decrypted = receive_trusted_data(&envelope, &bob_keypair.private)
            .expect("Failed to receive trusted data");

        assert_eq!(message, decrypted.as_slice());
    }

    #[tokio::test]
    async fn test_key_resolution() {
        let adapter = create_pubky_backend_random().expect("Failed to create adapter");

        let keypair =
            KeyPair::generate(AsymmetricAlgorithm::Ed25519).expect("Failed to generate key pair");

        let pubky_id = adapter
            .publish_public_key(&keypair.public)
            .expect("Failed to publish key");

        let resolved_key = adapter
            .resolve_public_key(&pubky_id)
            .expect("Failed to resolve key");

        assert_eq!(keypair.public.id(), resolved_key.id());
        assert_eq!(keypair.public.algorithm, resolved_key.algorithm);
    }
}
