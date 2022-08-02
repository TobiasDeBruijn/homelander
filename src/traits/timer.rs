use crate::CombinedDeviceError;

/// For instance, a smart sprinkler controller or smart light switch may have a built-in timer.
/// This trait can be used to control a built-in timer on devices,
/// such as starting a new timer as well as pausing and canceling a running timer,
/// and asking how much time is remaining.
pub trait Timer {
    /// Indicates the longest timer setting available on the device, measured in seconds.
    fn get_max_timer_limit_sec(&self) -> Result<i32, CombinedDeviceError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_timer(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Current time remaining in seconds, `-1`, or `[0, maxTimerLimitSec]`. Set to `-1` or [None] to indicate no timer is running.
    fn get_timer_remaining_sec(&self) -> Result<Option<i32>, CombinedDeviceError>;

    /// True if a active timer exists but is currently paused.
    fn is_timer_paused(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Start a new timer.
    /// - `seconds` Duration of the timer in seconds; must be within `[1, maxTimerLimitSec]`.
    fn start_timer(&mut self, seconds: i32) -> Result<(), CombinedDeviceError>;

    /// Adjust the timer duration.
    /// - `seconds` Positive or negative adjustment of the timer in seconds; must be within `[-maxTimerLimitSec, maxTimerLimitSec]`.
    fn adjust_timer(&mut self, seconds: i32) -> Result<(), CombinedDeviceError>;

    /// Pause timer.
    fn pause_timer(&mut self) -> Result<(), CombinedDeviceError>;

    /// Resume timer.
    fn resume_timer(&mut self) -> Result<(), CombinedDeviceError>;

    /// Cancel the timer.
    fn cancel_timer(&mut self) -> Result<(), CombinedDeviceError>;
}
