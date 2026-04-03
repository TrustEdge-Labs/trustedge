<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# Phase 66: Crypto & CLI Hardening - Context

**Gathered:** 2026-03-25
**Status:** Ready for planning

<domain>
## Phase Boundary

NetworkChunk::new() requires nonce as mandatory parameter (no zero-nonce default). All process::exit() calls in trst-cli replaced with proper error returns preserving exit codes. --chunk-size has a 256 MB upper bound.

</domain>

<decisions>
## Implementation Decisions

### NetworkChunk zero-nonce removal (CRYPT-01)
- **D-01:** Change `NetworkChunk::new()` to require nonce parameter — merge the current `new_with_nonce()` signature into `new()`. The new signature: `new(seq: u64, encrypted_data: Vec<u8>, manifest_bytes: Vec<u8>, nonce: [u8; NONCE_LEN])`.
- **D-02:** Remove the old zero-nonce `new()` and rename `new_with_nonce()` to `new()`. All callers must be updated.
- **D-03:** Callers of the old `new()` must be found and updated to pass an explicit nonce. Check: `crates/core/src/bin/trustedge-client.rs`, `crates/core/src/transport/tcp.rs`, `crates/core/src/transport/quic.rs`, and any other files calling `NetworkChunk::new(`.

### process::exit replacement (CLI-01)
- **D-04:** Replace all 11 `process::exit()` calls in `crates/trst-cli/src/main.rs` with anyhow error returns. Subcommand functions return `Result<()>` (most already do), and errors propagate to `main()`.
- **D-05:** In `main()`, map error types to specific exit codes: 10 (verify failure), 11 (integrity failure), 12 (signature failure), 14 (chain failure), 1 (general). Use anyhow's downcast or a simple string/context matching approach.
- **D-06:** The exit code mapping happens AFTER all local variables are dropped, ensuring Drop/Zeroize handlers run on key material before the process exits.
- **D-07:** Specific exit codes used today (from grep): 1 (general, lines 311, 980), 10 (verify failure, lines 884, 892, 932), 11 (integrity failure, lines 874, 939), 12 (signature failure, lines 796, 827), 14 (chain failure, line 840). Line 1237 maps HTTP status to exit code.

### Chunk-size upper bound (CLI-02)
- **D-08:** Add a `value_parser` or validation check on the `--chunk-size` clap argument in `crates/trst-cli/src/main.rs:103` that rejects values above 256 MB (268_435_456 bytes). Use clap's `value_parser!(..=268_435_456)` range or a manual check with `bail!`.
- **D-09:** Error message: `"--chunk-size must not exceed 256 MB (268435456 bytes)"`.

### Claude's Discretion
- Whether to use a custom error type or string-based context matching for exit code mapping
- Whether to use clap's value_parser range vs manual validation for chunk-size
- How to handle the HTTP status-to-exit-code mapping at line 1237 (may stay as-is if it's in a code path where no key material is in scope)

</decisions>

<canonical_refs>
## Canonical References

No external specs — requirements fully captured in decisions above.

### Files to modify
- `crates/core/src/lib.rs` — NetworkChunk::new() and new_with_nonce() (Finding 12, lines 208-240)
- `crates/trst-cli/src/main.rs` — 11 process::exit() call sites (Finding 13) + chunk_size validation (Finding 14, line 103)

### Callers to update (NetworkChunk::new)
- `crates/core/src/bin/trustedge-client.rs` — network client binary
- `crates/core/src/transport/tcp.rs` — TCP transport
- `crates/core/src/transport/quic.rs` — QUIC transport

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `new_with_nonce()` already exists with the correct signature — just needs to become `new()`
- Most subcommand functions already return `Result<()>` — error propagation is natural
- clap's `value_parser` supports range validation natively

### Established Patterns
- anyhow for CLI error handling (CLAUDE.md mandate)
- `bail!()` macro for early returns with error messages
- Exit code conventions: 10 (verify), 11 (integrity), 12 (signature), 14 (chain)

### Integration Points
- NetworkChunk is used in transport (TCP/QUIC) and client binary — all callers need nonce
- process::exit() calls are in verify, unwrap, emit-request subcommands — all need error return refactoring
- chunk_size is a clap argument on the WrapArgs struct

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard hardening patterns.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 66-crypto-cli-hardening*
*Context gathered: 2026-03-25*
