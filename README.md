# JFOX ESC Controller
**Electronic Speed Controller Firmware for the Baby Redbee Drone**

Developed by the Readbee Team at JFOX Aircraft Co., Ltd.

## Overview
This repository contains the bare-metal Rust firmware for the Electronic Speed Controller (ESC) used in the Baby Redbee drone project. Designed specifically to handle the high-RPM and high-discharge demands of coaxial EDF (Electric Ducted Fan) motors and thrust vectoring actuators, this ESC prioritizes deterministic timing, low latency, and strict safety-critical protocols.

## Features
- **Bare-Metal Rust (`no_std`)**: Zero-cost abstractions with guaranteed memory safety and no runtime overhead.
- **Targeted for ESP32**: Utilizing the ESP32 architecture (via `esp-hal`) for high-resolution PWM generation (MCPWM).
- **Safety-Critical Design**:
  - Hardware-timed dead-time insertion to prevent shoot-through on MOSFETs.
  - Failsafe throttle-cut on signal loss (RC/MAVLink heartbeat timeout).
  - Over-current and over-temperature protection monitoring.
- **LED Status Feedback**: Integrates with the JFOX standard LED status pattern signals for system state, arming, and fault indication.
- **Modular Architecture**: Clean separation between the hardware abstraction layer (HAL), commutation logic, and telemetry.

## Hardware Requirements
*(Update this section based on your specific KiCad 9.0 schematic)*
- **MCU**: ESP32 / ESP32-C3
- **Gate Drivers**: [e.g., IR2110 / DRV8300]
- **MOSFETs**: [e.g., CSD18540Q5B N-channel]
- **Motor Type**: 3-Phase BLDC (Coaxial EDF)
- **Sensors**: Shunt current sensor, NTC thermistor

## Software Requirements
- [Rust](https://www.rust-lang.org/tools/install) (Latest stable or nightly depending on `esp-hal` requirements)
- `espup` (For ESP32 Rust toolchain installation)
- `probe-rs` or `cargo-embed` for flashing and debugging
- `cargo-espflash` (Alternative for USB flashing)

## Building and Flashing

### 1. Setup the Toolchain
```bash
# Install espup for ESP32 Rust development
cargo install espup
espup install

# Install flashing tools
cargo install cargo-espflash
cargo install probe-rs-tools --features cli