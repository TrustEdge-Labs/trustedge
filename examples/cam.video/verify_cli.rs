//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//


use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

use trst_core::verify_archive;

fn main() -> Result<(), Box<dyn Error>> {
    let archive_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "examples/cam.video/sample.trst".into());
    let key_path = env::args().nth(2).unwrap_or_else(|| "device.pub".into());

    let device_pub = fs::read_to_string(&key_path)?;
    let report = verify_archive(Path::new(&archive_path), device_pub.trim())?;

    println!("Signature: {}", if report.signature { "PASS" } else { "FAIL" });
    println!("Continuity: {}", if report.continuity { "PASS" } else { "FAIL" });
    println!(
        "Segments: {}  Duration(s): {:.1}",
        report.segment_count,
        report.duration_seconds
    );

    Ok(())
}
