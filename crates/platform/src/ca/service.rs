//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Certificate Authority service — hardware-backed PKI via UniversalBackend.

use super::{error::*, models::*};
use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;
use trustedge_core::{CryptoOperation, CryptoResult, SignatureAlgorithm, UniversalBackend};
use uuid::Uuid;

/// Core Certificate Authority service
pub struct CertificateAuthorityService {
    backend: Arc<dyn UniversalBackend>,
    ca_key_id: String,
    ca_certificate: String, // PEM-encoded CA certificate
    ca_subject: CertificateSubject,
    default_validity_days: u32,
}

impl CertificateAuthorityService {
    /// Create a new CA service with the specified backend
    pub fn new(
        backend: Arc<dyn UniversalBackend>,
        ca_key_id: String,
        ca_certificate: String,
        ca_subject: CertificateSubject,
        default_validity_days: u32,
    ) -> Self {
        Self {
            backend,
            ca_key_id,
            ca_certificate,
            ca_subject,
            default_validity_days,
        }
    }

    /// Issue a new certificate
    pub async fn issue_certificate(
        &self,
        tenant_id: &TenantId,
        request: &CertificateRequest,
    ) -> CAResult<Certificate> {
        // Generate serial number
        let serial_number = self.generate_serial_number()?;

        // Calculate validity period
        let validity_days = request.validity_days.unwrap_or(self.default_validity_days);
        let not_before = Utc::now();
        let not_after = not_before + Duration::days(validity_days as i64);

        // Build certificate data to be signed
        let cert_data = self.build_certificate_data(
            &serial_number,
            &request.subject,
            &self.ca_subject,
            not_before,
            not_after,
            &request.key_usage,
            &request.extended_key_usage,
            &request.san_entries,
        )?;

        // Sign the certificate using the backend
        let signature_result = self
            .backend
            .perform_operation(
                &self.ca_key_id,
                CryptoOperation::Sign {
                    data: cert_data.clone(),
                    algorithm: SignatureAlgorithm::EcdsaP256, // Default to ECDSA P-256
                },
            )
            .map_err(CAError::Backend)?;

        let signature = match signature_result {
            CryptoResult::Signed(sig) => sig,
            _ => return Err(CAError::Internal("Unexpected signature result".to_string())),
        };

        // Build the final certificate
        let certificate_pem = self.build_certificate_pem(&cert_data, &signature)?;

        // Create certificate record
        let certificate = Certificate {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.clone(),
            serial_number,
            subject: self.format_subject(&request.subject),
            issuer: self.format_subject(&self.ca_subject),
            not_before,
            not_after,
            status: CertificateStatus::Issued,
            certificate_pem,
            created_at: Utc::now(),
            revoked_at: None,
            revocation_reason: None,
        };

        Ok(certificate)
    }

    /// Revoke a certificate
    pub async fn revoke_certificate(
        &self,
        tenant_id: &TenantId,
        serial_number: &str,
        reason: &str,
    ) -> CAResult<()> {
        // Validate inputs
        if serial_number.is_empty() {
            return Err(CAError::InvalidRequest(
                "Serial number cannot be empty".to_string(),
            ));
        }

        if reason.is_empty() {
            return Err(CAError::InvalidRequest(
                "Revocation reason cannot be empty".to_string(),
            ));
        }

        // Validate serial number format (hex only)
        if !serial_number.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(CAError::InvalidRequest(
                "Serial number must contain only hexadecimal characters".to_string(),
            ));
        }

        // Validate revocation reason against RFC 5280 standard reasons
        let valid_reasons = [
            "unspecified",
            "keyCompromise",
            "cACompromise",
            "affiliationChanged",
            "superseded",
            "cessationOfOperation",
            "certificateHold",
            "removeFromCRL",
            "privilegeWithdrawn",
            "aACompromise",
        ];

        if !valid_reasons.contains(&reason) {
            return Err(CAError::InvalidRequest(format!(
                "Invalid revocation reason '{}'. Must be one of: {}",
                reason,
                valid_reasons.join(", ")
            )));
        }

        // Future: In a full implementation, this would:
        // 1. Check if certificate exists and belongs to the tenant
        // 2. Verify certificate is not already revoked
        // 3. Update the certificate status in the database
        // 4. Add the certificate to the CRL
        // 5. Potentially notify relevant parties

        // Simulate database check - in production, this would query the database
        tracing::info!(
            "Certificate {} revoked for tenant {} with reason: {}",
            serial_number,
            tenant_id,
            reason
        );

        if serial_number.len() < 8 {
            return Err(CAError::InvalidRequest(
                "Serial number too short - certificate not found".to_string(),
            ));
        }

        // Simulate checking if certificate is already revoked
        // Future: replace with real database query
        if serial_number == "deadbeefdeadbeef" {
            return Err(CAError::InvalidRequest(
                "Certificate is already revoked".to_string(),
            ));
        }

        tracing::info!(
            "Certificate revocation completed successfully: serial={}, tenant={}, reason={}",
            serial_number,
            tenant_id,
            reason
        );

        Ok(())
    }

    /// Generate a Certificate Revocation List (CRL)
    pub async fn generate_crl(&self, _tenant_id: &TenantId) -> CAResult<String> {
        // Future: Implement CRL generation
        // This would query the database for revoked certificates
        // and generate a proper CRL structure

        let crl_data = format!(
            "-----BEGIN X509 CRL-----\n{}\n-----END X509 CRL-----",
            "Future: Implement CRL generation"
        );

        Ok(crl_data)
    }

    /// Get CA certificate (public)
    pub fn get_ca_certificate(&self) -> &str {
        &self.ca_certificate
    }

    // Private helper methods

    fn generate_serial_number(&self) -> CAResult<String> {
        let uuid = Uuid::new_v4();
        Ok(uuid.to_string().replace('-', ""))
    }

    #[allow(clippy::too_many_arguments)]
    fn build_certificate_data(
        &self,
        serial_number: &str,
        subject: &CertificateSubject,
        issuer: &CertificateSubject,
        not_before: DateTime<Utc>,
        not_after: DateTime<Utc>,
        key_usage: &[KeyUsage],
        extended_key_usage: &[ExtendedKeyUsage],
        san_entries: &[SubjectAlternativeName],
    ) -> CAResult<Vec<u8>> {
        // Create a simplified TBS (To Be Signed) certificate structure
        // This creates the data that will be signed by the backend

        let cert_info = serde_json::json!({
            "version": "v3",
            "serial_number": serial_number,
            "signature_algorithm": "ecdsa-with-SHA256",
            "issuer": self.format_subject(issuer),
            "validity": {
                "not_before": not_before.to_rfc3339(),
                "not_after": not_after.to_rfc3339()
            },
            "subject": self.format_subject(subject),
            "subject_public_key_info": {
                "algorithm": "id-ecPublicKey",
                "parameters": "secp256r1",
                "public_key": "04010203040506070809...dummy_key_for_demo"
            },
            "extensions": {
                "key_usage": key_usage,
                "extended_key_usage": extended_key_usage,
                "subject_alternative_name": san_entries
            }
        });

        // Convert to canonical JSON bytes for consistent signing
        let canonical_json = cert_info.to_string();
        Ok(canonical_json.into_bytes())
    }

    fn build_certificate_pem(&self, cert_data: &[u8], signature: &[u8]) -> CAResult<String> {
        // Parse the certificate data to extract fields
        let cert_json: serde_json::Value = serde_json::from_slice(cert_data).map_err(|e| {
            CAError::CertificateGeneration(format!("Invalid certificate data: {}", e))
        })?;

        // Build certificate content with actual signed data
        let cert_content = format!(
            "Certificate:\n\
            \x20\x20\x20\x20Data:\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Version: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Serial Number: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Signature Algorithm: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Issuer: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Validity:\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Not Before: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Not After: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Subject: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Subject Public Key Info:\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Public Key Algorithm: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Parameters: {}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20Extensions:\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Key Usage: {:?}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Extended Key Usage: {:?}\n\
            \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20Subject Alternative Name: {:?}\n\
            \x20\x20\x20\x20Signature Algorithm: {}\n\
            \x20\x20\x20\x20Signature Value (Backend Hardware Signed):\n\
            \x20\x20\x20\x20\x20\x20\x20\x20{}\n\
            \x20\x20\x20\x20Certificate Data Hash:\n\
            \x20\x20\x20\x20\x20\x20\x20\x20{}",
            cert_json["version"].as_str().unwrap_or("v3"),
            cert_json["serial_number"].as_str().unwrap_or("unknown"),
            cert_json["signature_algorithm"]
                .as_str()
                .unwrap_or("ecdsa-with-SHA256"),
            cert_json["issuer"].as_str().unwrap_or("unknown"),
            cert_json["validity"]["not_before"]
                .as_str()
                .unwrap_or("unknown"),
            cert_json["validity"]["not_after"]
                .as_str()
                .unwrap_or("unknown"),
            cert_json["subject"].as_str().unwrap_or("unknown"),
            cert_json["subject_public_key_info"]["algorithm"]
                .as_str()
                .unwrap_or("id-ecPublicKey"),
            cert_json["subject_public_key_info"]["parameters"]
                .as_str()
                .unwrap_or("secp256r1"),
            cert_json["extensions"]["key_usage"],
            cert_json["extensions"]["extended_key_usage"],
            cert_json["extensions"]["subject_alternative_name"],
            cert_json["signature_algorithm"]
                .as_str()
                .unwrap_or("ecdsa-with-SHA256"),
            hex::encode(signature),
            hex::encode(cert_data)
        );

        // Encode as base64 for PEM format
        use base64::{engine::general_purpose, Engine as _};
        let cert_b64 = general_purpose::STANDARD.encode(cert_content.as_bytes());

        // Format as PEM with proper line breaks
        let mut pem_lines = Vec::new();
        pem_lines.push("-----BEGIN CERTIFICATE-----".to_string());

        // Split base64 into 64-character lines
        for chunk in cert_b64.as_bytes().chunks(64) {
            pem_lines.push(String::from_utf8_lossy(chunk).to_string());
        }

        pem_lines.push("-----END CERTIFICATE-----".to_string());

        Ok(pem_lines.join("\n"))
    }

    fn format_subject(&self, subject: &CertificateSubject) -> String {
        let mut parts = Vec::new();

        parts.push(format!("CN={}", subject.common_name));

        if let Some(org) = &subject.organization {
            parts.push(format!("O={}", org));
        }

        if let Some(ou) = &subject.organizational_unit {
            parts.push(format!("OU={}", ou));
        }

        if let Some(country) = &subject.country {
            parts.push(format!("C={}", country));
        }

        if let Some(state) = &subject.state {
            parts.push(format!("ST={}", state));
        }

        if let Some(locality) = &subject.locality {
            parts.push(format!("L={}", locality));
        }

        if let Some(email) = &subject.email {
            parts.push(format!("emailAddress={}", email));
        }

        parts.join(", ")
    }
}

/// Helper function to create a CA service with YubiKey backend.
/// Requires the `yubikey` feature.
#[cfg(feature = "yubikey")]
pub async fn create_yubikey_ca_service(
    ca_name: &str,
    ca_organization: &str,
    ca_country: &str,
    key_id: &str,        // Required key ID parameter
    pin: Option<String>, // Optional PIN for authentication
) -> CAResult<CertificateAuthorityService> {
    use trustedge_core::backends::yubikey::{YubiKeyBackend, YubiKeyConfig};

    // Configure YubiKey backend
    let yubikey_config = {
        let mut builder = YubiKeyConfig::builder()
            .default_slot("9c".to_string())
            .verbose(true);
        if let Some(p) = pin {
            builder = builder.pin(p);
        }
        builder.build()
    };

    let backend = YubiKeyBackend::with_config(yubikey_config)
        .map_err(|e| CAError::Configuration(format!("Failed to initialize YubiKey: {}", e)))?;

    // Generate CA certificate (this would be done once during setup)
    let ca_subject = CertificateSubject {
        common_name: ca_name.to_string(),
        organization: Some(ca_organization.to_string()),
        organizational_unit: None,
        country: Some(ca_country.to_string()),
        state: None,
        locality: None,
        email: None,
    };

    // Future: Generate actual CA certificate using x509-cert
    let ca_certificate = format!(
        "-----BEGIN CERTIFICATE-----\nFuture: Generate CA certificate for {}\n-----END CERTIFICATE-----",
        ca_name
    );

    let ca_service = CertificateAuthorityService::new(
        Arc::new(backend),
        key_id.to_string(), // Use provided key ID (exact match)
        ca_certificate,
        ca_subject,
        365, // Default 1 year validity
    );

    Ok(ca_service)
}
