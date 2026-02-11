<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/TrustEdge-Labs/trustedge
-->


# External Integrations

**Analysis Date:** 2026-02-09

## APIs & External Services

**Pubky Network (Community/Experimental):**
- `pubky` 0.5.4 crate integration (only in `crates/pubky/` and `crates/pubky-advanced/`)
  - SDK/Client: `pubky` Rust crate from crates.io
  - Purpose: Decentralized key publishing and discovery for hybrid encryption
  - Integration type: Optional feature (not core product)
  - Used by: `trustedge-pubky` binary (key publishing) and `trustedge-pubky-advanced` (key resolution)

**HTTP/Network APIs:**
- `reqwest` 0.11.27 (HTTP client)
  - Used in: `crates/pubky-advanced/` for key resolution queries, `crates/trst-cli/` for archive operations
  - Pattern: Async HTTP calls via `tokio` runtime
  - Auth: None (request-response pattern only)

## Data Storage

**Databases:**
- Not detected - No relational database integration (PostgreSQL, MySQL, SQLite, etc.)
- No ORM usage (sqlx, diesel, prisma, etc.)

**File Storage:**
- Local filesystem only
- Archive format: `.trst` (TrustEdge archive) - custom container format in `crates/trst-core/`
- Manifest format: JSON serialized with `serde_json` (schema in `crates/trst-core/`)
- Storage locations:
  - CLI receives file paths as arguments (`PathBuf` via clap)
  - No cloud storage integration (S3, GCS, Azure Blob, etc.)

**Caching:**
- In-memory caching only:
  - YubiKey: `HashMap<String, CK_OBJECT_HANDLE>` for cached keys (file: `crates/core/src/backends/yubikey.rs`)
  - No distributed cache (Redis, Memcached, etc.)

## Authentication & Identity

**Auth Provider:**
- Custom mutual authentication (no external provider)
- Implementation approach:
  - Ed25519 signatures for identity verification
  - Session-based mutual authentication (file: `crates/core/src/auth.rs`)
  - Server certificate identity + optional client certificate identity
  - Keyring integration for local credential storage (`keyring` 2.0 crate)
  - Location: `crates/core/src/backends/keyring.rs` (Software HSM backend)

**Hardware Identity (YubiKey):**
- PKCS#11 interface to YubiKey devices (optional feature: `yubikey`)
- Implementation: `crates/core/src/backends/yubikey.rs`
- Certificate-based identity via X.509 (when feature enabled)

**Pubky Identity (Experimental):**
- Decentralized identity via Pubky network
- Public keys published to Pubky DHT
- No centralized auth server - purely decentralized

## Monitoring & Observability

**Error Tracking:**
- Not detected - No integration with Sentry, DataDog, New Relic, etc.
- Error handling via:
  - `anyhow` for CLIs (contextual error messages)
  - `thiserror` for libraries (custom error types)
  - Panic hooks in WASM (console_error_panic_hook for browser debugging)

**Logs:**
- Standard approach: `std::println!`, `eprintln!` (CLIs)
- Optional: `RUST_LOG` environment variable respected in tests
- No centralized logging service or structured logging framework detected
- Output to stdout/stderr only

**Benchmarking:**
- Criterion benchmarks in `crates/core/benches/`:
  - `crypto_benchmarks.rs` - Encryption/decryption performance
  - `network_benchmarks.rs` - Transport layer throughput
  - Generates HTML reports (enabled via `criterion` feature)

## CI/CD & Deployment

**Hosting:**
- GitHub Actions for CI/CD
- Release channel: GitHub Releases (implied by CHANGELOG.md)
- WASM deployment: npm packages (not yet published, but structure ready)
- No cloud deployment detected (AWS, GCP, Heroku, etc.)

**CI Pipeline:**
- Location: `.github/workflows/`
- Jobs:
  - `ci.yml`: Main Rust testing (format, clippy, tests with/without features)
  - `wasm-tests.yml`: WASM-specific tests via `wasm-pack test`
  - `copyright-check.yml`: License header verification
- Features tested conditionally:
  - `audio` - Only if ALSA/CoreAudio libraries available
  - `yubikey` - Only if PC/SC libraries available
  - `envelope` - Feature for attestation crate
- Test command: `cargo test --workspace --locked`

## Environment Configuration

**Required env vars (optional, have defaults):**
- `TRUSTEDGE_DEVICE_ID` - Device identifier (default: "trustedge-abc123")
- `TRUSTEDGE_SALT` - Key derivation salt (default: "trustedge-demo-salt")
- `RUST_LOG` - Logging level (tests only)
- `BENCH_FAST` - Enable fast benchmark mode (benchmarks only)
- CI detection: `CI`, `GITHUB_ACTIONS`, `GITLAB_CI`, `TRAVIS`, `CIRCLECI`

**Secrets location:**
- Keyring system (local OS credential storage): `keyring` 2.0 crate
- Server signing keys: Generated/stored locally as files (optional `--server-key` argument)
- No external secrets manager detected (HashiCorp Vault, AWS Secrets, etc.)
- YubiKey PIN: Interactive input only (not stored)

## Webhooks & Callbacks

**Incoming:**
- Not detected - No webhook endpoints defined

**Outgoing:**
- Not detected - No webhook calls made to external services
- Pubky integration is pull-based (key resolution) not push-based

## Transport & Network

**Network Protocols:**
- **TCP (framed):** Default transport with length-delimited codec (tokio-util)
  - Location: `crates/core/src/transport/tcp.rs`
  - Used by: `trustedge-server` and `trustedge-client` binaries
  - Framing: Length-prefix (u32 big-endian) for message boundaries

- **QUIC/TLS:** Optional secure transport
  - Location: `crates/core/src/transport/quic.rs`
  - Provider: `quinn` 0.11.9 with `rustls` 0.23
  - Mutually authenticated via certificates
  - Not used by default (TCP is default)

**Connection Handling:**
- Async via Tokio (`tokio::net::TcpListener`, `tokio::net::TcpStream`)
- Session management: `SessionManager` tracks authenticated connections
- Graceful shutdown: Ctrl+C handling on server

## Integration Patterns

**None Detected:**
- No payment processing (Stripe, PayPal, etc.)
- No email service (SendGrid, Mailgun, etc.)
- No SMS/messaging (Twilio, etc.)
- No analytics (Google Analytics, Mixpanel, etc.)
- No issue tracking (Jira, Linear, etc.)
- No documentation platform (Notion, Confluence, etc.)
- No container registry (Docker Hub, ECR, GCR, etc.)

---

*Integration audit: 2026-02-09*
