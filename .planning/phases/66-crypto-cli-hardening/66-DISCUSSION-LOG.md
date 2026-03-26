# Phase 66: Crypto & CLI Hardening - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.

**Date:** 2026-03-25
**Phase:** 66-crypto-cli-hardening
**Areas discussed:** NetworkChunk approach, process::exit strategy

---

## NetworkChunk Zero-Nonce

| Option | Description | Selected |
|--------|-------------|----------|
| Make nonce mandatory in new() (Recommended) | Change new() to require nonce, rename new_with_nonce() to new(), update callers | ✓ |
| Remove new() entirely | Delete new(), keep only new_with_nonce() | |
| Mark new() deprecated | #[deprecated] — softer but zero-nonce still compilable | |

**User's choice:** Make nonce mandatory in new() (Recommended)

---

## process::exit Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| anyhow + ExitCode mapping (Recommended) | Return errors, map to exit codes in main() after RAII cleanup | ✓ |
| Custom error enum with exit codes | TrstError variants with associated codes | |
| Always exit(1) | Simpler but loses specific exit codes | |

**User's choice:** anyhow + ExitCode mapping (Recommended)

---

## Claude's Discretion

- Error type approach for exit code mapping
- clap value_parser vs manual chunk-size validation
- HTTP status exit code handling

## Deferred Ideas

None.
