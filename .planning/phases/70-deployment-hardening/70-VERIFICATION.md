---
phase: 70-deployment-hardening
verified: 2026-03-26T21:00:00Z
status: passed
score: 4/4 must-haves verified
---

# Phase 70: Deployment Hardening Verification Report

**Phase Goal:** CI workflows have least-privilege permissions and all nginx configs emit a complete set of defensive HTTP headers
**Verified:** 2026-03-26T21:00:00Z
**Status:** PASSED
**Re-verification:** No -- initial verification

## Goal Achievement

### Observable Truths

| #   | Truth                                                                                          | Status     | Evidence                                                                                             |
| --- | ---------------------------------------------------------------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------- |
| 1   | wasm-tests.yml has explicit least-privilege permissions block                                  | VERIFIED | `permissions: contents: read` at line 27-28, between `env:` and `jobs:` blocks                      |
| 2   | All nginx-served responses include X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Content-Security-Policy headers | VERIFIED | All four headers present at server block level with `always` in both nginx.conf and nginx-ssl.conf.template |
| 3   | HTTPS vhost sends Strict-Transport-Security header                                             | VERIFIED | `add_header Strict-Transport-Security "max-age=31536000" always;` at line 18 of nginx-ssl.conf.template |
| 4   | HTTP port redirects to HTTPS except for /healthz                                               | VERIFIED | Second server block on port 8080: `/healthz` returns 200, `/` returns `301 https://$host:8443$request_uri` |

**Score:** 4/4 truths verified

### Required Artifacts

| Artifact                                   | Expected                                          | Status     | Details                                                                                                               |
| ------------------------------------------ | ------------------------------------------------- | ---------- | --------------------------------------------------------------------------------------------------------------------- |
| `.github/workflows/wasm-tests.yml`         | Least-privilege CI permissions                    | VERIFIED | Contains `permissions: contents: read` (line 27-28); matches ci.yml and semver.yml pattern exactly                   |
| `deploy/nginx.conf`                        | Security headers for plain HTTP dashboard         | VERIFIED | X-Content-Type-Options, X-Frame-Options, Referrer-Policy, CSP at server block level; `grep -c add_header` returns 6  |
| `deploy/nginx-ssl.conf.template`           | Security headers, HSTS, and HTTP redirect for SSL | VERIFIED | Five security headers in SSL server block; second server block on port 8080 with redirect and /healthz passthrough    |

### Key Link Verification

| From                               | To                                | Via                     | Status     | Details                                                                                                      |
| ---------------------------------- | --------------------------------- | ----------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ |
| `deploy/nginx-ssl.conf.template`   | `deploy/docker-entrypoint.sh`     | envsubst pipeline       | VERIFIED | docker-entrypoint.sh line 10-12: `envsubst '${SSL_CERT_PATH} ${SSL_KEY_PATH}' < /etc/nginx/nginx-ssl.conf.template > /tmp/nginx-ssl/ssl.conf` |
| `deploy/nginx-ssl.conf.template`   | docker-compose healthcheck        | `/healthz` on port 8080 | VERIFIED | nginx-ssl.conf.template contains 2 `/healthz` location blocks (one in SSL vhost, one in redirect server block) |

**envsubst safety check:** Template uses `${SSL_CERT_PATH}` and `${SSL_KEY_PATH}` (only two `${` patterns found). New redirect block uses `$host` and `$request_uri` (no curly braces) -- these pass through envsubst untouched. SAFE.

### Data-Flow Trace (Level 4)

Not applicable -- this phase modifies configuration files (nginx configs, YAML workflow), not components that render dynamic data. No state/prop data-flow to trace.

### Behavioral Spot-Checks

Step 7b: SKIPPED for nginx configs and CI YAML -- no runnable entry points to test without starting containers or triggering GitHub Actions. Correctness verified by direct file inspection and pattern matching.

### Requirements Coverage

| Requirement | Source Plan  | Description                                                                                                       | Status    | Evidence                                                                                          |
| ----------- | ------------ | ----------------------------------------------------------------------------------------------------------------- | --------- | ------------------------------------------------------------------------------------------------- |
| DEPL-01     | 70-01-PLAN.md | wasm-tests.yml has explicit `permissions: contents: read` block matching ci.yml and semver.yml                   | SATISFIED | `permissions: contents: read` confirmed at lines 27-28 of wasm-tests.yml; matches pattern        |
| DEPL-02     | 70-01-PLAN.md | nginx.conf includes X-Content-Type-Options, X-Frame-Options, Referrer-Policy, and Content-Security-Policy headers | SATISFIED | All four headers confirmed in nginx.conf at server block level with `always` parameter            |
| DEPL-03     | 70-01-PLAN.md | nginx-ssl.conf.template adds Strict-Transport-Security header and redirects HTTP to HTTPS when TLS is enabled    | SATISFIED | HSTS confirmed at line 18; redirect server block on port 8080 confirmed with 301 and /healthz passthrough |

**Orphaned requirement check:** REQUIREMENTS.md maps exactly DEPL-01, DEPL-02, DEPL-03 to Phase 70. All three are claimed in 70-01-PLAN.md. No orphaned requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| ---- | ---- | ------- | -------- | ------ |
| None | --   | --      | --       | --     |

No TODOs, placeholders, hardcoded empty returns, or stub patterns found in any of the three modified files.

### Human Verification Required

None. All success criteria are verifiable by direct config inspection:

- Permission scopes in YAML are visible in source.
- nginx header directives are deterministic -- if `add_header X-Content-Type-Options "nosniff" always;` is in the server block, the header is sent.
- The redirect block's logic (`return 301 https://...` vs `/healthz` returning 200) is unambiguous nginx config.

One item is noted as requiring a live environment to fully confirm but is not a blocker:

#### 1. HTTP redirect in SSL mode (live container test)

**Test:** Start container with `SSL_CERT_PATH` and `SSL_KEY_PATH` set, then `curl -I http://localhost:8080/` and `curl -I http://localhost:8080/healthz`
**Expected:** `/` returns `301` with `Location: https://...`; `/healthz` returns `200`
**Why human:** Requires a running Docker container with TLS certs provisioned. The config is correct and passes envsubst safety analysis, but live confirmation is not automated here.

### Gaps Summary

No gaps. All four truths verified, all three artifacts are substantive and wired, both key links confirmed, all three DEPL requirements satisfied, no anti-patterns detected.

---

## Verification Details

**Plan verification commands from 70-01-PLAN.md (all pass):**

1. `grep -c "permissions:" .github/workflows/wasm-tests.yml` -- returns **1** (PASS)
2. `grep -c "add_header" deploy/nginx.conf` -- returns **6** (4 security + Cache-Control + Content-Type; PASS, expected >= 6)
3. `grep -c "Strict-Transport-Security" deploy/nginx-ssl.conf.template` -- returns **1** (PASS)
4. `grep -c "server {" deploy/nginx-ssl.conf.template` -- returns **2** (SSL server + redirect server; PASS)
5. `grep -c "location /healthz" deploy/nginx-ssl.conf.template` -- returns **2** (one per server block; PASS)
6. `grep -n '\${' deploy/nginx-ssl.conf.template` -- only `${SSL_CERT_PATH}` and `${SSL_KEY_PATH}` (lines 7-8; PASS, no unsafe substitution variables)

**Commits verified:**
- `5a93ff0` -- feat(70-01): wasm-tests.yml permissions + nginx.conf security headers (3 insertions in 2 files)
- `6f8ac56` -- feat(70-01): SSL template security headers + HSTS + HTTP redirect (24 insertions in 1 file)

---

_Verified: 2026-03-26T21:00:00Z_
_Verifier: Claude (gsd-verifier)_
