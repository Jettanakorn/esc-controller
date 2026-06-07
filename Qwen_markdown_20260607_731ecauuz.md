# JFOX ESC Controller

**Electronic Speed Controller Firmware & Programming Library for the Baby Redbee Drone**

Developed by the Readbee Team at [JFOX Aircraft Co., Ltd.](https://github.com/Jettanakorn)

---

## Overview

This repository contains a bare-metal, `no_std` Rust library for programming and controlling 2S–7S brushless Electronic Speed Controllers (ESCs) that utilize the Morse-code beep programming protocol (Music-Tone + N-Beeps sequences). 

Designed specifically to handle the high-RPM and high-discharge demands of coaxial EDF (Electric Ducted Fan) motors and thrust vectoring actuators in the Baby Redbee drone project, this ESC prioritizes deterministic timing, low latency, and strict safety-critical protocols.

## Features

- **Bare-Metal Rust (`no_std`)**: Zero-cost abstractions with guaranteed memory safety (`#![forbid(unsafe_code)]`) and no runtime overhead.
- **State-Machine Programming Protocol**: A robust `EscProgrammer` state machine that guides the caller through the exact PWM microsecond values to output at each step of the ESC's audio programming sequence.
- **Comprehensive Parameter Modeling**: Transcribed verbatim from standard ESC manuals, supporting all 6 programmable parameters including Cell Type, Throttle Settings, Brake Strength, Direction/Cutoff, Timing Advance, and PWM Frequency.
- **Safety-Critical Design**:
  - Hardware-timed dead-time insertion to prevent shoot-through on MOSFETs.
  - Failsafe throttle-cut on signal loss (RC/MAVLink heartbeat timeout).
  - Over-current and over-temperature protection monitoring.
- **LED Status Feedback**: Integrates with the JFOX standard LED status pattern signals for system state, arming, and fault indication.
- **Modular Architecture**: Clean separation between the hardware abstraction layer (HAL), commutation logic, telemetry, and the programming protocol.

## How the Programming Protocol Works

The ESC is programmed through specific throttle-stick positions (PWM pulse widths) and audio beep sequences:

1. **Enter Programming Mode**: Apply full-up throttle (`2000 µs`) at power-on. Wait for the initial musical tone.
2. **Select a Parameter**: The ESC will announce parameters via a Music-Tone followed by $N$ beeps. When you hear the correct sequence for the parameter you want to change, move the stick to the center (`1500 µs`).
3. **Select an Option**: The ESC will replay Morse-code sub-options (short and long beeps) for that parameter. When you hear the desired option, move the stick to full-up (`2000 µs`) to select and save it.
4. **Finish**: Move the stick to the lowest position (`1000 µs`) to save all changes and exit programming mode.

This library models every parameter and option, providing the exact PWM values via the `EscProgrammer` state machine.

## Supported Parameters

The library fully models the following 6 programmable parameters, including their Morse-code selection sequences and factory defaults:

1. **Cell Type & Count** (LiPo 2S-7S, NiMh/NiCd)
2. **Throttle Setting** (Auto Range, Fixed 1ms-1.8ms, Hard/Soft Start)
3. **Brake Setting** (No Brake, Soft, Medium, Hard)
4. **Direction & Cutoff** (CW/CCW Rotation, Soft/Hard Low-Voltage Cutoff)
5. **Timing Mode** (1°, 7°, 15°, 30° advance for different motor pole counts)
6. **PWM Frequency** (8 kHz, 16 kHz)

## Usage Example

The core of the library is the `EscProgrammer` state machine. You initialize it with your desired configuration, call `start()`, and then repeatedly call `advance()` every time you hear an audio event from the ESC.

```rust
use esc_controller::{
    EscProgrammer, EscConfig, ProgrammerState, StepResult,
    CellType, BrakeSetting,
};

// 1. Build the desired configuration (only override what you need).
let mut cfg = EscConfig::defaults();
cfg.cell_type = Some(CellType::LiPo3S);
cfg.brake = Some(BrakeSetting::NoBrake);

let mut prog = EscProgrammer::new(cfg);

// 2. Phase 1: Power-on with full-up throttle.
let StepResult { pwm_us, state } = prog.start();
assert_eq!(pwm_us, 2000); // 2000 µs to enter programming mode
// ... set your PWM output to `pwm_us`, wait for the musical tone ...

// 3. Drive the state machine via `prog.advance()` each time you hear
// a beep or tone, until state == ProgrammerState::Done.
loop {
    // Wait for an audio event from the ESC (tone, beep, or Morse sequence end)
    // ...

    let StepResult { pwm_us, state } = prog.advance();
    
    // Apply `pwm_us` to your PWM output pin
    // ...

    if state == ProgrammerState::Done {
        break; // Programming complete!
    }
}