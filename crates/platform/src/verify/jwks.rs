//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! JWKS key management — Ed25519 signing key lifecycle with rotation support.

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::Path};
use trustedge_core::{SigningKey, VerifyingKey};

#[derive(Debug, Clone)]
pub struct KeyManager {
    current_key: SigningKey,
    current_kid: String,
    previous_key: Option<SigningKey>,
    previous_kid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoredKey {
    kid: String,
    private_key: String,
    created_at: String,
}

impl KeyManager {
    pub fn new() -> Result<Self> {
        let dev_key_path = "target/dev/signing_key.json";

        if Path::new(dev_key_path).exists() {
            Self::load_from_file(dev_key_path)
        } else {
            Self::generate_new()
        }
    }

    fn generate_new() -> Result<Self> {
        let signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let kid = format!("key_{}", uuid::Uuid::new_v4().simple());

        let key_manager = KeyManager {
            current_key: signing_key,
            current_kid: kid,
            previous_key: None,
            previous_kid: None,
        };

        key_manager.save_to_file("target/dev/signing_key.json")?;
        key_manager.write_jwks_file()?;

        Ok(key_manager)
    }

    fn load_from_file(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let stored: StoredKey = serde_json::from_str(&content)?;

        let private_key_bytes = BASE64.decode(&stored.private_key)?;
        let signing_key = SigningKey::from_bytes(
            &private_key_bytes
                .try_into()
                .map_err(|_| anyhow!("Invalid private key length"))?,
        );

        Ok(KeyManager {
            current_key: signing_key,
            current_kid: stored.kid,
            previous_key: None,
            previous_kid: None,
        })
    }

    fn save_to_file(&self, path: &str) -> Result<()> {
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)?;
        }

        let stored = StoredKey {
            kid: self.current_kid.clone(),
            private_key: BASE64.encode(self.current_key.to_bytes()),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let content = serde_json::to_string_pretty(&stored)?;
        fs::write(path, content)?;

        Ok(())
    }

    fn write_jwks_file(&self) -> Result<()> {
        let jwks_path = "target/dev/jwks.json";
        if let Some(parent) = Path::new(jwks_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let jwks = self.to_jwks();
        let content = serde_json::to_string_pretty(&jwks)?;
        fs::write(jwks_path, content)?;

        Ok(())
    }

    pub fn current_kid(&self) -> String {
        self.current_kid.clone()
    }

    pub fn current_signing_key(&self) -> &SigningKey {
        &self.current_key
    }

    pub fn to_jwks(&self) -> Value {
        let mut keys = Vec::new();

        // Current key
        let current_verifying_key = self.current_key.verifying_key();
        keys.push(self.key_to_jwk(&current_verifying_key, &self.current_kid));

        // Previous key if it exists
        if let (Some(prev_key), Some(prev_kid)) = (&self.previous_key, &self.previous_kid) {
            let prev_verifying_key = prev_key.verifying_key();
            keys.push(self.key_to_jwk(&prev_verifying_key, prev_kid));
        }

        json!({
            "keys": keys
        })
    }

    fn key_to_jwk(&self, verifying_key: &VerifyingKey, kid: &str) -> Value {
        let public_key_bytes = verifying_key.as_bytes();

        json!({
            "kty": "OKP",
            "crv": "Ed25519",
            "kid": kid,
            "use": "sig",
            "alg": "EdDSA",
            "x": BASE64.encode(public_key_bytes)
        })
    }

    pub fn rotate_key(&mut self) -> Result<()> {
        let new_signing_key = SigningKey::generate(&mut rand::rngs::OsRng);
        let new_kid = format!("key_{}", uuid::Uuid::new_v4().simple());

        self.previous_key = Some(self.current_key.clone());
        self.previous_kid = Some(self.current_kid.clone());
        self.current_key = new_signing_key;
        self.current_kid = new_kid;

        self.save_to_file("target/dev/signing_key.json")?;
        self.write_jwks_file()?;

        Ok(())
    }
}
