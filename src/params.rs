//! All programmable parameters and their options, transcribed verbatim from
//! the ESC manual.
//!
//! Every enum variant is annotated with:
//! * The Morse-code beep sequence used to select it (`short + long` counts).
//! * Whether it is the factory default (`*`).

// ---------------------------------------------------------------------------
// Parameter 1 – Cell Type and Number of Cells
// (Music Tone + 1 Beep)
// ---------------------------------------------------------------------------

/// **Parameter 1** – Battery cell type and number of cells.
///
/// Announced by: Music Tone + **1 Beep**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CellType {
    /// `• —`  1 Short + 1 Long → NiMh/NiCD Auto Cell Count – 0.8 V/cell cutoff.
    NiMhNiCdAutoCount,
    /// `• — —`  1 Short + 2 Long → 7S Li-Po (25.9 V) – 21 V cutoff.
    LiPo7S,
    /// `• — — —`  1 Short + 3 Long → 6S Li-Po (22.2 V) – 18 V cutoff.
    LiPo6S,
    /// `• — — — —`  1 Short + 4 Long → 5S Li-Po (18.5 V) – 15 V cutoff.
    LiPo5S,
    /// `• — — — — —`  1 Short + 5 Long → 4S Li-Po (14.8 V) – 12 V cutoff  *(default)*.
    LiPo4S,
    /// `• — — — — — —`  1 Short + 6 Long → 3S Li-Po (11.1 V) – 9 V cutoff.
    LiPo3S,
    /// `• — — — — — — —`  1 Short + 7 Long → 2S Li-Po (7.4 V) – 8 V cutoff.
    LiPo2S,
}

impl CellType {
    /// Returns `true` if this option is the factory default.
    pub const fn is_default(self) -> bool {
        matches!(self, CellType::LiPo4S)
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            CellType::NiMhNiCdAutoCount => (1, 1),
            CellType::LiPo7S           => (1, 2),
            CellType::LiPo6S           => (1, 3),
            CellType::LiPo5S           => (1, 4),
            CellType::LiPo4S           => (1, 5),
            CellType::LiPo3S           => (1, 6),
            CellType::LiPo2S           => (1, 7),
        }
    }

    /// Cutoff voltage in millivolts (0 = auto / not applicable).
    pub const fn cutoff_mv(self) -> u32 {
        match self {
            CellType::NiMhNiCdAutoCount => 0,
            CellType::LiPo7S           => 21_000,
            CellType::LiPo6S           => 18_000,
            CellType::LiPo5S           => 15_000,
            CellType::LiPo4S           => 12_000,
            CellType::LiPo3S           =>  9_000,
            CellType::LiPo2S           =>  8_000,
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter 2 – Throttle Setting
// (Music Tone + 2 Beeps)
// ---------------------------------------------------------------------------

/// **Parameter 2** – Throttle range and start behaviour.
///
/// Announced by: Music Tone + **2 Beeps**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThrottleSetting {
    /// `•• —`  2 Short + 1 Long → Auto Throttle Range  *(default)*.
    AutoThrottleRange,
    /// `•• — —`  2 Short + 2 Long → 1 ms to 1.8 ms fixed range.
    Fixed1msTo1_8ms,
    /// `•• — — —`  2 Short + 3 Long → Hard Start  *(default for start type)*.
    HardStart,
    /// `•• — — — —`  2 Short + 4 Long → Soft Start.
    SoftStart,
}

impl ThrottleSetting {
    /// Returns `true` if this option is a factory default.
    pub const fn is_default(self) -> bool {
        matches!(self, ThrottleSetting::AutoThrottleRange | ThrottleSetting::HardStart)
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            ThrottleSetting::AutoThrottleRange => (2, 1),
            ThrottleSetting::Fixed1msTo1_8ms   => (2, 2),
            ThrottleSetting::HardStart         => (2, 3),
            ThrottleSetting::SoftStart         => (2, 4),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter 3 – Brake Setting (for normal aircraft)
// (Music Tone + 3 Beeps)
// ---------------------------------------------------------------------------

/// **Parameter 3** – Brake setting (normal aircraft).
///
/// Announced by: Music Tone + **3 Beeps**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BrakeSetting {
    /// `••• —`  3 Short + 1 Long → No Brake.
    NoBrake,
    /// `••• — —`  3 Short + 2 Long → Soft Brake  *(default)*.
    SoftBrake,
    /// `••• — — —`  3 Short + 3 Long → Medium Brake.
    MediumBrake,
    /// `••• — — — —`  3 Short + 4 Long → Hard Brake.
    HardBrake,
}

impl BrakeSetting {
    /// Returns `true` if this option is the factory default.
    pub const fn is_default(self) -> bool {
        matches!(self, BrakeSetting::SoftBrake)
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            BrakeSetting::NoBrake     => (3, 1),
            BrakeSetting::SoftBrake   => (3, 2),
            BrakeSetting::MediumBrake => (3, 3),
            BrakeSetting::HardBrake   => (3, 4),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter 4 – Direction and Cutoff Type
// (Music Tone + 4 Beeps)
// ---------------------------------------------------------------------------

/// **Parameter 4** – Motor rotation direction and low-voltage cutoff type.
///
/// Announced by: Music Tone + **4 Beeps**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum DirectionCutoff {
    /// `•••• —`  4 Short + 1 Long → Clockwise Rotation  *(default)*.
    ClockwiseRotation,
    /// `•••• — —`  4 Short + 2 Long → Counter-clockwise Rotation.
    CounterclockwiseRotation,
    /// `•••• — — —`  4 Short + 3 Long → Soft Cutoff.
    SoftCutoff,
    /// `•••• — — — —`  4 Short + 4 Long → Hard Cutoff  *(default)*.
    HardCutoff,
}

impl DirectionCutoff {
    /// Returns `true` if this option is a factory default.
    pub const fn is_default(self) -> bool {
        matches!(
            self,
            DirectionCutoff::ClockwiseRotation | DirectionCutoff::HardCutoff
        )
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            DirectionCutoff::ClockwiseRotation        => (4, 1),
            DirectionCutoff::CounterclockwiseRotation => (4, 2),
            DirectionCutoff::SoftCutoff               => (4, 3),
            DirectionCutoff::HardCutoff               => (4, 4),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter 5 – Timing Mode
// (Music Tone + 5 Beeps)
// ---------------------------------------------------------------------------

/// **Parameter 5** – Motor timing advance.
///
/// Announced by: Music Tone + **5 Beeps**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimingMode {
    /// `••••• —`  5 Short + 1 Long → 1° for 2–4 pole inrunner motors  *(default)*.
    Deg1For2To4PoleInrunner,
    /// `••••• — —`  5 Short + 2 Long → 7° for 6–8 pole motors.
    Deg7For6To8Pole,
    /// `••••• — — —`  5 Short + 3 Long → 15° for 10–14 pole outrunner motors.
    Deg15For10To14PoleOutrunner,
    /// `••••• — — — —`  5 Short + 4 Long → 30° for 10–14 pole high-RPM outrunner motors.
    Deg30For10To14PoleHighRpmOutrunner,
}

impl TimingMode {
    /// Returns `true` if this option is the factory default.
    pub const fn is_default(self) -> bool {
        matches!(self, TimingMode::Deg1For2To4PoleInrunner)
    }

    /// Timing advance in degrees.
    pub const fn degrees(self) -> u8 {
        match self {
            TimingMode::Deg1For2To4PoleInrunner           =>  1,
            TimingMode::Deg7For6To8Pole                   =>  7,
            TimingMode::Deg15For10To14PoleOutrunner        => 15,
            TimingMode::Deg30For10To14PoleHighRpmOutrunner => 30,
        }
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            TimingMode::Deg1For2To4PoleInrunner           => (5, 1),
            TimingMode::Deg7For6To8Pole                   => (5, 2),
            TimingMode::Deg15For10To14PoleOutrunner        => (5, 3),
            TimingMode::Deg30For10To14PoleHighRpmOutrunner => (5, 4),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter 6 – PWM Frequency
// (Music Tone + 6 Beeps)
// ---------------------------------------------------------------------------

/// **Parameter 6** – PWM switching frequency.
///
/// Announced by: Music Tone + **6 Beeps**.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PwmFrequency {
    /// `•••••• —`  6 Short + 1 Long → 8 kHz – for low RPM / low pole-count motors
    /// *(default)*.
    Khz8,
    /// `•••••• — —`  6 Short + 2 Long → 16 kHz – for most outrunner motors.
    Khz16,
}

impl PwmFrequency {
    /// Returns `true` if this option is the factory default.
    pub const fn is_default(self) -> bool {
        matches!(self, PwmFrequency::Khz8)
    }

    /// Frequency in Hz.
    pub const fn hz(self) -> u32 {
        match self {
            PwmFrequency::Khz8  =>  8_000,
            PwmFrequency::Khz16 => 16_000,
        }
    }

    /// Morse sequence: `(shorts, longs)`.
    pub const fn morse(self) -> (u8, u8) {
        match self {
            PwmFrequency::Khz8  => (6, 1),
            PwmFrequency::Khz16 => (6, 2),
        }
    }
}

// ---------------------------------------------------------------------------
// Top-level parameter index
// ---------------------------------------------------------------------------

/// Which of the six programmable parameters is currently being announced.
///
/// The ESC cycles through these in order, each announced by
/// Music Tone + N beeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Parameter {
    /// Music Tone + 1 Beep.
    CellTypeAndCount,
    /// Music Tone + 2 Beeps.
    ThrottleSetting,
    /// Music Tone + 3 Beeps.
    BrakeSetting,
    /// Music Tone + 4 Beeps.
    DirectionAndCutoff,
    /// Music Tone + 5 Beeps.
    TimingMode,
    /// Music Tone + 6 Beeps.
    PwmFrequency,
}

impl Parameter {
    /// Number of beeps following the music tone for this parameter.
    pub const fn beep_count(self) -> u8 {
        match self {
            Parameter::CellTypeAndCount  => 1,
            Parameter::ThrottleSetting   => 2,
            Parameter::BrakeSetting      => 3,
            Parameter::DirectionAndCutoff=> 4,
            Parameter::TimingMode        => 5,
            Parameter::PwmFrequency      => 6,
        }
    }

    /// Returns the next parameter in the cycle, or `None` if this is the last.
    pub const fn next(self) -> Option<Parameter> {
        match self {
            Parameter::CellTypeAndCount   => Some(Parameter::ThrottleSetting),
            Parameter::ThrottleSetting    => Some(Parameter::BrakeSetting),
            Parameter::BrakeSetting       => Some(Parameter::DirectionAndCutoff),
            Parameter::DirectionAndCutoff => Some(Parameter::TimingMode),
            Parameter::TimingMode         => Some(Parameter::PwmFrequency),
            Parameter::PwmFrequency       => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Composite configuration snapshot
// ---------------------------------------------------------------------------

/// A complete ESC configuration snapshot.
///
/// All fields are `Option<T>` so partial configurations can be expressed; call
/// [`EscConfig::defaults`] to get the factory defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscConfig {
    /// Battery cell type / count selection.
    pub cell_type: Option<CellType>,
    /// Throttle range and start type.
    pub throttle: Option<ThrottleSetting>,
    /// Brake strength.
    pub brake: Option<BrakeSetting>,
    /// Motor direction and cutoff type.
    pub direction_cutoff: Option<DirectionCutoff>,
    /// Timing advance.
    pub timing: Option<TimingMode>,
    /// PWM switching frequency.
    pub pwm_freq: Option<PwmFrequency>,
}

impl EscConfig {
    /// Factory default configuration as described in the manual.
    pub const fn defaults() -> Self {
        Self {
            cell_type:        Some(CellType::LiPo4S),
            throttle:         Some(ThrottleSetting::AutoThrottleRange),
            brake:            Some(BrakeSetting::SoftBrake),
            direction_cutoff: Some(DirectionCutoff::ClockwiseRotation),
            timing:           Some(TimingMode::Deg1For2To4PoleInrunner),
            pwm_freq:         Some(PwmFrequency::Khz8),
        }
    }

    /// Completely empty configuration (all `None`).
    pub const fn empty() -> Self {
        Self {
            cell_type:        None,
            throttle:         None,
            brake:            None,
            direction_cutoff: None,
            timing:           None,
            pwm_freq:         None,
        }
    }
}
