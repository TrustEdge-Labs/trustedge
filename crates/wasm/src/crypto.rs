//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/
//
// Project: trustedge — Privacy and trust at the edge.
//

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use base64::{engine::general_purpose, Engine as _};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Encrypted data structure that can be serialized to/from JSON
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct EncryptedData {
    ciphertext: String,
    nonce: String,
    key_id: Option<String>,
}

#[wasm_bindgen]
impl EncryptedData {
    #[wasm_bindgen(constructor)]
    pub fn new(ciphertext: String, nonce: String, key_id: Option<String>) -> EncryptedData {
        EncryptedData {
            ciphertext,
            nonce,
            key_id,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn ciphertext(&self) -> String {
        self.ciphertext.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn nonce(&self) -> String {
        self.nonce.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn key_id(&self) -> Option<String> {
        self.key_id.clone()
    }

    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    #[wasm_bindgen]
    pub fn from_json(json: &str) -> Result<EncryptedData, JsValue> {
        serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("Deserialization error: {}", e)))
    }
}

// Generate a random 256-bit key for AES-256-GCM
#[wasm_bindgen]
pub fn generate_key() -> String {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    general_purpose::STANDARD.encode(&key[..])
}

// Generate a random nonce for AES-256-GCM
#[wasm_bindgen]
pub fn generate_nonce() -> String {
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    general_purpose::STANDARD.encode(&nonce[..])
}

// Encrypt data using AES-256-GCM
#[wasm_bindgen]
pub fn encrypt(
    data: &str,
    key_b64: &str,
    nonce_b64: Option<String>,
) -> Result<EncryptedData, JsValue> {
    console_log!("Starting AES-256-GCM encryption");

    // Decode the base64 key
    let key_bytes = general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| JsValue::from_str(&format!("Invalid key format: {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(JsValue::from_str("Key must be 32 bytes (256 bits)"));
    }

    // Convert slice to array, then to Key (avoids deprecated GenericArray::from_slice)
    let key_array: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| JsValue::from_str("Key conversion failed"))?;
    let key: Key<Aes256Gcm> = key_array.into();
    let cipher = Aes256Gcm::new(&key);

    // Use provided nonce or generate a new one
    let nonce = if let Some(nonce_str) = nonce_b64 {
        let nonce_bytes = general_purpose::STANDARD
            .decode(nonce_str)
            .map_err(|e| JsValue::from_str(&format!("Invalid nonce format: {}", e)))?;

        if nonce_bytes.len() != 12 {
            return Err(JsValue::from_str("Nonce must be 12 bytes (96 bits)"));
        }

        // Convert slice to array, then to Nonce (avoids deprecated GenericArray::from_slice)
        let nonce_array: [u8; 12] = nonce_bytes
            .as_slice()
            .try_into()
            .map_err(|_| JsValue::from_str("Nonce conversion failed"))?;
        nonce_array.into()
    } else {
        Aes256Gcm::generate_nonce(&mut OsRng)
    };

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(|e| JsValue::from_str(&format!("Encryption failed: {}", e)))?;

    let result = EncryptedData {
        ciphertext: general_purpose::STANDARD.encode(&ciphertext),
        nonce: general_purpose::STANDARD.encode(&nonce[..]),
        key_id: None,
    };

    console_log!("AES-256-GCM encryption completed successfully");
    Ok(result)
}

// Decrypt data using AES-256-GCM
#[wasm_bindgen]
pub fn decrypt(encrypted_data: &EncryptedData, key_b64: &str) -> Result<String, JsValue> {
    console_log!("Starting AES-256-GCM decryption");

    // Decode the base64 key
    let key_bytes = general_purpose::STANDARD
        .decode(key_b64)
        .map_err(|e| JsValue::from_str(&format!("Invalid key format: {}", e)))?;

    if key_bytes.len() != 32 {
        return Err(JsValue::from_str("Key must be 32 bytes (256 bits)"));
    }

    // Convert slice to array, then to Key (avoids deprecated GenericArray::from_slice)
    let key_array: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .map_err(|_| JsValue::from_str("Key conversion failed"))?;
    let key: Key<Aes256Gcm> = key_array.into();
    let cipher = Aes256Gcm::new(&key);

    // Decode the nonce and ciphertext
    let nonce_bytes = general_purpose::STANDARD
        .decode(&encrypted_data.nonce)
        .map_err(|e| JsValue::from_str(&format!("Invalid nonce format: {}", e)))?;

    let ciphertext_bytes = general_purpose::STANDARD
        .decode(&encrypted_data.ciphertext)
        .map_err(|e| JsValue::from_str(&format!("Invalid ciphertext format: {}", e)))?;

    if nonce_bytes.len() != 12 {
        return Err(JsValue::from_str("Nonce must be 12 bytes (96 bits)"));
    }

    // Convert slice to array reference (avoids deprecated GenericArray::from_slice)
    let nonce_array: &[u8; 12] = nonce_bytes
        .as_slice()
        .try_into()
        .map_err(|_| JsValue::from_str("Nonce conversion failed"))?;

    // Decrypt the data
    let plaintext = cipher
        .decrypt(nonce_array.into(), ciphertext_bytes.as_slice())
        .map_err(|e| JsValue::from_str(&format!("Decryption failed: {}", e)))?;

    let result = String::from_utf8(plaintext)
        .map_err(|e| JsValue::from_str(&format!("Invalid UTF-8 in decrypted data: {}", e)))?;

    console_log!("AES-256-GCM decryption completed successfully");
    Ok(result)
}

// Convenience function to encrypt with auto-generated nonce
#[wasm_bindgen]
pub fn encrypt_simple(data: &str, key_b64: &str) -> Result<EncryptedData, JsValue> {
    encrypt(data, key_b64, None)
}

// Generate secure random bytes
#[wasm_bindgen]
pub fn generate_random_bytes(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    OsRng.fill_bytes(&mut bytes);
    general_purpose::STANDARD.encode(&bytes)
}

// Validate key format
#[wasm_bindgen]
pub fn validate_key(key_b64: &str) -> bool {
    match general_purpose::STANDARD.decode(key_b64) {
        Ok(bytes) => bytes.len() == 32,
        Err(_) => false,
    }
}

// Validate nonce format
#[wasm_bindgen]
pub fn validate_nonce(nonce_b64: &str) -> bool {
    match general_purpose::STANDARD.decode(nonce_b64) {
        Ok(bytes) => bytes.len() == 12,
        Err(_) => false,
    }
}
