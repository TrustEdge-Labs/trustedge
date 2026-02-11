<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Coding Conventions

**Analysis Date:** 2026-02-09

## Naming Patterns

**Files:**
- Rust files use snake_case: `envelope.rs`, `auth.rs`, `software_hsm.rs`
- Module files follow cargo conventions: `src/lib.rs`, `src/main.rs`, `src/bin/*.rs`
- Binary targets in `src/bin/`: `trustedge-server.rs`, `trustedge-client.rs`
- Test files: integration tests in `tests/*.rs` (e.g., `tests/auth_integration.rs`), unit tests with `mod tests` block at end of source files

**Functions:**
- Use snake_case for function names: `seal()`, `unseal()`, `verify()`, `create_signed_attestation()`
- Async functions prefixed with `pub async fn`: seen in auth module (`client_authenticate`, `server_authenticate`)
- Private functions use leading underscore if shadowing internal: `_create_test_manifest()`
- Helper functions typically lowercase: `derive_shared_encryption_key()`, `create_test_data()`

**Variables:**
- Use snake_case for variables and mutable bindings: `signing_key`, `verifying_key`, `session_id`, `chunk_count`
- Constants use UPPER_SNAKE_CASE: `NONCE_LEN`, `DEFAULT_CHUNK_SIZE`, `PBKDF2_ITERATIONS`, `SESSION_TIMEOUT`
- Array/buffer variables: `[u8; 32]` for fixed-size, `Vec<u8>` for dynamic

**Types:**
- Structs use PascalCase: `Envelope`, `ClientCertificate`, `SessionManager`, `ServerCertificate`
- Enums use PascalCase variants: `AuthMessageType::ClientHello`, `KeySource::Generate`, `OutputFormat::SealedEnvelope`
- Trait names use PascalCase: `KeyBackend`, `InputReader`
- Type aliases lowercase: `CryptoResult` for operation results

**Modules:**
- Module files named after the logical grouping: `backends/`, `transport/`
- Module re-exports organized in mod.rs: `pub use keyring::KeyringBackend`, `pub use universal::*`

## Code Style

**Formatting:**
- Tool: `cargo fmt` (rustfmt with default Rust edition 2021 settings)
- Line length: Standard rustfmt defaults (typically 100 columns)
- Indentation: 4 spaces (enforced by rustfmt)
- Must pass: `cargo fmt` before committing

**Linting:**
- Tool: `cargo clippy -- -D warnings` (all warnings treated as errors)
- All code must pass clippy in strict mode
- Security-focused: uses `#![forbid(unsafe_code)]` in CLI binaries (see `crates/trustedge-cli/src/main.rs`)
- No `unwrap()` in production code paths; use `anyhow::Result<T>` or `thiserror::Error` for error handling

**Common patterns observed:**
- Early returns with `?` operator for error propagation
- Context wrapping: `.context()` and `.with_context()` from `anyhow` crate
- Error messages include UTF-8 symbols for terminal output: `✔`, `✖`, `⚠`, `●`

## Import Organization

**Order:**
1. Standard library imports (`std::*`)
2. External crate imports (cryptography, async, serialization)
3. Internal workspace imports (from `trustedge_core`, etc.)
4. Type imports and trait imports

**Example from `crates/core/src/envelope.rs`:**
```rust
use crate::format::{build_aad, AeadAlgorithm, HashAlgorithm, SignatureAlgorithm, SignedManifest};
use crate::{NetworkChunk, NONCE_LEN};
use anyhow::{Context, Result};
use blake3;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use pbkdf2::pbkdf2_hmac;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroize;
```

**Path Aliases:**
- No path aliases configured; imports use explicit `crate::` paths
- Workspace members imported fully: `use trustedge_core::Envelope;`

**Re-exports:**
- Barrel files (`mod.rs`) re-export public API: `pub use envelope::{Envelope, EnvelopeMetadata};`
- Selective re-exports to control public surface: `pub use backends::{...}` in `crates/core/src/lib.rs`

## Error Handling

**Patterns:**
- **Libraries:** Use `thiserror::Error` for custom error types with `#[error(...)]` attributes (seen in `crypto.rs`)
- **CLIs:** Use `anyhow::Result<T>` and `anyhow::anyhow!()` for error creation
- **Error wrapping:** Chain context with `.context()` for additional debugging info:
  ```rust
  std::fs::read(&config.artifact_path).with_context(|| {
      format!("Failed to read artifact: {}", config.artifact_path.display())
  })?
  ```
- **No panics in production:** Replace `unwrap()` with proper `Result` handling
- **Signature verification failures:** Return descriptive errors like `SignatureVerificationFailed`

**Example from `crypto.rs`:**
```rust
#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Invalid key format: {0}")]
    InvalidKeyFormat(String),
    #[error("Signature verification failed")]
    SignatureVerificationFailed,
}
```

## Logging

**Framework:** `println!()` for CLI output; structured logging not required for this codebase

**Patterns:**
- Use UTF-8 symbols for status messages: `println!("✔ Test passed!")`, `println!("⚠ Warning")`
- Verbose output with indentation for nested information:
  ```rust
  println!("✔ Backend {} capabilities:", backend_name);
  println!("  • Hardware backed: {}", capabilities.hardware_backed);
  ```
- No trailing newlines needed; `println!` adds them automatically
- Debug output with `eprintln!()` for errors sent to stderr

## Comments

**When to Comment:**
- Document public API with `///` doc comments (required for library crates)
- Add module-level doc comments: `//!` at start of files and module blocks
- Explain non-obvious security decisions and cryptographic operations
- Mark TODOs with `// TODO:` for future work (seen in `envelope_v2_bridge.rs`)

**JSDoc/TSDoc:**
- Use Rust's `///` doc comments for public functions
- Include examples in doc comments for complex operations
- Document parameters and return values
- Example from `attestation/src/lib.rs`:
  ```rust
  /// Create a cryptographically signed software attestation
  ///
  /// This is the main entry point for creating attestations. It handles:
  /// - Analyzing the artifact and extracting metadata
  /// - Creating the attestation data structure
  /// - Optionally sealing in a cryptographic envelope
  ///
  /// # Arguments
  /// * `config` - Configuration specifying artifact, builder, output format, and keys
  ///
  /// # Returns
  /// * `AttestationResult` containing the attestation, serialized output, and verification info
  ///
  /// # Example
  /// ```rust
  /// let result = create_signed_attestation(config)?;
  /// ```
  pub fn create_signed_attestation(config: AttestationConfig) -> Result<AttestationResult>
  ```

## Function Design

**Size:**
- Most functions keep logic under 50 lines; longer functions (100+ lines) are typically high-level orchestrators
- Example: `Envelope::seal()` in `envelope.rs` is 43 lines, `create_signed_attestation()` is 60 lines
- Prefer small, composable functions; extract complex operations to helper functions

**Parameters:**
- Use strong typing: avoid primitive booleans, prefer enums (e.g., `OutputFormat::JsonOnly` vs `bool`)
- Prefer borrowed references: `&[u8]` for byte slices, `&str` for strings
- Use `Result<T>` for fallible operations, no `Option<Result<T>>` chains without flattening
- Example: `fn seal(payload: &[u8], signing_key: &SigningKey, beneficiary_key: &VerifyingKey) -> Result<Self>`

**Return Values:**
- Use `Result<T>` for operations that can fail
- Return owned types when transforming: `Result<Vec<u8>>` for decryption output
- Return references for queries: `&EnvelopeMetadata` for metadata access
- Example: `pub fn unseal(&self, decryption_key: &SigningKey) -> Result<Vec<u8>>`

**Memory Safety:**
- Use `zeroize::Zeroize` for sensitive key material: `key_material.zeroize();`
- Implement `Drop` for keypairs to ensure automatic cleanup:
  ```rust
  impl Drop for DeviceKeypair {
      fn drop(&mut self) {
          self.secret.zeroize();
      }
  }
  ```
- `#[serde(skip)]` for private keys to prevent accidental serialization

## Module Design

**Exports:**
- Organize public API in main crate lib.rs: `crates/core/src/lib.rs` re-exports all public types
- Use selective `pub use` to control surface area
- Keep internal implementation details private (not exposed in `pub use`)
- Example from `lib.rs`:
  ```rust
  pub use envelope::{Envelope, EnvelopeMetadata};
  pub use backends::{BackendRegistry, KeyBackend};
  pub use auth::{ClientCertificate, ServerCertificate, SessionManager};
  ```

**Barrel Files:**
- `backends/mod.rs` collects all backend types: `pub use universal::*;`, `pub use universal_registry::*;`
- Permits convenient imports: `use trustedge_core::backends::*;`
- Top-level module documentation in `mod.rs` files describes the module's purpose

**Internal Organization:**
- Separate concerns into focused modules: `envelope.rs` for sealing/unsealing, `crypto.rs` for primitives, `auth.rs` for authentication
- Use feature flags to conditionally compile: `#[cfg(feature = "audio")]` for audio capture, `#[cfg(feature = "yubikey")]` for hardware support

## File Structure Conventions

**Copyright Headers:** All `.rs` files require the MPL-2.0 header (enforced by CI):
```rust
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
```

**Deny Unsafe:**
- CLI binaries use `#![forbid(unsafe_code)]` at the top
- Libraries allow unsafe only in crypto backends where PKCS#11 FFI requires it

---

*Convention analysis: 2026-02-09*
