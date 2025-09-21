//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use serde::Serialize;

/// Exit codes for the trst verify command
#[repr(u8)]
#[allow(dead_code)]
pub enum ExitCode {
    /// Success - signature pass, continuity pass
    Ok = 0,
    /// Signature verification failure
    SigFail = 10,
    /// Continuity failure (gap, out-of-order, truncation)
    ContinuityFail = 11,
    /// IO/schema error (missing files, unreadable JSON, bad layout)
    IoOrSchema = 12,
    /// Invalid CLI arguments
    InvalidArgs = 13,
    /// Internal error (unexpected panics/caught errors)
    Internal = 14,
}

/// Details about out-of-order segments
#[derive(Serialize)]
pub struct OutOfOrder {
    pub expected: u32,
    pub found: u32,
}

/// Standardized verification report for JSON output
#[derive(Serialize, Default)]
pub struct VerifyReport {
    /// Signature verification status: "pass" | "fail" | "unknown"
    pub signature: String,
    /// Continuity verification status: "pass" | "fail" | "skip" | "unknown"
    pub continuity: String,
    /// Number of segments in the archive
    pub segments: u32,
    /// Total duration in seconds
    pub duration_s: f32,
    /// Profile name (e.g., "cam.video")
    pub profile: String,
    /// Device identifier (e.g., "te:cam:XYZ123")
    pub device_id: String,
    /// Index of first gap in continuity chain (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_gap_index: Option<u32>,
    /// Details about out-of-order segments (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_order: Option<OutOfOrder>,
    /// Error message for failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Verification time in milliseconds
    pub verify_time_ms: u64,
}

impl VerifyReport {
    /// Create a new report with unknown status
    pub fn new() -> Self {
        Self {
            signature: "unknown".to_string(),
            continuity: "unknown".to_string(),
            segments: 0,
            duration_s: 0.0,
            profile: String::new(),
            device_id: String::new(),
            first_gap_index: None,
            out_of_order: None,
            error: None,
            verify_time_ms: 0,
        }
    }

    /// Set signature verification result
    pub fn set_signature_pass(&mut self) {
        self.signature = "pass".to_string();
    }

    /// Set signature verification failure
    pub fn set_signature_fail(&mut self) {
        self.signature = "fail".to_string();
        self.continuity = "skip".to_string();
    }

    /// Set continuity verification result
    pub fn set_continuity_pass(&mut self) {
        self.continuity = "pass".to_string();
    }

    /// Set continuity verification failure
    pub fn set_continuity_fail(&mut self) {
        self.continuity = "fail".to_string();
    }

    /// Set continuity gap at specific index
    pub fn set_continuity_gap(&mut self, gap_index: u32) {
        self.continuity = "fail".to_string();
        self.first_gap_index = Some(gap_index);
    }

    /// Set out-of-order segments
    pub fn set_out_of_order(&mut self, expected: u32, found: u32) {
        self.continuity = "fail".to_string();
        self.out_of_order = Some(OutOfOrder { expected, found });
    }

    /// Set error message
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }

    /// Set verification time
    pub fn set_verify_time(&mut self, duration_ms: u64) {
        self.verify_time_ms = duration_ms;
    }

    /// Set archive metadata
    pub fn set_metadata(
        &mut self,
        segments: u32,
        duration_s: f32,
        profile: String,
        device_id: String,
    ) {
        self.segments = segments;
        self.duration_s = duration_s;
        self.profile = profile;
        self.device_id = device_id;
    }

    /// Get the appropriate exit code based on the report status
    pub fn exit_code(&self) -> ExitCode {
        match (self.signature.as_str(), self.continuity.as_str()) {
            ("pass", "pass") => ExitCode::Ok,
            ("fail", _) => ExitCode::SigFail,
            ("unknown", _) => ExitCode::IoOrSchema,
            (_, "fail") => ExitCode::ContinuityFail,
            (_, "unknown") => ExitCode::IoOrSchema,
            _ => ExitCode::Internal,
        }
    }

    /// Print the report as JSON
    pub fn print_json(&self) -> Result<(), serde_json::Error> {
        println!("{}", serde_json::to_string(self)?);
        Ok(())
    }

    /// Print the report in human-readable format
    pub fn print_human(&self) {
        let signature_status = match self.signature.as_str() {
            "pass" => "PASS",
            "fail" => "FAIL",
            _ => "UNKNOWN",
        };

        let continuity_status = match self.continuity.as_str() {
            "pass" => "PASS",
            "fail" => {
                if let Some(gap_index) = self.first_gap_index {
                    return println!("Signature: {}\nContinuity: FAIL (gap at index {})\nSegments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
                        signature_status, gap_index, self.segments, self.duration_s,
                        if self.segments > 0 { self.duration_s / self.segments as f32 } else { 0.0 });
                } else if let Some(ref out_of_order) = self.out_of_order {
                    return println!("Signature: {}\nContinuity: FAIL (out of order: expected {}, found {})\nSegments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
                        signature_status, out_of_order.expected, out_of_order.found, self.segments, self.duration_s,
                        if self.segments > 0 { self.duration_s / self.segments as f32 } else { 0.0 });
                } else {
                    "FAIL"
                }
            }
            "skip" => "SKIP",
            _ => "UNKNOWN",
        };

        println!("Signature: {}", signature_status);
        println!("Continuity: {}", continuity_status);
        println!(
            "Segments: {}  Duration(s): {:.1}  Chunk(s): {:.1}",
            self.segments,
            self.duration_s,
            if self.segments > 0 {
                self.duration_s / self.segments as f32
            } else {
                0.0
            }
        );

        // Print errors to stderr for backward compatibility with acceptance tests
        if let Some(ref error) = self.error {
            eprintln!("{}", error);
        }
    }
}
