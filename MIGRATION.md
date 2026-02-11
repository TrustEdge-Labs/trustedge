<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->

# TrustEdge Migration Guide

## Migrating from 0.2.x to 0.3.x: Crate Consolidation

### Overview

As part of the TrustEdge workspace consolidation project (Phases 4-7), all receipts and attestation functionality has been merged into `trustedge-core`. The standalone `trustedge-receipts` and `trustedge-attestation` crates now serve as deprecated facades that re-export from core.

**Why this change:**
- **Single source of truth**: Core cryptographic operations centralized in one place
- **Reduced compilation units**: Faster builds with fewer dependency edges
- **Better maintainability**: One codebase for all applications-layer functionality
- **Identical APIs**: All function signatures and behaviors remain unchanged

**Timeline:**

| Version | Date | Status |
|---------|------|--------|
| 0.2.0 | January 2026 | Facades active, no warnings |
| 0.3.0 | February 2026 | Facades deprecated, compiler warnings issued |
| 0.4.0 | August 2026 | Facades removed from workspace (breaking change) |

You have a **6-month migration window** to update your code.

---

## Migration Steps

### Step 1: Update Cargo.toml Dependencies

**Before (using facade crates):**
```toml
[dependencies]
trustedge-receipts = "0.2.0"
trustedge-attestation = "0.2.0"
```

**After (using consolidated core):**
```toml
[dependencies]
trustedge-core = "0.2.0"
```

If you're using both receipts and attestation, you only need to add `trustedge-core` once.

### Step 2: Update Import Statements

All type names and function signatures remain identical. Only the module path changes.

#### Receipts Migration

**Before:**
```rust
use trustedge_receipts::{
    Receipt,
    create_receipt,
    assign_receipt,
    extract_receipt,
    verify_receipt_chain,
};
```

**After:**
```rust
use trustedge_core::{
    Receipt,
    create_receipt,
    assign_receipt,
    extract_receipt,
    verify_receipt_chain,
};
```

#### Attestation Migration

**Before:**
```rust
use trustedge_attestation::{
    Attestation,
    AttestationConfig,
    AttestationResult,
    OutputFormat,
    KeySource,
    VerificationConfig,
    VerificationResult,
    VerificationDetails,
    VerificationInfo,
    create_signed_attestation,
    verify_attestation,
};
```

**After:**
```rust
use trustedge_core::{
    Attestation,
    AttestationConfig,
    AttestationResult,
    OutputFormat,
    KeySource,
    VerificationConfig,
    VerificationResult,
    VerificationDetails,
    VerificationInfo,
    create_signed_attestation,
    verify_attestation,
};
```

### Step 3: Verify Compilation

After updating dependencies and imports, verify everything still works:

```bash
# Clean build to ensure no stale artifacts
cargo clean

# Check for compilation errors
cargo check

# Run your test suite
cargo test

# If you have clippy enabled, ensure no new warnings
cargo clippy -- -D warnings
```

---

## API Compatibility

**Important: There are NO breaking changes to function signatures or behavior.**

- All type names remain identical (`Receipt`, `Attestation`, etc.)
- All function names remain identical (`create_receipt`, `verify_attestation`, etc.)
- All function signatures remain identical (same parameters, same return types)
- All runtime behavior remains identical (same cryptographic operations)

**Only import paths have changed.** This is why the migration window exists — you need time to update imports, but your code logic requires no changes.

---

## Troubleshooting

### "Cannot find type `Receipt` in this scope"

**Cause:** You're still importing from `trustedge_receipts`, but the crate is deprecated.

**Solution:**
1. Update your `Cargo.toml` to include `trustedge-core = "0.2.0"`
2. Change `use trustedge_receipts::Receipt;` to `use trustedge_core::Receipt;`

### "Cannot find type `Attestation` in this scope"

**Cause:** You're still importing from `trustedge_attestation`, but the crate is deprecated.

**Solution:**
1. Update your `Cargo.toml` to include `trustedge-core = "0.2.0"`
2. Change `use trustedge_attestation::Attestation;` to `use trustedge_core::Attestation;`

### "Multiple versions of `trustedge-core` in dependency tree"

**Cause:** Some of your dependencies still use the old facade crates, which themselves depend on `trustedge-core`.

**Solution:**
1. Run `cargo tree` to identify which dependencies are using old facades
2. Check if those dependencies have newer versions that use `trustedge-core` directly
3. Update those dependencies in your `Cargo.toml`
4. If upstream dependencies haven't migrated yet, you may need to wait or contact the maintainers

### Deprecation warnings during build

**Cause:** You're using version 0.3.0 of the facade crates, which emit deprecation warnings.

**Solution:**
- This is expected behavior during the migration window
- Follow the migration steps above to update to `trustedge-core`
- Once migrated, warnings will disappear

### Shell crates (CLIs, WASM) fail to build after migration

**Cause:** This should NOT happen. All TrustEdge shell crates (CLI tools, WASM bindings) already use `trustedge-core` directly.

**Solution:**
- If you encounter this, it's a bug. Please report it: https://github.com/TrustEdge-Labs/trustedge/issues
- Include your `Cargo.toml`, error message, and Rust version in the report

---

## Need Help?

If you encounter issues not covered in this guide:

- **Check the CHANGELOG**: See [CHANGELOG.md](CHANGELOG.md) for deprecation notices and version history
- **Open an issue**: https://github.com/TrustEdge-Labs/trustedge/issues
- **Review facade READMEs**: Each deprecated crate has a README with specific migration instructions:
  - [crates/receipts/README.md](crates/receipts/README.md)
  - [crates/attestation/README.md](crates/attestation/README.md)

---

## For Library Maintainers

If you maintain a library that depends on `trustedge-receipts` or `trustedge-attestation`:

1. **Update your dependencies** to use `trustedge-core` as soon as possible
2. **Publish a new version** of your library with the updated dependency
3. **Document the change** in your CHANGELOG so your users know to upgrade
4. **Consider SemVer impact**:
   - If you re-export TrustEdge types in your public API, this is a breaking change (requires major version bump)
   - If TrustEdge is only an internal dependency, this is a non-breaking change (patch or minor version bump)

---

## Timeline Reminder

- **Now through August 2026**: Update at your convenience using this guide
- **August 2026 (version 0.4.0)**: Facade crates will be removed
- **After 0.4.0**: Projects still using facades will fail to compile

We recommend migrating as soon as possible to avoid last-minute issues before the 0.4.0 deadline.
