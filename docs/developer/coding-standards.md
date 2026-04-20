<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: sealedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/sealedge
-->

# Sealedge Coding Standards

This document defines the coding standards and conventions for the Sealedge project. Following these standards ensures consistency, maintainability, and a professional codebase.

## 🦀 Rust Code Standards

### Formatting and Style

**Use standard Rust formatting**:
```bash
cargo fmt
```

**Core principles**:
- 4-space indentation (enforced by rustfmt)
- 100-character line limit where practical
- Use `rustfmt.toml` configuration (if present)

### Linting

**All code must pass Clippy without warnings**:
```bash
cargo clippy -- -D warnings
```

**Specific requirements**:
- No `unwrap()` calls in production code (use proper error handling)
- No `panic!()` calls except in test code
- Use `#[must_use]` for functions where ignoring the result indicates a bug
- Prefer `&str` over `String` in function parameters when possible

### Naming Conventions

**Functions and variables**: `snake_case`
```rust
fn encrypt_audio_chunk() -> Result<Vec<u8>> { }
let encrypted_data = chunk.encrypt()?;
```

**Types and traits**: `PascalCase`
```rust
struct AudioCapture;
trait KeyBackend;
enum DataType;
```

**Constants**: `SCREAMING_SNAKE_CASE`
```rust
const DEFAULT_CHUNK_SIZE: usize = 4096;
const MAGIC_BYTES: &[u8] = b"SEAL";
```

**Modules**: `snake_case`
```rust
mod audio_capture;
mod key_backends;
```

### Error Handling

**Use proper error types**:
```rust
// ✅ Good: Specific error handling
fn parse_key(input: &str) -> Result<[u8; 32], KeyParseError> {
    // Implementation
}

// ❌ Bad: Generic error handling
fn parse_key(input: &str) -> Result<[u8; 32], Box<dyn Error>> {
    // Implementation
}
```

**Use `anyhow` for application errors, `thiserror` for library errors**:
```rust
// For CLI applications
use anyhow::{Result, Context};

fn process_file(path: &Path) -> Result<()> {
    std::fs::read(path).context("Failed to read input file")?;
    Ok(())
}
```

### Documentation

**All public APIs must have documentation**:
```rust
/// Encrypts audio data using AES-256-GCM with format preservation.
///
/// # Arguments
/// * `data` - Raw audio data to encrypt
/// * `key` - 32-byte encryption key
/// * `manifest` - Audio metadata and format information
///
/// # Returns
/// Encrypted audio chunk with embedded manifest
///
/// # Errors
/// Returns `EncryptionError` if encryption fails or key is invalid.
pub fn encrypt_audio_chunk(
    data: &[u8], 
    key: &[u8; 32], 
    manifest: &AudioManifest
) -> Result<EncryptedChunk, EncryptionError> {
    // Implementation
}
```

## 🖥️ Terminal Output Standards

### Professional UTF-8 Symbols

**Use professional UTF-8 symbols instead of emojis** for all terminal output:

| **Use Case** | **Symbol** | **Unicode** | **Example** |
|--------------|------------|-------------|-------------|
| **Success** | ✔ | U+2714 | `✔ Encryption complete` |
| **Error** | ✖ | U+2716 | `✖ Failed to read file` |
| **Warning** | ⚠ | U+26A0 | `⚠ Large file detected` |
| **Information** | ● | U+25CF | `● Processing 1024 chunks` |
| **Audio operations** | ♪ | U+266A | `♪ Audio capture started` |
| **Video operations** | ■ | U+25A0 | `■ Video stream detected` |
| **Progress** | ● | U+25CF | `● Step 1 of 3 complete` |

**Examples**:
```rust
// ✅ Good: Professional symbols
eprintln!("✔ File encrypted successfully");
eprintln!("● Processing {} chunks", count);
eprintln!("♪ Audio capture started");

// ❌ Bad: Emojis
eprintln!("✅ File encrypted successfully");
eprintln!("📊 Processing {} chunks", count);
eprintln!("🎵 Audio capture started");
```

### Error Messages

**Provide actionable error messages**:
```rust
// ✅ Good: Actionable error
eprintln!("✖ Failed to connect to server at {}", addr);
eprintln!("  Try: Check network connection or verify server is running");

// ❌ Bad: Vague error
eprintln!("✖ Connection failed");
```

**Use consistent formatting**:
```rust
// Main error with symbol
eprintln!("✖ {}", error_description);

// Context/suggestions with indentation
eprintln!("  Caused by: {}", underlying_cause);
eprintln!("  Try: {}", suggestion);
```

## 🔒 Security Standards

### Cryptographic Code

**Use established libraries**:
- `aes-gcm` for symmetric encryption
- `ed25519-dalek` for digital signatures
- `ring` or `rustcrypto` for cryptographic primitives

**Secure practices**:
```rust
// ✅ Good: Explicit zeroization
let mut key = [0u8; 32];
// ... use key ...
key.zeroize(); // Clear sensitive data

// ✅ Good: Constant-time operations for sensitive data
use subtle::ConstantTimeEq;
if key1.ct_eq(key2).into() {
    // Keys match
}
```

### Input Validation

**Validate all external inputs**:
```rust
fn parse_hex_key(input: &str) -> Result<[u8; 32]> {
    if input.len() != 64 {
        bail!("Key must be exactly 64 hex characters (32 bytes)");
    }
    
    let bytes = hex::decode(input)
        .context("Invalid hex characters in key")?;
    
    bytes.try_into()
        .map_err(|_| anyhow!("Key must be exactly 32 bytes"))
}
```

## 📁 File Organization

### Module Structure

**Organize by functionality**:
```
src/
├── lib.rs              # Public API and core types
├── main.rs             # CLI application entry
├── audio/              # Audio handling
│   ├── mod.rs
│   ├── capture.rs
│   └── processing.rs
├── crypto/             # Cryptographic operations
│   ├── mod.rs
│   ├── encryption.rs
│   └── keys.rs
├── formats/            # File format handling
│   ├── mod.rs
│   └── manifest.rs
└── network/            # Network operations
    ├── mod.rs
    ├── client.rs
    └── server.rs
```

### Import Organization

**Group imports logically**:
```rust
// Standard library
use std::fs::File;
use std::path::Path;

// External crates
use anyhow::{Context, Result};
use clap::Parser;

// Internal modules
use crate::crypto::EncryptionKey;
use crate::formats::Manifest;
```

## 🧪 Testing Standards

### Test Organization

**Co-locate tests with code**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Test implementation
    }
}
```

**Integration tests in `tests/` directory**:
```
tests/
├── integration/
│   ├── audio_pipeline.rs
│   ├── network_flow.rs
│   └── encryption_roundtrip.rs
└── common/
    └── test_utils.rs
```

### Test Naming

**Use descriptive test names**:
```rust
#[test]
fn encrypt_audio_preserves_format_information() { }

#[test]
fn server_rejects_invalid_authentication() { }

#[test]
fn key_derivation_produces_consistent_results() { }
```

## 📚 Documentation Standards

### Code Comments

**Explain the "why", not the "what"**:
```rust
// ✅ Good: Explains reasoning
// Use GCM mode to provide both confidentiality and authenticity
let cipher = Aes256Gcm::new(key);

// Buffer audio data to reduce system call overhead on high-frequency capture
let mut buffer = Vec::with_capacity(OPTIMAL_BUFFER_SIZE);

// ❌ Bad: States the obvious
// Create a new vector
let mut buffer = Vec::new();
```

### API Documentation

**Include examples in documentation**:
```rust
/// Encrypts a file while preserving its format metadata.
///
/// # Example
/// ```rust
/// use sealedge_core::{encrypt_file, generate_key};
/// 
/// let key = generate_key()?;
/// let result = encrypt_file("audio.mp3", &key)?;
/// println!("Encrypted {} bytes", result.len());
/// ```
pub fn encrypt_file(path: &Path, key: &Key) -> Result<Vec<u8>> {
    // Implementation
}
```

## 🔧 Build and CI Standards

### Cargo.toml

**Organize dependencies**:
```toml
[dependencies]
# Cryptography
aes-gcm = "0.10"
ed25519-dalek = "2.0"

# Serialization
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
tempfile = "3.0"
```

### CI Requirements

**All code must pass**:
- `cargo fmt --check` (formatting)
- `cargo clippy -- -D warnings` (linting)
- `cargo test` (all tests)
- Security audit (if applicable)

## 📝 Git Commit Standards

### Commit Message Format

```
type(scope): brief description

Longer explanation if needed.

Closes #issue-number
```

**Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `security`
**Scopes**: `cli`, `server`, `client`, `crypto`, `audio`, `format`, `docs`, `ci`

**Examples**:
```
feat(audio): add real-time chunking with temporal alignment
fix(crypto): handle edge case in nonce generation  
docs(api): add examples to encryption functions
security(keys): implement secure key zeroization
```

---

## 🚀 Quick Reference

**Before committing, run**:
```bash
cargo fmt
cargo clippy -- -D warnings  
cargo test
```

**Symbol quick reference**:
- Success: ✔ (U+2714)
- Error: ✖ (U+2716) 
- Warning: ⚠ (U+26A0)
- Info: ● (U+25CF)
- Audio: ♪ (U+266A)
- Video: ■ (U+25A0)

**Remember**: These standards ensure consistency, maintainability, and a professional codebase. When in doubt, prioritize clarity and security.
