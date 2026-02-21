//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! REST API endpoints for Certificate Authority operations.

use super::{error::CAError, models::*, service::CertificateAuthorityService};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::sync::Arc;

pub type AppState = Arc<CertificateAuthorityService>;

pub fn create_router(ca_service: Arc<CertificateAuthorityService>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/ca/certificate", get(get_ca_certificate))
        .route("/certificates", post(issue_certificate))
        .route("/certificates", get(list_certificates))
        .route("/certificates/:serial/revoke", post(revoke_certificate))
        .with_state(ca_service)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn get_ca_certificate(State(ca_service): State<AppState>) -> Result<String, StatusCode> {
    Ok(ca_service.get_ca_certificate().to_string())
}

/// Issue a new certificate
///
/// POST /certificates
///
/// Request body: IssueCertificateRequest
/// Response: IssueCertificateResponse with the issued certificate
async fn issue_certificate(
    State(ca_service): State<AppState>,
    Json(request): Json<IssueCertificateRequest>,
) -> Result<Json<IssueCertificateResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate the certificate request
    if let Err(validation_error) = validate_certificate_request(&request.certificate_request) {
        tracing::warn!("Invalid certificate request: {}", validation_error);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid Request".to_string(),
                message: validation_error,
                code: Some("INVALID_CERT_REQUEST".to_string()),
            }),
        ));
    }

    // Phase 26: Extract tenant ID from JWT auth instead of creating a new one
    let tenant_id = TenantId::new();

    tracing::info!(
        "Issuing certificate for subject: {}",
        request.certificate_request.subject.common_name
    );

    match ca_service
        .issue_certificate(&tenant_id, &request.certificate_request)
        .await
    {
        Ok(certificate) => {
            tracing::info!(
                "Certificate issued successfully: serial={}, subject={}",
                certificate.serial_number,
                certificate.subject
            );
            Ok(Json(IssueCertificateResponse { certificate }))
        }
        Err(e) => {
            tracing::error!("Failed to issue certificate: {}", e);
            let (status, error_response) = map_ca_error_to_http(e);
            Err((status, Json(error_response)))
        }
    }
}

/// Validate certificate request parameters
fn validate_certificate_request(request: &CertificateRequest) -> Result<(), String> {
    // Validate common name
    if request.subject.common_name.is_empty() {
        return Err("Common name cannot be empty".to_string());
    }

    if request.subject.common_name.len() > 64 {
        return Err("Common name cannot exceed 64 characters".to_string());
    }

    // Basic DNS name validation for common name
    if request.subject.common_name.contains(' ') {
        return Err("Common name cannot contain spaces".to_string());
    }

    // Validate organization if provided
    if let Some(org) = &request.subject.organization {
        if org.is_empty() {
            return Err("Organization cannot be empty if provided".to_string());
        }
        if org.len() > 64 {
            return Err("Organization cannot exceed 64 characters".to_string());
        }
    }

    // Validate country if provided
    if let Some(country) = &request.subject.country {
        if country.len() != 2 {
            return Err(
                "Country code must be exactly 2 characters (ISO 3166-1 alpha-2)".to_string(),
            );
        }
        if !country
            .chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_uppercase())
        {
            return Err("Country code must be uppercase letters only".to_string());
        }
    }

    // Validate validity period
    if let Some(days) = request.validity_days {
        if days == 0 {
            return Err("Validity period must be at least 1 day".to_string());
        }
        if days > 3650 {
            // 10 years max
            return Err("Validity period cannot exceed 10 years (3650 days)".to_string());
        }
    }

    // Validate key usage - must have at least one
    if request.key_usage.is_empty() {
        return Err("At least one key usage must be specified".to_string());
    }

    // Validate extended key usage - must have at least one
    if request.extended_key_usage.is_empty() {
        return Err("At least one extended key usage must be specified".to_string());
    }

    // Validate SAN entries
    if request.san_entries.is_empty() {
        return Err("At least one Subject Alternative Name must be specified".to_string());
    }

    for san in &request.san_entries {
        match san {
            SubjectAlternativeName::DnsName(name) => {
                if name.is_empty() {
                    return Err("DNS name in SAN cannot be empty".to_string());
                }
                if name.len() > 253 {
                    return Err("DNS name in SAN cannot exceed 253 characters".to_string());
                }
                // Basic DNS validation
                if name.starts_with('.') || name.ends_with('.') {
                    return Err("DNS name cannot start or end with a dot".to_string());
                }
                if name.contains("..") {
                    return Err("DNS name cannot contain consecutive dots".to_string());
                }
            }
            SubjectAlternativeName::Email(email) => {
                if email.is_empty() {
                    return Err("Email in SAN cannot be empty".to_string());
                }
                if !email.contains('@') || email.matches('@').count() != 1 {
                    return Err("Invalid email format in SAN".to_string());
                }
                if email.len() > 254 {
                    return Err("Email in SAN cannot exceed 254 characters".to_string());
                }
            }
            SubjectAlternativeName::IpAddress(ip) => {
                if ip.parse::<std::net::IpAddr>().is_err() {
                    return Err("Invalid IP address format in SAN".to_string());
                }
            }
            SubjectAlternativeName::Uri(uri) => {
                if uri.is_empty() {
                    return Err("URI in SAN cannot be empty".to_string());
                }
                if uri.len() > 2048 {
                    return Err("URI in SAN cannot exceed 2048 characters".to_string());
                }
                // Basic URI validation
                if !uri.starts_with("http://") && !uri.starts_with("https://") {
                    return Err("URI in SAN must start with http:// or https://".to_string());
                }
            }
        }
    }

    Ok(())
}

/// Validate certificate revocation request
fn validate_revocation_request(
    serial: &str,
    request: &RevokeCertificateRequest,
) -> Result<(), String> {
    // Validate serial number format
    if serial.is_empty() {
        return Err("Serial number cannot be empty".to_string());
    }

    if serial.len() > 64 {
        return Err("Serial number cannot exceed 64 characters".to_string());
    }

    // Validate hex format
    if !serial.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Serial number must contain only hexadecimal characters".to_string());
    }

    // Validate revocation reason
    if request.reason.is_empty() {
        return Err("Revocation reason cannot be empty".to_string());
    }

    if request.reason.len() > 255 {
        return Err("Revocation reason cannot exceed 255 characters".to_string());
    }

    // Validate reason is one of the standard RFC 5280 reasons
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

    if !valid_reasons.contains(&request.reason.as_str()) {
        return Err(format!(
            "Invalid revocation reason. Must be one of: {}",
            valid_reasons.join(", ")
        ));
    }

    Ok(())
}

/// Map CA errors to HTTP status codes and error responses
fn map_ca_error_to_http(error: CAError) -> (StatusCode, ErrorResponse) {
    match error {
        CAError::InvalidRequest(msg) => (
            StatusCode::BAD_REQUEST,
            ErrorResponse {
                error: "Bad Request".to_string(),
                message: msg,
                code: Some("INVALID_REQUEST".to_string()),
            },
        ),
        CAError::Authentication(msg) => (
            StatusCode::UNAUTHORIZED,
            ErrorResponse {
                error: "Unauthorized".to_string(),
                message: msg,
                code: Some("AUTH_REQUIRED".to_string()),
            },
        ),
        CAError::Authorization(msg) => (
            StatusCode::FORBIDDEN,
            ErrorResponse {
                error: "Forbidden".to_string(),
                message: msg,
                code: Some("INSUFFICIENT_PERMISSIONS".to_string()),
            },
        ),
        CAError::TenantNotFound(msg) => (
            StatusCode::NOT_FOUND,
            ErrorResponse {
                error: "Not Found".to_string(),
                message: msg,
                code: Some("TENANT_NOT_FOUND".to_string()),
            },
        ),
        CAError::CertificateGeneration(msg) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                error: "Certificate Generation Failed".to_string(),
                message: msg,
                code: Some("CERT_GEN_ERROR".to_string()),
            },
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorResponse {
                error: "Internal Server Error".to_string(),
                message: "An unexpected error occurred".to_string(),
                code: Some("INTERNAL_ERROR".to_string()),
            },
        ),
    }
}

#[derive(Debug, Deserialize)]
struct ListCertificatesQuery {
    /// Filter by certificate status
    status: Option<String>,
    /// Filter by subject (partial match)
    subject: Option<String>,
    /// Filter by serial number (exact match)
    serial: Option<String>,
    /// Filter by common name (partial match)
    common_name: Option<String>,
    /// Filter by organization (partial match)
    organization: Option<String>,
    /// Filter certificates expiring before this date (ISO 8601)
    expires_before: Option<String>,
    /// Filter certificates created after this date (ISO 8601)
    created_after: Option<String>,
    /// Maximum number of results to return
    limit: Option<u32>,
    /// Number of results to skip (for pagination)
    offset: Option<u32>,
    /// Sort field (serial, subject, created_at, expires_at)
    sort_by: Option<String>,
    /// Sort order (asc, desc)
    sort_order: Option<String>,
}

/// List certificates with optional filtering and pagination
///
/// GET /certificates?status=Issued&limit=50&offset=0
async fn list_certificates(
    State(_ca_service): State<AppState>,
    Query(query): Query<ListCertificatesQuery>,
) -> Result<Json<ListCertificatesResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Validate query parameters
    if let Err(validation_error) = validate_list_query(&query) {
        tracing::warn!("Invalid list certificates query: {}", validation_error);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid Query Parameters".to_string(),
                message: validation_error,
                code: Some("INVALID_QUERY".to_string()),
            }),
        ));
    }

    // Phase 26: Extract tenant ID from JWT auth instead of creating a new one
    let _tenant_id = TenantId::new();

    // Phase 26: Implement actual database query instead of mock data
    let mock_certificates = create_mock_certificates(&query);

    let response = ListCertificatesResponse {
        certificates: mock_certificates,
        total: 2, // Phase 26: Return real count from database
        limit: query.limit.unwrap_or(50),
        offset: query.offset.unwrap_or(0),
    };

    tracing::info!(
        "Listed certificates: returned={} results",
        response.certificates.len()
    );

    Ok(Json(response))
}

/// Validate list certificates query parameters
fn validate_list_query(query: &ListCertificatesQuery) -> Result<(), String> {
    // Validate status filter
    if let Some(status) = &query.status {
        match status.as_str() {
            "Pending" | "Issued" | "Revoked" | "Expired" => {}
            _ => {
                return Err(
                    "Invalid status filter. Must be one of: Pending, Issued, Revoked, Expired"
                        .to_string(),
                )
            }
        }
    }

    // Validate search strings
    if let Some(subject) = &query.subject {
        if subject.len() > 200 {
            return Err("Subject filter cannot exceed 200 characters".to_string());
        }
    }

    if let Some(serial) = &query.serial {
        if serial.is_empty() {
            return Err("Serial number filter cannot be empty".to_string());
        }
        if serial.len() > 64 {
            return Err("Serial number filter cannot exceed 64 characters".to_string());
        }
        // Validate hex format
        if !serial.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Serial number must contain only hexadecimal characters".to_string());
        }
    }

    if let Some(common_name) = &query.common_name {
        if common_name.is_empty() {
            return Err("Common name filter cannot be empty".to_string());
        }
        if common_name.len() > 64 {
            return Err("Common name filter cannot exceed 64 characters".to_string());
        }
    }

    if let Some(organization) = &query.organization {
        if organization.is_empty() {
            return Err("Organization filter cannot be empty".to_string());
        }
        if organization.len() > 64 {
            return Err("Organization filter cannot exceed 64 characters".to_string());
        }
    }

    // Validate limit
    if let Some(limit) = query.limit {
        if limit == 0 {
            return Err("Limit must be at least 1".to_string());
        }
        if limit > 1000 {
            return Err("Limit cannot exceed 1000".to_string());
        }
    }

    // Validate offset
    if let Some(offset) = query.offset {
        if offset > 1_000_000 {
            return Err("Offset cannot exceed 1,000,000".to_string());
        }
    }

    // Validate sort_by
    if let Some(sort_by) = &query.sort_by {
        match sort_by.as_str() {
            "serial" | "subject" | "created_at" | "expires_at" => {}
            _ => return Err(
                "Invalid sort_by field. Must be one of: serial, subject, created_at, expires_at"
                    .to_string(),
            ),
        }
    }

    // Validate sort_order
    if let Some(sort_order) = &query.sort_order {
        match sort_order.as_str() {
            "asc" | "desc" => {}
            _ => return Err("Invalid sort_order. Must be 'asc' or 'desc'".to_string()),
        }
    }

    // Validate date formats
    if let Some(expires_before) = &query.expires_before {
        if chrono::DateTime::parse_from_rfc3339(expires_before).is_err() {
            return Err("Invalid expires_before date format. Use ISO 8601 format (e.g., 2025-12-31T23:59:59Z)".to_string());
        }
    }

    if let Some(created_after) = &query.created_after {
        if chrono::DateTime::parse_from_rfc3339(created_after).is_err() {
            return Err("Invalid created_after date format. Use ISO 8601 format (e.g., 2025-01-01T00:00:00Z)".to_string());
        }
    }

    // Validate date logic
    if let (Some(created_after), Some(expires_before)) =
        (&query.created_after, &query.expires_before)
    {
        let created = chrono::DateTime::parse_from_rfc3339(created_after).unwrap();
        let expires = chrono::DateTime::parse_from_rfc3339(expires_before).unwrap();

        if created >= expires {
            return Err("created_after date must be before expires_before date".to_string());
        }
    }

    Ok(())
}

/// Create mock certificates for demonstration
/// Phase 26: Replace with actual database queries
fn create_mock_certificates(query: &ListCertificatesQuery) -> Vec<Certificate> {
    use chrono::Utc;
    use uuid::Uuid;

    let now = Utc::now();
    let tenant_id = TenantId::new();

    let mut certificates = vec![
        Certificate {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.clone(),
            serial_number: "3701e7c1d6134032917ece67be150e25".to_string(),
            subject: "CN=api-test.example.com, O=API Test Organization, C=US".to_string(),
            issuer: "CN=TrustEdge Enterprise CA, O=TrustEdge Labs LLC, C=US".to_string(),
            not_before: now - chrono::Duration::days(1),
            not_after: now + chrono::Duration::days(89),
            status: CertificateStatus::Issued,
            certificate_pem:
                "-----BEGIN CERTIFICATE-----\nMOCK_CERT_DATA_1\n-----END CERTIFICATE-----"
                    .to_string(),
            created_at: now - chrono::Duration::days(1),
            revoked_at: None,
            revocation_reason: None,
        },
        Certificate {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.clone(),
            serial_number: "4812f8e2e7245143a28fdf78cf261f36".to_string(),
            subject: "CN=test.example.com, O=Test Organization, C=US".to_string(),
            issuer: "CN=TrustEdge Enterprise CA, O=TrustEdge Labs LLC, C=US".to_string(),
            not_before: now - chrono::Duration::days(7),
            not_after: now + chrono::Duration::days(83),
            status: CertificateStatus::Issued,
            certificate_pem:
                "-----BEGIN CERTIFICATE-----\nMOCK_CERT_DATA_2\n-----END CERTIFICATE-----"
                    .to_string(),
            created_at: now - chrono::Duration::days(7),
            revoked_at: None,
            revocation_reason: None,
        },
    ];

    // Apply filters (simplified for demo)
    if let Some(status_filter) = &query.status {
        let filter_status = match status_filter.as_str() {
            "Pending" => CertificateStatus::Pending,
            "Issued" => CertificateStatus::Issued,
            "Revoked" => CertificateStatus::Revoked,
            "Expired" => CertificateStatus::Expired,
            _ => CertificateStatus::Issued,
        };
        certificates.retain(|cert| {
            std::mem::discriminant(&cert.status) == std::mem::discriminant(&filter_status)
        });
    }

    if let Some(subject_filter) = &query.subject {
        certificates.retain(|cert| {
            cert.subject
                .to_lowercase()
                .contains(&subject_filter.to_lowercase())
        });
    }

    if let Some(serial_filter) = &query.serial {
        certificates.retain(|cert| cert.serial_number == *serial_filter);
    }

    if let Some(cn_filter) = &query.common_name {
        certificates.retain(|cert| {
            cert.subject
                .to_lowercase()
                .contains(&format!("cn={}", cn_filter.to_lowercase()))
        });
    }

    if let Some(org_filter) = &query.organization {
        certificates.retain(|cert| {
            cert.subject
                .to_lowercase()
                .contains(&format!("o={}", org_filter.to_lowercase()))
        });
    }

    // Apply pagination
    let offset = query.offset.unwrap_or(0) as usize;
    let limit = query.limit.unwrap_or(50) as usize;

    if offset >= certificates.len() {
        return vec![];
    }

    let end = std::cmp::min(offset + limit, certificates.len());
    certificates[offset..end].to_vec()
}

/// Revoke a certificate
///
/// POST /certificates/{serial}/revoke
async fn revoke_certificate(
    State(ca_service): State<AppState>,
    Path(serial): Path<String>,
    Json(request): Json<RevokeCertificateRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    // Validate the revocation request
    if let Err(validation_error) = validate_revocation_request(&serial, &request) {
        tracing::warn!("Invalid revocation request: {}", validation_error);
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Invalid Request".to_string(),
                message: validation_error,
                code: Some("INVALID_REVOCATION_REQUEST".to_string()),
            }),
        ));
    }

    // Phase 26: Extract tenant ID from JWT auth instead of creating a new one
    let tenant_id = TenantId::new();

    tracing::info!(
        "Revoking certificate: serial={}, reason={}",
        serial,
        request.reason
    );

    match ca_service
        .revoke_certificate(&tenant_id, &serial, &request.reason)
        .await
    {
        Ok(()) => {
            tracing::info!("Certificate revoked successfully: serial={}", serial);
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            tracing::error!("Failed to revoke certificate: {}", e);
            let (status, error_response) = map_ca_error_to_http(e);
            Err((status, Json(error_response)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_certificate_request_valid() {
        let request = CertificateRequest {
            subject: CertificateSubject {
                common_name: "test.example.com".to_string(),
                organization: Some("Test Org".to_string()),
                organizational_unit: None,
                country: Some("US".to_string()),
                state: None,
                locality: None,
                email: None,
            },
            validity_days: Some(30),
            key_usage: vec![KeyUsage::DigitalSignature],
            extended_key_usage: vec![ExtendedKeyUsage::ServerAuth],
            san_entries: vec![SubjectAlternativeName::DnsName(
                "test.example.com".to_string(),
            )],
        };

        assert!(validate_certificate_request(&request).is_ok());
    }

    #[test]
    fn test_validate_certificate_request_empty_common_name() {
        let request = CertificateRequest {
            subject: CertificateSubject {
                common_name: "".to_string(),
                organization: None,
                organizational_unit: None,
                country: None,
                state: None,
                locality: None,
                email: None,
            },
            validity_days: Some(30),
            key_usage: vec![KeyUsage::DigitalSignature],
            extended_key_usage: vec![ExtendedKeyUsage::ServerAuth],
            san_entries: vec![SubjectAlternativeName::DnsName(
                "test.example.com".to_string(),
            )],
        };

        let result = validate_certificate_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Common name cannot be empty"));
    }

    #[test]
    fn test_validate_certificate_request_invalid_country() {
        let request = CertificateRequest {
            subject: CertificateSubject {
                common_name: "test.example.com".to_string(),
                organization: None,
                organizational_unit: None,
                country: Some("USA".to_string()), // Invalid: 3 letters
                state: None,
                locality: None,
                email: None,
            },
            validity_days: Some(30),
            key_usage: vec![KeyUsage::DigitalSignature],
            extended_key_usage: vec![ExtendedKeyUsage::ServerAuth],
            san_entries: vec![SubjectAlternativeName::DnsName(
                "test.example.com".to_string(),
            )],
        };

        let result = validate_certificate_request(&request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Country code must be exactly 2 characters"));
    }

    #[test]
    fn test_validate_list_query_valid() {
        let query = ListCertificatesQuery {
            status: Some("Issued".to_string()),
            subject: None,
            serial: None,
            common_name: None,
            organization: None,
            expires_before: None,
            created_after: None,
            limit: Some(10),
            offset: Some(0),
            sort_by: Some("created_at".to_string()),
            sort_order: Some("desc".to_string()),
        };

        assert!(validate_list_query(&query).is_ok());
    }

    #[test]
    fn test_validate_list_query_invalid_status() {
        let query = ListCertificatesQuery {
            status: Some("InvalidStatus".to_string()),
            subject: None,
            serial: None,
            common_name: None,
            organization: None,
            expires_before: None,
            created_after: None,
            limit: None,
            offset: None,
            sort_by: None,
            sort_order: None,
        };

        let result = validate_list_query(&query);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid status filter"));
    }

    #[test]
    fn test_validate_revocation_request_valid() {
        let request = RevokeCertificateRequest {
            reason: "keyCompromise".to_string(),
        };

        assert!(validate_revocation_request("3701e7c1d6134032917ece67be150e25", &request).is_ok());
    }

    #[test]
    fn test_validate_revocation_request_empty_serial() {
        let request = RevokeCertificateRequest {
            reason: "keyCompromise".to_string(),
        };

        let result = validate_revocation_request("", &request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Serial number cannot be empty"));
    }

    #[test]
    fn test_validate_revocation_request_invalid_serial_format() {
        let request = RevokeCertificateRequest {
            reason: "keyCompromise".to_string(),
        };

        let result = validate_revocation_request("invalid-serial-123", &request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Serial number must contain only hexadecimal characters"));
    }

    #[test]
    fn test_validate_revocation_request_empty_reason() {
        let request = RevokeCertificateRequest {
            reason: "".to_string(),
        };

        let result = validate_revocation_request("3701e7c1d6134032917ece67be150e25", &request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Revocation reason cannot be empty"));
    }

    #[test]
    fn test_validate_revocation_request_invalid_reason() {
        let request = RevokeCertificateRequest {
            reason: "invalidReason".to_string(),
        };

        let result = validate_revocation_request("3701e7c1d6134032917ece67be150e25", &request);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid revocation reason"));
    }

    #[test]
    fn test_validate_revocation_request_long_reason() {
        let request = RevokeCertificateRequest {
            reason: "a".repeat(256), // Exceeds 255 character limit
        };

        let result = validate_revocation_request("3701e7c1d6134032917ece67be150e25", &request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Revocation reason cannot exceed 255 characters"));
    }

    #[test]
    fn test_validate_revocation_request_all_valid_reasons() {
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

        for reason in &valid_reasons {
            let request = RevokeCertificateRequest {
                reason: reason.to_string(),
            };
            assert!(
                validate_revocation_request("3701e7c1d6134032917ece67be150e25", &request).is_ok(),
                "Reason '{}' should be valid",
                reason
            );
        }
    }
}
