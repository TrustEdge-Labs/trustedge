/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Integration Tests - DEPRECATED
//!
//! ⚠ WARNING: This file has been replaced by:
//! - yubikey_simulation_tests.rs (CI-safe software validation)
//! - yubikey_hardware_tests.rs (real hardware operations, #[ignore])
//! - yubikey_hardware_detection.rs (hardware detection framework)
//!
//! This file is kept for compatibility during transition.

use anyhow::Result;

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_yubikey_backend_initialization() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure  
#[tokio::test]
async fn test_phase1_certificate_validation() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_phase2_certificate_generation() -> Result<()> {
    println!("⚠ DEPRECATED: Real hardware tests moved to yubikey_hardware_tests.rs");
    println!("● Run: cargo test --ignored --test yubikey_hardware_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_phase3_quic_integration() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_yubikey_capabilities() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_certificate_quic_compatibility() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_yubikey_error_handling() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}

/// Compatibility stub - redirects to new test structure
#[tokio::test]
async fn test_multi_slot_operations() -> Result<()> {
    println!("⚠ DEPRECATED: This test has been moved to yubikey_simulation_tests.rs");
    println!("● Run: cargo test --test yubikey_simulation_tests --features yubikey");
    Ok(())
}
