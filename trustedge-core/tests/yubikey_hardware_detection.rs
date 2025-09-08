/*
 * Copyright (c) 2025 TRUSTEDGE LABS LLC
 * This source code is subject to the terms of the Mozilla Public License, v. 2.0.
 * If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Project: trustedge — Privacy and trust at the edge.
 */

//! YubiKey Hardware Detection Framework
//!
//! Provides reliable hardware detection with graceful CI fallback for YubiKey testing.

use anyhow::{anyhow, Result};
use std::path::Path;
use trustedge_core::backends::YubiKeyConfig;

/// YubiKey test environment configuration based on hardware availability
#[derive(Debug, Clone)]
pub enum YubikeyTestEnvironment {
    /// Real YubiKey hardware detected and available
    Hardware {
        config: YubiKeyConfig,
        slots_available: Vec<String>,
        pkcs11_module: String,
    },
    /// Simulation mode - no hardware required (CI-safe)
    Simulation {
        mock_slots: Vec<String>,
        test_config: YubiKeyConfig,
    },
    /// Hardware detection failed - skip hardware tests
    Unavailable { reason: String },
}

impl YubikeyTestEnvironment {
    /// Detect YubiKey hardware and return appropriate test environment
    pub fn detect() -> Self {
        // Check if we're in CI environment
        if Self::is_ci_environment() {
            return Self::simulation_environment();
        }

        // Try to detect real hardware
        match Self::detect_hardware() {
            Ok(env) => env,
            Err(e) => Self::Unavailable {
                reason: format!("Hardware detection failed: {}", e),
            },
        }
    }

    /// Check if running in CI environment
    fn is_ci_environment() -> bool {
        std::env::var("CI").is_ok()
            || std::env::var("GITHUB_ACTIONS").is_ok()
            || std::env::var("GITLAB_CI").is_ok()
            || std::env::var("TRAVIS").is_ok()
            || std::env::var("CIRCLECI").is_ok()
    }

    /// Create simulation environment for CI
    fn simulation_environment() -> Self {
        Self::Simulation {
            mock_slots: vec![
                "9a".to_string(),
                "9c".to_string(),
                "9d".to_string(),
                "9e".to_string(),
            ],
            test_config: YubiKeyConfig {
                pkcs11_module_path: "/mock/pkcs11.so".to_string(),
                pin: Some("123456".to_string()),
                slot: Some(0),
                verbose: false,
            },
        }
    }

    /// Attempt to detect real YubiKey hardware
    fn detect_hardware() -> Result<Self> {
        // Step 1: Check for PKCS#11 module availability
        let pkcs11_paths = [
            "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
            "/usr/lib/opensc-pkcs11.so",
            "/usr/local/lib/opensc-pkcs11.so",
            "/opt/homebrew/lib/opensc-pkcs11.so", // macOS
            "/usr/lib64/opensc-pkcs11.so",        // CentOS/RHEL
        ];

        let pkcs11_module = pkcs11_paths
            .iter()
            .find(|path| Path::new(path).exists())
            .ok_or_else(|| anyhow!("No PKCS#11 module found"))?;

        // Step 2: Check for YubiKey hardware presence (basic detection)
        let yubikey_detected = Self::check_yubikey_presence();

        if !yubikey_detected {
            return Err(anyhow!("No YubiKey hardware detected"));
        }

        // Step 3: Create basic configuration (don't test backend creation here)
        let config = YubiKeyConfig {
            pkcs11_module_path: pkcs11_module.to_string(),
            pin: None, // Don't require PIN for detection
            slot: None,
            verbose: false,
        };

        // Hardware is available - let individual tests handle backend creation
        let slots_available = vec![
            "9a".to_string(),
            "9c".to_string(),
            "9d".to_string(),
            "9e".to_string(),
        ];

        Ok(Self::Hardware {
            config,
            slots_available,
            pkcs11_module: pkcs11_module.to_string(),
        })
    }

    /// Check for YubiKey hardware presence using multiple detection methods
    fn check_yubikey_presence() -> bool {
        // Method 1: Check via lsusb (most reliable)
        if let Ok(output) = std::process::Command::new("lsusb").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.contains("Yubico") || stdout.contains("1050:") {
                return true;
            }
        }

        // Method 2: Check via pkcs11-tool (if available)
        if let Ok(output) = std::process::Command::new("pkcs11-tool")
            .arg("--list-slots")
            .arg("--module")
            .arg("/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so")
            .output()
        {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("YubiKey") || stdout.contains("token present") {
                    return true;
                }
            }
        }

        // Method 3: Check /sys/bus/usb/devices for YubiKey
        if let Ok(entries) = std::fs::read_dir("/sys/bus/usb/devices") {
            for entry in entries.flatten() {
                if let Ok(id_vendor) = std::fs::read_to_string(entry.path().join("idVendor")) {
                    if id_vendor.trim() == "1050" {
                        // Yubico vendor ID
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if hardware is available for testing
    pub fn has_hardware(&self) -> bool {
        matches!(self, Self::Hardware { .. })
    }

    /// Check if this is simulation mode
    pub fn is_simulation(&self) -> bool {
        matches!(self, Self::Simulation { .. })
    }

    /// Get test configuration
    pub fn get_config(&self) -> Option<YubiKeyConfig> {
        match self {
            Self::Hardware { config, .. } => Some(config.clone()),
            Self::Simulation { test_config, .. } => Some(test_config.clone()),
            Self::Unavailable { .. } => None,
        }
    }

    /// Get available slots for testing
    pub fn get_slots(&self) -> Vec<String> {
        match self {
            Self::Hardware {
                slots_available, ..
            } => slots_available.clone(),
            Self::Simulation { mock_slots, .. } => mock_slots.clone(),
            Self::Unavailable { .. } => {
                // Even when unavailable, provide standard PIV slots for testing
                vec![
                    "9a".to_string(),
                    "9c".to_string(),
                    "9d".to_string(),
                    "9e".to_string(),
                ]
            }
        }
    }

    /// Get descriptive string for logging
    pub fn description(&self) -> String {
        match self {
            Self::Hardware {
                pkcs11_module,
                slots_available,
                ..
            } => {
                format!(
                    "Hardware YubiKey detected (PKCS#11: {}, {} slots)",
                    pkcs11_module,
                    slots_available.len()
                )
            }
            Self::Simulation { mock_slots, .. } => {
                format!("Simulation mode ({} mock slots)", mock_slots.len())
            }
            Self::Unavailable { reason } => {
                format!("Hardware unavailable: {}", reason)
            }
        }
    }
}

/// Helper macro for hardware-specific tests
#[macro_export]
macro_rules! hardware_test {
    ($test_name:ident, $test_body:expr) => {
        #[tokio::test]
        #[ignore] // Only run with --ignored flag when hardware present
        async fn $test_name() -> anyhow::Result<()> {
            let env = YubikeyTestEnvironment::detect();

            if !env.has_hardware() {
                println!("⚠ Skipping hardware test - {}", env.description());
                return Ok(());
            }

            println!("● Running hardware test - {}", env.description());
            $test_body(env).await
        }
    };
}

/// Helper macro for simulation tests (CI-safe)
#[macro_export]
macro_rules! simulation_test {
    ($test_name:ident, $test_body:expr) => {
        #[tokio::test]
        async fn $test_name() -> anyhow::Result<()> {
            let env = YubikeyTestEnvironment::detect();
            println!("● Running simulation test - {}", env.description());
            $test_body(env).await
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_detection() -> Result<()> {
        let env = YubikeyTestEnvironment::detect();

        // Should always succeed in some form
        match env {
            YubikeyTestEnvironment::Hardware {
                ref config,
                ref slots_available,
                ..
            } => {
                println!("✔ Hardware detected: {} slots", slots_available.len());
                assert!(!config.pkcs11_module_path.is_empty());
                assert!(!slots_available.is_empty());
            }
            YubikeyTestEnvironment::Simulation { ref mock_slots, .. } => {
                println!("● Simulation mode: {} mock slots", mock_slots.len());
                assert_eq!(mock_slots.len(), 4);
            }
            YubikeyTestEnvironment::Unavailable { ref reason } => {
                println!("⚠ Hardware unavailable: {}", reason);
                assert!(!reason.is_empty());
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_ci_environment_detection() -> Result<()> {
        // Test CI detection logic
        let original_ci = std::env::var("CI").ok();

        // Simulate CI environment
        std::env::set_var("CI", "true");
        let env = YubikeyTestEnvironment::detect();
        assert!(env.is_simulation(), "Should be simulation mode in CI");

        // Restore original environment
        match original_ci {
            Some(val) => std::env::set_var("CI", val),
            None => std::env::remove_var("CI"),
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_pkcs11_module_validation() -> Result<()> {
        let env = YubikeyTestEnvironment::detect();

        if let Some(config) = env.get_config() {
            // Validate PKCS#11 module path format
            assert!(!config.pkcs11_module_path.is_empty());

            if env.has_hardware() {
                // Real hardware should have valid file path
                assert!(
                    config.pkcs11_module_path.ends_with(".so")
                        || config.pkcs11_module_path.ends_with(".dylib")
                        || config.pkcs11_module_path.ends_with(".dll")
                );
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_slot_enumeration() -> Result<()> {
        let env = YubikeyTestEnvironment::detect();
        let slots = env.get_slots();

        // Should have standard PIV slots
        let expected_slots = ["9a", "9c", "9d", "9e"];

        for expected in &expected_slots {
            assert!(
                slots.contains(&expected.to_string()),
                "Missing expected PIV slot: {}",
                expected
            );
        }

        Ok(())
    }
}
