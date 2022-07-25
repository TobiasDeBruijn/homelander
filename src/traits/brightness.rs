use crate::traits::CombinedDeviceError;
use std::error::Error;
use thiserror::Error;

/// Absolute brightness setting is in a normalized range from 0 to 100
/// (individual lights may not support every point in the range based on their LED configuration).
pub trait Brightness {

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    fn is_command_only_brightness(&self) -> Result<bool, CombinedDeviceError>;

    /// Current brightness level of the device.
    fn get_brightness(&self) -> Result<i32, CombinedDeviceError>;

    /// New absolute brightness percentage.
    fn set_brightness_absolute(&mut self, brightness: i32) -> Result<(), CombinedDeviceError>;

    /// The exact percentage of brightness to change.
    fn set_brightness_relative_percent(&mut self, brightness: i32) -> Result<(), CombinedDeviceError>;

    /// This indicates the ambiguous amount of the brightness change. From small amount to large amount, this param will be scaled to integer 0 to 5, with the sign to indicate direction.
    fn set_brightness_relative_weight(&mut self, weight: i32) -> Result<(), CombinedDeviceError>;
}