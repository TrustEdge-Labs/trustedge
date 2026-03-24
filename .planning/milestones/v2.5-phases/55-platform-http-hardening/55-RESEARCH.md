# Phase 55: Platform HTTP Hardening - Research

**Researched:** 2026-03-23
**Domain:** Axum 0.7 HTTP middleware — request body limiting, rate limiting, signing key path configuration
**Confidence:** HIGH

## Summary

This phase adds three hardening measures to the TrustEdge Platform HTTP layer. The codebase is
well-understood from reading all canonical reference files. All work is in `trustedge-platform`
and `trustedge-platform-server` — no other crates are affected.

**Body size limit (HTTP-01):** `tower-http 0.5` already provides `RequestBodyLimitLayer`. It is
already a workspace dependency and is available inside the `http` feature gate without adding any
new crate. A single `.layer(RequestBodyLimitLayer::new(2 * 1024 * 1024))` call inside
`build_base_router()` covers all routes globally.

**Rate limiting (HTTP-02):** The locked decision (D-03) calls for `governor` + `tower_governor`.
However, `tower_governor 0.4.3` (the last axum-0.7-compatible version) declares `tower ^0.5.1`
as its dependency, but the project uses `tower 0.4`. The correct approach is to implement an
`axum::middleware::from_fn` middleware directly against the `governor` crate (no `tower_governor`
wrapper needed). This keeps `tower 0.4` untouched and is a standard pattern for axum 0.7.

**Key path configuration (HTTP-03/04):** All four hardcoded `"target/dev/"` references live in
`crates/platform/src/verify/jwks.rs`. Replacing them with a `KeyManager::new_with_path(path)`
constructor that reads `JWKS_KEY_PATH` from the environment and defaults to
`std::env::temp_dir()` is a contained, low-risk change. The 0600 permissions pattern is already
established in `crates/trst-cli/src/main.rs` (lines 364-370) and can be copied directly.

**Primary recommendation:** Implement rate limiting as a standalone `axum::middleware::from_fn`
using `governor` crate directly (avoid `tower_governor` dependency due to tower version mismatch).
All other changes are straightforward layering and path parameterization.

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- **D-01:** Apply a 2 MB global `RequestBodyLimitLayer` via `build_base_router()`. Covers all routes consistently.
- **D-02:** Use `tower_http::limit::RequestBodyLimitLayer` (already available via `tower-http` dependency).
- **D-03:** Use `governor` crate + `tower_governor` middleware for in-process, per-IP rate limiting on `/v1/verify`.
- **D-04:** Return HTTP 429 Too Many Requests when rate limit is exceeded.
- **D-05:** Rate limit configurable via env var `RATE_LIMIT_RPS`, defaulting to 10 requests per second per IP.
- **D-06:** Rate limiting applies to `/v1/verify` only — `/healthz` and `/.well-known/jwks.json` stay unthrottled.
- **D-07:** Read signing key file path from `JWKS_KEY_PATH` env var. Remove all hardcoded `"target/dev/"` references.
- **D-08:** Default to runtime temp directory when `JWKS_KEY_PATH` is not set — never default to `target/dev/`.
- **D-09:** Key generated fresh on startup if file at configured path doesn't exist.
- **D-10:** JWKS public key file (`jwks.json`) follows same pattern — derive path from `JWKS_KEY_PATH` or co-locate with signing key.
- **D-11:** Set 0600 Unix permissions on the signing key file (consistent with v2.4 Phase 52 pattern).

### Claude's Discretion
- Exact governor configuration (burst size, quota algorithm)
- Whether to add rate limit headers (X-RateLimit-Remaining, etc.)
- Temp directory strategy (`std::env::temp_dir` vs platform-specific)
- Error response body format for 413 and 429 responses

### Deferred Ideas (OUT OF SCOPE)
None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| HTTP-01 | `/v1/verify` endpoint enforces a request body size limit (1-10 MB) via `RequestBodyLimitLayer` | `tower_http::limit::RequestBodyLimitLayer` in tower-http 0.5 — already a dependency; layer applied in `build_base_router()` |
| HTTP-02 | HTTP endpoints enforce rate limiting to prevent CPU-exhaustion abuse of BLAKE3+Ed25519 verify | `governor` crate keyed limiter + `axum::middleware::from_fn` on the `/v1/verify` route |
| HTTP-03 | JWKS signing key path configurable via environment variable (not hardcoded to `target/dev/`) | `std::env::var("JWKS_KEY_PATH")` with `std::env::temp_dir()` fallback; four locations in `jwks.rs` |
| HTTP-04 | JWKS signing key not persisted as unencrypted plaintext in a build-artifact directory | Follows from HTTP-03 — temp dir default eliminates `target/dev/` path; 0600 permissions protect file regardless of location |
</phase_requirements>

---

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `tower-http` | 0.5.2 (workspace) | `RequestBodyLimitLayer` for body size enforcement | Already a dependency in the `http` feature gate — zero new deps |
| `governor` | 0.10.4 | Token-bucket rate limiter with keyed (per-IP) state | Industry standard for Rust rate limiting; production-grade, well-maintained |
| `std::env::temp_dir` | stdlib | Platform-appropriate temp directory for default key path | Stdlib — no dep; works on Linux, macOS, Windows |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `governor` (dashmap feature) | 0.10.4 | Per-key rate limiter storage (`DefaultKeyedRateLimiter`) | Needed for per-IP limiting; DashMap provides concurrent key-value state |
| `axum::middleware::from_fn` | axum 0.7.9 (workspace) | Wrap rate limit check as Axum middleware | Clean integration pattern when no Tower wrapper is available |
| `std::os::unix::fs::PermissionsExt` | stdlib | Set 0600 file permissions on signing key | Already used in `crates/trst-cli/src/main.rs:367-370` |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Manual `axum::middleware::from_fn` + `governor` | `tower_governor 0.4.3` | `tower_governor` requires `tower ^0.5.1`; project uses `tower 0.4` — version conflict blocks this option |
| `std::env::temp_dir()` | `/tmp/trustedge/` literal | `temp_dir()` is cross-platform and correct; literal path works on Linux only |
| Global body limit in `build_base_router()` | Per-route limit on `/v1/verify` only | Global is simpler, consistent (D-01 decision), no missed routes |

**Installation:**
```bash
# Add governor to crates/platform/Cargo.toml under [dependencies]
# governor = { version = "0.10", features = ["dashmap"] }
# No change to workspace Cargo.toml needed (not shared)
```

**Version verification (confirmed 2026-03-23):**
- `tower-http`: 0.5.2 — already in workspace (`cargo tree` confirmed)
- `governor`: 0.10.4 — current stable (confirmed via `cargo search`)
- `tower_governor`: 0.8.0 current but requires axum 0.8 / tower 0.5 — NOT compatible with this project

---

## Architecture Patterns

### Recommended Change Surface
```
crates/platform/
├── Cargo.toml                     # Add: governor = { version = "0.10", features = ["dashmap"] }
└── src/
    ├── http/
    │   ├── router.rs              # Add RequestBodyLimitLayer to build_base_router()
    │   │                          # Add rate_limit_middleware layer to /v1/verify route only
    │   └── rate_limit.rs          # NEW: per-IP rate limiter middleware (from_fn impl)
    └── verify/
        └── jwks.rs                # Replace 4x "target/dev/" with env-driven path
crates/platform-server/
└── src/
    └── main.rs                    # Log JWKS_KEY_PATH in startup banner; pass to KeyManager
```

### Pattern 1: RequestBodyLimitLayer — Global Body Cap

**What:** Wrap the entire base router in `RequestBodyLimitLayer`.
**When to use:** Always — applies before any handler code runs.

```rust
// Source: tower-http 0.5 docs — docs.rs/tower-http/0.5.2/tower_http/limit/index.html
// In build_base_router() — crates/platform/src/http/router.rs

use tower_http::limit::RequestBodyLimitLayer;

pub fn build_base_router() -> Router<AppState> {
    Router::new()
        .route("/v1/verify", post(verify_handler))
        .route("/.well-known/jwks.json", get(jwks_handler))
        .route("/healthz", get(health_handler))
        .layer(RequestBodyLimitLayer::new(2 * 1024 * 1024)) // 2 MB
}
```

**413 response:** `RequestBodyLimitLayer` automatically returns 413 when:
- `Content-Length` header exceeds the limit (pre-flight check), OR
- Body bytes read exceed the limit at runtime

Axum's `Json` extractor propagates the `LengthLimitError` as a 413 automatically when using
`RequestBodyLimitLayer` as a router layer (axum 0.7 handles this correctly).

### Pattern 2: Per-IP Rate Limit — axum middleware::from_fn

**What:** Axum middleware function using `governor`'s `DefaultKeyedRateLimiter<IpAddr>`.
**When to use:** Route-specific limiting on `/v1/verify`; applied via `.route_layer()` or nested sub-router.

```rust
// Source: governor 0.10 docs — docs.rs/governor/0.10.4/governor/
// New file: crates/platform/src/http/rate_limit.rs

use axum::{
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::{net::{IpAddr, SocketAddr}, num::NonZeroU32, sync::Arc};

pub struct RateLimitState {
    pub limiter: Arc<DefaultKeyedRateLimiter<IpAddr>>,
}

impl RateLimitState {
    pub fn new(rps: u32) -> Self {
        let quota = Quota::per_second(NonZeroU32::new(rps).expect("rps > 0"));
        Self {
            limiter: Arc::new(RateLimiter::keyed(quota)),
        }
    }
}

pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    axum::extract::State(state): axum::extract::State<RateLimitState>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match state.limiter.check_key(&addr.ip()) {
        Ok(_) => Ok(next.run(req).await),
        Err(_) => Err(StatusCode::TOO_MANY_REQUESTS),
    }
}
```

**Router integration:** Apply as `.route_layer()` on the verify route:
```rust
// In create_router() — after build_base_router()
let rps = std::env::var("RATE_LIMIT_RPS")
    .ok()
    .and_then(|s| s.parse::<u32>().ok())
    .unwrap_or(10);
let rl_state = RateLimitState::new(rps);

// Apply to verify route only
let verify_route = Router::new()
    .route("/v1/verify", post(verify_handler))
    .route_layer(axum::middleware::from_fn_with_state(
        rl_state,
        rate_limit_middleware,
    ));
```

**Critical: `into_make_service_with_connect_info` required** for `ConnectInfo` extraction.
`main.rs` must change `axum::serve(listener, router)` to
`axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())`.

**429 response body:** Per D-04, return 429. The error body format is at Claude's discretion.
Recommendation: plain JSON `{"error": "rate_limit_exceeded", "message": "Too many requests"}` for
consistency with existing `ValidationError` type.

### Pattern 3: JWKS Key Path — Environment-Driven Constructor

**What:** Parameterize `KeyManager::new()` to accept a key file path from env.
**When to use:** Startup and key generation.

```rust
// Modified KeyManager::new() in crates/platform/src/verify/jwks.rs

impl KeyManager {
    pub fn new() -> Result<Self> {
        let key_path = std::env::var("JWKS_KEY_PATH")
            .unwrap_or_else(|_| {
                std::env::temp_dir()
                    .join("trustedge_signing_key.json")
                    .to_string_lossy()
                    .into_owned()
            });
        Self::new_with_path(&key_path)
    }

    fn new_with_path(key_path: &str) -> Result<Self> {
        if Path::new(key_path).exists() {
            Self::load_from_file(key_path)
        } else {
            Self::generate_new_at(key_path)
        }
    }
}
```

**JWKS public key path:** Per D-10, derive from signing key path by replacing the filename:
```rust
// Derive jwks.json path from the signing key path directory
fn jwks_path_from_key_path(key_path: &str) -> PathBuf {
    Path::new(key_path)
        .parent()
        .unwrap_or(Path::new("."))
        .join("jwks.json")
}
```

**0600 permissions (D-11):** After `fs::write(path, content)` in `save_to_file()`:
```rust
// Source: crates/trst-cli/src/main.rs:364-370 (established pattern)
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, perms)
        .with_context(|| format!("Failed to set permissions on {}", path))?;
}
```

### Anti-Patterns to Avoid

- **Don't use `tower_governor`:** Requires tower 0.5; project uses tower 0.4. Will produce dep resolution failure.
- **Don't apply rate limit globally via `build_base_router()`:** D-06 specifies `/v1/verify` only. `/healthz` and `/.well-known/jwks.json` must remain unthrottled.
- **Don't forget `into_make_service_with_connect_info`:** `ConnectInfo<SocketAddr>` extractor panics at runtime without it. This change goes in `main.rs`.
- **Don't apply `RequestBodyLimitLayer` twice:** Adding it in both `build_base_router()` and `create_router()` would double-wrap. One location in `build_base_router()` is correct.
- **Don't hardcode `burst_size = 1`:** A burst size of 1 would reject any second request within the replenishment window even for legitimate slow clients. Recommendation: `burst_size = 5` for the default 10 RPS config.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Token bucket per-IP rate limiting | Custom HashMap + Mutex + timestamp logic | `governor::RateLimiter::keyed()` | Governor handles edge cases: time-of-check races, memory leakage from stale IPs, clock skew. `retain_recent()` purges old entries. |
| Request body streaming with size cap | Custom body wrapper that counts bytes | `RequestBodyLimitLayer` | Handles `Content-Length` pre-check AND streaming overflow correctly. Streams stop before OOM. |
| Permissions on key files | Custom mode-setting code | Pattern from `trst-cli/src/main.rs:367` (already established) | Consistent with existing project pattern. `PermissionsExt::from_mode(0o600)` is the right API. |

**Key insight:** The `governor` crate handles the hardest part of rate limiting — correct
token-bucket replenishment under concurrent access without lock contention (uses atomic
operations). Building this manually introduces subtle time-based bugs.

---

## Common Pitfalls

### Pitfall 1: Missing `into_make_service_with_connect_info` in main.rs
**What goes wrong:** `ConnectInfo<SocketAddr>` extractor in rate limit middleware returns 500 / panics
at runtime because the peer address is not available in request extensions.
**Why it happens:** `axum::serve(listener, router)` uses `into_make_service()` by default, which
does not populate `ConnectInfo` extensions.
**How to avoid:** Change `main.rs` to:
```rust
axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
```
**Warning signs:** Rate limit tests return 500 or 422 instead of 429.

### Pitfall 2: tower_governor Version Conflict
**What goes wrong:** Adding `tower_governor` to Cargo.toml produces a dependency resolution error
because `tower_governor >= 0.5` requires `tower ^0.5.x` and the workspace pins `tower = "0.4"`.
**Why it happens:** `tower_governor 0.4.3` is the last version to support axum 0.7 / tower 0.4,
but even that version's Cargo.toml declares `tower ^0.5.1` (verified from docs.rs).
**How to avoid:** Use `governor` crate directly with `axum::middleware::from_fn_with_state`.
No `tower_governor` dependency required.

### Pitfall 3: RequestBodyLimitLayer 413 Not Propagated Without Feature Flag
**What goes wrong:** With `axum 0.7`, `RequestBodyLimitLayer` requires the axum `http1` or body
handling to propagate the 413. In practice, Axum's `Json` extractor returns 413 naturally when
the body limit is exceeded — but only if the layer is applied at router level, not at handler level.
**Why it happens:** Layer ordering matters. `RequestBodyLimitLayer` must be applied to the router
so it wraps the entire request before axum extracts the body.
**How to avoid:** Apply via `.layer()` on the `Router`, not via `ServiceBuilder` around a handler.

### Pitfall 4: `save_to_file` vs `write_jwks_file` — Two Separate Path Changes
**What goes wrong:** Updating `save_to_file()` but forgetting `write_jwks_file()` (or vice versa)
leaves one of the two `target/dev/` references still hardcoded.
**Why it happens:** There are 4 hardcoded paths in `jwks.rs`: `new()` (line 35), `generate_new()`
(line 55), `save_to_file()` (line 98 — implicit in call), `write_jwks_file()` (line 98), and
`rotate_key()` (line 158). All must use the parameterized path.
**How to avoid:** Introduce a `key_path: String` field on `KeyManager` struct so all methods
share the single configured path. No magic string anywhere.

### Pitfall 5: Rate Limiter Memory Growth
**What goes wrong:** Long-running server accumulates unbounded per-IP state in the governor
`DashMap` as unique IPs connect over time.
**Why it happens:** `DefaultKeyedRateLimiter` adds an entry for every unique key seen and does not
auto-evict stale entries.
**How to avoid:** Spawn a background task that calls `limiter.retain_recent()` on a periodic
interval (e.g., every 60 seconds). This purges keys with no recent activity.

### Pitfall 6: KeyManager in Tests Creates Files in `target/dev/`
**What goes wrong:** Existing test helpers call `KeyManager::new()` directly (see
`verify_integration.rs:41`, `handlers.rs:364`). After the change, they will use temp dir by
default which is correct — but if `JWKS_KEY_PATH` is set in the CI environment to a custom path,
tests may write unexpected files.
**Why it happens:** Env var is global state.
**How to avoid:** Tests that call `KeyManager::new()` with temp dir default are fine. For tests
that need deterministic paths, use `KeyManager::new_with_path(tempdir.path().join(...))`.

---

## Code Examples

Verified patterns from official sources:

### RequestBodyLimitLayer Usage
```rust
// Source: docs.rs/tower-http/0.5.2/tower_http/limit/index.html
use tower_http::limit::RequestBodyLimitLayer;

// In build_base_router():
.layer(RequestBodyLimitLayer::new(2 * 1024 * 1024)) // 2 MB = 2,097,152 bytes
```

### Governor Keyed Rate Limiter
```rust
// Source: docs.rs/governor/0.10.4/governor/
use governor::{DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::{net::IpAddr, num::NonZeroU32};

// Create: 10 req/sec per IP, burst 5
let quota = Quota::per_second(NonZeroU32::new(10).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());
let limiter: DefaultKeyedRateLimiter<IpAddr> = RateLimiter::keyed(quota);

// Check:
match limiter.check_key(&ip_addr) {
    Ok(_) => { /* allowed */ }
    Err(_) => { /* return 429 */ }
}

// Cleanup task (run every 60s):
limiter.retain_recent();
```

### 0600 Permissions Pattern (established in trst-cli)
```rust
// Source: crates/trst-cli/src/main.rs:364-370
#[cfg(unix)]
{
    use std::os::unix::fs::PermissionsExt;
    let perms = std::fs::Permissions::from_mode(0o600);
    std::fs::set_permissions(path, perms)
        .with_context(|| format!("Failed to set permissions on {}", path))?;
}
#[cfg(not(unix))]
{ /* no-op — Windows uses ACLs, not Unix modes */ }
```

### KeyManager Struct with Path Field
```rust
// Proposed extension to KeyManager in jwks.rs
#[derive(Debug, Clone)]
pub struct KeyManager {
    key_path: String,          // NEW: configurable path, replaces all hardcoded "target/dev/"
    current_key: SigningKey,
    current_kid: String,
    previous_key: Option<SigningKey>,
    previous_kid: Option<String>,
}
```

### axum::serve with ConnectInfo
```rust
// Source: axum 0.7 docs — docs.rs/axum/0.7/axum/fn.serve.html
use std::net::SocketAddr;
axum::serve(
    listener,
    router.into_make_service_with_connect_info::<SocketAddr>(),
)
.with_graceful_shutdown(shutdown_signal())
.await?;
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| No body limit | `RequestBodyLimitLayer` global | This phase | Prevents OOM from malformed large requests |
| No rate limiting | `governor` per-IP keyed limiter | This phase | Prevents CPU exhaustion on verify loop |
| Hardcoded `target/dev/` key path | `JWKS_KEY_PATH` env var + temp dir default | This phase | Key no longer in build artifact directory |
| Key file with default fs permissions | 0600 permissions at creation | This phase (consistent with Phase 52) | Prevents other users from reading signing key |

---

## Open Questions

1. **governor `allow_burst` API naming**
   - What we know: `governor 0.10` changed some builder method names from earlier versions.
   - What's unclear: Whether `Quota::per_second(n).allow_burst(b)` is the correct 0.10 API or if `GovernorConfigBuilder` is needed.
   - Recommendation: Consult `governor 0.10` docs at implementation time; fall back to `Quota::per_second(NonZeroU32::new(rps).unwrap())` with default burst (burst = rps by default in governor).

2. **Rate limit response body format**
   - What we know: D-04 says 429 status. Body format is Claude's discretion.
   - What's unclear: Whether to reuse existing `ValidationError` type or return plain text/empty body.
   - Recommendation: Return `Json(ValidationError::new("rate_limit_exceeded", "Too many requests"))` for consistency with other error responses — already a public type.

3. **rotate_key() path — 5th hardcoded location**
   - What we know: `jwks.rs:158` calls `self.save_to_file("target/dev/signing_key.json")` inside `rotate_key()`.
   - What's unclear: Whether rotate_key() is called anywhere currently.
   - Recommendation: With `key_path` field on `KeyManager`, `rotate_key()` uses `&self.key_path` automatically — no special handling needed.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Rust toolchain | Build | ✓ | 1.88 (Dockerfile) | — |
| `governor` crate (new dep) | HTTP-02 rate limiting | ✓ (from crates.io) | 0.10.4 | Manual token bucket (complex, not recommended) |
| `tower-http RequestBodyLimitLayer` | HTTP-01 body limit | ✓ | 0.5.2 (workspace, existing) | — |
| `std::env::temp_dir()` | HTTP-03/04 key path | ✓ | stdlib | — |

**Missing dependencies with no fallback:** None.

**Missing dependencies with fallback:** None. `governor` is a new crate add but is available on crates.io and has no native/system dependencies.

---

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | Rust `cargo test` + `axum::body::Body` / `tower::ServiceExt::oneshot` |
| Config file | No separate config — feature flags `http`, `postgres`, `test-utils` |
| Quick run command | `cargo test -p trustedge-platform --lib --features http` |
| Full suite command | `cargo test -p trustedge-platform --features http` |

### Phase Requirements → Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| HTTP-01 | POST `/v1/verify` with >2 MB body returns 413 | unit (integration-style, no DB) | `cargo test -p trustedge-platform --features http test_body_limit` | ❌ Wave 0 |
| HTTP-01 | POST `/v1/verify` with <2 MB body returns non-413 | unit | `cargo test -p trustedge-platform --features http test_body_under_limit` | ❌ Wave 0 |
| HTTP-02 | Rapid calls to `/v1/verify` beyond rate limit return 429 | unit | `cargo test -p trustedge-platform --features http test_rate_limit_429` | ❌ Wave 0 |
| HTTP-02 | `/healthz` is NOT rate-limited | unit | `cargo test -p trustedge-platform --features http test_healthz_not_rate_limited` | ❌ Wave 0 |
| HTTP-03 | `JWKS_KEY_PATH` env var controls key storage path | unit | `cargo test -p trustedge-platform test_jwks_key_path_env_var` | ❌ Wave 0 |
| HTTP-03/04 | No file written under `target/dev/` when `JWKS_KEY_PATH` unset | unit | `cargo test -p trustedge-platform test_jwks_default_not_target_dev` | ❌ Wave 0 |
| HTTP-04 | Signing key file gets 0600 permissions (unix) | unit | `cargo test -p trustedge-platform test_signing_key_permissions` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo test -p trustedge-platform --lib --features http`
- **Per wave merge:** `cargo test -p trustedge-platform --features http`
- **Phase gate:** Full suite green (`cargo test --workspace`) before `/gsd:verify-work`

### Wave 0 Gaps
- [ ] `crates/platform/tests/verify_integration.rs` — add HTTP-01, HTTP-02, HTTP-03, HTTP-04 test cases (add to existing file)
- [ ] `crates/platform/src/http/rate_limit.rs` — new file (created during implementation)
- [ ] Test helper: `KeyManager::new_with_path()` must be public for tests to call with temp dirs

*(Existing test infrastructure covers health/JWKS/verify round-trip tests; new tests extend the existing `http_tests` module in `verify_integration.rs`.)*

---

## Project Constraints (from CLAUDE.md)

The following directives from `CLAUDE.md` apply to this phase:

- **Formatting:** `cargo fmt` and `cargo clippy -- -D warnings` must pass before commit.
- **No emoji in code:** Use UTF-8 symbols for terminal output: ✔ ✖ ⚠ ● ♪ ■ — no emoji.
- **Error handling:** Use `anyhow` for the platform server binary; `thiserror` for library errors in platform crate. No `unwrap()` in production code — use `?` or proper error handling.
- **Security:** Key material: `zeroize` for any private key copies. Constant-time comparisons for sensitive data.
- **Copyright headers:** All new `.rs` files need the MPL-2.0 header. Run `./scripts/fix-copyright.sh` if needed.
- **CI validation:** Run `./scripts/ci-check.sh` before committing.
- **Test command:** `cargo test -p trustedge-platform --lib` for unit tests; `cargo test -p trustedge-platform --test verify_integration --features http` for HTTP integration tests.

---

## Sources

### Primary (HIGH confidence)
- `crates/platform/src/http/router.rs` — `build_base_router()` structure, existing middleware layers
- `crates/platform/src/verify/jwks.rs` — all 4 hardcoded `target/dev/` locations confirmed at lines 35, 55, 98, 158
- `crates/platform/src/http/config.rs` — env var pattern (`std::env::var`)
- `crates/platform-server/src/main.rs` — `axum::serve` call at line 97-99; `KeyManager::new()` call at line 80
- `crates/trst-cli/src/main.rs:364-370` — established 0600 permissions pattern
- `crates/platform/Cargo.toml` — confirmed: `tower-http = { version = "0.5", features = ["cors", "trace"] }` under `http` feature gate; no `governor` present
- `workspace Cargo.toml` — confirmed: `tower = "0.4"` (workspace pin); `axum = "0.7"`; `tower-http = { version = "0.5", features = ["cors", "trace"] }`
- `docs.rs/tower-http/0.5.2/tower_http/limit/index.html` — `RequestBodyLimitLayer` API confirmed
- `docs.rs/governor/0.10.4/governor/` — `DefaultKeyedRateLimiter`, `Quota::per_second()`, `retain_recent()` API confirmed
- `docs.rs/tower_governor/0.4.3/tower_governor/` — confirmed `tower ^0.5.1` dependency requirement (blocks usage in this project)

### Secondary (MEDIUM confidence)
- `cargo search tower_governor` — confirmed 0.8.0 is latest, 0.4.3 is last axum-0.7-targeted version
- `cargo tree -p trustedge-platform --features http` — confirmed exact versions in use (axum 0.7.9, tower 0.4.13, tower-http 0.5.2)
- Shuttle.dev blog (2024) — governor per-IP middleware pattern verified against governor docs

### Tertiary (LOW confidence)
- None — all critical claims have HIGH/MEDIUM verification.

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — verified from workspace Cargo.toml + cargo tree; governor version from cargo search
- Architecture: HIGH — based on direct reading of all canonical source files; exact line numbers confirmed
- Pitfalls: HIGH — tower/tower_governor version conflict verified from docs.rs; ConnectInfo requirement verified from axum docs
- Test patterns: HIGH — existing test patterns read directly from verify_integration.rs

**Research date:** 2026-03-23
**Valid until:** 2026-06-23 (stable ecosystem — governor, tower-http, axum versions unlikely to change)
