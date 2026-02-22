//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! CA data models — certificate, tenant, and request types.
//!
//! Status: Stable types used by the CA service and its consumers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use trustedge_core::Secret;
use uuid::Uuid;

/// Unique identifier for tenants
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TenantId(pub Uuid);

impl TenantId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TenantId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TenantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Unique identifier for users
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

/// Tenant information for multi-tenant CA service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub organization: String,
    pub contact_email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub tenant_id: TenantId,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    Operator,
    Viewer,
}

/// Certificate status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificateStatus {
    Pending,
    Issued,
    Revoked,
    Expired,
}

/// Certificate information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: Uuid,
    pub tenant_id: TenantId,
    pub serial_number: String,
    pub subject: String,
    pub issuer: String,
    pub not_before: DateTime<Utc>,
    pub not_after: DateTime<Utc>,
    pub status: CertificateStatus,
    pub certificate_pem: String,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revocation_reason: Option<String>,
}

/// Certificate signing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateRequest {
    pub subject: CertificateSubject,
    pub validity_days: Option<u32>,
    pub key_usage: Vec<KeyUsage>,
    pub extended_key_usage: Vec<ExtendedKeyUsage>,
    pub san_entries: Vec<SubjectAlternativeName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateSubject {
    pub common_name: String,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyUsage {
    DigitalSignature,
    KeyEncipherment,
    DataEncipherment,
    KeyAgreement,
    KeyCertSign,
    CrlSign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtendedKeyUsage {
    ServerAuth,
    ClientAuth,
    CodeSigning,
    EmailProtection,
    TimeStamping,
    OcspSigning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubjectAlternativeName {
    DnsName(String),
    IpAddress(String),
    Email(String),
    Uri(String),
}

/// API request/response types
#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCertificateRequest {
    pub certificate_request: CertificateRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueCertificateResponse {
    pub certificate: Certificate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeCertificateRequest {
    pub reason: String,
}

/// Login request — password is Secret<String> and cannot be printed or serialized by accident.
///
/// - No `derive(Serialize)` — any attempt to serialize a `LoginRequest` is a compile error.
/// - Custom `Deserialize` implementation wraps the password in `Secret<String>` at the
///   deserialization boundary, preventing the raw string from ever existing unwrapped.
/// - Manual `Debug` implementation redacts the password field.
pub struct LoginRequest {
    pub email: String,
    password: Secret<String>,
}

impl LoginRequest {
    /// Construct a `LoginRequest`, wrapping `password` in `Secret<String>` immediately.
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password: Secret::new(password),
        }
    }

    /// Access the password as a `&str`.
    ///
    /// The caller is responsible for not logging or persisting the returned value.
    pub fn password(&self) -> &str {
        self.password.expose_secret().as_str()
    }
}

impl fmt::Debug for LoginRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LoginRequest")
            .field("email", &self.email)
            .field("password", &"[REDACTED]")
            .finish()
    }
}

/// Private helper for deserializing `LoginRequest`.
#[derive(Deserialize)]
struct LoginRequestRaw {
    email: String,
    password: String,
}

impl<'de> Deserialize<'de> for LoginRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = LoginRequestRaw::deserialize(deserializer)?;
        Ok(LoginRequest {
            email: raw.email,
            password: Secret::new(raw.password),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCertificatesResponse {
    pub certificates: Vec<Certificate>,
    pub total: u32,
    pub limit: u32,
    pub offset: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateResponse {
    pub certificate: Certificate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_request_debug_redacts_password() {
        let req = LoginRequest::new("user@example.com".to_string(), "s3cr3t!".to_string());
        let debug_output = format!("{:?}", req);
        assert!(
            debug_output.contains("[REDACTED]"),
            "Debug output must contain [REDACTED], got: {debug_output}"
        );
        assert!(
            !debug_output.contains("s3cr3t!"),
            "Debug output must NOT contain the actual password, got: {debug_output}"
        );
        assert!(
            debug_output.contains("user@example.com"),
            "Debug output should contain the email, got: {debug_output}"
        );
    }

    #[test]
    fn test_login_request_password_getter() {
        let req = LoginRequest::new("a@b.com".to_string(), "my-password".to_string());
        assert_eq!(req.password(), "my-password");
    }

    #[test]
    fn test_login_request_deserialize() {
        let json = r#"{"email":"a@b.com","password":"secret123"}"#;
        let req: LoginRequest = serde_json::from_str(json).expect("deserialization failed");
        assert_eq!(req.email, "a@b.com");
        assert_eq!(req.password(), "secret123");
    }

    #[test]
    fn test_login_request_deserialize_debug_redacts() {
        let json = r#"{"email":"test@test.com","password":"hunter2"}"#;
        let req: LoginRequest = serde_json::from_str(json).expect("deserialization failed");
        let debug_output = format!("{:?}", req);
        assert!(
            debug_output.contains("[REDACTED]"),
            "Debug of deserialized LoginRequest must redact password"
        );
        assert!(
            !debug_output.contains("hunter2"),
            "Debug must not expose password, got: {debug_output}"
        );
    }
}
