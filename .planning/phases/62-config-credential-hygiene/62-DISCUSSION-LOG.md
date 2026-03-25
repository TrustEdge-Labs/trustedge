# Phase 62: Config & Credential Hygiene - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-25
**Phase:** 62-config-credential-hygiene
**Areas discussed:** DATABASE_URL enforcement, CA placeholder rejection

---

## DATABASE_URL Enforcement

| Option | Description | Selected |
|--------|-------------|----------|
| Runtime error in release (Recommended) | cfg!(debug_assertions) gates the fallback: debug keeps default, release returns Err | ✓ |
| Always require, no fallback | Remove fallback entirely, all builds must set DATABASE_URL | |
| Log warning + allow | Keep fallback but log warning, doesn't prevent the issue | |

**User's choice:** Runtime error in release (Recommended)
**Notes:** Debug builds retain dev convenience. Release builds fail explicitly.

---

## CA Placeholder JWT Secret Rejection

| Option | Description | Selected |
|--------|-------------|----------|
| Validate at build() time (Recommended) | CAConfigBuilder::build() checks for placeholder value, panics/errors. Tests set explicit test secret. | ✓ |
| Remove Default impl entirely | No CAConfig::default() at all, forces explicit construction everywhere | |
| Runtime check on first use | Validate lazily when JWT is first used for signing | |

**User's choice:** Validate at build() time (Recommended)
**Notes:** Guard at construction prevents placeholder from reaching any code path. Tests updated to use explicit test values.

---

## Claude's Discretion

- Whether build() panics or returns Result
- Exact DATABASE_URL error message wording
- Docker-compose comment about re-enabling postgres port for dev access

## Deferred Ideas

None — discussion stayed within phase scope.
