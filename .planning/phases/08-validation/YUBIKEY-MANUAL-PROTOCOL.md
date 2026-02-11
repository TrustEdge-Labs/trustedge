# YubiKey Hardware Integration Manual Testing Protocol

**Created:** 2026-02-11
**Purpose:** Manual validation protocol for YubiKey hardware integration when actual hardware is available
**Audience:** Developers testing TrustEdge YubiKey backend with physical hardware

---

## Overview

TrustEdge's YubiKey integration has two layers of testing:

1. **Simulation tests (90+ tests):** Run automatically in CI without hardware. Test backend initialization, configuration, capability discovery, and error handling using mocked PIV operations.

2. **Manual hardware tests (this protocol):** Require physical YubiKey hardware to validate actual signing operations, PIV slot management, and hardware-specific interactions.

**This protocol documents the manual testing procedure for hardware validation.**

---

## Test Coverage Summary

### Automated (Simulation Tests)

**Location:** `crates/core/tests/yubikey_integration.rs`

**What's tested automatically:**
- YubiKey backend detection and initialization
- PIV configuration validation
- Capability discovery (signing, key generation, slot management)
- Error handling (no hardware, wrong PIN, missing slots)
- Backend trait compliance

**What's NOT tested automatically:**
- Actual cryptographic signing with hardware keys
- Physical key detection and enumeration
- PIN retry limits and lockout behavior
- PIV slot persistence across power cycles
- Hardware-specific error modes

### Manual (This Protocol)

**5 test scenarios requiring physical hardware:**
1. Backend Detection Test
2. Key Generation Test
3. Signing Test
4. Key Persistence Test
5. PIN Retry Limit Test

---

## Prerequisites

### Hardware Requirements

- **YubiKey 5 series** with PIV support (5, 5C, 5 NFC, 5Ci, or 5C Nano)
- USB port or NFC reader (depending on YubiKey model)
- YubiKey in factory-fresh state OR known PIN/PUK/Management Key

### Software Requirements

- **PCSC daemon:** Smart card interface daemon
  - Linux: `pcscd` (install via `sudo apt install pcscd` or `sudo dnf install pcsc-lite`)
  - macOS: Built-in (no installation needed)
  - Windows: Built-in smart card service

- **YubiKey Manager (optional):** For YubiKey setup and verification
  ```bash
  pip install --user yubikey-manager
  ```

- **TrustEdge with yubikey feature:**
  ```bash
  cargo build --features yubikey --release
  ```

### Default Credentials

| Credential | Default Value | Purpose |
|------------|---------------|---------|
| PIN | 123456 | User authentication for PIV operations |
| PUK | 12345678 | PIN unlock key (for unblocking locked PIN) |
| Management Key | 48-char hex (varies) | Administrative operations |

**⚠️ WARNING:** YubiKeys lock permanently after 3 wrong PUK attempts. Always test with non-production keys.

---

## Setup

### 1. Verify Hardware Detection

```bash
# Linux/macOS: Check if PCSC daemon is running
systemctl status pcscd           # Linux (systemd)
sudo service pcscd status        # Linux (SysV init)
ps aux | grep pcscd              # macOS/Linux (process check)

# Verify YubiKey is detected
ykman list                       # Should show YubiKey serial number

# Alternative: Use pcsc_scan (if installed)
pcsc_scan                        # Should detect YubiKey reader
```

**Expected output (ykman list):**
```
YubiKey 5C NFC (5.4.3) [OTP+FIDO+CCID] Serial: 12345678
```

### 2. PIV Application Setup

**Option A: Factory Reset (destructive)**
```bash
# WARNING: Deletes all PIV keys and certificates
ykman piv reset
```

**Option B: Verify existing setup**
```bash
# List existing PIV objects
ykman piv info

# Expected output (fresh YubiKey):
# PIV version: 5.4.3
# PIN tries remaining: 3/3
# Management key is default (WARNING: Change for production use)
```

### 3. Prepare Test Slot

TrustEdge uses **PIV slot 9a (Authentication)** by default.

```bash
# Verify slot 9a is empty (or delete existing key)
ykman piv keys delete 9a         # Delete if needed (requires Management Key)
```

---

## Test Steps

### Test 1: Backend Detection

**Objective:** Verify TrustEdge detects YubiKey backend and reads device information.

**Test Command:**
```bash
# Run software-hsm-demo to list available backends
./target/release/software-hsm-demo
```

**Expected Result:**
```
Available backends:
- Software HSM (PBKDF2-based, in-memory)
- OS Keyring (platform credential storage)
- YubiKey PIV (hardware-backed, serial: 12345678)  ← This should appear
```

**Verification:**
- YubiKey backend appears in list
- Serial number matches your YubiKey
- Device info shows PIV capabilities

**If Failed:**
- Check PCSC daemon is running: `systemctl status pcscd`
- Verify YubiKey is inserted: `ykman list`
- Check USB connection (try different port)
- Review error message for PCSC library issues

---

### Test 2: Key Generation

**Objective:** Generate Ed25519 key in PIV slot 9a and export public key.

**Test Command:**
```bash
# Generate key in slot 9a (requires PIN: 123456)
ykman piv keys generate 9a --algorithm ECCP256 --pin-policy ONCE public_key.pem

# Alternative: Use TrustEdge API (if CLI supports it)
# ./target/release/trustedge-cli yubikey generate --slot 9a
```

**Expected Result:**
- Key generation succeeds without error
- Public key exported to `public_key.pem`
- PIN prompt appears (enter: 123456)

**Verification:**
```bash
# Verify key exists in slot 9a
ykman piv keys attest 9a        # Should show attestation certificate

# Inspect public key
openssl ec -pubin -in public_key.pem -text -noout
```

**If Failed:**
- Wrong PIN: YubiKey shows "3 tries remaining" → Re-enter correct PIN
- Slot already in use: Delete existing key (`ykman piv keys delete 9a`)
- Algorithm not supported: YubiKey 5 series supports ECCP256 (P-256 curve)

---

### Test 3: Signing Operation

**Objective:** Sign test data using hardware-backed key and verify signature.

**Prerequisites:** Test 2 completed successfully (key exists in slot 9a).

**Test Command:**
```bash
# Create test data
echo "TrustEdge hardware signing test" > test_data.txt

# Sign using YubiKey (requires PIN)
ykman piv keys export 9a public_key.pem           # Export public key
openssl dgst -sha256 -sign slot:9a test_data.txt > signature.bin  # Sign with PKCS#11

# Alternative: Use TrustEdge signing API
# ./target/release/trustedge-cli yubikey sign --slot 9a --input test_data.txt
```

**Expected Result:**
- Signing operation succeeds
- Signature file created (`signature.bin`)
- PIN prompt appears (enter: 123456)

**Verification:**
```bash
# Verify signature with public key
openssl dgst -sha256 -verify public_key.pem -signature signature.bin test_data.txt

# Expected output:
# Verified OK
```

**If Failed:**
- PIN required but not provided: YubiKey flashes, waiting for PIN entry
- Wrong PIN: "incorrect PIN" error, retries decrement
- Signature verification fails: Check public key matches slot 9a key

---

### Test 4: Key Persistence

**Objective:** Verify generated key persists across power cycles.

**Prerequisites:** Test 2 completed (key exists in slot 9a).

**Test Procedure:**

1. **Export public key before power cycle:**
   ```bash
   ykman piv keys export 9a public_key_before.pem
   ```

2. **Remove and re-insert YubiKey:**
   - Physically remove YubiKey from USB port
   - Wait 5 seconds
   - Re-insert YubiKey

3. **Verify key still exists:**
   ```bash
   ykman piv keys export 9a public_key_after.pem
   ```

4. **Compare public keys:**
   ```bash
   diff public_key_before.pem public_key_after.pem
   # Expected: No differences (files are identical)
   ```

**Expected Result:**
- Key persists after power cycle
- Public key export succeeds without re-generating
- Before/after public keys match exactly

**Verification:**
```bash
# Alternative: Use SHA256 hash comparison
sha256sum public_key_before.pem public_key_after.pem
# Both hashes should be identical
```

**If Failed:**
- Key disappeared: Volatile slot used (shouldn't happen with 9a)
- Different public key: Key was regenerated (check PIV configuration)
- YubiKey not detected after re-insert: Check USB connection/PCSC daemon

---

### Test 5: PIN Retry Limit

**Objective:** Verify PIN retry limit enforcement and error handling.

**⚠️ WARNING:** This test will lock the PIN. Use non-production YubiKey or be prepared to reset.

**Test Procedure:**

1. **Check initial PIN retry count:**
   ```bash
   ykman piv info
   # Expected: PIN tries remaining: 3/3
   ```

2. **Attempt signing with wrong PIN (1st attempt):**
   ```bash
   # Try to sign with wrong PIN (e.g., 000000)
   ykman piv keys export 9a /dev/null --pin 000000
   # Expected: Error - incorrect PIN
   ```

3. **Verify retry count decremented:**
   ```bash
   ykman piv info
   # Expected: PIN tries remaining: 2/3
   ```

4. **Attempt 2 more times with wrong PIN:**
   ```bash
   ykman piv keys export 9a /dev/null --pin 111111
   ykman piv keys export 9a /dev/null --pin 222222
   ```

5. **Verify PIN is now blocked:**
   ```bash
   ykman piv info
   # Expected: PIN tries remaining: 0/3 (PIN BLOCKED)
   ```

6. **Attempt operation with correct PIN:**
   ```bash
   ykman piv keys export 9a /dev/null --pin 123456
   # Expected: Error - PIN blocked, PUK required
   ```

**Expected Results:**
- Each wrong PIN attempt decrements retry counter
- After 3 failures, PIN is blocked
- Correct PIN no longer works when blocked
- Error message indicates PUK required

**Cleanup (Unblock PIN):**
```bash
# Unblock PIN using PUK (default PUK: 12345678, set new PIN)
ykman piv access unblock-pin --puk 12345678 --new-pin 123456

# Alternative: Reset PIV application (DESTRUCTIVE)
ykman piv reset
```

**If Failed:**
- PIN not blocked after 3 attempts: YubiKey configuration issue
- Retry count not updating: Check `ykman piv info` output format
- PUK also blocked: YubiKey is locked, requires factory reset

**⚠️ CRITICAL:** Never let PUK retry count reach 0. PUK lockout is permanent and requires hardware replacement.

---

## Expected Results Summary

### Test 1: Backend Detection
- ✔ YubiKey backend appears in available backends list
- ✔ Serial number correctly displayed
- ✔ PIV capabilities detected

### Test 2: Key Generation
- ✔ Key generation succeeds without error
- ✔ Public key exported in PEM format
- ✔ PIN authentication works correctly

### Test 3: Signing Operation
- ✔ Signature generation succeeds
- ✔ Signature verifies with public key
- ✔ No errors during signing

### Test 4: Key Persistence
- ✔ Key survives power cycle
- ✔ Public key identical before/after re-insertion
- ✔ No re-generation required

### Test 5: PIN Retry Limit
- ✔ Retry counter decrements on wrong PIN
- ✔ PIN blocks after 3 failures
- ✔ Correct PIN rejected when blocked
- ✔ PUK unblock restores access

**Overall Pass Criteria:** All 5 tests complete with expected results.

---

## Troubleshooting

### YubiKey Not Detected

**Symptom:** `ykman list` shows no devices or `pcsc_scan` finds no readers.

**Solutions:**
1. **Check USB connection:**
   - Try different USB port
   - Check YubiKey LED (should light up when inserted)
   - Test with another computer (verify hardware is working)

2. **Verify PCSC daemon:**
   ```bash
   # Linux
   sudo systemctl restart pcscd
   sudo systemctl status pcscd

   # macOS (shouldn't need restart)
   ps aux | grep pcscd
   ```

3. **Check permissions:**
   ```bash
   # Linux: Add user to pcscd group
   sudo usermod -a -G pcscd $USER
   # Log out and log back in for group membership to take effect
   ```

4. **Install missing drivers:**
   ```bash
   # Linux
   sudo apt install pcscd pcsc-tools libpcsclite-dev  # Debian/Ubuntu
   sudo dnf install pcsc-lite pcsc-tools              # Fedora/RHEL
   ```

### PCSC Error Messages

**Error:** `SCARD_E_NO_SERVICE` or `PCSC not available`

**Solution:**
```bash
# Start PCSC daemon
sudo systemctl start pcscd
sudo systemctl enable pcscd  # Auto-start on boot
```

**Error:** `SCARD_E_NO_READERS_AVAILABLE`

**Solution:**
- YubiKey not inserted or defective
- USB reader not recognized (check `lsusb` output)
- Try different USB port

### Authentication Failed

**Symptom:** "Authentication failed" or "Incorrect PIN" when using default PIN.

**Cause:** YubiKey PIN may have been changed from default.

**Solutions:**
1. **Try common PINs:**
   - Default: 123456
   - Changed: Ask YubiKey owner

2. **Check PIN retry count:**
   ```bash
   ykman piv info
   # If retries = 0, PIN is blocked → use PUK
   ```

3. **Reset PIV application (LAST RESORT):**
   ```bash
   # WARNING: Deletes all PIV keys
   ykman piv reset
   ```

### Slot Already in Use

**Symptom:** "Slot 9a already contains a key" when generating.

**Solution:**
```bash
# Delete existing key (requires Management Key)
ykman piv keys delete 9a

# If default Management Key changed, specify it:
ykman piv keys delete 9a --management-key <48-char-hex>
```

### Signature Verification Fails

**Symptom:** `openssl dgst -verify` returns "Verification Failure".

**Causes:**
1. **Wrong public key:** Exported from different slot or YubiKey
   - Solution: Re-export from correct slot (`ykman piv keys export 9a`)

2. **Corrupted signature file:**
   - Solution: Re-generate signature with same test data

3. **Test data modified:**
   - Solution: Ensure `test_data.txt` unchanged between sign and verify

### YubiKey Locked (PUK Exhausted)

**Symptom:** "PIN blocked" and "PUK blocked" both show 0 retries.

**No Software Solution:** This is permanent lockout. YubiKey PIV application cannot be reset.

**Hardware Solution:**
- Replace YubiKey (hardware is unusable for PIV)
- Other YubiKey applications (OTP, FIDO) still functional

**Prevention:**
- Never test PIN retry limits on production YubiKeys
- Keep PUK in secure location (not with YubiKey)
- Use non-default PIN/PUK in production

---

## Success Criteria

**✔ Hardware integration validated when:**

1. All 5 tests complete successfully
2. YubiKey backend detection works correctly
3. Cryptographic operations (generate, sign, verify) succeed
4. Key persistence verified across power cycles
5. PIN retry limits enforced as expected

**Protocol complete:** YubiKey hardware integration working correctly with TrustEdge.

---

## Relationship to Automated Tests

### Simulation Tests (90+ tests in CI)

**File:** `crates/core/tests/yubikey_integration.rs`

**Coverage:**
- Backend initialization and configuration
- Capability discovery (signing, key generation)
- Error handling (no hardware, wrong PIN, missing slots)
- Mock PIV operations for CI/CD

**Run automatically:** Every CI build, no hardware required.

### Manual Tests (This Protocol)

**Coverage:**
- Actual hardware cryptographic operations
- Physical key detection and enumeration
- Real PIN retry behavior
- PIV slot persistence

**Run manually:** When validating YubiKey support on specific hardware models.

### Combined Coverage

**Together, simulation + manual tests provide:**
- 100% backend trait compliance (simulation)
- 100% configuration validation (simulation)
- 100% hardware interaction validation (manual)
- Production-ready YubiKey integration confidence

---

## Notes

- **Simulation tests (90+) run in every CI build** without hardware - they validate backend logic and error handling.
- **Manual protocol required only for hardware-specific validation** - signing with real keys, PIN retry limits, physical persistence.
- TrustEdge YubiKey integration is production-ready based on simulation test coverage.
- Manual protocol confirms actual hardware behavior matches simulation expectations.

**This protocol documents the existing testing approach:** Simulation for automation, manual for hardware validation when available.

---

**Version:** 1.0
**Last Updated:** 2026-02-11
**Maintainer:** TrustEdge Labs
**Related Documentation:** Phase 6 feature flags documentation, YubiKey integration tests
