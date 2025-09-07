// Copyright (c) 2025 TRUSTEDGE LABS LLC
// MPL-2.0: https://mozilla.org/MPL/2.0/
// Project: trustedge — Privacy and trust at the edge.

//! Domain separation tests for manifest signatures
//! Tests that domain separation prevents signature reuse across contexts

use anyhow::Result;
use ed25519_dalek::{Signer, SigningKey, Verifier};
use trustedge_core::format::{
    sign_manifest_with_domain, verify_manifest_with_domain, MANIFEST_DOMAIN_SEP,
};

#[test]
fn test_domain_separation_basic_functionality() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"test manifest data";

    // Sign with domain separation
    let signature = sign_manifest_with_domain(&signing_key, manifest_bytes);

    // Verify with domain separation should succeed
    verify_manifest_with_domain(&verifying_key, manifest_bytes, &signature)?;

    println!("✔ Domain separation basic functionality works");
    Ok(())
}

#[test]
fn test_domain_separation_prevents_raw_signature_reuse() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"test manifest data";

    // Create a raw signature (without domain separation)
    let raw_signature = signing_key.sign(manifest_bytes);

    // Attempting to verify domain-separated signature with raw signature should fail
    let result = verify_manifest_with_domain(&verifying_key, manifest_bytes, &raw_signature);
    assert!(
        result.is_err(),
        "Raw signature should not verify with domain separation"
    );

    println!("✔ Domain separation prevents raw signature reuse");
    Ok(())
}

#[test]
fn test_domain_separation_prevents_cross_context_reuse() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"test manifest data";

    // Sign with a different (malicious) domain prefix
    let wrong_domain = b"malicious.manifest.v1";
    let mut wrong_message = Vec::with_capacity(wrong_domain.len() + manifest_bytes.len());
    wrong_message.extend_from_slice(wrong_domain);
    wrong_message.extend_from_slice(manifest_bytes);
    let malicious_signature = signing_key.sign(&wrong_message);

    // Should fail verification with correct domain
    let result = verify_manifest_with_domain(&verifying_key, manifest_bytes, &malicious_signature);
    assert!(
        result.is_err(),
        "Signature with wrong domain should not verify"
    );

    println!("✔ Domain separation prevents cross-context signature reuse");
    Ok(())
}

#[test]
fn test_domain_separation_tampered_prefix_fails() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"test manifest data";

    // Sign with correct domain
    let signature = sign_manifest_with_domain(&signing_key, manifest_bytes);

    // Manually create message with tampered domain prefix
    let tampered_domain = b"tampered.manifest.v1";
    let mut tampered_message = Vec::with_capacity(tampered_domain.len() + manifest_bytes.len());
    tampered_message.extend_from_slice(tampered_domain);
    tampered_message.extend_from_slice(manifest_bytes);

    // Direct verification with tampered domain should fail
    let result = verifying_key.verify(&tampered_message, &signature);
    assert!(
        result.is_err(),
        "Signature should not verify with tampered domain prefix"
    );

    println!("✔ Tampered domain prefix causes verification failure");
    Ok(())
}

#[test]
fn test_domain_separation_different_manifests() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();

    let manifest1 = b"first manifest";
    let manifest2 = b"second manifest";

    // Sign first manifest
    let signature1 = sign_manifest_with_domain(&signing_key, manifest1);

    // Try to verify signature1 against manifest2 (should fail)
    let result = verify_manifest_with_domain(&verifying_key, manifest2, &signature1);
    assert!(
        result.is_err(),
        "Signature should not verify against different manifest"
    );

    // Verify signature1 against correct manifest1 (should succeed)
    verify_manifest_with_domain(&verifying_key, manifest1, &signature1)?;

    println!("✔ Domain separation works correctly with different manifests");
    Ok(())
}

#[test]
fn test_domain_string_content() {
    // Verify the domain separation string is what we expect
    assert_eq!(MANIFEST_DOMAIN_SEP, b"trustedge.manifest.v1");
    println!(
        "✔ Domain separation string is correct: {:?}",
        std::str::from_utf8(MANIFEST_DOMAIN_SEP).unwrap()
    );
}

#[test]
fn test_signature_determinism_with_domain_separation() -> Result<()> {
    let signing_key = SigningKey::generate(&mut rand_core::OsRng);
    let verifying_key = signing_key.verifying_key();
    let manifest_bytes = b"deterministic test manifest";

    // Ed25519 signatures should be deterministic for the same input
    let sig1 = sign_manifest_with_domain(&signing_key, manifest_bytes);
    let sig2 = sign_manifest_with_domain(&signing_key, manifest_bytes);

    // Both signatures should be identical
    assert_eq!(
        sig1.to_bytes(),
        sig2.to_bytes(),
        "Ed25519 signatures should be deterministic"
    );

    // Both should verify correctly
    verify_manifest_with_domain(&verifying_key, manifest_bytes, &sig1)?;
    verify_manifest_with_domain(&verifying_key, manifest_bytes, &sig2)?;

    println!("✔ Domain-separated signatures are deterministic");
    Ok(())
}
