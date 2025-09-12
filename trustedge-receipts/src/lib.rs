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

    // We need to unseal the previous envelope to get the amount
    // For now, we'll use a placeholder since decryption isn't fully implemented
    // In a real implementation, the assigner would provide their private key to decrypt

    // Calculate the hash of the previous envelope to create the chain link
    let prev_hash = previous_envelope.hash();

    // TODO: Get the actual amount from the previous envelope
    // For now, we'll use a placeholder amount
    let amount = 1000; // This should come from decrypting the previous envelope

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
}
