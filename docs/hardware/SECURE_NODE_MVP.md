<!--
Copyright (c) 2025 TRUSTEDGE LABS LLC
MPL-2.0: https://mozilla.org/MPL/2.0/
Project: trustedge — Privacy and trust at the edge.
GitHub: https://github.com/johnzilla/trustedge
-->

# TrustEdge Secure Node MVP (ESP32-WROOM-32SE)

The Secure Node MVP is a small ESP32-based reference board that demonstrates how to run TrustEdge on a device with a hardware secure element and a verifiable boot chain.

It uses an **ESP32-WROOM-32SE** module (ESP32 + Microchip ATECC608A secure element) and exposes TrustEdge over both USB (for development) and a dedicated UART header for host devices.

---

## Hardware overview

- **MCU / module:** ESP32-WROOM-32SE  
  - Wi‑Fi + BLE  
  - ESP-IDF secure boot and flash encryption  
  - Integrated ATECC608A secure element on I²C for device keys and ECDSA operations.

- **Power:**  
  - USB‑C 5 V input → MP2315‑class 3.3 V buck regulator (≥800 mA).  
  - ESD, over‑current (0.5 A PPTC), and reverse‑polarity protection on VBUS.

- **Connectivity:**  
  - USB‑C → CP2102N → ESP32 UART0 (flashing + debug console).  
  - 4‑pin 2.54 mm header exposing UART2 (TXD2/RXD2, 3V3, GND) as the **host interface** for TrustEdge.

- **User I/O:**  
  - Buttons: **BOOT**, **Reset**, **Secure Reset/Provision**.  
  - LEDs: **Power**, **Secure Boot Status**, **Secure Session/Attestation**.

---

## Key security properties

- **Hardware root of trust:**  
  - ATECC608A holds long‑term device identity keys; private keys never leave the secure element.  
  - ESP32 uses the secure element for ECDSA sign/verify (mutual TLS, attestation, manifest signing).

- **Boot and storage security:**  
  - ESP-IDF **Secure Boot v2** and **Flash Encryption** enabled.  
  - Firmware images are encrypted at rest and must be correctly signed before execution; production keys stored in eFuses.

- **Host integration:**  
  - External MCU or SBC connects over UART2 and treats the board as a “crypto + attestation module on a cable.”  
  - Planned TrustEdge UART protocol operations:
    - Request attestation report  
    - Sign/verify blobs  
    - Encrypt/decrypt payloads under device/session keys.

---

## Schematic and BOM (summary)

**Major components**

- ESP32-WROOM-32SE module  
- MP2315DN (or similar) 3.3 V buck converter + inductor + bulk caps  
- CP2102N USB‑to‑UART bridge  
- USB‑C receptacle (SMD, with CC pins).  
- USBLC6‑2SC6 ESD array on D+/D‑  
- 0.5 A PPTC polyfuse + SS14 Schottky diode on VBUS  
- 3 × 6×6 mm tactile buttons (BOOT, Reset, Secure Provision)  
- 3 × LEDs (Power, Secure Boot, Secure Session) + 1 kΩ resistors  
- Decoupling capacitors near ESP32 and regulator per datasheets.

A full 3e8‑generated schematic and BOM live alongside this file (see `/hardware/` directory in this repo).

---

## Bring‑up and firmware workflow

1. **Assemble board** (or order assembled): solder SMD passives, buck converter, CP2102N, ESP32-WROOM-32SE, then USB‑C, buttons, LEDs, headers.  
2. **Power‑on test:** supply 5 V over USB‑C, verify 3.3 V rail and Power LED.  
3. **Development mode:**
   - Use USB (CP2102N) to flash ESP‑IDF “blink” and then TrustEdge firmware.  
   - Keep debug headers populated; secure boot/flash encryption in **development** mode.  
4. **Production‑like mode:**
   - Burn eFuses for Secure Boot v2 and Flash Encryption with production keys.  
   - Permanently disable JTAG and (optionally) depopulate debug headers.  
   - Use **Secure Reset/Provision** button at power‑on to enter a limited provisioning flow.

---

## TrustEdge mapping

Firmware on this board is a thin hardware backend for TrustEdge:

- Uses ESP-IDF crypto + ATECC608A for AES‑GCM, hashing, and ECDSA.  
- Implements the TrustEdge “universal backend” over UART2 and/or Wi‑Fi (mTLS) so higher‑level TrustEdge clients can offload crypto and retrieve attestation reports.

See the `hardware/` and `examples/` directories for firmware and host‑side examples (WIP).


