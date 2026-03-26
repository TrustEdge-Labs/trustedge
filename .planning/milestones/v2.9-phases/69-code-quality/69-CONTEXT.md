# Phase 69: Code Quality - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning

<domain>
## Phase Boundary

Two targeted code quality improvements: (1) compile the BLAKE3 hash regex once via LazyLock instead of per-request, and (2) emit a visible stderr security warning whenever `--unencrypted` is used in trst-cli.

</domain>

<decisions>
## Implementation Decisions

### Regex Compilation (QUAL-01)
- **D-01:** Use `std::sync::LazyLock<Regex>` at module level in `validation.rs` to compile the `^b3:[0-9a-f]{64}$` pattern exactly once. Zero new dependencies — LazyLock is stable since Rust 1.80, project uses Rust 1.88.
- **D-02:** Replace the `Regex::new(...).unwrap()` call inside `validate_segment_hashes()` (line 96) with a reference to the static. Function signature and behavior unchanged.

### --unencrypted Warning (QUAL-02)
- **D-03:** Warning text: `"⚠ WARNING: --unencrypted generates/reads plaintext key files. Key material is NOT protected at rest. Use only for CI/automation."` Uses ⚠ symbol per CLAUDE.md conventions.
- **D-04:** Emit via `eprintln!` at the start of each subcommand handler (`wrap`, `unwrap`, `keygen`) when `--unencrypted` is `true`. Once per invocation, before any work begins. Not behind tracing — ensures visibility regardless of log subscriber state.
- **D-05:** The `--unencrypted` flag appears on 3 clap structs: `WrapArgs`, `UnwrapArgs`, `KeygenArgs`. All three paths must emit the warning.

### Claude's Discretion
- Whether to extract the warning into a helper function (e.g., `warn_unencrypted()`) or inline the `eprintln!` at each call site. Either is acceptable.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Regex Location
- `crates/platform/src/verify/validation.rs` — `validate_segment_hashes()` at line 95-111, regex at line 96

### CLI Warning Locations
- `crates/trst-cli/src/main.rs` — `--unencrypted` flag on WrapArgs (line 207), UnwrapArgs (line 248), KeygenArgs (line 267); `load_or_generate_keypair()` at line 1153

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `regex` crate already in platform dependencies — no new dep needed
- `eprintln!` used elsewhere in trst-cli for user-facing messages
- CLAUDE.md specifies UTF-8 symbols for terminal output: ✔ ✖ ⚠ ● ♪ ■

### Established Patterns
- Platform validation uses standalone functions (not methods on structs) — LazyLock static fits naturally
- trst-cli uses `anyhow` for error handling, `clap` for arg parsing
- Warning should not interfere with acceptance test output (tests check exit codes and stdout, not stderr)

### Integration Points
- `validate_segment_hashes()` is called from both `validate_verify_request()` and `validate_verify_request_full()` — both paths benefit from the static regex
- 28 acceptance tests in `trst-cli` — warning on stderr should not break them

</code_context>

<specifics>
## Specific Ideas

No specific requirements — standard code quality improvements following security review findings.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 69-code-quality*
*Context gathered: 2026-03-26*
