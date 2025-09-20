//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

//! # TrustEdge WebAssembly
//!
//! WebAssembly bindings for TrustEdge cryptographic operations, enabling privacy-preserving
//! edge computing in browser and Node.js environments.
//!
//! This crate provides JavaScript/TypeScript bindings for core TrustEdge functionality,
//! allowing web applications to perform secure encryption, decryption, and key management
//! operations directly in the browser.
//!
//! ## Features
//!
//! - **Browser Compatibility**: Works in all modern browsers with WebAssembly support
//! - **Node.js Support**: Compatible with Node.js environments
//! - **TypeScript Definitions**: Includes TypeScript type definitions
//! - **Memory Safety**: Rust's memory safety guarantees extend to WebAssembly
//! - **Performance**: Near-native performance for cryptographic operations
//!
//! ## Usage
//!
//! ```javascript
//! import init, { encrypt_data, decrypt_data } from 'trustedge-wasm';
//!
//! async function example() {
//!     await init();
//!     
//!     const data = new Uint8Array([1, 2, 3, 4, 5]);
//!     const encrypted = encrypt_data(data, key);
//!     const decrypted = decrypt_data(encrypted, key);
//! }
//! ```
//!
//! For detailed usage examples and API documentation, see the crate's README.md.

use wasm_bindgen::prelude::*;

mod crypto;
mod utils;

pub use crypto::*;
pub use utils::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console logging easier
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `extern` block, but for importing JS functions
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}! TrustEdge WASM is working!", name));
}

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log!("TrustEdge WASM module initialized successfully");
}

// Basic test function to verify WASM is working
#[wasm_bindgen]
pub fn test_basic_functionality() -> String {
    console_log!("Testing basic WASM functionality");
    "TrustEdge WASM is working correctly!".to_string()
}

// Version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
