# Phase 41: Documentation - Context

**Gathered:** 2026-03-16
**Status:** Ready for planning

<domain>
## Phase Boundary

Rewrite root README.md for a developer evaluating TrustEdge. Move existing architecture/YubiKey content to docs/. Add use case examples with copy-paste commands. The README should let a new user understand what TrustEdge does and run the demo within 5 minutes. This phase does NOT add new features or change code.

</domain>

<decisions>
## Implementation Decisions

### README Structure & Flow
- Lead with problem statement: "Prove that data from an edge device hasn't been tampered with"
- Immediately follow with 3-command quick start: clone → docker-compose up → demo.sh
- Section order: Problem → Quick Start → Use Cases → How It Works (brief) → Architecture (link) → License
- Single self-contained file (DOCS-04)
- Keep badges at top (CI, license, Rust)

### Use Case Framing
- 4 use cases: drone inspection, sensor logs, body cam, audio capture
- Each use case: 2-3 sentence scenario + copy-paste `trst wrap` command with realistic metadata flags
- Commands use `--data-type`, `--source`, `--description` flags from the generic profile (Phase 38)
- Compact and actionable, not narrative (DOCS-05)

### Architecture Detail Depth
- One paragraph summary in README: "TrustEdge is a Rust workspace with 9 crates..."
- Link to `docs/architecture.md` for full crate breakdown, module hierarchy, data flow
- Move existing YubiKey content to `docs/yubikey-guide.md`
- Move existing architecture sections to `docs/architecture.md`
- Nothing is deleted — reorganized from README to docs/ (DOCS-03)

### Tone & Audience
- Primary audience: developer or tech lead evaluating whether TrustEdge fits their use case
- Direct and technical tone — show what it does with code, no marketing fluff
- Like ripgrep or jq README style: problem → install → use → details
- Commercial support section kept but brief (existing content)

### Claude's Discretion
- Exact wording of problem statement and section headings
- How to structure the "How It Works" brief section
- Whether to include a diagram or keep text-only
- Badge selection and ordering
- How to handle the version badge (update to v2.0)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Current README
- `README.md` — Current 465-line README to be rewritten (preserve copyright header and badges)

### Demo artifacts (referenced in quick start)
- `deploy/docker-compose.yml` — docker-compose command for quick start
- `scripts/demo.sh` — Demo script referenced in quick start

### CLI tools (referenced in use cases)
- `crates/trst-cli/src/main.rs` — trst CLI with wrap/verify/keygen/emit-request subcommands

### Requirements
- `.planning/REQUIREMENTS.md` — DOCS-01 through DOCS-05 define success criteria

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `README.md` lines 1-13: Copyright header and badges — preserve and update
- `scripts/demo.sh`: The exact commands the quick start will reference
- `deploy/docker-compose.yml` header comments: Usage instructions that match quick start

### Established Patterns
- MPL-2.0 copyright header on all files
- Existing docs at `scripts/README.md` — docs/ directory doesn't exist yet, needs creation

### Integration Points
- `README.md` — Complete rewrite
- `docs/architecture.md` — New file (content from current README architecture sections)
- `docs/yubikey-guide.md` — New file (content from current README YubiKey sections)

</code_context>

<specifics>
## Specific Ideas

- The preview shown during discussion captures the target style: problem statement → 3-command quick start → brief use case with `trst wrap` command
- Version badge should update from v1.7 to v2.0
- The "How It Works" section should briefly mention: sign at source → encrypt → wrap into .trst archive → verify → receipt

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 41-documentation*
*Context gathered: 2026-03-16*
