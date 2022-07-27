use serde::{Serialize, Deserialize};
use crate::CombinedDeviceError;


/// The basic on and off functionality for any device that has binary on and off, including plugs and switches as well as many future devices.
///
/// ## See also
/// <https://developers.google.com/assistant/smarthome/traits/onoff>
pub trait OnOff {
    /// Indicates if the device can only controlled through commands, and cannot be queried for state information.
    /// Default: false
    fn is_command_only(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device can only be queried for state information, and cannot be controlled through commands.
    /// Default: false
    fn is_query_only(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Whether a device with an on/off switch is on or off.
    fn is_on(&self) -> Result<bool, CombinedDeviceError>;

    /// Turn the device on or off.
    /// - `on` Whether to turn the device on or off.
    fn set_on(&mut self, on: bool) -> Result<(), CombinedDeviceError>;
}