# JFOX Aircraft - ESC Controller

A **`no_std` Rust library** for programming and controlling **2S-7S brushless ESCs** via PWM/servo signal programming using the **Morse-code beep protocol** (Music-Tone + N-Beeps sequences).

> **Author**: Jettanakorn Pengsiri
> **Project**: JFOX Aircraft
> **License**: MIT

---

## 📋 Overview

This library provides a **memory-safe, bare-metal Rust** implementation for programming Electronic Speed Controllers (ESCs) that use the standard Morse-code beep programming protocol. It models every parameter and option from ESC programming manuals, exposing a simple **state-machine** (`EscProgrammer`) that tells the caller exactly which PWM value to output at each step.

### ✨ Key Features

| Feature | Description |
|---------|-------------|
| **`no_std` Compatible** | Fully compatible with embedded environments without an allocator |
| **Memory Safe** | Uses `#![forbid(unsafe_code)]` to guarantee memory safety |
| **Zero-Cost Abstractions** | Designed for bare-metal embedded environments |
| **State-Machine Driven** | `EscProgrammer` handles complex timing and state transitions |
| **Comprehensive Parameters** | Models all 6 standard programmable parameters with Morse-code sequences |
| **Optional Logging** | Supports `defmt` for embedded-friendly logging via feature flag |

---

## 🎯 Use Cases

- **Drone/RC Aircraft** firmware development
- **Embedded flight controllers**
- **Custom ESC programming tools**
- **Automated ESC configuration** in manufacturing or testing

---

## 📦 Installation

```toml
[dependencies]
esc-controller = { git = "https://github.com/Jettanakorn/esc-controller" }
```

### Features

| Feature | Description |
|---------|-------------|
| `std` | Enable standard library support |
| `defmt` | Enable `defmt` logging for embedded environments |

```toml
[dependencies]
esc-controller = { git = "https://github.com/Jettanakorn/esc-controller", features = ["defmt"] }
```

---

## 🔧 Programming Protocol

### Steps

1. **Enter Programming Mode** – Power on with full throttle (2000 µs), wait for music tone
2. **Select Parameter** – Move stick to centre (1500 µs) when you hear Music-Tone + N-Beeps
3. **Select Option** – Move stick to full-up (2000 µs) when you hear the desired Morse sub-option
4. **Save & Exit** – Move stick to lowest position (1000 µs) to save and exit

### PWM Reference

| Position | PWM Value | Description |
|----------|-----------|-------------|
| Full Down | 1000 µs | Saves settings and exits |
| Center | 1500 µs | Enters sub-option menu |
| Full Up | 2000 µs | Enters programming mode / selects option |

---

## 🚀 Quick Start

```rust
use esc_controller::{
    EscProgrammer, EscConfig, ProgrammerState, StepResult,
    CellType, BrakeSetting, ThrottleSetting, DirectionCutoff,
    TimingMode, PwmFrequency,
};

// Build configuration
let mut cfg = EscConfig::defaults();
cfg.cell_type = Some(CellType::LiPo3S);
cfg.brake = Some(BrakeSetting::NoBrake);

let mut prog = EscProgrammer::new(cfg);

// Start programming
let StepResult { pwm_us, state } = prog.start();
assert_eq!(pwm_us, 2000); // Full-up to enter programming mode

// Drive state machine
while prog.state() != ProgrammerState::Done {
    let StepResult { pwm_us, state } = prog.advance();
    // Set PWM output to pwm_us, wait for audio event...
}
```

---

## ⚙️ Configuration Parameters

### Parameter 1: Cell Type (Music Tone + 1 Beep)
**Default**: LiPo 4S

| Option | Morse | Voltage | Cutoff |
|--------|-------|---------|--------|
| `NiMhNiCdAutoCount` | `• —` | Auto | 0.8 V/cell |
| `LiPo7S` | `• — —` | 25.9 V | 21 V |
| `LiPo6S` | `• — — —` | 22.2 V | 18 V |
| `LiPo5S` | `• — — — —` | 18.5 V | 15 V |
| `LiPo4S` | `• — — — — —` | 14.8 V | 12 V |
| `LiPo3S` | `• — — — — — —` | 11.1 V | 9 V |
| `LiPo2S` | `• — — — — — — —` | 7.4 V | 8 V |

### Parameter 2: Throttle Setting (Music Tone + 2 Beeps)
**Default**: Auto Throttle Range

| Option | Morse | Description |
|--------|-------|-------------|
| `AutoThrottleRange` | `•• —` | Auto throttle range *(default)* |
| `Fixed1msTo1_8ms` | `•• — —` | 1 ms to 1.8 ms fixed range |
| `HardStart` | `•• — — —` | Hard start *(default for start)* |
| `SoftStart` | `•• — — — —` | Soft start |

### Parameter 3: Brake Setting (Music Tone + 3 Beeps)
**Default**: Soft Brake

| Option | Morse | Description |
|--------|-------|-------------|
| `NoBrake` | `••• —` | No brake |
| `SoftBrake` | `••• — —` | Soft brake *(default)* |
| `MediumBrake` | `••• — — —` | Medium brake |
| `HardBrake` | `••• — — — —` | Hard brake |

### Parameter 4: Direction & Cutoff (Music Tone + 4 Beeps)
**Default**: Clockwise + Hard Cutoff

| Option | Morse | Description |
|--------|-------|-------------|
| `ClockwiseRotation` | `•••• —` | Clockwise *(default)* |
| `CounterclockwiseRotation` | `•••• — —` | Counter-clockwise |
| `SoftCutoff` | `•••• — — —` | Soft cutoff |
| `HardCutoff` | `•••• — — — —` | Hard cutoff *(default)* |

### Parameter 5: Timing Mode (Music Tone + 5 Beeps)
**Default**: 1° for 2-4 pole inrunner

| Option | Morse | Degrees | Motor Type |
|--------|-------|---------|------------|
| `Deg1For2To4PoleInrunner` | `••••• —` | 1° | 2-4 pole inrunner *(default)* |
| `Deg7For6To8Pole` | `••••• — —` | 7° | 6-8 pole |
| `Deg15For10To14PoleOutrunner` | `••••• — — —` | 15° | 10-14 pole outrunner |
| `Deg30For10To14PoleHighRpmOutrunner` | `••••• — — — —` | 30° | High-RPM outrunner |

### Parameter 6: PWM Frequency (Music Tone + 6 Beeps)
**Default**: 8 kHz

| Option | Morse | Frequency | Use Case |
|--------|-------|-----------|----------|
| `Khz8` | `•••••• —` | 8 kHz | Low RPM / low pole-count *(default)* |
| `Khz16` | `•••••• — —` | 16 kHz | Most outrunner motors |

---
---
## 🔍 API Reference

### `EscConfig`
Factory defaults: `EscConfig::defaults()` or empty: `EscConfig::empty()`

### `EscProgrammer`
- `new(config) -> Self`
- `start() -> StepResult`
- `advance() -> StepResult`
- `state() -> ProgrammerState`
- `current_param() -> Option<Parameter>`

### `StepResult`
```rust
pub struct StepResult {
    pub pwm_us: u16,
    pub state: ProgrammerState,
}
```

### `ProgrammerState`
- `WaitingForInitTone`
- `ListeningForParameter(Parameter)`
- `WaitingForSubOptions(Parameter)`
- `ListeningForOption { parameter, option_index }`
- `Done`

---
---
## 🧪 Testing

```bash
cargo test

---
## 📊 Project Structure

```
esc-controller/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── lib.rs
    ├── params.rs
    ├── programmer.rs
    └── pwm.rs
```
## 🔗 Links

- **GitHub**: [https://github.com/Jettanakorn/esc-controller](https://github.com/Jettanakorn/esc-controller)
- **Author**: Jettanakorn Pengsiri (JFOX Aircraft)
- **License**: MIT