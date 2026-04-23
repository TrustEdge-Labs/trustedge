# Phase 83: Crate & Binary Rename - Pattern Map

**Mapped:** 2026-04-18
**Phase type:** workspace-wide mechanical rename (no new files — rename + sed sweep)
**Files in rename surface:** ~80 source files + 13 Cargo.toml + 4 workflows + 13 scripts + 2 package.json + 5 bin sources
**Analogs found:** n/a — this is a sweep phase, not a construction phase

## Phase Shape

Phase 83 does not "create new files that mirror existing patterns." It **renames existing files and string tokens in lock-step across the workspace**. The pattern map below enumerates every category of file/token touched and gives the planner one concrete excerpt per category, so plan tasks can be written with exact find-and-replace targets and so acceptance_criteria can cite exact `rg` commands that return 0 matches after the sweep.

All paths below are relative to the repo root (`/home/john/vault/projects/github.com/trustedge`). **All `.claude/worktrees/agent-*` paths are out of scope** — that is a scratch worktree that shadows `crates/` and should be ignored.

## Summary of Token Transformations

Mechanical token table the planner hands to the executor:

| Category | From | To |
|----------|------|-----|
| Package (Cargo) | `trustedge-core` | `sealedge-core` |
| Package (Cargo) | `trustedge-types` | `sealedge-types` |
| Package (Cargo) | `trustedge-platform` | `sealedge-platform` |
| Package (Cargo) | `trustedge-platform-server` | `sealedge-platform-server` |
| Package (Cargo) | `trustedge-cli` | `sealedge-cli` |
| Package (Cargo) | `trustedge-wasm` | `sealedge-wasm` |
| Package (Cargo) | `trustedge-trst-protocols` | `sealedge-seal-protocols` |
| Package (Cargo) | `trustedge-trst-cli` | `sealedge-seal-cli` |
| Package (Cargo) | `trustedge-trst-wasm` | `sealedge-seal-wasm` |
| Package (Cargo, experimental) | `trustedge-pubky` | `sealedge-pubky` |
| Package (Cargo, experimental) | `trustedge-pubky-advanced` | `sealedge-pubky-advanced` |
| Package (Cargo, examples) | `trustedge-cam-video-examples` | `sealedge-cam-video-examples` |
| Binary target | `trustedge` | `sealedge` |
| Binary target | `trst` | `seal` |
| Binary target | `trustedge-server` | `sealedge-server` |
| Binary target | `trustedge-client` | `sealedge-client` |
| Binary target | `trustedge-platform-server` | `sealedge-platform-server` |
| Binary target | `trustedge-pubky` | `sealedge-pubky` |
| Library name (Rust) | `trustedge_core` | `sealedge_core` |
| Library name (Rust) | `trustedge_types` | `sealedge_types` |
| Library name (Rust) | `trustedge_platform` | `sealedge_platform` |
| Library name (Rust) | `trustedge_wasm` | `sealedge_wasm` |
| Library name (Rust) | `trustedge_trst_protocols` | `sealedge_seal_protocols` |
| Library name (Rust) | `trustedge_trst_wasm` | `sealedge_seal_wasm` |
| Library name (Rust) | `trustedge_pubky` | `sealedge_pubky` |
| Library name (Rust) | `trustedge_pubky_advanced` | `sealedge_pubky_advanced` |
| Archive extension | `.trst` | `.seal` |
| Crate directory | `crates/trustedge-cli/` | `crates/cli/` |
| Crate directory | `crates/trst-cli/` | `crates/seal-cli/` |
| Crate directory | `crates/trst-protocols/` | `crates/seal-protocols/` |
| Crate directory | `crates/trst-wasm/` | `crates/seal-wasm/` |
| Binary source file | `crates/core/src/bin/trustedge-server.rs` | `crates/core/src/bin/sealedge-server.rs` |
| Binary source file | `crates/core/src/bin/trustedge-client.rs` | `crates/core/src/bin/sealedge-client.rs` |
| Binary source file | `crates/core/src/bin/inspect-trst.rs` | `crates/core/src/bin/inspect-seal.rs` |
| Binary source file | `crates/experimental/pubky/src/bin/trustedge-pubky.rs` | `crates/experimental/pubky/src/bin/sealedge-pubky.rs` |
| npm package name | `trustedge-dashboard` | `sealedge-dashboard` |

Out-of-scope tokens that look similar but belong to later phases:
- `TRUSTEDGE-KEY-V1`, `TRUSTEDGE_ENVELOPE_V1` (wire-format constants) — Phase 84
- `.te-attestation.json` (attestation extension) — Phase 84
- `TRUSTEDGE_*` environment variables — Phase 85
- Copyright headers / `Project: trustedge` / prose in `.md` / CLI help text — Phases 85–86
- GitHub repo URLs (`github.com/TrustEdge-Labs/trustedge`) — Phase 87
- `@trustedge/wasm` npm scope in `crates/{wasm,trst-wasm}/package.json` — **flag for user** (not explicitly claimed by any phase; see Open Questions)
- `trustedge.dev` endpoint URL in `scripts/demo-attestation.sh:30` — Phase 85 (prose/metadata)
- `trustedge` keyword in `package.json` `keywords` / Cargo.toml `keywords` — Phase 85 (metadata)

## File Inventory by Rename Category

### Category 1: Workspace Root `Cargo.toml`

**Glob:** `Cargo.toml` (repo root, single file)
**Scope:** workspace `members` list, `[workspace.dependencies]` internal crates, `[workspace.metadata]` (keywords/categories prose stays — Phase 85)

**Current excerpt** (`Cargo.toml:9-21`):
```toml
[workspace]
members = [
    "crates/core",
    "crates/types",
    "crates/platform",
    "crates/platform-server",
    "crates/trustedge-cli",
    "crates/wasm",
    "crates/trst-protocols",
    "crates/trst-cli",
    "crates/trst-wasm",
    "examples/cam.video",
]
```

**Current excerpt** (`Cargo.toml:55-56`):
```toml
# Internal workspace crates
trustedge-types = { path = "crates/types" }
```

**Expected transformation:**
- Rename directories: `crates/trustedge-cli` → `crates/cli`, `crates/trst-protocols` → `crates/seal-protocols`, `crates/trst-cli` → `crates/seal-cli`, `crates/trst-wasm` → `crates/seal-wasm`.
- Update `members` array to the new dirs.
- Update `[workspace.dependencies]` entry `trustedge-types` → `sealedge-types` with new path `crates/types` (unchanged) or adjusted path.
- **Out of scope** (Phase 85): the `#` header comment `Project: trustedge — Privacy and trust at the edge.`, the `documentation = "https://docs.rs/trustedge-core"` metadata URL, and all keywords.

### Category 2: Per-Crate `Cargo.toml` — Package Name Field

**Glob:** `crates/*/Cargo.toml`, `crates/experimental/*/Cargo.toml`, `examples/cam.video/Cargo.toml`
**File count:** 12 (9 root workspace + 2 experimental + 1 examples)

**Current excerpt** (`crates/core/Cargo.toml:8-9`):
```toml
[package]
name = "trustedge-core"
```

**Current excerpt** (`crates/trst-cli/Cargo.toml:6-7`):
```toml
[package]
name = "trustedge-trst-cli"
```

**Expected transformation:** Mechanical substitution using the Package table above. Every `name = "trustedge-*"` becomes its `sealedge-*` counterpart.

### Category 3: Per-Crate `Cargo.toml` — `[lib]` Name Field

**Glob:** `crates/*/Cargo.toml`
**Scope:** Only crates that declare an explicit `[lib] name = "..."` (others default from package name via hyphen→underscore).

**Current excerpt** (`crates/core/Cargo.toml:19-21`):
```toml
[lib]
name = "trustedge_core"
path = "src/lib.rs"
```

**Current excerpt** (`crates/types/Cargo.toml:19-21`):
```toml
[lib]
name = "trustedge_types"
path = "src/lib.rs"
```

Also: `crates/platform/Cargo.toml` (`trustedge_platform`), `crates/wasm/Cargo.toml` (`trustedge_wasm`).

**Expected transformation:** Replace `trustedge_<name>` with `sealedge_<name>` using the Library name row in the token table.

### Category 4: Per-Crate `Cargo.toml` — `[[bin]]` Blocks

**Glob:** `crates/*/Cargo.toml`
**Scope:** Crates with explicit `[[bin]]` targets and their `name = ...` + `path = ...` fields.

**Current excerpt** (`crates/core/Cargo.toml:23-33`):
```toml
[[bin]]
name = "trustedge-server"
path = "src/bin/trustedge-server.rs"

[[bin]]
name = "trustedge-client"
path = "src/bin/trustedge-client.rs"

[[bin]]
name = "software-hsm-demo"
path = "src/bin/software-hsm-demo.rs"
```

**Current excerpt** (`crates/trustedge-cli/Cargo.toml:19-21`):
```toml
[[bin]]
name = "trustedge"
path = "src/main.rs"
```

**Current excerpt** (`crates/trst-cli/Cargo.toml:14-17`):
```toml
[[bin]]
name = "trst"
path = "src/main.rs"
test = false
```

**Current excerpt** (`crates/platform-server/Cargo.toml:20-22`):
```toml
[[bin]]
name = "trustedge-platform-server"
path = "src/main.rs"
```

**Current excerpt** (`crates/experimental/pubky/Cargo.toml:17-19`):
```toml
[[bin]]
name = "trustedge-pubky"
path = "src/bin/trustedge-pubky.rs"
```

**Expected transformation:**
- `name = "trustedge"` → `name = "sealedge"`
- `name = "trst"` → `name = "seal"`
- `name = "trustedge-server"` → `name = "sealedge-server"` + rename file `src/bin/trustedge-server.rs` → `src/bin/sealedge-server.rs` + update `path`
- `name = "trustedge-client"` → `name = "sealedge-client"` + rename file + update `path`
- `name = "trustedge-platform-server"` → `name = "sealedge-platform-server"`
- `name = "trustedge-pubky"` → `name = "sealedge-pubky"` + rename file + update `path`
- `software-hsm-demo` **stays** (no "trustedge" token in it — skip).

### Category 5: Per-Crate `Cargo.toml` — Inter-Crate `[dependencies]` Entries

**Glob:** `crates/*/Cargo.toml`, `crates/experimental/*/Cargo.toml`, `examples/cam.video/Cargo.toml`

**Representative excerpts:**

`crates/core/Cargo.toml:66-67`:
```toml
trustedge-trst-protocols = { path = "../trst-protocols" }
trustedge-types = { workspace = true }
```

`crates/platform/Cargo.toml:26-27`:
```toml
trustedge-types = { workspace = true }
trustedge-core = { path = "../core" }
```

`crates/platform-server/Cargo.toml:25-26`:
```toml
trustedge-platform = { path = "../platform", features = ["http", "openapi"] }
trustedge-core = { path = "../core" }
```

`crates/trustedge-cli/Cargo.toml:28`:
```toml
trustedge-core = { path = "../core" }
```

`crates/trst-cli/Cargo.toml:37-38`:
```toml
trustedge-core = { path = "../core" }
trustedge-types = { workspace = true }
```

`crates/trst-wasm/Cargo.toml:26`:
```toml
trustedge-trst-protocols = { path = "../trst-protocols" }
```

`examples/cam.video/Cargo.toml:22`:
```toml
trustedge-core = { path = "../../crates/core" }
```

`crates/experimental/pubky/Cargo.toml:27`:
```toml
trustedge-core = { path = "../../core" }
```

`crates/experimental/pubky-advanced/Cargo.toml:23`:
```toml
trustedge-core = { path = "../../core" }
```

**Feature-flag references** (same file type, different syntax — `[features]` section lines cross-reference the package name):

`crates/core/Cargo.toml:95`:
```toml
yubikey = ["dep:yubikey", "x509-cert", "der", "spki", "signature", "rcgen"]
```
`crates/platform/Cargo.toml:72`:
```toml
yubikey = ["ca", "trustedge-core/yubikey"]
```
`crates/platform-server/Cargo.toml:42-43`:
```toml
postgres = ["trustedge-platform/postgres"]
ca = ["trustedge-platform/ca"]
```
`crates/trustedge-cli/Cargo.toml:46-47`:
```toml
audio = ["trustedge-core/audio"]
keyring = ["trustedge-core/keyring"]
```
`crates/trst-cli/Cargo.toml:24`:
```toml
yubikey = ["trustedge-core/yubikey"]
```

**Expected transformation:** Every key and every feature string referencing a `trustedge-*` package gets substituted to `sealedge-*`. Path values (`../core`, `../../core`) stay unchanged **unless** the directory was renamed (i.e., `../trst-protocols` → `../seal-protocols` after dir rename).

**Path-dependency directory rename also changes paths:**
- `crates/core/Cargo.toml:66`: `path = "../trst-protocols"` → `path = "../seal-protocols"`
- `crates/trst-wasm/Cargo.toml:26`: `path = "../trst-protocols"` → `path = "../seal-protocols"`

### Category 6: Experimental Workspace Root `Cargo.toml`

**File:** `crates/experimental/Cargo.toml` (single file, separate workspace root)

**Current excerpt** (`crates/experimental/Cargo.toml:9-15`):
```toml
[workspace]
members = [
    "pubky",
    "pubky-advanced",
]
resolver = "2"
```

**Expected transformation:** None structurally — member names `pubky` / `pubky-advanced` are directory names, not package names. They stay. The two member Cargo.toml files inside already covered by Categories 2 & 5 above.

### Category 7: Experimental Crate `[[bin]]` + Source File

**File:** `crates/experimental/pubky/src/bin/trustedge-pubky.rs`

**Current excerpt** (filename itself is the pattern):
```
crates/experimental/pubky/src/bin/trustedge-pubky.rs
```

**Expected transformation:** Rename to `crates/experimental/pubky/src/bin/sealedge-pubky.rs`, update `[[bin]] name` and `path` in `crates/experimental/pubky/Cargo.toml`.

### Category 8: Binary Source Files (`src/bin/*.rs` whose filename matches the binary target)

**Glob:** `crates/*/src/bin/*.rs` + `crates/experimental/*/src/bin/*.rs`
**Matched files:**
```
crates/core/src/bin/inspect-trst.rs
crates/core/src/bin/software-hsm-demo.rs       (NO rename — doesn't match token)
crates/core/src/bin/trustedge-client.rs
crates/core/src/bin/trustedge-server.rs
crates/experimental/pubky/src/bin/trustedge-pubky.rs
```

**Expected transformation:**
- `inspect-trst.rs` → `inspect-seal.rs`
- `trustedge-client.rs` → `sealedge-client.rs`
- `trustedge-server.rs` → `sealedge-server.rs`
- `trustedge-pubky.rs` → `sealedge-pubky.rs`
- `software-hsm-demo.rs` stays

Each file rename must be paired with the matching `path = "..."` update in its crate's Cargo.toml `[[bin]]` block (Category 4).

### Category 9: `use trustedge_*::*` Statements in `.rs` Sources

**Grep:** `use trustedge_` (and bare path references `trustedge_core::`, `trustedge_types::`, etc.)
**File count:** **64 source files** in real tree (excluding `.claude/worktrees/`).
**Token count:** ~264 `trustedge_*` occurrences across those files.

**Representative excerpts** (from `crates/trst-cli/tests/acceptance.rs:23`, `crates/core/src/bin/inspect-trst.rs`, `crates/platform-server/src/main.rs`):

`crates/trustedge-cli/src/main.rs:24`:
```rust
use trustedge_core::format;
```

`crates/trst-cli/tests/acceptance.rs:23`:
```rust
use trustedge_core::TrstManifest;
```

`crates/experimental/pubky-advanced/src/envelope.rs:17`:
```rust
use trustedge_core::{format::AeadAlgorithm, NetworkChunk, NONCE_LEN};
```

`crates/experimental/pubky/src/lib.rs:18`:
```rust
use trustedge_core::backends::{
    ...
};
use trustedge_core::error::BackendError;
use trustedge_core::{PrivateKey, PublicKey};
```

`crates/platform-server/src/main.rs:19-23`:
```rust
use trustedge_platform::http::{create_router, AppState, Config};
use trustedge_platform::verify::jwks::KeyManager;
use trustedge_platform::database::{create_connection_pool, run_migrations};
```

`crates/experimental/pubky-advanced/examples/hybrid_encryption_demo.rs:12`:
```rust
use trustedge_pubky_advanced::{DualKeyPair, EnvelopeV2};
```

`crates/experimental/pubky/examples/your_exact_api.rs:19-20`:
```rust
use trustedge_pubky::mock::MockPubkyBackend;
use trustedge_pubky::receive_trusted_data;
```

`crates/core/src/lib.rs:31`:
```rust
//! use trustedge_core::Envelope;
```
(doctest — **in scope**, the `use` itself is executable Rust. Surrounding `//!` prose stays for Phase 86.)

**Expected transformation:** Every identifier token `trustedge_core` / `trustedge_types` / `trustedge_platform` / `trustedge_wasm` / `trustedge_pubky` / `trustedge_pubky_advanced` / `trustedge_trst_protocols` / `trustedge_trst_wasm` substituted to its `sealedge_*` / `sealedge_seal_*` counterpart per the Library name row in the token table.

**Caveat:** also catches `trustedge_wasm_bg.wasm`, `trustedge_trst_wasm_bg.wasm` filenames in JS bindings (Category 13) and `trustedge_wasm.js` imports. Those are Category 13, treated separately.

### Category 10: `.trst` Archive Extension Literals in `.rs` Sources

**Grep:** `\.trst\b` (word boundary — avoids matching `trustedge`)
**File count:** 23 `.rs` files, ~103 occurrences.

**Representative excerpts:**

`crates/core/src/archive.rs:214`:
```rust
    format!("clip-{}.trst", id)
```

`crates/core/src/archive.rs:424-425` (assertions encode the literal extension):
```rust
        assert_eq!(archive_dir_name("test123"), "clip-test123.trst");
        assert_eq!(archive_dir_name("CAM-001"), "clip-CAM-001.trst");
```

`crates/trst-cli/src/main.rs:518-519` (validation message):
```rust
    if !archive_name.ends_with(".trst") {
        anyhow::bail!("Output directory must end with .trst");
    }
```

`crates/core/src/archive.rs:274` (test path literal):
```rust
        let archive_path = temp_dir.path().join("test.trst");
```

`crates/core/src/io/mod.rs:19` (doc comment — **prose**, defer to Phase 86):
```rust
//! - Archive read/write operations (.trst format)
```

`crates/trst-cli/src/main.rs:99` (CLI help text — **user-facing**, defer to Phase 85):
```rust
#[command(author, version, about = "TrustEdge .trst archival tool", long_about = None)]
```

**Expected transformation:** All `.trst` string literals in executable code (format strings, path joins, `ends_with`, assertions, test fixtures) substituted to `.seal`. **Boundary call:**
- Executable literal (`format!(".trst")`, `.ends_with(".trst")`, path construction, test expectations) → in scope.
- Doc comment (`//!`, `///`) referencing `.trst format` → **Phase 86**.
- CLI help string passed to `#[command(about = "... .trst ...")]` → user-visible text → **Phase 85**.
- `anyhow::bail!("Output directory must end with .trst")` → error message text → arguably user-facing, but it is a **runtime invariant string** tied to the extension — rename in lock-step with the extension literal (this agent recommends in-scope for Phase 83 — flag for planner review).

**File list with extension literals (real tree only):**
```
crates/core/src/archive.rs                       (11 hits)
crates/core/src/hybrid.rs                        (2 hits — both in doc comments; defer)
crates/core/src/io/mod.rs                        (5 hits — doc comments; defer)
crates/core/src/vectors.rs                       (5 hits)
crates/core/src/bin/inspect-trst.rs              (3 hits)
crates/core/examples/attest.rs                   (1 hit)
crates/core/examples/verify_attestation.rs       (1 hit)
crates/trst-cli/src/main.rs                      (8 hits — mix of CLI text + validation + code)
crates/trst-cli/tests/acceptance.rs              (14 hits)
crates/trst-cli/tests/integration_tests.rs       (18 hits)
crates/trst-cli/tests/security_*.rs              (10 hits across 4 files)
crates/trst-protocols/src/archive/manifest.rs    (6 hits)
crates/trst-protocols/src/archive/mod.rs         (1 hit)
crates/trst-protocols/src/archive/chunks.rs      (1 hit)
crates/trst-protocols/src/archive/signatures.rs  (1 hit)
crates/trst-wasm/src/lib.rs                      (3 hits)
crates/trst-wasm/tests/archive_verification_tests.rs (1 hit)
crates/trustedge-cli/src/main.rs                 (2 hits)
crates/experimental/pubky/src/bin/trustedge-pubky.rs (5 hits)
crates/experimental/pubky/tests/integration_tests.rs (5 hits)
```

### Category 11: Dashboard `package.json`

**File:** `web/dashboard/package.json` (single file)

**Current excerpt** (`web/dashboard/package.json:2`):
```json
{
  "name": "trustedge-dashboard",
  ...
}
```

**Expected transformation:** `"name": "trustedge-dashboard"` → `"name": "sealedge-dashboard"`. **Only the `name` field.** CONTEXT.md explicitly scopes: directory stays, no import-path churn, no CI workflow rewrites.

### Category 12: GitHub Actions Workflow Files

**Glob:** `.github/workflows/*.yml`
**File count:** 4 (`ci.yml`, `cla.yml`, `semver.yml`, `wasm-tests.yml`)
**Total `trustedge` occurrences:** 35

**Representative excerpts:**

`.github/workflows/ci.yml:117-125` (clippy matrix — every crate by package name):
```yaml
          cargo clippy -p trustedge-core --all-targets --all-features -- -D warnings
          cargo clippy -p trustedge-platform --all-targets --features "http,ca,openapi,yubikey" -- -D warnings
          cargo clippy -p trustedge-platform-server --all-targets -- -D warnings
          cargo clippy -p trustedge-cli --all-targets --all-features -- -D warnings
          cargo clippy -p trustedge-trst-cli --all-targets -- -D warnings
          cargo clippy -p trustedge-trst-protocols --all-targets -- -D warnings
          cargo clippy -p trustedge-types --all-targets -- -D warnings
          cargo clippy -p trustedge-wasm --all-targets -- -D warnings
          cargo clippy -p trustedge-trst-wasm --all-targets -- -D warnings
```

`.github/workflows/ci.yml:136-151` (test + build matrix):
```yaml
        run: cargo test -p trustedge-core --features "audio,git-attestation,keyring,insecure-tls" --locked
        ...
        run: cargo test -p trustedge-core --features yubikey --lib --locked
        ...
          cargo test -p trustedge-platform --lib --locked
          cargo test -p trustedge-platform --test verify_integration --locked
          cargo test -p trustedge-platform --test verify_integration --features http --locked
        ...
          cargo check -p trustedge-wasm --target wasm32-unknown-unknown
          cargo check -p trustedge-trst-wasm --target wasm32-unknown-unknown
```

`.github/workflows/ci.yml:188`:
```yaml
        run: cargo build -p trustedge-trst-cli --release --locked
```

`.github/workflows/semver.yml:34-35`:
```yaml
      - name: API compatibility (trustedge-core)
        run: cargo semver-checks --package trustedge-core --baseline-rev HEAD~1
```

`.github/workflows/wasm-tests.yml:53`:
```yaml
        wasm_size=$(wc -c < pkg/trustedge_wasm_bg.wasm)
```
`.github/workflows/wasm-tests.yml:67`:
```yaml
        wasm_size=$(wc -c < pkg/trustedge_trst_wasm_bg.wasm)
```

**Expected transformation:**
- Every `-p trustedge-*` flag → `-p sealedge-*` / `-p sealedge-seal-*` per token table.
- `--package trustedge-core` → `--package sealedge-core`.
- `pkg/trustedge_wasm_bg.wasm` → `pkg/sealedge_wasm_bg.wasm` (this is the wasm-pack artifact filename — auto-derived from the Cargo `[lib] name`, which we are renaming).
- `pkg/trustedge_trst_wasm_bg.wasm` → `pkg/sealedge_seal_wasm_bg.wasm`.
- `cla.yml` references to `https://github.com/TrustEdge-Labs/trustedge/blob/main/CLA.md` → **Phase 87** (repo rename), not Phase 83.
- Prose echo strings (e.g., `echo "trustedge-wasm size: $wasm_size bytes"`, `echo "✅ trustedge-wasm size is acceptable"`) → **Phase 85** (user-facing log text). The variable name used in the grep (`wasm_size`) doesn't carry the token.

### Category 13: Scripts (`scripts/*.sh`)

**Glob:** `scripts/*.sh` + `scripts/project/*.sh`
**File count:** 13 files with matches, 77 total occurrences.

**In-scope patterns (binary invocations, crate flags, extension literals in paths):**

`scripts/demo.sh:45`:
```sh
TRST="cargo run -q -p trustedge-trst-cli --"
```

`scripts/demo.sh:118-163` (representative — all `.trst` paths in live code):
```sh
# ── Step 3: Wrap data into .trst archive ──────────────────────────────────────
step_banner "Wrap data into .trst archive"
...
        --out "$DEMO_DIR/sample.trst" \
...
    pass "Created $DEMO_DIR/sample.trst archive"
...
if [ -n "${DEVICE_PUB:-}" ] && [ -f "$DEMO_DIR/sample.trst/manifest.json" ]; then
    if $TRST verify "$DEMO_DIR/sample.trst" --device-pub "$DEVICE_PUB" 2>&1; then
...
    TRST_YUBIKEY="cargo run -q -p trustedge-trst-cli --features yubikey --"
...
            --out "$DEMO_DIR/sample-yubikey.trst" \
```

`scripts/demo-attestation.sh:83, 131`:
```sh
TRST="cargo run -q -p trustedge-trst-cli --"
...
if cargo build -p trustedge-trst-cli --release 2>&1; then
    TRST_BINARY="target/release/trst"
```

`scripts/ci-check.sh:113-221` (parallels `.github/workflows/ci.yml`):
```sh
    if cargo clippy -p trustedge-core --all-targets --all-features -- -D warnings && \
       cargo clippy -p trustedge-platform --all-targets --features "http,ca,openapi,yubikey" -- -D warnings && \
    ...
    if cargo test -p trustedge-core --features "$CORE_FEATURES" --locked; then
    ...
       cargo check -p trustedge-trst-wasm --target wasm32-unknown-unknown; then
```

**Expected transformation:**
- Every `cargo ... -p trustedge-*` → `-p sealedge-*` / `-p sealedge-seal-*`.
- Every file path / extension literal `.trst` in variable assignments, `--out`, `--archive`, `-f` tests → `.seal`.
- Shell variable names like `TRST=...`, `TRST_BINARY=...`, `TRST_YUBIKEY=...` → `SEAL=...` / `SEAL_BINARY=...` / `SEAL_YUBIKEY=...` (cosmetic — but keeps scripts readable after the rename; planner can choose to include).
- `target/release/trst` (built binary path) → `target/release/seal` (matches new `[[bin]] name = "seal"`).
- **Out of scope** (Phase 85): every `# Project: trustedge — Privacy and trust at the edge.` header comment; echo/log prose like `step_banner "Wrap data into .trst archive"` argument (it is runtime prose) and `pass "Created $DEMO_DIR/sample.trst archive"` argument — though the **path literal inside** must rename in Phase 83 because the file on disk genuinely has the new extension. Planner: treat the path substring as in-scope, leave the surrounding prose untouched where possible, or defer whole-line to Phase 85 and flag for user.

### Category 14: WASM JavaScript Bindings (`crates/{wasm,trst-wasm}/js/*.js`)

**Glob:** `crates/wasm/js/*.js`, `crates/trst-wasm/js/*.js` (hand-written bindings — not `pkg-bundler/` artifacts which regenerate).

**Current excerpts:**

`crates/wasm/js/trustedge.js:24, 238`:
```js
} from '../pkg/trustedge_wasm.js';
...
export const trustedge = new TrustEdge();
```

`crates/wasm/js/trustedge-node.js:24, 50`:
```js
} from '../pkg-bundler/trustedge_wasm.js';
...
            const wasmPath = join(__dirname, '../pkg-bundler/trustedge_wasm_bg.wasm');
```

`crates/trst-wasm/js/trustedge.js:24, 238`:
```js
} from '../pkg/trustedge_wasm.js';       // ← NOTE: trst-wasm's JS also imports from 'trustedge_wasm.js' — might be a bug or a copy
...
export const trustedge = new TrustEdge();
```

**Expected transformation:**
- `trustedge_wasm.js` / `trustedge_wasm_bg.wasm` import paths → `sealedge_wasm.js` / `sealedge_wasm_bg.wasm` in `crates/wasm/js/`.
- For `crates/trst-wasm/js/*.js`, paths should become `sealedge_seal_wasm.js` / `sealedge_seal_wasm_bg.wasm` (matches renamed lib name for `trst-wasm` crate).
- `export const trustedge = new TrustEdge()` — this is an exported JS identifier chosen by the crate author. Rename to `sealedge` for consistency with the new Cargo lib name — **flag for user confirmation** (planner decision: in-scope because it's the public JS API surface mirroring the wasm lib name).
- **Also rename the JS source filenames themselves**: `crates/wasm/js/trustedge.js` → `crates/wasm/js/sealedge.js`, `trustedge-node.js` → `sealedge-node.js`, and mirror for `trst-wasm` → `seal-wasm`. The filenames are referenced from `package.json` `main`/`browser`/`types` fields (see Category 15).

### Category 15: WASM Crate `package.json` Files (non-dashboard)

**Files:** `crates/wasm/package.json`, `crates/trst-wasm/package.json`

**Current excerpt** (`crates/wasm/package.json:1-7`):
```json
{
  "name": "@trustedge/wasm",
  "version": "0.1.0",
  "description": "WebAssembly bindings for TrustEdge cryptographic operations",
  "main": "js/trustedge-node.js",
  "browser": "js/trustedge.js",
  "types": "js/trustedge.d.ts",
```

**Expected transformation — FLAG FOR USER:** These are npm package definitions for the wasm crate bindings. They carry:
- `"name": "@trustedge/wasm"` — npm scoped package name (npm *scope* `@trustedge` is a brand handle, like the GitHub org).
- Filenames (`trustedge-node.js`, `trustedge.js`, `trustedge.d.ts`, plus `pkg/trustedge_wasm.js`) referenced from `main`/`browser`/`types`/`exports`.
- `"url": "https://github.com/trustedge-labs/trustedge.git"` (repo URL → Phase 87).
- `"directory": "trustedge-wasm"` (internal reference — **broken already**, the crate dir is `crates/wasm/` not `trustedge-wasm/`).
- keywords, description, homepage — all prose/metadata → Phase 85.

**CONTEXT.md** explicitly only scopes `web/dashboard/package.json` for Phase 83. These two files are **not mentioned anywhere** in CONTEXT.md. They are, however, the npm-package manifestations of the two wasm crates being renamed. The planner should:
1. Rename the `main`/`browser`/`types`/`exports` path fields to reference the renamed JS files (Category 14) — **required for internal consistency** of the crate rename.
2. Flag to the user whether the `@trustedge/wasm` scoped name should also rename (→ `@sealedge/wasm` / `@sealedge/seal-wasm`) in Phase 83 vs. Phase 85 (metadata) vs. Phase 87 (npm org). Recommend: rename alongside the crate since it is the *published identity of the crate*.

### Category 16: Example Binaries (`examples/cam.video/`)

**Files:** `examples/cam.video/{record_and_wrap.rs, verify_cli.rs}`, `examples/cam.video/Cargo.toml`

`examples/cam.video/Cargo.toml:6-7`:
```toml
[package]
name = "trustedge-cam-video-examples"
```

`examples/cam.video/Cargo.toml:13-19`:
```toml
[[bin]]
name = "record_and_wrap"
path = "record_and_wrap.rs"

[[bin]]
name = "verify_cli"
path = "verify_cli.rs"
```

`examples/cam.video/verify_cli.rs:1` (uses `trustedge_core::`).
`examples/cam.video/record_and_wrap.rs:1` (uses `trustedge_core::`).

**Expected transformation:** Package name → `sealedge-cam-video-examples`. Binary names `record_and_wrap` / `verify_cli` are descriptive and **don't contain the token** — leave. Source-code `use trustedge_core` → `use sealedge_core` (covered by Category 9).

## Shared Patterns / Cross-Cutting Concerns

### Hyphen ↔ Underscore Rule

Rust auto-converts package names to lib names by replacing `-` with `_`. This means every package rename drives two downstream renames in lock-step:
- Cargo.toml: `name = "trustedge-core"` → `name = "sealedge-core"` (hyphenated)
- Source code: `use trustedge_core::...` → `use sealedge_core::...` (underscored)
- WASM artifacts: `trustedge_core.wasm` / `trustedge_core_bg.wasm` (underscored — matches lib name)

**The planner must ensure** every package-level rename has a matching source-code sweep.

### Ordering Constraints Within the Phase

From CONTEXT.md §"Claude's Discretion": single workspace must compile at every commit boundary. Recommended atomic commits:
1. Root-workspace crates: `core`, `types`, `platform`, `platform-server`, `cli`, `wasm` — package + lib + inter-crate deps + `use` statements in lock-step.
2. Trst-family crates: `trst-protocols`, `trst-cli`, `trst-wasm` → `seal-protocols`, `seal-cli`, `seal-wasm` — package + lib + dir rename + binary `trst` → `seal` + all `use trustedge_trst_*` → `use sealedge_seal_*`.
3. `.trst` extension → `.seal` extension across all source literals + script paths + CI workflow test fixtures.
4. Dashboard `package.json` name field.
5. Experimental workspace: `pubky`, `pubky-advanced`.

Each commit must pass `cargo check --workspace` at HEAD.

### Test Coverage of the Rename

The sanity-check pattern from CONTEXT.md §"Specific Ideas": `seal wrap input.bin → input.seal`. After the rename, this exact command must be executable. A test — likely an integration test in `crates/seal-cli/tests/acceptance.rs` — should assert the archive directory `ends_with(".seal")` and should no longer accept `.trst` inputs (D-02 clean break).

## Verification Greps (Post-Rename)

These ripgrep commands **must return 0 results** after Phase 83 is complete (excluding `.planning/`, `target/`, `.claude/worktrees/`, and `.md` files which are Phase 85/86):

```bash
# 1. No trustedge-* package names in any Cargo.toml
rg -n 'trustedge-(core|types|platform|platform-server|cli|wasm|trst-protocols|trst-cli|trst-wasm|pubky|pubky-advanced|cam-video-examples|dashboard)' \
   --glob 'Cargo.toml' --glob '!target/**' --glob '!.claude/**'

# 2. No trustedge_* library identifiers in any .rs
rg -n 'trustedge_(core|types|platform|wasm|trst_protocols|trst_wasm|pubky|pubky_advanced)' \
   --glob '*.rs' --glob '!target/**' --glob '!.claude/**'

# 3. No "trustedge" or "trst" binary invocations in shell scripts (as -p flags)
rg -n -- '-p\s+trustedge-' scripts/ .github/workflows/

# 4. No [[bin]] blocks with legacy binary names
rg -n 'name\s*=\s*"(trustedge|trst|trustedge-server|trustedge-client|trustedge-platform-server|trustedge-pubky)"' \
   --glob 'Cargo.toml' --glob '!target/**' --glob '!.claude/**'

# 5. No .trst extension literals in Rust executable code
#    (this will also hit doc comments and help text — Phase 85/86. Planner should craft a tighter regex:
#     exclude lines starting with `//` or `//!` or inside `#[command(... about = "...")]`.)
rg -n '\.trst\b' --glob '*.rs' --glob '!target/**' --glob '!.claude/**'

# 6. No .trst in live shell scripts (paths, not comments)
rg -n '\.trst\b' scripts/ .github/workflows/ --glob '!*.md'

# 7. No trustedge_wasm or trustedge_trst_wasm artifact references
rg -n 'trustedge_(wasm|trst_wasm)' --glob '*.js' --glob '*.yml' --glob '!target/**' --glob '!.claude/**'

# 8. Dashboard package.json renamed
rg -n '"name":\s*"trustedge-dashboard"' web/dashboard/package.json

# 9. Binary source files renamed (these should now NOT exist)
ls crates/core/src/bin/trustedge-server.rs 2>/dev/null && echo "FAIL"
ls crates/core/src/bin/trustedge-client.rs 2>/dev/null && echo "FAIL"
ls crates/core/src/bin/inspect-trst.rs 2>/dev/null && echo "FAIL"
ls crates/experimental/pubky/src/bin/trustedge-pubky.rs 2>/dev/null && echo "FAIL"

# 10. Crate directories renamed (these should now NOT exist)
ls -d crates/trustedge-cli crates/trst-cli crates/trst-protocols crates/trst-wasm 2>/dev/null && echo "FAIL"

# 11. Workspace + experimental workspace both still green
cargo check --workspace --locked
(cd crates/experimental && cargo check --workspace --locked)
```

And these commands **must return new tokens**, proving the rename landed:

```bash
# 12. sealedge-* package names present
rg -n 'name = "sealedge-' --glob 'Cargo.toml'    # should match every crate

# 13. sealedge_* / sealedge_seal_* library identifiers present
rg -c 'sealedge_(core|types|platform|wasm|seal_protocols|seal_wasm|pubky)' \
   --glob '*.rs' --glob '!target/**' | head

# 14. .seal extension literals present in archive.rs
rg -n '\.seal' crates/core/src/archive.rs

# 15. seal binary target declared
rg -n 'name = "seal"' crates/seal-cli/Cargo.toml

# 16. sealedge binary target declared
rg -n 'name = "sealedge"' crates/cli/Cargo.toml
```

## Open Questions for Planner / User

1. **Error message strings** like `anyhow::bail!("Output directory must end with .trst")` (in `crates/trst-cli/src/main.rs:519`): rename in Phase 83 (lockstep with file-extension semantics) or defer to Phase 85 (user-facing text policy)? Recommend: Phase 83 — the extension *is* the error.

2. **`@trustedge/wasm` scoped npm package names** in `crates/wasm/package.json` and `crates/trst-wasm/package.json`: Phase 83 or Phase 85? CONTEXT.md does not mention them. Recommend: Phase 83 (they are the npm identity of the crate being renamed; filenames referenced from these files must rename together).

3. **`export const trustedge = new TrustEdge()`** in `crates/{wasm,trst-wasm}/js/trustedge.js:238`: rename the JS identifier `trustedge` to `sealedge`? This is public JS API surface. Recommend: Phase 83.

4. **REBRAND-04 amendment** (flagged in CONTEXT.md §D-02): must happen before Phase 83 executes, or the `.trst` → `.seal` rename is technically scope-creep against REBRAND-04 as currently written.

5. **CLI help text** in `#[command(about = "TrustEdge .trst archival tool")]`: does the extension literal in help text rename in Phase 83 (matches file system truth) or Phase 85 (user-facing prose)? Recommend: Phase 85 for the `TrustEdge` brand word, Phase 83 for the `.trst` → `.seal` extension substring.

## Metadata

**Rename surface footprint (real tree, excluding `.claude/worktrees/` and `target/`):**
- 13 `Cargo.toml` files (1 root workspace + 9 root crates + 1 exp workspace + 2 exp crates + 1 example)
- 4 `.github/workflows/*.yml` files
- 13 `scripts/*.sh` files
- 4 wasm `js/*.js` files (2 per wasm crate × 2 crates)
- 2 wasm `package.json` files
- 1 dashboard `package.json` file
- 5 `src/bin/*.rs` files (including `inspect-trst.rs` and `trustedge-pubky.rs`)
- ~64 `.rs` source files with `use trustedge_*` identifiers
- 23 `.rs` source files with `.trst` extension literals

**Pattern extraction date:** 2026-04-18
**Analog search scope:** entire real tree under `/home/john/vault/projects/github.com/trustedge`, excluding `target/` and `.claude/worktrees/agent-*`.
**Files scanned (Grep):** 100+ representative files spanning every category above.
