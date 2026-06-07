//! PWM microsecond values and throttle-position abstraction.
//!
//! Standard RC servo / ESC signal: 1000 µs (full down) … 2000 µs (full up),
//! centre at 1500 µs.  These three positions are the only ones the programming
//! protocol requires.

/// Full-down / lowest throttle position (µs).
/// Used to exit programming mode and save all options.
pub const PWM_FULL_DOWN_US: u16 = 1000;

/// Centre / neutral throttle position (µs).
/// Used to enter a sub-option menu for the currently announced parameter.
pub const PWM_CENTER_US: u16 = 1500;

/// Full-up / highest throttle position (µs).
/// Used to enter programming mode at power-on, and to select/save an option.
pub const PWM_FULL_UP_US: u16 = 2000;

/// The three throttle positions that the ESC programming protocol uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThrottlePosition {
    /// 1000 µs – lowest stick position.  Saves all settings and exits.
    FullDown,
    /// 1500 µs – centre stick position.  Enters the sub-option menu for the
    /// currently announced parameter.
    Center,
    /// 2000 µs – highest stick position.  Selects the currently announced
    /// sub-option, or enters programming mode at power-on.
    FullUp,
}

impl ThrottlePosition {
    /// Convert this position to its PWM pulse-width in microseconds.
    #[inline]
    pub const fn to_us(self) -> u16 {
        match self {
            ThrottlePosition::FullDown => PWM_FULL_DOWN_US,
            ThrottlePosition::Center => PWM_CENTER_US,
            ThrottlePosition::FullUp => PWM_FULL_UP_US,
        }
    }
}
