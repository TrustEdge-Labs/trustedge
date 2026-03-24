# Phase 59: CLI & Deploy Hardening - Context

**Gathered:** 2026-03-24
**Status:** Ready for planning

<domain>
## Phase Boundary

Stop trustedge CLI from printing AES key to stderr and add conditional TLS termination to the nginx deploy stack.

</domain>

<decisions>
## Implementation Decisions

### CLI key suppression
- **D-01:** Add `--show-key` flag to `Args` struct in `crates/trustedge-cli/src/main.rs`. Boolean flag, default false.
- **D-02:** When encrypting with a random key and neither `--key-out` nor `--show-key` is provided, **error out** with message: `"specify --key-out <file> or --show-key to display the encryption key"`. This prevents silent key loss.
- **D-03:** When `--show-key` is provided, print the key to stderr as before (line 376 behavior, but gated behind the flag).
- **D-04:** Remove the current unconditional `eprintln!("NOTE (demo): AES-256 key (hex) = ...")` at main.rs:376.

### nginx TLS
- **D-05:** Add a conditional HTTPS server block to `deploy/nginx.conf` that activates when `SSL_CERT_PATH` and `SSL_KEY_PATH` environment variables are set. Use `envsubst` or nginx template approach.
- **D-06:** HTTP on port 80 stays as-is (no redirect to HTTPS). Both HTTP and HTTPS can coexist — HTTP for local dev, HTTPS for production.
- **D-07:** Update `deploy/docker-compose.yml` to expose port 443, add optional cert volume mount, and pass SSL env vars to the nginx container.
- **D-08:** Document the TLS configuration in `.env.example` or comments in docker-compose.yml.

### Claude's Discretion
- Exact nginx template mechanism (envsubst vs conf.d includes)
- Whether to add a health check on port 443
- Error message wording for the key-out/show-key requirement

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### CLI
- `crates/trustedge-cli/src/main.rs` — `Args` struct (line ~140), `key_out` field (line 163), key generation block (lines 370-379), `eprintln!` at line 376

### Deploy stack
- `deploy/nginx.conf` — Current HTTP-only config (port 80, SPA fallback, healthz)
- `deploy/docker-compose.yml` — Service definitions (platform-server, postgres, dashboard/nginx)
- `deploy/.env.example` — Environment variable documentation

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `#[arg(long)]` clap pattern already used for `key_out`, `key_hex`, `decrypt` etc.
- `anyhow::bail!` available for error-out pattern
- docker-compose already has env_file and volume mount patterns

### Established Patterns
- CLI uses clap derive macros for argument parsing
- Error messages use `anyhow` context pattern
- docker-compose uses `.env` file for configuration
- nginx.conf is a static file copied into container

### Integration Points
- `select_aes_key_with_backend()` at main.rs:329 — where the key generation logic lives
- `deploy/docker-compose.yml` dashboard service — nginx container definition
- `deploy/Dockerfile` — nginx image selection (if custom)

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

*Phase: 59-cli-deploy-hardening*
*Context gathered: 2026-03-24*
