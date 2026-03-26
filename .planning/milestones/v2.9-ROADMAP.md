# Roadmap: TrustEdge

## Milestones

- [Previous milestones archived to .planning/milestones/]
- 🚧 **v2.9 Security Review P2 Remediation** - Phases 68-70 (in progress)

## Phases

- [x] **Phase 68: Insecure Defaults** - Remove or guard dangerous default impls in CAConfig and SoftwareHsmConfig (completed 2026-03-26)
- [x] **Phase 69: Code Quality** - Compile regex once via LazyLock; emit --unencrypted security warning in trst-cli (completed 2026-03-26)
- [x] **Phase 70: Deployment Hardening** - Lock down CI workflow permissions, add missing HTTP security headers, enforce HSTS with HTTP redirect (completed 2026-03-26)

## Phase Details

### Phase 68: Insecure Defaults
**Goal**: Dangerous default configurations cannot reach production without an explicit guard
**Depends on**: Nothing (first phase of milestone)
**Requirements**: DFLT-01, DFLT-02
**Success Criteria** (what must be TRUE):
  1. Attempting to use CAConfig::default() in a non-test build either panics at construction or the Default impl no longer exists — a misconfigured server cannot start silently with a placeholder JWT secret
  2. SoftwareHsmConfig constructed with the "changeme123!" passphrase panics (or is rejected at build time) outside of `#[cfg(test)]` contexts — the demo credential cannot silently protect production keys
  3. The CI test suite continues to compile and pass, confirming test builds are unaffected by the guards
**Plans:** 1/1 plans complete
Plans:
- [ ] 68-01-PLAN.md — Remove Default impls from CAConfig and SoftwareHsmConfig, add test helpers

### Phase 69: Code Quality
**Goal**: The regex hot-path is compile-once and operator-visible warnings accompany insecure CLI usage
**Depends on**: Phase 68
**Requirements**: QUAL-01, QUAL-02
**Success Criteria** (what must be TRUE):
  1. The regex used in validate_segment_hashes() is initialized exactly once via std::sync::LazyLock — no per-request allocation observable under profiling or code inspection
  2. Running `trst wrap --unencrypted` (or any trst-cli command with --unencrypted) prints a visible stderr warning describing the security implication before proceeding
  3. Existing acceptance tests continue to pass, confirming the warning does not break scripted usage
**Plans:** 1/1 plans complete
Plans:
- [x] 69-01-PLAN.md — LazyLock regex in validation.rs + stderr warning for --unencrypted in trst-cli

### Phase 70: Deployment Hardening
**Goal**: CI workflows have least-privilege permissions and all nginx configs emit a complete set of defensive HTTP headers
**Depends on**: Phase 69
**Requirements**: DEPL-01, DEPL-02, DEPL-03
**Success Criteria** (what must be TRUE):
  1. wasm-tests.yml contains an explicit `permissions: contents: read` block at the workflow level, matching the pattern in ci.yml and semver.yml — visible in the YAML source
  2. The nginx.conf served by the dashboard container includes X-Content-Type-Options, X-Frame-Options, Referrer-Policy, and Content-Security-Policy response headers — verifiable with `curl -I`
  3. nginx-ssl.conf.template includes a Strict-Transport-Security header on the TLS vhost and an HTTP-to-HTTPS redirect on port 80 — verifiable by inspecting the template and confirmed by the existing SSL conditional entrypoint
**Plans:** 1/1 plans complete
Plans:
- [x] 70-01-PLAN.md — CI permissions, nginx security headers, HSTS + HTTP redirect

## Progress

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 68. Insecure Defaults | 0/1 | Complete    | 2026-03-26 |
| 69. Code Quality | 1/1 | Complete    | 2026-03-26 |
| 70. Deployment Hardening | 1/1 | Complete    | 2026-03-26 |
