//
// Copyright (c) 2025 TRUSTEDGE LABS LLC
// This source code is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
// Project: trustedge — Privacy and trust at the edge.
//

//! Secret wrapper type for sensitive values.
//!
//! Provides [`Secret<T>`] — a wrapper that:
//! - Redacts the inner value in `Debug` output (always prints `[REDACTED]`)
//! - Zeroizes memory on drop via [`ZeroizeOnDrop`]
//! - Requires explicit access through [`Secret::expose_secret`]
//! - Does NOT implement `Display`, `Deref`, `Serialize`, or `Deserialize`

use std::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// A wrapper around a sensitive value `T` that zeroizes memory on drop and
/// always redacts the inner value in `Debug` output.
///
/// # Usage
///
/// ```rust
/// use trustedge_core::Secret;
///
/// let api_key = Secret::new("my-secret-api-key".to_string());
///
/// // Debug output is always redacted
/// assert!(format!("{:?}", api_key).contains("[REDACTED]"));
///
/// // Access inner value explicitly
/// assert_eq!(api_key.expose_secret(), "my-secret-api-key");
/// ```
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Secret<T: Zeroize>(T);

impl<T: Zeroize> Secret<T> {
    /// Wrap a sensitive value.
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// Access the inner secret value.
    ///
    /// The caller is responsible for not leaking the returned reference
    /// (e.g., do not log it, store it in non-zeroizing types, etc.).
    pub fn expose_secret(&self) -> &T {
        &self.0
    }
}

impl<T: Zeroize> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}

impl<T: Clone + Zeroize> Clone for Secret<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Zeroize + PartialEq> PartialEq for Secret<T> {
    fn eq(&self, other: &Self) -> bool {
        self.expose_secret() == other.expose_secret()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_redacted() {
        let s = Secret::new("my-secret".to_string());
        let output = format!("{:?}", s);
        assert!(
            output.contains("[REDACTED]"),
            "Debug output should contain [REDACTED], got: {output}"
        );
        assert!(
            !output.contains("my-secret"),
            "Debug output must NOT contain the secret value, got: {output}"
        );
    }

    #[test]
    fn test_expose_secret() {
        let s = Secret::new("value".to_string());
        assert_eq!(s.expose_secret(), "value");
    }

    #[test]
    fn test_clone() {
        let original = Secret::new("clone-me".to_string());
        let cloned = original.clone();
        assert_eq!(original.expose_secret(), cloned.expose_secret());
    }

    #[test]
    fn test_debug_in_struct() {
        struct Config {
            name: String,
            token: Secret<String>,
        }

        impl fmt::Debug for Config {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("Config")
                    .field("name", &self.name)
                    .field("token", &self.token)
                    .finish()
            }
        }

        let cfg = Config {
            name: "production".to_string(),
            token: Secret::new("super-secret-token".to_string()),
        };

        let output = format!("{:?}", cfg);
        assert!(
            output.contains("[REDACTED]"),
            "Struct Debug should contain [REDACTED], got: {output}"
        );
        assert!(
            !output.contains("super-secret-token"),
            "Struct Debug must NOT contain the actual token, got: {output}"
        );
    }

    #[test]
    fn test_partial_eq() {
        let a = Secret::new("same".to_string());
        let b = Secret::new("same".to_string());
        let c = Secret::new("different".to_string());

        assert_eq!(a, b, "Secrets with same value should be equal");
        assert_ne!(a, c, "Secrets with different values should not be equal");
    }
}
