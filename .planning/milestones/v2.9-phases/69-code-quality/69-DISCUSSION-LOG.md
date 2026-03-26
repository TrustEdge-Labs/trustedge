# Phase 69: Code Quality - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-26
**Phase:** 69-code-quality
**Areas discussed:** Regex strategy, Warning message tone, Warning placement

---

## Regex Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| std::sync::LazyLock | Static LazyLock<Regex> at module level. Zero new deps, stable since Rust 1.80. | ✓ |
| OnceLock + function | Function-local OnceLock<Regex> initialized on first call. More boilerplate. | |
| You decide | Claude picks the cleanest approach. | |

**User's choice:** std::sync::LazyLock (Recommended)
**Notes:** Matches the finding's recommendation directly. Zero new dependencies.

---

## Warning Message Tone

| Option | Description | Selected |
|--------|-------------|----------|
| Security warning | "WARNING: --unencrypted generates/reads plaintext key files. Key material is NOT protected at rest. Use only for CI/automation." | ✓ |
| Minimal caution | "Note: using unencrypted key file." Brief, non-alarming. | |
| You decide | Claude writes appropriate text. | |

**User's choice:** Security warning (Recommended)
**Notes:** Clear, actionable, appropriate severity for a security-relevant flag.

---

## Warning Placement

| Option | Description | Selected |
|--------|-------------|----------|
| eprintln! early | Emit via eprintln! at start of each subcommand handler when --unencrypted is true. Once per invocation. Uses ⚠ symbol. | ✓ |
| tracing::warn! | Structured logging. May be silent if subscriber not initialized. | |
| You decide | Claude picks approach ensuring visibility. | |

**User's choice:** eprintln! early (Recommended)
**Notes:** Ensures visibility regardless of logging configuration.

## Claude's Discretion

- Whether to extract warning into helper function or inline at each call site

## Deferred Ideas

None.
