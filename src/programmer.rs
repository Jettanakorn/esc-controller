//! ESC programming state machine.
//!
//! [`EscProgrammer`] drives the caller through the three-phase programming
//! protocol described in the manual, returning the exact PWM value (in µs)
//! that should be output at each step.
//!
//! ## Typical usage
//!
//! ```
//! use esc_controller::{
//!     EscProgrammer, EscConfig, ProgrammerState, StepResult,
//!     CellType, BrakeSetting,
//! };
//!
//! // Build the desired configuration (only override what you need).
//! let mut cfg = EscConfig::defaults();
//! cfg.cell_type = Some(CellType::LiPo3S);
//! cfg.brake     = Some(BrakeSetting::NoBrake);
//!
//! let mut prog = EscProgrammer::new(cfg);
//!
//! // Phase 1: power-on with full-up throttle.
//! let StepResult { pwm_us, state } = prog.start();
//! assert_eq!(pwm_us, 2000); // full-up to enter programming mode
//! // … set your PWM output to pwm_us, wait for the musical tone …
//!
//! // Then drive the state machine via `prog.advance()` each time you hear
//! // a beep or tone, until state == ProgrammerState::Done.
//! ```

use crate::{
    params::{
        BrakeSetting, CellType, DirectionCutoff, EscConfig, Parameter, PwmFrequency,
        ThrottleSetting, TimingMode,
    },
    pwm::{PWM_CENTER_US, PWM_FULL_DOWN_US, PWM_FULL_UP_US},
};

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// What the programmer wants the caller to do right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProgrammerState {
    /// Hold full-up throttle; wait for the music tone that signals programming
    /// mode is active.
    WaitingForInitTone,

    /// The ESC is announcing parameters.  Hold the current PWM and listen for
    /// the Music-Tone + N-Beeps sequence for the given [`Parameter`].
    /// When heard, call [`EscProgrammer::advance`].
    ListeningForParameter(Parameter),

    /// We just moved to centre to enter sub-options.  Hold centre and wait for
    /// the first Morse sub-option sequence.
    WaitingForSubOptions(Parameter),

    /// The ESC is replaying the Morse sequences for sub-options of the given
    /// parameter.  Hold the current PWM and listen.
    ListeningForOption {
        /// Which parameter we are currently setting.
        parameter: Parameter,
        /// How many options (Morse sequences) have been heard so far.
        option_index: u8,
    },

    /// Programming complete.  All options have been saved; PWM is at full-down.
    Done,
}

/// The result of a single [`EscProgrammer::advance`] call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StepResult {
    /// Output this PWM pulse-width (µs) on your ESC signal pin right now.
    pub pwm_us: u16,
    /// The new programmer state after this transition.
    pub state: ProgrammerState,
}

// ---------------------------------------------------------------------------
// Programmer
// ---------------------------------------------------------------------------

/// State machine that guides the caller through ESC programming.
///
/// Create one with [`EscProgrammer::new`], then:
/// 1. Call [`EscProgrammer::start`] to get the initial PWM value.
/// 2. Each time you hear an audio event from the ESC (tone, beep, or Morse
///    sequence end), call [`EscProgrammer::advance`].
/// 3. Stop when [`ProgrammerState::Done`] is returned.
#[derive(Debug, Clone)]
pub struct EscProgrammer {
    config:  EscConfig,
    state:   ProgrammerState,
    current_param: Option<Parameter>,
}

impl EscProgrammer {
    /// Create a new programmer for the given configuration.
    ///
    /// Only parameters whose field is `Some(_)` will be programmed; the rest
    /// are skipped (left at whatever the ESC already has).
    pub const fn new(config: EscConfig) -> Self {
        Self {
            config,
            state: ProgrammerState::WaitingForInitTone,
            current_param: None,
        }
    }

    /// Begin the programming sequence.
    ///
    /// Returns a [`StepResult`] with `pwm_us = 2000` (full-up).  Set your PWM
    /// output to this value **before** connecting the battery.
    pub fn start(&mut self) -> StepResult {
        self.state = ProgrammerState::WaitingForInitTone;
        StepResult {
            pwm_us: PWM_FULL_UP_US,
            state:  self.state,
        }
    }

    /// Advance the state machine by one audio event.
    ///
    /// Call this every time you hear the ESC produce an audio event:
    /// * The initial music tone (boot).
    /// * A Music-Tone + N-Beeps parameter announcement.
    /// * The long confirmation beep after saving an option.
    /// * The end of a full Morse sub-option sequence.
    ///
    /// Returns the [`StepResult`] describing what PWM to output next.
    pub fn advance(&mut self) -> StepResult {
        use ProgrammerState::*;

        match self.state {
            // ----------------------------------------------------------------
            // Phase 1 init tone heard → start listening for parameters
            // ----------------------------------------------------------------
            WaitingForInitTone => {
                let first_param = Parameter::CellTypeAndCount;
                self.state = ListeningForParameter(first_param);
                self.current_param = Some(first_param);
                // Keep full-up; just listen now
                StepResult { pwm_us: PWM_FULL_UP_US, state: self.state }
            }

            // ----------------------------------------------------------------
            // A parameter announcement was heard
            // ----------------------------------------------------------------
            ListeningForParameter(param) => {
                if self.should_program(param) {
                    // Move to centre to enter sub-options for this parameter
                    self.state = WaitingForSubOptions(param);
                    StepResult { pwm_us: PWM_CENTER_US, state: self.state }
                } else {
                    // Skip – wait for the next parameter
                    match param.next() {
                        Some(next) => {
                            self.state = ListeningForParameter(next);
                            self.current_param = Some(next);
                            StepResult { pwm_us: PWM_FULL_UP_US, state: self.state }
                        }
                        None => self.finish(),
                    }
                }
            }

            // ----------------------------------------------------------------
            // Centre reached; ESC starts Morse sequences
            // ----------------------------------------------------------------
            WaitingForSubOptions(param) => {
                self.state = ListeningForOption { parameter: param, option_index: 0 };
                StepResult { pwm_us: PWM_CENTER_US, state: self.state }
            }

            // ----------------------------------------------------------------
            // A Morse sub-option sequence was heard
            // ----------------------------------------------------------------
            ListeningForOption { parameter, option_index } => {
                let desired_index = self.desired_option_index(parameter);

                if option_index == desired_index {
                    // This is the option we want → full-up to select & save
                    self.state = match parameter.next() {
                        Some(next) => {
                            self.current_param = Some(next);
                            ListeningForParameter(next)
                        }
                        None => return self.finish(),
                    };
                    StepResult { pwm_us: PWM_FULL_UP_US, state: self.state }
                } else {
                    // Keep listening for the next Morse sequence
                    self.state = ListeningForOption {
                        parameter,
                        option_index: option_index + 1,
                    };
                    StepResult { pwm_us: PWM_CENTER_US, state: self.state }
                }
            }

            // ----------------------------------------------------------------
            // Already done – no-op
            // ----------------------------------------------------------------
            Done => StepResult { pwm_us: PWM_FULL_DOWN_US, state: Done },
        }
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn finish(&mut self) -> StepResult {
        self.state = ProgrammerState::Done;
        StepResult {
            pwm_us: PWM_FULL_DOWN_US,
            state:  ProgrammerState::Done,
        }
    }

    /// Returns `true` if the programmer should interact with this parameter.
    fn should_program(&self, param: Parameter) -> bool {
        match param {
            Parameter::CellTypeAndCount   => self.config.cell_type.is_some(),
            Parameter::ThrottleSetting    => self.config.throttle.is_some(),
            Parameter::BrakeSetting       => self.config.brake.is_some(),
            Parameter::DirectionAndCutoff => self.config.direction_cutoff.is_some(),
            Parameter::TimingMode         => self.config.timing.is_some(),
            Parameter::PwmFrequency       => self.config.pwm_freq.is_some(),
        }
    }

    /// Returns the 0-based index of the desired option for this parameter
    /// (i.e. how many Morse sequences to skip before selecting).
    fn desired_option_index(&self, param: Parameter) -> u8 {
        match param {
            Parameter::CellTypeAndCount => match self.config.cell_type {
                Some(CellType::NiMhNiCdAutoCount) => 0,
                Some(CellType::LiPo7S)            => 1,
                Some(CellType::LiPo6S)            => 2,
                Some(CellType::LiPo5S)            => 3,
                Some(CellType::LiPo4S)            => 4,
                Some(CellType::LiPo3S)            => 5,
                Some(CellType::LiPo2S)            => 6,
                None                              => 0,
            },
            Parameter::ThrottleSetting => match self.config.throttle {
                Some(ThrottleSetting::AutoThrottleRange) => 0,
                Some(ThrottleSetting::Fixed1msTo1_8ms)   => 1,
                Some(ThrottleSetting::HardStart)         => 2,
                Some(ThrottleSetting::SoftStart)         => 3,
                None                                     => 0,
            },
            Parameter::BrakeSetting => match self.config.brake {
                Some(BrakeSetting::NoBrake)     => 0,
                Some(BrakeSetting::SoftBrake)   => 1,
                Some(BrakeSetting::MediumBrake) => 2,
                Some(BrakeSetting::HardBrake)   => 3,
                None                            => 0,
            },
            Parameter::DirectionAndCutoff => match self.config.direction_cutoff {
                Some(DirectionCutoff::ClockwiseRotation)        => 0,
                Some(DirectionCutoff::CounterclockwiseRotation) => 1,
                Some(DirectionCutoff::SoftCutoff)               => 2,
                Some(DirectionCutoff::HardCutoff)               => 3,
                None                                            => 0,
            },
            Parameter::TimingMode => match self.config.timing {
                Some(TimingMode::Deg1For2To4PoleInrunner)           => 0,
                Some(TimingMode::Deg7For6To8Pole)                   => 1,
                Some(TimingMode::Deg15For10To14PoleOutrunner)        => 2,
                Some(TimingMode::Deg30For10To14PoleHighRpmOutrunner) => 3,
                None                                                  => 0,
            },
            Parameter::PwmFrequency => match self.config.pwm_freq {
                Some(PwmFrequency::Khz8)  => 0,
                Some(PwmFrequency::Khz16) => 1,
                None                      => 0,
            },
        }
    }

    /// Current state (read-only peek).
    pub const fn state(&self) -> ProgrammerState {
        self.state
    }

    /// The parameter currently being programmed, if any.
    pub const fn current_param(&self) -> Option<Parameter> {
        self.current_param
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::*;

    fn drive_to_done(prog: &mut EscProgrammer, max_steps: usize) -> bool {
        for _ in 0..max_steps {
            let r = prog.advance();
            if r.state == ProgrammerState::Done {
                return true;
            }
        }
        false
    }

    #[test]
    fn start_returns_full_up() {
        let mut p = EscProgrammer::new(EscConfig::empty());
        let r = p.start();
        assert_eq!(r.pwm_us, PWM_FULL_UP_US);
        assert_eq!(r.state, ProgrammerState::WaitingForInitTone);
    }

    #[test]
    fn empty_config_finishes_quickly() {
        let mut p = EscProgrammer::new(EscConfig::empty());
        p.start();
        assert!(drive_to_done(&mut p, 20));
    }

    #[test]
    fn full_defaults_config_programs_all_params() {
        let mut p = EscProgrammer::new(EscConfig::defaults());
        p.start();
        assert!(drive_to_done(&mut p, 100));
    }

    #[test]
    fn lipo3s_no_brake_config() {
        let mut cfg = EscConfig::empty();
        cfg.cell_type = Some(CellType::LiPo3S);
        cfg.brake     = Some(BrakeSetting::NoBrake);

        let mut p = EscProgrammer::new(cfg);
        p.start();
        assert!(drive_to_done(&mut p, 60));
    }

    #[test]
    fn done_is_idempotent() {
        let mut p = EscProgrammer::new(EscConfig::empty());
        p.start();
        drive_to_done(&mut p, 50);
        // Calling advance after Done should stay Done at full-down
        let r = p.advance();
        assert_eq!(r.state, ProgrammerState::Done);
        assert_eq!(r.pwm_us, PWM_FULL_DOWN_US);
    }

    #[test]
    fn cell_type_cutoff_voltages() {
        assert_eq!(CellType::LiPo4S.cutoff_mv(), 12_000);
        assert_eq!(CellType::LiPo2S.cutoff_mv(),  8_000);
        assert_eq!(CellType::NiMhNiCdAutoCount.cutoff_mv(), 0);
    }

    #[test]
    fn pwm_frequency_hz() {
        assert_eq!(PwmFrequency::Khz8.hz(),  8_000);
        assert_eq!(PwmFrequency::Khz16.hz(), 16_000);
    }

    #[test]
    fn timing_degrees() {
        assert_eq!(TimingMode::Deg1For2To4PoleInrunner.degrees(), 1);
        assert_eq!(TimingMode::Deg30For10To14PoleHighRpmOutrunner.degrees(), 30);
    }

    #[test]
    fn throttle_position_to_us() {
        use crate::pwm::ThrottlePosition;
        assert_eq!(ThrottlePosition::FullDown.to_us(), 1000);
        assert_eq!(ThrottlePosition::Center.to_us(),   1500);
        assert_eq!(ThrottlePosition::FullUp.to_us(),   2000);
    }
}
