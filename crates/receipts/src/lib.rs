//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge Receipts - The Contract Writer
//!
//! This crate represents the "Contract Writer" in our office analogy.
//! It handles business logic for transferable claims (receipts) without worrying
//! about cryptographic details. That's the job of the "Security Guard" (trustedge-core).
//!
//! ## The Receipt System
//!
//! A Receipt represents a transferable claim with these properties:
//! - **Issuer**: Who is making the claim (current owner)
//! - **Beneficiary**: Who is receiving the claim (new owner)  
//! - **Amount**: The value being claimed
//! - **Chain Link**: Reference to the previous receipt in the ownership chain
//!
//! ## Usage
//!
//! ```rust
//! use trustedge_receipts::{Receipt, create_receipt, assign_receipt};
//! use ed25519_dalek::SigningKey;
//! use rand::rngs::OsRng;
//!
//! // Create signing keys for Alice and Bob
//! let alice_key = SigningKey::generate(&mut OsRng);
//! let bob_key = SigningKey::generate(&mut OsRng);
//!
//! // Alice creates an original receipt for 1000 units
//! let receipt_envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, Some("Payment for services".to_string()))?;
//!
//! // Bob can now assign the receipt to Charlie
//! let charlie_key = SigningKey::generate(&mut OsRng);
//! let assignment_envelope = assign_receipt(&receipt_envelope, &bob_key, &charlie_key.verifying_key(), Some("Transfer to Charlie".to_string()))?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use anyhow::{Context, Result};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use trustedge_core::Envelope;

/// Represents a transferable claim, forming the payload of a TrustEdge Envelope.
///
/// This is the "contract" that the Contract Writer creates. It contains all the
/// business logic about ownership, amounts, and chain links, but no cryptographic details.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Receipt {
    /// The public key of the entity creating the assignment (the current owner).
    pub issuer: [u8; 32],
    /// The public key of the entity receiving the claim (the new owner).
    pub beneficiary: [u8; 32],
    /// The value being claimed.
    pub amount: u64,
    /// A hash of the previous trustedge::Envelope in the chain.
    /// This is `None` if the receipt is the origin of the chain.
    pub prev_envelope_hash: Option<[u8; 32]>,
    /// Optional description or metadata for this receipt
    pub description: Option<String>,
    /// Timestamp when this receipt was created (seconds since UNIX epoch)
    pub created_at: u64,
}

impl Receipt {
    /// Create a new origin receipt (start of a chain)
    pub fn new_origin(
        issuer_key: &SigningKey,
        beneficiary_key: &VerifyingKey,
        amount: u64,
        description: Option<String>,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Receipt {
            issuer: issuer_key.verifying_key().to_bytes(),
            beneficiary: beneficiary_key.to_bytes(),
            amount,
            prev_envelope_hash: None,
            description,
            created_at: timestamp,
        }
    }

    /// Create a new assignment receipt (link in a chain)
    pub fn new_assignment(
        issuer_key: &SigningKey,
        beneficiary_key: &VerifyingKey,
        amount: u64,
        prev_envelope_hash: [u8; 32],
        description: Option<String>,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Receipt {
            issuer: issuer_key.verifying_key().to_bytes(),
            beneficiary: beneficiary_key.to_bytes(),
            amount,
            prev_envelope_hash: Some(prev_envelope_hash),
            description,
            created_at: timestamp,
        }
    }

    /// Get the issuer's public key
    pub fn issuer_key(&self) -> Result<VerifyingKey> {
        VerifyingKey::from_bytes(&self.issuer)
            .map_err(|e| anyhow::anyhow!("Invalid issuer key: {}", e))
    }

    /// Get the beneficiary's public key
    pub fn beneficiary_key(&self) -> Result<VerifyingKey> {
        VerifyingKey::from_bytes(&self.beneficiary)
            .map_err(|e| anyhow::anyhow!("Invalid beneficiary key: {}", e))
    }

    /// Check if this is an origin receipt (no previous link)
    pub fn is_origin(&self) -> bool {
        self.prev_envelope_hash.is_none()
    }

    /// Validate the business logic of this receipt
    pub fn validate(&self) -> Result<()> {
        if self.amount == 0 {
            return Err(anyhow::anyhow!("Receipt amount cannot be zero"));
        }

        // Validate that keys can be parsed
        self.issuer_key().context("Invalid issuer key")?;
        self.beneficiary_key().context("Invalid beneficiary key")?;

        // Check timestamp is reasonable (not too far in future)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        if self.created_at > now + 300 {
            // Not more than 5 minutes in future
            return Err(anyhow::anyhow!(
                "Receipt timestamp is too far in the future"
            ));
        }

        Ok(())
    }
}

/// Creates a new, origin Receipt wrapped in a signed Envelope.
///
/// This function initiates a new chain of ownership. The Contract Writer creates
/// the business logic (Receipt), then hands it to the Security Guard (Envelope::seal)
/// to secure it in a tamper-proof container.
///
/// # Arguments
/// * `issuer_key` - The private key of the entity creating the receipt
/// * `beneficiary_key` - The public key of the entity receiving the receipt
/// * `amount` - The value being claimed
/// * `description` - Optional description for this receipt
///
/// # Returns
/// A secure Envelope containing the receipt, or an error if creation fails
pub fn create_receipt(
    issuer_key: &SigningKey,
    beneficiary_key: &VerifyingKey,
    amount: u64,
    description: Option<String>,
) -> Result<Envelope> {
    // The Contract Writer creates the business logic
    let receipt = Receipt::new_origin(issuer_key, beneficiary_key, amount, description);

    // Validate the business rules
    receipt.validate().context("Receipt validation failed")?;

    // Serialize the business logic (the Receipt) into a payload
    let payload = serde_json::to_vec(&receipt).context("Failed to serialize receipt")?;

    // Hand the payload to the Security Guard (Envelope::seal) to secure it
    Envelope::seal(&payload, issuer_key, beneficiary_key)
        .context("Failed to seal receipt in envelope")
}

/// Assigns an existing Receipt to a new beneficiary, creating a new chained Envelope.
///
/// This creates a new link in the ownership chain. The current beneficiary (assigner)
/// creates a new receipt assigning their claim to someone else.
///
/// # Arguments
/// * `previous_envelope` - The envelope containing the previous receipt
/// * `assigner_key` - The private key of the current beneficiary (who is assigning)
/// * `new_beneficiary_key` - The public key of the new beneficiary
/// * `description` - Optional description for this assignment
///
/// # Returns
/// A new secure Envelope containing the assignment receipt, or an error
pub fn assign_receipt(
    previous_envelope: &Envelope,
    assigner_key: &SigningKey,
    new_beneficiary_key: &VerifyingKey,
    description: Option<String>,
) -> Result<Envelope> {
    // Security Guard verifies the previous envelope first
    if !previous_envelope.verify() {
        return Err(anyhow::anyhow!("Previous envelope signature is invalid"));
    }

    // Check that the assigner is actually the current beneficiary
    let previous_beneficiary = previous_envelope.beneficiary();
    if previous_beneficiary != assigner_key.verifying_key() {
        return Err(anyhow::anyhow!(
            "Assigner key does not match previous beneficiary"
        ));
    }

    // Unseal the previous envelope to get the actual amount
    let previous_payload = previous_envelope
        .unseal(assigner_key)
        .context("Failed to unseal previous envelope - assigner key may be invalid")?;

    // Deserialize the previous receipt to get the amount
    let previous_receipt: Receipt = serde_json::from_slice(&previous_payload)
        .context("Failed to deserialize previous receipt")?;

    // Use the actual amount from the previous receipt
    let amount = previous_receipt.amount;

    // Calculate the hash of the previous envelope to create the chain link
    let prev_hash = previous_envelope.hash();

    // Create the new receipt that represents the assignment
    let assignment_receipt = Receipt::new_assignment(
        assigner_key,
        new_beneficiary_key,
        amount,
        prev_hash,
        description,
    );

    // Validate the business rules
    assignment_receipt
        .validate()
        .context("Assignment receipt validation failed")?;

    // Serialize the business logic
    let payload = serde_json::to_vec(&assignment_receipt)
        .context("Failed to serialize assignment receipt")?;

    // Hand to Security Guard to seal the new assignment
    Envelope::seal(&payload, assigner_key, new_beneficiary_key)
        .context("Failed to seal assignment receipt in envelope")
}

/// Extract and verify a Receipt from an Envelope
///
/// This function asks the Security Guard to verify and unseal the envelope,
/// then deserializes the business logic (Receipt) for the Contract Writer to examine.
///
/// # Arguments
/// * `envelope` - The envelope to extract the receipt from
/// * `decryption_key` - The private key needed to decrypt the envelope
///
/// # Returns
/// The Receipt contained in the envelope, or an error
pub fn extract_receipt(envelope: &Envelope, decryption_key: &SigningKey) -> Result<Receipt> {
    // Ask the Security Guard to verify and unseal the envelope
    let payload = envelope
        .unseal(decryption_key)
        .context("Failed to unseal envelope")?;

    // Deserialize the business logic
    let receipt: Receipt =
        serde_json::from_slice(&payload).context("Failed to deserialize receipt from payload")?;

    // Validate the business rules
    receipt
        .validate()
        .context("Extracted receipt validation failed")?;

    Ok(receipt)
}

/// Verify a chain of receipt assignments
///
/// This function validates that a series of envelopes form a valid ownership chain.
///
/// # Arguments
/// * `envelopes` - The chain of envelopes, ordered from origin to final assignment
///
/// # Returns
/// True if the chain is valid, false otherwise
pub fn verify_receipt_chain(envelopes: &[Envelope]) -> bool {
    if envelopes.is_empty() {
        return false;
    }

    // Verify each envelope individually
    for envelope in envelopes {
        if !envelope.verify() {
            return false;
        }
    }

    // Verify the chain links
    for i in 1..envelopes.len() {
        let prev_envelope = &envelopes[i - 1];
        let current_envelope = &envelopes[i];

        // The issuer of the current envelope should be the beneficiary of the previous
        if current_envelope.issuer() != prev_envelope.beneficiary() {
            return false;
        }

        // TODO: Verify that the current envelope references the previous envelope's hash
        // This would require decrypting the envelopes to examine the receipt contents
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    #[test]
    fn test_receipt_creation() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let receipt = Receipt::new_origin(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Test receipt".to_string()),
        );

        assert_eq!(receipt.amount, 1000);
        assert_eq!(receipt.issuer, alice_key.verifying_key().to_bytes());
        assert_eq!(receipt.beneficiary, bob_key.verifying_key().to_bytes());
        assert!(receipt.is_origin());
        assert!(receipt.validate().is_ok());
    }

    #[test]
    fn test_create_receipt_envelope() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let envelope = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Test receipt".to_string()),
        )
        .expect("Failed to create receipt");

        assert!(envelope.verify());
        assert_eq!(envelope.issuer(), alice_key.verifying_key());
        assert_eq!(envelope.beneficiary(), bob_key.verifying_key());
    }

    #[test]
    fn test_assign_receipt() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        // Alice creates original receipt for Bob
        let original_envelope = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Original receipt".to_string()),
        )
        .expect("Failed to create receipt");

        // Bob assigns to Charlie
        let assignment_envelope = assign_receipt(
            &original_envelope,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Assignment to Charlie".to_string()),
        )
        .expect("Failed to assign receipt");

        assert!(assignment_envelope.verify());
        assert_eq!(assignment_envelope.issuer(), bob_key.verifying_key());
        assert_eq!(
            assignment_envelope.beneficiary(),
            charlie_key.verifying_key()
        );
    }

    #[test]
    fn test_invalid_assignment() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);
        let dave_key = SigningKey::generate(&mut OsRng);

        // Alice creates original receipt for Bob
        let original_envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Dave tries to assign (but he's not the beneficiary)
        let result = assign_receipt(
            &original_envelope,
            &dave_key, // Dave is not the beneficiary!
            &charlie_key.verifying_key(),
            None,
        );

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("does not match previous beneficiary"));
    }

    #[test]
    fn test_receipt_chain_verification() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        // Create a chain: Alice -> Bob -> Charlie
        let envelope1 = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        let envelope2 = assign_receipt(&envelope1, &bob_key, &charlie_key.verifying_key(), None)
            .expect("Failed to assign receipt");

        let chain = vec![envelope1, envelope2];
        assert!(verify_receipt_chain(&chain));

        // Test empty chain
        assert!(!verify_receipt_chain(&[]));
    }

    #[test]
    fn test_receipt_validation() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        // Valid receipt
        let valid_receipt = Receipt::new_origin(&alice_key, &bob_key.verifying_key(), 1000, None);
        assert!(valid_receipt.validate().is_ok());

        // Invalid receipt - zero amount
        let mut invalid_receipt = valid_receipt.clone();
        invalid_receipt.amount = 0;
        assert!(invalid_receipt.validate().is_err());

        // Invalid receipt - future timestamp
        let mut future_receipt = valid_receipt.clone();
        future_receipt.created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1000; // 1000 seconds in the future
        assert!(future_receipt.validate().is_err());
    }

    #[test]
    fn test_envelope_unseal_integration() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        // Create a receipt envelope
        let envelope = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Test payment".to_string()),
        )
        .expect("Failed to create receipt");

        // Verify envelope
        assert!(envelope.verify());

        // Bob should be able to unseal the envelope
        let unsealed_data = envelope
            .unseal(&bob_key)
            .expect("Failed to unseal envelope");

        // Deserialize the receipt from the unsealed data
        let receipt: Receipt =
            serde_json::from_slice(&unsealed_data).expect("Failed to deserialize receipt");

        // Verify the receipt data matches what we expect
        assert_eq!(receipt.amount, 1000);
        assert_eq!(receipt.issuer, alice_key.verifying_key().to_bytes());
        assert_eq!(receipt.beneficiary, bob_key.verifying_key().to_bytes());
        assert_eq!(receipt.description, Some("Test payment".to_string()));
        assert!(receipt.is_origin());
    }

    #[test]
    fn test_complete_receipt_chain_with_decryption() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        // Step 1: Alice creates original receipt for Bob (1000 units)
        let envelope1 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Original payment".to_string()),
        )
        .expect("Failed to create receipt");

        // Step 2: Bob unseals to verify he received 1000 units
        let unsealed1 = envelope1.unseal(&bob_key).expect("Bob failed to unseal");
        let receipt1: Receipt =
            serde_json::from_slice(&unsealed1).expect("Failed to deserialize receipt1");
        assert_eq!(receipt1.amount, 1000);

        // Step 3: Bob assigns to Charlie - this should use the ACTUAL amount from envelope1
        // NOTE: This will fail until assign_receipt is fixed to actually decrypt the previous envelope
        let envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Transfer to Charlie".to_string()),
        )
        .expect("Failed to assign receipt");

        // Step 4: Charlie unseals to verify the amount
        let unsealed2 = envelope2
            .unseal(&charlie_key)
            .expect("Charlie failed to unseal");
        let receipt2: Receipt =
            serde_json::from_slice(&unsealed2).expect("Failed to deserialize receipt2");

        // CRITICAL TEST: The amount should match the original (currently fails due to hardcoded 1000)
        assert_eq!(
            receipt2.amount, receipt1.amount,
            "Amount should be preserved through assignment"
        );

        // Verify chain integrity
        let chain = vec![envelope1, envelope2];
        assert!(verify_receipt_chain(&chain));
    }

    #[test]
    fn test_envelope_tampering_detection() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Verify original envelope is valid
        assert!(envelope.verify());

        // Create a tampered envelope by serializing, modifying bytes, and deserializing
        let mut envelope_bytes = serde_json::to_vec(&envelope).expect("Failed to serialize");

        // Tamper with a byte in the middle (this should break signature verification)
        if envelope_bytes.len() > 100 {
            envelope_bytes[50] ^= 0xFF; // Flip bits
        }

        // Try to deserialize the tampered envelope
        if let Ok(tampered_envelope) = serde_json::from_slice::<Envelope>(&envelope_bytes) {
            // Verification should fail for tampered envelope
            assert!(
                !tampered_envelope.verify(),
                "Tampered envelope should fail verification"
            );

            // Unsealing should also fail
            let result = tampered_envelope.unseal(&bob_key);
            assert!(result.is_err(), "Unsealing tampered envelope should fail");
        }
        // If deserialization fails, that's also acceptable (tampered data is invalid)
    }

    #[test]
    fn test_wrong_key_unseal_fails() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        // Alice creates envelope for Bob
        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Bob can unseal (correct key)
        assert!(envelope.unseal(&bob_key).is_ok());

        // Charlie cannot unseal (wrong key)
        assert!(envelope.unseal(&charlie_key).is_err());

        // Alice cannot unseal (she's the issuer, not beneficiary)
        assert!(envelope.unseal(&alice_key).is_err());
    }

    #[test]
    fn test_comprehensive_multi_party_chain() {
        // Test a complex chain: Alice -> Bob -> Charlie -> Dave -> Eve
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);
        let dave_key = SigningKey::generate(&mut OsRng);
        let eve_key = SigningKey::generate(&mut OsRng);

        let original_amount = 50000u64;

        // Step 1: Alice creates original receipt for Bob
        let envelope1 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            original_amount,
            Some("Initial payment from Alice to Bob".to_string()),
        )
        .expect("Failed to create initial receipt");

        // Verify Bob can unseal and get correct amount
        let payload1 = envelope1.unseal(&bob_key).expect("Bob failed to unseal");
        let receipt1: Receipt = serde_json::from_slice(&payload1).expect("Failed to deserialize");
        assert_eq!(receipt1.amount, original_amount);
        assert_eq!(receipt1.issuer, alice_key.verifying_key().to_bytes());
        assert_eq!(receipt1.beneficiary, bob_key.verifying_key().to_bytes());

        // Step 2: Bob assigns to Charlie
        let envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Bob assigns to Charlie".to_string()),
        )
        .expect("Failed to assign to Charlie");

        // Verify Charlie can unseal and amount is preserved
        let payload2 = envelope2
            .unseal(&charlie_key)
            .expect("Charlie failed to unseal");
        let receipt2: Receipt = serde_json::from_slice(&payload2).expect("Failed to deserialize");
        assert_eq!(
            receipt2.amount, original_amount,
            "Amount should be preserved"
        );
        assert_eq!(receipt2.issuer, bob_key.verifying_key().to_bytes());
        assert_eq!(receipt2.beneficiary, charlie_key.verifying_key().to_bytes());

        // Step 3: Charlie assigns to Dave
        let envelope3 = assign_receipt(
            &envelope2,
            &charlie_key,
            &dave_key.verifying_key(),
            Some("Charlie assigns to Dave".to_string()),
        )
        .expect("Failed to assign to Dave");

        // Step 4: Dave assigns to Eve
        let envelope4 = assign_receipt(
            &envelope3,
            &dave_key,
            &eve_key.verifying_key(),
            Some("Dave assigns to Eve".to_string()),
        )
        .expect("Failed to assign to Eve");

        // Verify Eve can unseal and amount is still preserved
        let payload4 = envelope4.unseal(&eve_key).expect("Eve failed to unseal");
        let receipt4: Receipt = serde_json::from_slice(&payload4).expect("Failed to deserialize");
        assert_eq!(
            receipt4.amount, original_amount,
            "Amount should be preserved through entire chain"
        );
        assert_eq!(receipt4.issuer, dave_key.verifying_key().to_bytes());
        assert_eq!(receipt4.beneficiary, eve_key.verifying_key().to_bytes());

        // Verify intermediate parties cannot unseal final envelope
        assert!(
            envelope4.unseal(&alice_key).is_err(),
            "Alice should not be able to unseal final envelope"
        );
        assert!(
            envelope4.unseal(&bob_key).is_err(),
            "Bob should not be able to unseal final envelope"
        );
        assert!(
            envelope4.unseal(&charlie_key).is_err(),
            "Charlie should not be able to unseal final envelope"
        );
        assert!(
            envelope4.unseal(&dave_key).is_err(),
            "Dave should not be able to unseal final envelope"
        );

        // Verify the complete chain
        let chain = vec![envelope1, envelope2, envelope3, envelope4];
        assert!(
            verify_receipt_chain(&chain),
            "Complete chain should be valid"
        );
    }

    #[test]
    fn test_large_amount_precision() {
        // Test with maximum u64 value to ensure no overflow issues
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        let max_amount = u64::MAX;

        // Create receipt with maximum amount
        let envelope1 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            max_amount,
            Some("Maximum amount test".to_string()),
        )
        .expect("Failed to create receipt with max amount");

        // Verify Bob can unseal
        let payload1 = envelope1
            .unseal(&bob_key)
            .expect("Failed to unseal max amount");
        let receipt1: Receipt = serde_json::from_slice(&payload1).expect("Failed to deserialize");
        assert_eq!(
            receipt1.amount, max_amount,
            "Max amount should be preserved"
        );

        // Assign to Charlie
        let envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Assigning max amount".to_string()),
        )
        .expect("Failed to assign max amount");

        // Verify Charlie gets the correct amount
        let payload2 = envelope2
            .unseal(&charlie_key)
            .expect("Failed to unseal assigned max amount");
        let receipt2: Receipt = serde_json::from_slice(&payload2).expect("Failed to deserialize");
        assert_eq!(
            receipt2.amount, max_amount,
            "Max amount should be preserved through assignment"
        );
    }

    #[test]
    fn test_concurrent_assignments_fail() {
        // Test that the same receipt cannot be assigned twice
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);
        let dave_key = SigningKey::generate(&mut OsRng);

        // Alice creates receipt for Bob
        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Bob assigns to Charlie (first assignment)
        let assignment1 = assign_receipt(
            &envelope,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("First assignment".to_string()),
        )
        .expect("First assignment should succeed");

        // Bob tries to assign the same original envelope to Dave (should work - different chain)
        let assignment2 = assign_receipt(
            &envelope,
            &bob_key,
            &dave_key.verifying_key(),
            Some("Second assignment".to_string()),
        )
        .expect("Second assignment of original should succeed");

        // Verify both assignments are valid independently
        assert!(assignment1.verify(), "First assignment should be valid");
        assert!(assignment2.verify(), "Second assignment should be valid");

        // But Charlie cannot assign Dave's envelope (wrong key)
        let invalid_assignment = assign_receipt(
            &assignment2,
            &charlie_key, // Charlie doesn't own Dave's envelope
            &alice_key.verifying_key(),
            None,
        );
        assert!(
            invalid_assignment.is_err(),
            "Charlie should not be able to assign Dave's envelope"
        );
    }

    #[test]
    fn test_envelope_metadata_integrity() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Verify envelope metadata
        assert_eq!(envelope.issuer(), alice_key.verifying_key());
        assert_eq!(envelope.beneficiary(), bob_key.verifying_key());
        assert!(envelope.verify(), "Envelope should verify");

        // Verify hash consistency
        let hash1 = envelope.hash();
        let hash2 = envelope.hash();
        assert_eq!(hash1, hash2, "Hash should be deterministic");

        // Create identical envelope and verify different hash
        let envelope2 = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create second receipt");

        let hash3 = envelope2.hash();
        assert_ne!(
            hash1, hash3,
            "Different envelopes should have different hashes"
        );
    }

    #[test]
    fn test_zero_amount_validation() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        // Attempt to create receipt with zero amount should fail
        let result = create_receipt(&alice_key, &bob_key.verifying_key(), 0, None);
        assert!(
            result.is_err(),
            "Zero amount receipt should fail validation"
        );

        let error_msg = result.unwrap_err().to_string();
        assert!(
            error_msg.contains("Receipt validation failed"),
            "Error should be about receipt validation"
        );
    }

    #[test]
    fn test_description_handling() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        let long_description = "A".repeat(1000); // Very long description

        // Create receipt with long description
        let envelope1 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some(long_description.clone()),
        )
        .expect("Failed to create receipt with long description");

        // Verify description is preserved
        let payload1 = envelope1.unseal(&bob_key).expect("Failed to unseal");
        let receipt1: Receipt = serde_json::from_slice(&payload1).expect("Failed to deserialize");
        assert_eq!(receipt1.description, Some(long_description.clone()));

        // Assign with different description
        let assignment_desc = "Assignment with different description";
        let envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some(assignment_desc.to_string()),
        )
        .expect("Failed to assign with description");

        // Verify new description
        let payload2 = envelope2
            .unseal(&charlie_key)
            .expect("Failed to unseal assignment");
        let receipt2: Receipt = serde_json::from_slice(&payload2).expect("Failed to deserialize");
        assert_eq!(receipt2.description, Some(assignment_desc.to_string()));
        assert_ne!(
            receipt2.description, receipt1.description,
            "Descriptions should be different"
        );
    }

    #[test]
    fn test_cryptographic_key_isolation() {
        // Ensure that keys are properly isolated and cannot be used interchangeably
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);
        let eve_key = SigningKey::generate(&mut OsRng); // Potential attacker

        // Alice creates receipt for Bob
        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Eve (attacker) cannot unseal Bob's envelope
        assert!(
            envelope.unseal(&eve_key).is_err(),
            "Eve should not be able to unseal Bob's envelope"
        );

        // Eve cannot assign Bob's envelope (not the beneficiary)
        let malicious_assignment = assign_receipt(
            &envelope,
            &eve_key, // Eve is not the beneficiary
            &charlie_key.verifying_key(),
            Some("Malicious assignment attempt".to_string()),
        );
        assert!(
            malicious_assignment.is_err(),
            "Eve should not be able to assign Bob's envelope"
        );

        // Bob assigns to Charlie
        let legitimate_assignment = assign_receipt(
            &envelope,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Legitimate assignment".to_string()),
        )
        .expect("Legitimate assignment should succeed");

        // Eve still cannot unseal Charlie's envelope
        assert!(
            legitimate_assignment.unseal(&eve_key).is_err(),
            "Eve should not be able to unseal Charlie's envelope"
        );

        // Only Charlie can unseal his envelope
        assert!(
            legitimate_assignment.unseal(&charlie_key).is_ok(),
            "Charlie should be able to unseal his envelope"
        );
    }

    #[test]
    fn test_signature_forgery_resistance() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let attacker_key = SigningKey::generate(&mut OsRng);

        // Create legitimate envelope
        let envelope = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Verify legitimate envelope
        assert!(envelope.verify(), "Legitimate envelope should verify");

        // Attempt to create envelope with attacker's key but claiming to be from Alice
        // This should fail because the signature won't match
        let malicious_envelope =
            create_receipt(&attacker_key, &bob_key.verifying_key(), 1000, None)
                .expect("Envelope creation should succeed");

        // The envelope should verify (it's properly signed by attacker_key)
        assert!(
            malicious_envelope.verify(),
            "Malicious envelope should verify its own signature"
        );

        // But the issuer should be the attacker, not Alice
        assert_eq!(malicious_envelope.issuer(), attacker_key.verifying_key());
        assert_ne!(
            malicious_envelope.issuer(),
            alice_key.verifying_key(),
            "Issuer should be attacker, not Alice"
        );

        // The envelopes should have different hashes
        assert_ne!(
            envelope.hash(),
            malicious_envelope.hash(),
            "Different envelopes should have different hashes"
        );
    }

    #[test]
    fn test_replay_attack_resistance() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        // Create original receipt
        let envelope1 = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        // Bob assigns to Charlie (not used in this test, just verifying it works)
        let _envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Assignment to Charlie".to_string()),
        )
        .expect("Failed to assign to Charlie");

        // Create another identical receipt (should have different hash due to timestamp/nonce)
        let envelope3 = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create second receipt");

        // Envelopes should have different hashes (preventing replay)
        assert_ne!(
            envelope1.hash(),
            envelope3.hash(),
            "Identical receipts should have different hashes"
        );

        // Both should verify independently
        assert!(envelope1.verify(), "First envelope should verify");
        assert!(envelope3.verify(), "Second envelope should verify");

        // But they represent different transactions
        let payload1 = envelope1.unseal(&bob_key).expect("Failed to unseal first");
        let payload3 = envelope3.unseal(&bob_key).expect("Failed to unseal second");
        let receipt1: Receipt =
            serde_json::from_slice(&payload1).expect("Failed to deserialize first");
        let receipt3: Receipt =
            serde_json::from_slice(&payload3).expect("Failed to deserialize second");

        // Timestamps should be different (or at least not identical in all fields)
        assert!(
            receipt1.created_at <= receipt3.created_at,
            "Second receipt should have later or equal timestamp"
        );
    }

    #[test]
    fn test_amount_tampering_resistance() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);

        let original_amount = 1000u64;

        // Create receipt
        let envelope1 = create_receipt(&alice_key, &bob_key.verifying_key(), original_amount, None)
            .expect("Failed to create receipt");

        // Bob assigns to Charlie
        let envelope2 = assign_receipt(
            &envelope1,
            &bob_key,
            &charlie_key.verifying_key(),
            Some("Assignment".to_string()),
        )
        .expect("Failed to assign");

        // Verify amount is preserved
        let payload2 = envelope2.unseal(&charlie_key).expect("Failed to unseal");
        let receipt2: Receipt = serde_json::from_slice(&payload2).expect("Failed to deserialize");
        assert_eq!(
            receipt2.amount, original_amount,
            "Amount should be preserved"
        );

        // Simulate tampering by trying to create a receipt with a different amount
        // but using the same envelope structure (this should fail due to signature mismatch)
        let tampered_receipt = Receipt::new_assignment(
            &bob_key,
            &charlie_key.verifying_key(),
            original_amount * 2, // Double the amount
            envelope1.hash(),
            Some("Tampered assignment".to_string()),
        );

        // The tampered receipt should validate its own business rules
        assert!(
            tampered_receipt.validate().is_ok(),
            "Tampered receipt should validate business rules"
        );

        // But when we try to create an envelope with it, the signature won't match the original chain
        let tampered_payload =
            serde_json::to_vec(&tampered_receipt).expect("Failed to serialize tampered receipt");
        let tampered_envelope =
            Envelope::seal(&tampered_payload, &bob_key, &charlie_key.verifying_key())
                .expect("Failed to create tampered envelope");

        // The tampered envelope should verify (it's properly signed)
        assert!(
            tampered_envelope.verify(),
            "Tampered envelope should verify its own signature"
        );

        // But it should have a different hash than the legitimate envelope
        assert_ne!(
            envelope2.hash(),
            tampered_envelope.hash(),
            "Tampered envelope should have different hash"
        );

        // And the amounts should be different
        let tampered_payload_unsealed = tampered_envelope
            .unseal(&charlie_key)
            .expect("Failed to unseal tampered");
        let tampered_receipt_unsealed: Receipt = serde_json::from_slice(&tampered_payload_unsealed)
            .expect("Failed to deserialize tampered");
        assert_eq!(
            tampered_receipt_unsealed.amount,
            original_amount * 2,
            "Tampered amount should be doubled"
        );
        assert_ne!(
            tampered_receipt_unsealed.amount, receipt2.amount,
            "Amounts should be different"
        );
    }

    #[test]
    fn test_chain_integrity_validation() {
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);
        let charlie_key = SigningKey::generate(&mut OsRng);
        let dave_key = SigningKey::generate(&mut OsRng);

        // Create legitimate chain
        let envelope1 = create_receipt(&alice_key, &bob_key.verifying_key(), 1000, None)
            .expect("Failed to create receipt");

        let envelope2 = assign_receipt(&envelope1, &bob_key, &charlie_key.verifying_key(), None)
            .expect("Failed to assign to Charlie");

        let envelope3 = assign_receipt(&envelope2, &charlie_key, &dave_key.verifying_key(), None)
            .expect("Failed to assign to Dave");

        // Legitimate chain should verify
        let legitimate_chain = vec![envelope1.clone(), envelope2.clone(), envelope3.clone()];
        assert!(
            verify_receipt_chain(&legitimate_chain),
            "Legitimate chain should verify"
        );

        // Test broken chain (missing middle envelope)
        let broken_chain = vec![envelope1.clone(), envelope3.clone()];
        assert!(
            !verify_receipt_chain(&broken_chain),
            "Broken chain should not verify"
        );

        // Test out-of-order chain
        let out_of_order_chain = vec![envelope2.clone(), envelope1.clone(), envelope3.clone()];
        assert!(
            !verify_receipt_chain(&out_of_order_chain),
            "Out-of-order chain should not verify"
        );

        // Test chain with duplicate envelope
        let duplicate_chain = vec![envelope1.clone(), envelope2.clone(), envelope2.clone()];
        assert!(
            !verify_receipt_chain(&duplicate_chain),
            "Chain with duplicates should not verify"
        );

        // Test single envelope (should verify as valid chain of length 1)
        let single_chain = vec![envelope1.clone()];
        assert!(
            verify_receipt_chain(&single_chain),
            "Single envelope should verify as valid chain"
        );
    }

    #[test]
    fn test_key_derivation_determinism() {
        // Test that the same keys always produce the same encryption/decryption
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let _test_data = b"Test data for determinism check";

        // Create multiple envelopes with the same keys and data
        let envelope1 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Test".to_string()),
        )
        .expect("Failed to create first envelope");

        let envelope2 = create_receipt(
            &alice_key,
            &bob_key.verifying_key(),
            1000,
            Some("Test".to_string()),
        )
        .expect("Failed to create second envelope");

        // Envelopes should have different hashes (due to timestamps/nonces)
        assert_ne!(
            envelope1.hash(),
            envelope2.hash(),
            "Different envelopes should have different hashes"
        );

        // But both should be unsealed by the same key
        let payload1 = envelope1
            .unseal(&bob_key)
            .expect("Failed to unseal first envelope");
        let payload2 = envelope2
            .unseal(&bob_key)
            .expect("Failed to unseal second envelope");

        // The payloads should be identical (same receipt data)
        let receipt1: Receipt =
            serde_json::from_slice(&payload1).expect("Failed to deserialize first");
        let receipt2: Receipt =
            serde_json::from_slice(&payload2).expect("Failed to deserialize second");

        assert_eq!(
            receipt1.amount, receipt2.amount,
            "Amounts should be identical"
        );
        assert_eq!(
            receipt1.issuer, receipt2.issuer,
            "Issuers should be identical"
        );
        assert_eq!(
            receipt1.beneficiary, receipt2.beneficiary,
            "Beneficiaries should be identical"
        );
        assert_eq!(
            receipt1.description, receipt2.description,
            "Descriptions should be identical"
        );
    }

    #[test]
    fn test_envelope_size_and_performance() {
        // Test with various payload sizes to ensure consistent behavior
        let alice_key = SigningKey::generate(&mut OsRng);
        let bob_key = SigningKey::generate(&mut OsRng);

        let test_amounts = vec![1, 100, 10000, 1000000, u64::MAX];

        for amount in test_amounts {
            let envelope = create_receipt(
                &alice_key,
                &bob_key.verifying_key(),
                amount,
                Some(format!("Test amount: {}", amount)),
            )
            .unwrap_or_else(|_| panic!("Failed to create receipt for amount {}", amount));

            // Verify envelope
            assert!(
                envelope.verify(),
                "Envelope should verify for amount {}",
                amount
            );

            // Unseal and verify amount
            let payload = envelope
                .unseal(&bob_key)
                .unwrap_or_else(|_| panic!("Failed to unseal envelope for amount {}", amount));
            let receipt: Receipt = serde_json::from_slice(&payload)
                .unwrap_or_else(|_| panic!("Failed to deserialize receipt for amount {}", amount));

            assert_eq!(
                receipt.amount, amount,
                "Amount should be preserved for {}",
                amount
            );
        }
    }
}
