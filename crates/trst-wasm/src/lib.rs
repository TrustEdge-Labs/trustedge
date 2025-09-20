//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use trustedge_trst_core::{verify_manifest_bytes, ArchiveError, VerifyOutcome};

#[derive(Serialize)]
struct VerificationReport {
    signature: bool,
    continuity: bool,
    segment_count: u32,
}

#[wasm_bindgen]
pub fn verify_manifest(manifest_bytes: &[u8], device_pub: &str) -> Result<JsValue, JsValue> {
    let report = verify_manifest_bytes(manifest_bytes, device_pub).map_err(to_js_error)?;
    to_js(report)
}

#[wasm_bindgen]
pub fn verify_archive(archive_bytes: &[u8], device_pub: &str) -> Result<JsValue, JsValue> {
    // TODO: support full archive verification (manifest + chunks). For P0 we
    // interpret the provided bytes as a manifest JSON payload and reuse the
    // manifest verifier.
    let report = verify_manifest_bytes(archive_bytes, device_pub).map_err(to_js_error)?;
    to_js(report)
}

fn to_js(report: VerifyOutcome) -> Result<JsValue, JsValue> {
    let payload = VerificationReport {
        signature: report.signature,
        continuity: report.continuity,
        segment_count: report.segment_count as u32,
    };
    to_value(&payload).map_err(|err| JsValue::from_str(&err.to_string()))
}

fn to_js_error(err: ArchiveError) -> JsValue {
    JsValue::from_str(&err.to_string())
}
