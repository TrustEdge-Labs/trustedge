//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! CA data models.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
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
