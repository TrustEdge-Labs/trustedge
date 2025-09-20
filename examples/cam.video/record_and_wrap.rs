//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//


use std::error::Error;

use base64::Engine;
use chrono::Utc;
use ed25519_dalek::SigningKey;
use rand_core::OsRng;
use trst_core::{wrap_file, DeviceInfo, ManifestCapture, WrapConfig};

fn main() -> Result<(), Box<dyn Error>> {
    // Derive a fresh signing key for the example demo.
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();

    let mut config = WrapConfig::default();
    config.device = DeviceInfo {
        id: "te:cam:EXAMPLE".into(),
        fw: "1.0.0".into(),
        model: "TrustEdgeRefCam".into(),
        public_key: format!(
            "ed25519:{}",
            base64::engine::general_purpose::STANDARD.encode(verifying_key.as_bytes())
        ),
    };
    config.capture = ManifestCapture {
        started_at: Utc::now().to_rfc3339(),
        tz: "UTC".into(),
        fps: 30,
        resolution: "1920x1080".into(),
        codec: "raw".into(),
    };

    wrap_file(
        "examples/cam.video/sample.bin",
        "examples/cam.video/sample.trst",
        &signing_key,
        config,
    )?;

    println!("Generated archive at examples/cam.video/sample.trst");
    Ok(())
}
