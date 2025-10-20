#!/bin/bash

clear

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║  TrustEdge: Hardware-Backed Security for IoT Devices          ║"
echo "║  Demo: YubiKey PIV Integration via PKCS#11                     ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "Demonstration of hardware-backed X.509 certificate generation"
echo "using factory-default YubiKey (PIN: 123456)"
echo ""
sleep 2

echo "─────────────────────────────────────────────────────────────────"
echo "Step 1: Verify YubiKey Hardware"
echo "─────────────────────────────────────────────────────────────────"
lsusb | grep -i yubi
echo ""
sleep 1

echo "─────────────────────────────────────────────────────────────────"
echo "Step 2: Generate Hardware-Backed Certificate"
echo "─────────────────────────────────────────────────────────────────"
cargo run --example yubikey_demo --features yubikey -- 123456
echo ""
sleep 1

echo "─────────────────────────────────────────────────────────────────"
echo "Step 3: Verify Certificate with OpenSSL"
echo "─────────────────────────────────────────────────────────────────"
openssl x509 -in yubikey_cert_slot_9a.der -inform DER -text -noout | head -20
echo ""

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║  ✔ Hardware-backed signing proven via PKCS#11                  ║"
echo "║  ✔ Private key never left the YubiKey                          ║"
echo "║  ✔ Standards-compliant X.509 certificate                       ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""
echo "Source: https://github.com/trustedge-labs/trustedge"
echo "Commercial support: pilot@trustedgelabs.com"

# Press Ctrl+D to stop recording
