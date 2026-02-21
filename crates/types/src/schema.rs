//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! JSON schema generation for TrustEdge wire types.
//!
//! Uses `schemars` to generate JSON Schema (draft-07) representations of
//! all wire types. Schema output is pinned against fixture files to prevent
//! inadvertent drift.

use std::collections::BTreeMap;

use schemars::{schema::RootSchema, schema_for};
use serde_json::Value;

use crate::receipt::Receipt;
use crate::verification::{VerifyRequest, VerifyResponse};
use crate::verify_report::VerifyReport;

/// Generate JSON schemas for all 4 wire types.
///
/// Returns a `BTreeMap` mapping schema name (e.g., `"verify_report.v1"`) to
/// the schema serialized as a `serde_json::Value`.
pub fn generate() -> BTreeMap<String, Value> {
    let mut map = BTreeMap::new();
    map.insert(
        "verify_report.v1".to_string(),
        serde_json::to_value(verify_report_schema()).expect("schema serialization failed"),
    );
    map.insert(
        "receipt.v1".to_string(),
        serde_json::to_value(receipt_schema()).expect("schema serialization failed"),
    );
    map.insert(
        "verify_request.v1".to_string(),
        serde_json::to_value(verify_request_schema()).expect("schema serialization failed"),
    );
    map.insert(
        "verify_response.v1".to_string(),
        serde_json::to_value(verify_response_schema()).expect("schema serialization failed"),
    );
    map
}

/// Generate the JSON schema for `VerifyReport`.
pub fn verify_report_schema() -> RootSchema {
    schema_for!(VerifyReport)
}

/// Generate the JSON schema for `Receipt`.
pub fn receipt_schema() -> RootSchema {
    schema_for!(Receipt)
}

/// Generate the JSON schema for `VerifyRequest`.
pub fn verify_request_schema() -> RootSchema {
    schema_for!(VerifyRequest)
}

/// Generate the JSON schema for `VerifyResponse`.
pub fn verify_response_schema() -> RootSchema {
    schema_for!(VerifyResponse)
}
