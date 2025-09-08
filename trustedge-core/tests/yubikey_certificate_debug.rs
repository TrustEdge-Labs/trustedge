/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Certificate ID Debug Test - Phase 2 Final Validation

use anyhow::Result;

mod yubikey_hardware_detection;
use yubikey_hardware_detection::YubikeyTestEnvironment;

#[cfg(feature = "yubikey")]
use trustedge_core::{backends::YubiKeyBackend, UniversalBackend};

/// Test certificate ID parsing and PIV slot mapping
#[tokio::test]
#[ignore] // Requires real hardware
async fn test_certificate_id_debug() -> Result<()> {
    let env = YubikeyTestEnvironment::detect();

    if !env.has_hardware() {
        println!("⚠ Skipping real hardware test - {}", env.description());
        return Ok(());
    }

    #[cfg(feature = "yubikey")]
    {
        println!("● Testing certificate ID parsing and PIV slot mapping...");

        // Use verbose configuration
        let mut config = env.get_config().expect("Hardware should have config");
        config.verbose = true;

        let backend = YubiKeyBackend::with_config(config)?;

        println!("\n● Backend key enumeration with verbose output:");
        match backend.list_keys() {
            Ok(keys) => {
                println!("  • Backend reports {} keys", keys.len());
                for (i, key) in keys.iter().enumerate() {
                    println!("    Key {}: ", i + 1);
                    println!("      ID bytes: {:?}", key.key_id);
                    println!(
                        "      ID as string: '{}'",
                        String::from_utf8_lossy(&key.key_id).trim_end_matches('\0')
                    );
                    println!("      Description: '{}'", key.description);
                    println!("      Backend data: {:?}", key.backend_data);
                }
            }
            Err(e) => {
                println!("  • Backend key enumeration failed: {}", e);
            }
        }

        println!("\n✔ Certificate ID debug complete");
    }

    Ok(())
}
