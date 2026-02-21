//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Snapshot regression tests for JSON schema generation.
//!
//! These tests ensure schema output from `trustedge_types::schema` exactly
//! matches the baseline fixtures in `tests/fixtures/`. Any schema drift
//! (field additions, type changes, etc.) will cause these tests to fail,
//! making it safe to rely on schema stability for downstream consumers.

use trustedge_types::schema;

/// Load and parse a fixture JSON file.
fn load_fixture(name: &str) -> serde_json::Value {
    let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"));
    serde_json::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse fixture {path}: {e}"))
}

/// Compare generated schema against fixture and produce a helpful diff on failure.
fn assert_schema_matches(schema_value: &serde_json::Value, fixture_name: &str) {
    let fixture = load_fixture(fixture_name);

    if schema_value != &fixture {
        let actual_pretty = serde_json::to_string_pretty(schema_value)
            .expect("Failed to pretty-print actual schema");
        let expected_pretty =
            serde_json::to_string_pretty(&fixture).expect("Failed to pretty-print fixture");
        panic!(
            "Schema mismatch for {}.\n\nExpected (fixture):\n{}\n\nActual (generated):\n{}",
            fixture_name, expected_pretty, actual_pretty
        );
    }
}

#[test]
fn schema_verify_report_matches_fixture() {
    let root_schema = schema::verify_report_schema();
    let schema_value =
        serde_json::to_value(&root_schema).expect("Failed to serialize VerifyReport schema");
    assert_schema_matches(&schema_value, "verify_report.v1.json");
}

#[test]
fn schema_receipt_matches_fixture() {
    let root_schema = schema::receipt_schema();
    let schema_value =
        serde_json::to_value(&root_schema).expect("Failed to serialize Receipt schema");
    assert_schema_matches(&schema_value, "receipt.v1.json");
}

#[test]
fn schema_verify_request_matches_fixture() {
    let root_schema = schema::verify_request_schema();
    let schema_value =
        serde_json::to_value(&root_schema).expect("Failed to serialize VerifyRequest schema");
    assert_schema_matches(&schema_value, "verify_request.v1.json");
}

#[test]
fn schema_verify_response_matches_fixture() {
    let root_schema = schema::verify_response_schema();
    let schema_value =
        serde_json::to_value(&root_schema).expect("Failed to serialize VerifyResponse schema");
    assert_schema_matches(&schema_value, "verify_response.v1.json");
}

#[test]
fn schema_generate_returns_all_four_schemas() {
    let schemas = schema::generate();

    assert!(
        schemas.contains_key("verify_report.v1"),
        "generate() missing verify_report.v1"
    );
    assert!(
        schemas.contains_key("receipt.v1"),
        "generate() missing receipt.v1"
    );
    assert!(
        schemas.contains_key("verify_request.v1"),
        "generate() missing verify_request.v1"
    );
    assert!(
        schemas.contains_key("verify_response.v1"),
        "generate() missing verify_response.v1"
    );
    assert_eq!(
        schemas.len(),
        4,
        "generate() should return exactly 4 schemas"
    );
}
