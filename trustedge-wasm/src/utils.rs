//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//


use wasm_bindgen::prelude::*;

// Utility functions for WASM operations

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Convert bytes to hex string for debugging
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// Convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, String> {
    if hex.len() % 2 != 0 {
        return Err("Hex string must have even length".to_string());
    }
    
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .map_err(|e| format!("Invalid hex character: {}", e))
}

// Safe logging function that handles potential panics
#[wasm_bindgen]
pub fn safe_log(message: &str) {
    console_log!("{}", message);
}

// Performance timing utilities
#[wasm_bindgen]
pub struct Timer {
    start_time: f64,
}

#[wasm_bindgen]
impl Timer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Timer {
        let start_time = js_sys::Date::now();
        Timer { start_time }
    }
    
    #[wasm_bindgen]
    pub fn elapsed(&self) -> f64 {
        js_sys::Date::now() - self.start_time
    }
    
    #[wasm_bindgen]
    pub fn log_elapsed(&self, operation: &str) {
        let elapsed = self.elapsed();
        console_log!("{} completed in {:.2}ms", operation, elapsed);
    }
}