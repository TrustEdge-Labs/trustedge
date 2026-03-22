TrustEdge Code & Security Review - Prioritized Improvements

Review Date: 2026-03-22
Scope: 9 crate workspace (core, types, platform, CLI, WASM, protocols)
Overall Risk: Low - No critical vulnerabilities found
🔴 P0 - Critical (Immediate Action Required)

None identified. The codebase has no critical security vulnerabilities requiring immediate fixes.
🟠 P1 - High (Address Before Production)
ID	Issue	Location	Impact
P1-1	Custom Base64 Implementation	crates/core/src/crypto.rs:573-617	Uses custom base64 instead of standard base64 crate; increases attack surface and risk of subtle bugs
P1-2	Excessive unwrap()/expect()	envelope.rs:138, auth.rs:215, CLI code	Panic risk in security-critical paths; could cause DoS or unexpected termination
P1-3	Bidirectional Timestamp Check	crates/core/src/auth.rs:297	abs_diff() allows future-dated responses (up to 5 min), enabling replay attacks with manipulated clock skew
P1-4	Missing File Permissions	crates/trst-cli/src/main.rs (keygen)	Key files written without Unix 0600 permissions; defense-in-depth gap
P1-5	YubiKey PIN Timing Side-Channel	crates/core/src/backends/yubikey.rs:210-230	Timing differences between success/failure PIN verification paths
🟡 P2 - Medium (Important Improvements)
ID	Issue	Location	Impact
P2-1	Hardcoded PBKDF2 Iterations	crates/core/src/crypto.rs:183	600,000 iterations fixed with no API for adjustment or versioning
P2-2	Incomplete YubiKey Features	yubikey.rs:265-290	Key generation and attestation return "not supported" errors; capability mismatch with docs
P2-3	Manual ASN.1 DER Encoding	YubiKey certificate handling	Manual X.509 certificate parsing is fragile; should use x509-cert crate
P2-4	Missing Error Path Tests	Various test files	Limited negative testing for wrong passphrases, malformed metadata, clock skew
P2-5	v2 Envelope Chunk Limits	envelope.rs:258-274	Deterministic nonce construction needs explicit chunk limit enforcement
P2-6	Unused Dependencies	Various Cargo.toml	Unjustified dependencies increase attack surface
🟢 P3 - Low (Nice to Have)
ID	Issue	Location	Impact
P3-1	Documentation Gaps	Various	Missing # Safety docs, TOFU pattern references
P3-2	Feature Flag Warnings	Cargo.toml	insecure-tls flag lacks runtime warnings when enabled
P3-3	Test Coverage Gaps	Various	Network error handling, transport layer edge cases
P3-4	Code Organization	trst-cli/src/main.rs	Large file (900+ lines); could benefit from module splitting
✅ Security Strengths

    Proper Zeroization: Secret<T> wrapper correctly uses ZeroizeOnDrop
    Fail-Closed Design: YubiKey backend fails safely when hardware unavailable
    Well-Vetted Dependencies: AES-GCM, Ed25519, BLAKE3, HKDF from reputable crates
    Algorithm Agility: Supports both Ed25519 and ECDSA P-256
    Domain Separation: HKDF uses proper info strings; signatures use domain prefixes
    Secure Key Storage: TRUSTEDGE-KEY-V1 format uses PBKDF2+AES-GCM
    Bounds Checking: Chunk size limits (128MB) and record limits enforced
    Constant-Time Operations: Delegated to well-vetted crypto crates

🎯 Top 5 Priority Actions

    Replace custom base64 with standard base64 crate (Low effort, security hygiene)
    Fix timestamp check to be unidirectional: timestamp > now + tolerance (Low effort, security fix)
    Audit all unwrap()/expect() in security paths and convert to proper error handling (Medium effort, reliability)
    Add file permission enforcement (0600) for generated key files on Unix (Low effort, defense-in-depth)
    Document PBKDF2 iteration policy and add version field to encrypted key format (Low effort, future-proofing)

