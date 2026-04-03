<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 65: Key Material Safety - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Key files auto-generated during `trst wrap` get 0600 Unix permissions (matching `keygen`). PrivateKey struct cannot be accidentally serialized to wire formats — serde derives removed and key_bytes field restricted.

</domain>

<decisions>
## Implementation Decisions

### Auto-generated key file permissions (KEY-01)
- **D-01:** In `load_or_generate_keypair()` in `crates/trst-cli/src/main.rs`, after `fs::write(&secret_path, ...)` (both the unencrypted path at ~line 1115 and the encrypted path at ~line 1128), set 0600 Unix permissions. The public key file keeps default permissions.
- **D-02:** Use the same `set_permissions` pattern already used in the `keygen` subcommand. On non-Unix platforms, log a warning (matching existing keygen behavior).
- **D-03:** Add a test that auto-generates a keypair via the wrap code path and verifies the secret key file has 0600 permissions.

### PrivateKey serde removal (KEY-02)
- **D-04:** Remove `Serialize` and `Deserialize` derives from `PrivateKey` in `crates/core/src/asymmetric.rs:30`. This prevents any code from accidentally serializing private key material to JSON/wire.
- **D-05:** Make `key_bytes` field `pub(crate)` instead of `pub`. The `key_bytes()` accessor method (already exists at line 89) provides read-only access. External consumers already use this accessor.
- **D-06:** Update tests in `asymmetric.rs` that directly access `keypair.private.key_bytes` to use `keypair.private.key_bytes()` instead. These are within the crate so `pub(crate)` would still work, but using the accessor is cleaner.
- **D-07:** Check for any Serialize/Deserialize usage of PrivateKey outside the crate. If found, refactor to use explicit export methods instead.

### Claude's Discretion
- Whether to also make `algorithm` and `key_id` fields `pub(crate)` (they're not sensitive but making them consistent is cleaner)
- Exact error/warning message for non-Unix permission setting
- Whether to extract the permission-setting logic into a shared helper used by both keygen and wrap

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above.

### Files to modify
- `crates/trst-cli/src/main.rs` — `load_or_generate_keypair()` function, auto-gen path missing 0600 (Finding 10)
- `crates/core/src/asymmetric.rs` — PrivateKey struct with Serialize/Deserialize derives and pub key_bytes (Finding 11)

### Reference patterns
- `crates/trst-cli/src/main.rs` keygen subcommand — already sets 0600 on generated key files (use as reference)

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- keygen subcommand's permission-setting code — can be extracted into a shared helper or copy the pattern
- `key_bytes()` accessor already exists on PrivateKey — external consumers likely already use it

### Established Patterns
- `#[cfg(unix)]` gate for `std::os::unix::fs::PermissionsExt` — used in keygen and trst-cli key file operations
- `Secret<T>` wrapper used for other sensitive fields in the project (v1.7) — PrivateKey uses Zeroize + Drop instead (appropriate for byte arrays)

### Integration Points
- PrivateKey is used in `asymmetric.rs` crypto operations, `backends/software_hsm.rs`, and potentially WASM bindings
- `load_or_generate_keypair()` is called only from the `wrap` subcommand handler
- Removing Serialize may break compilation if any code serializes PrivateKey — must check all usages

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard key material protection patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 65-key-material-safety*
*Context gathered: 2026-03-25*
