// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Simple Demo of Clean Pubky Adapter
//!
//! This example demonstrates the clean adapter API using mock storage
//! to avoid network dependencies.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use trustedge_core::{KeyPair, backends::AsymmetricAlgorithm};
use trustedge_pubky::{
    mock::{MockPubkyAdapter, mock_send_trusted_data},
    receive_trusted_data,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new()?;
    
    rt.block_on(async {
        println!("🧪 TrustEdge Pubky Simple Demo (Mock Network)");
        println!("==============================================\n");

        // Step 1: Create shared mock storage
        println!("📋 Step 1: Setting up mock Pubky network...");
        let storage = Arc::new(Mutex::new(HashMap::new()));
        
        let alice_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());
        let bob_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());

        println!("✅ Alice's mock Pubky ID: {}", alice_adapter.our_pubky_id());
        println!("✅ Bob's mock Pubky ID: {}", bob_adapter.our_pubky_id());
        println!();

        // Step 2: Generate TrustEdge key pairs
        println!("📋 Step 2: Generating TrustEdge key pairs...");
        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;
        let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Rsa2048)?;

        println!("✅ Alice's TrustEdge key: {}", alice_keypair.public.id());
        println!("✅ Bob's TrustEdge key: {}", bob_keypair.public.id());
        println!();

        // Step 3: Publish keys to mock network
        println!("📋 Step 3: Publishing keys to mock network...");
        let alice_pubky_id = alice_adapter.publish_public_key(&alice_keypair.public).await?;
        let bob_pubky_id = bob_adapter.publish_public_key(&bob_keypair.public).await?;

        println!("✅ Alice published: {}", alice_pubky_id);
        println!("✅ Bob published: {}", bob_pubky_id);
        println!();

        // Step 4: Demonstrate the clean API
        println!("📋 Step 4: Using the clean adapter API...");
        let secret_message = b"Hello from Alice to Bob via clean Pubky adapter!";
        
        println!("   Message: {:?}", std::str::from_utf8(secret_message).unwrap());

        // This demonstrates your exact API specification!
        let sealed_envelope = mock_send_trusted_data(
            secret_message,
            &bob_pubky_id,  // recipient_id: &str
            storage.clone(), // mock network storage
        ).await?;

        println!("✅ Message sealed using clean API!");
        println!("   Envelope size: {} bytes", sealed_envelope.len());
        println!();

        // Step 5: Bob receives the message
        println!("📋 Step 5: Bob receives the message...");
        let decrypted_message = receive_trusted_data(&sealed_envelope, &bob_keypair.private).await?;

        println!("✅ Message decrypted successfully!");
        println!("   Decrypted: {:?}", std::str::from_utf8(&decrypted_message).unwrap());
        println!("   Data matches: {}", secret_message == decrypted_message.as_slice());
        println!();

        // Step 6: Show key resolution works
        println!("📋 Step 6: Testing key resolution...");
        let resolved_key = alice_adapter.resolve_public_key(&bob_pubky_id).await?;
        
        println!("✅ Key resolution successful!");
        println!("   Original key ID: {}", bob_keypair.public.id());
        println!("   Resolved key ID: {}", resolved_key.id());
        println!("   Keys match: {}", bob_keypair.public.id() == resolved_key.id());
        println!();

        println!("🎉 Simple Demo completed successfully!");
        println!("   This demonstrates the clean architecture:");
        println!("   ✅ trustedge-core: Handles all cryptography");
        println!("   ✅ trustedge-pubky: Bridges to Pubky network");
        println!("   ✅ Clean separation of concerns");
        println!("   ✅ Simple, easy-to-use API");

        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_demo_workflow() {
        let storage = Arc::new(Mutex::new(HashMap::new()));
        
        let alice_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());
        let bob_adapter = MockPubkyAdapter::with_shared_storage(storage.clone());

        let alice_keypair = KeyPair::generate(AsymmetricAlgorithm::Ed25519)
            .expect("Failed to generate Alice's key");
        let bob_keypair = KeyPair::generate(AsymmetricAlgorithm::Ed25519)
            .expect("Failed to generate Bob's key");

        let bob_pubky_id = bob_adapter.publish_public_key(&bob_keypair.public).await
            .expect("Failed to publish Bob's key");

        let message = b"Test workflow";
        
        let envelope = mock_send_trusted_data(message, &bob_pubky_id, storage).await
            .expect("Failed to send data");

        let decrypted = receive_trusted_data(&envelope, &bob_keypair.private).await
            .expect("Failed to receive data");

        assert_eq!(message, decrypted.as_slice());
    }
}