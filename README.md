# ESC Controller

A `no_std` Rust library for controlling a 2S-7S brushless ESC via PWM/servo signal programming.

## Overview

This library models every parameter and option from standard ESC programming manuals, providing PWM microsecond values for each throttle position. It exposes a simple state-machine (`EscProgrammer`) that tells the caller exactly which PWM value to output at each step of the Morse-code beep programming protocol (Music-Tone + N-Beeps sequences).

Designed for bare-metal embedded environments, it ensures zero-cost abstractions with guaranteed memory safety.

## Features

- **Bare-Metal Rust (`no_std`)**: Fully compatible with embedded environments without an allocator.
- **Memory Safe**: Uses `#![forbid(unsafe_code)]` to guarantee memory safety.
- **State-Machine Driven**: The `EscProgrammer` struct handles the complex timing and state transitions of the ESC programming protocol.
- **Comprehensive Parameter Support**: Models all 6 standard programmable parameters, including Morse-code sequences and factory defaults.
- **Optional Logging**: Supports `defmt` for embedded-friendly logging via the `defmt` feature flag.

## Usage

The ESC is programmed through throttle-stick positions and beep sequences:

1. **Enter programming mode** – full throttle (2000 µs) at power-on.
2. **Select a parameter** – move stick to centre (1500 µs) when you hear the correct Music-Tone + N-Beeps sequence.
3. **Select an option** – move stick to full-up (2000 µs) when you hear the desired Morse sub-option sequence.
4. **Finish** – move stick to lowest position (1000 µs) to save and exit.

### Example

```rust
use esc_controller::{
    EscProgrammer, EscConfig, ProgrammerState, StepResult,
    CellType, BrakeSetting,
};

// Build the desired configuration (only override what you need).
let mut cfg = EscConfig::defaults();
cfg.cell_type = Some(CellType::LiPo3S);
cfg.brake = Some(BrakeSetting::NoBrake);

let mut prog = EscProgrammer::new(cfg);

// Phase 1: power-on with full-up throttle.
let StepResult { pwm_us, state } = prog.start();
assert_eq!(pwm_us, 2000); // full-up to enter programming mode
// … set your PWM output to pwm_us, wait for the musical tone …

// Then drive the state machine via `prog.advance()` each time you hear
// a beep or tone, until state == ProgrammerState::Done.