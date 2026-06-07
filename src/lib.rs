//! # ESC Controller
//!
//! A `no_std` Rust library for programming and controlling a 2S–7S brushless
//! ESC that uses the Morse-code beep programming protocol (Music-Tone +
//! N-Beeps sequences).
//!
//! ## Overview
//!
//! The ESC is programmed through throttle-stick positions and beep sequences:
//!
//! 1. **Enter programming mode** – full throttle at power-on.
//! 2. **Select a parameter** – move stick to centre when you hear the correct
//!    Music-Tone + N-Beeps sequence.
//! 3. **Select an option** – move stick to full-up when you hear the desired
//!    Morse sub-option sequence.
//! 4. **Finish** – move stick to lowest position to save and exit.
//!
//! This library models every parameter and option from the manual, provides
//! PWM microsecond values for each throttle position, and exposes a simple
//! state-machine ([`EscProgrammer`]) that tells the caller exactly which PWM
//! value to output at each step.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod params;
pub mod programmer;
pub mod pwm;

pub use params::*;
pub use programmer::{EscProgrammer, ProgrammerState, StepResult};
pub use pwm::{ThrottlePosition, PWM_CENTER_US, PWM_FULL_DOWN_US, PWM_FULL_UP_US};
