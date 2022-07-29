use crate::CombinedDeviceError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, PartialEq, Error, Serialize)]
#[serde(rename = "camelCase")]
pub enum DeviceError {
    #[error("LockedState")]
    LockedState,
    #[error("DeviceJammingDetected")]
    DeviceJammingDetected,
}

#[derive(Debug, PartialEq, Error)]
pub enum OpenCloseError {
    #[error("{0}")]
    Device(#[from] DeviceError),
    #[error("{0}")]
    OpenClose(#[from] CombinedDeviceError),
}

/// Direction in which the device is opened.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum OpenDirection {
    Up,
    Down,
    Left,
    Right,
    In,
    Out,
}

/// Current state for the given open direction.
#[derive(Debug, PartialEq, Serialize)]
pub struct OpenState {
    /// Indicates the percentage that a device is opened, where 0 is closed and 100 is fully open.
    open_percent: f32,
    /// Direction in which the device is opened.
    open_direction: OpenDirection,
}

/// This trait belongs to devices that support opening and closing,
/// and in some cases opening and closing partially or potentially in more
/// than one direction. For example, some blinds may open either to the left or to the right.
/// In some cases, opening certain devices may be a security sensitive action which can
/// require two-factor authentication authentication. See [Two-factor authentication](https://developers.google.com/assistant/smarthome/two-factor-authentication).
pub trait OpenClose {
    /// When set to true, this indicates that the device must either be fully open or fully closed (that is, it does not support values between 0% and 100%).
    /// Default: false
    fn is_discrete_only_open_close(&self) -> Result<Option<bool>, OpenCloseError> {
        Ok(None)
    }

    /// List of supported directions in which the device can open or close. Include this attribute if the device supports opening and closing in more than one direction.
    /// Default: None
    fn get_supported_opening_directions(&self) -> Result<Option<Vec<OpenDirection>>, OpenCloseError> {
        Ok(None)
    }

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_open_close(&self) -> Result<Option<bool>, OpenCloseError> {
        Ok(None)
    }

    /// Indicates if the device can only be queried for state information and cannot be controlled. Sensors that can only report open state should set this field to true.
    /// Default: false
    fn is_query_only_open_close(&self) -> Result<Option<bool>, OpenCloseError> {
        Ok(None)
    }

    /// Indicates the percentage that a device is opened, where 0 is closed and 100 is fully open.
    /// You should return [None] unless [Self::get_supported_opening_directions] returns [Some] with a [Vec] which is not empty
    fn get_open_percent(&self) -> Result<Option<f32>, OpenCloseError>;

    /// List of states for each supported open direction.
    /// You should return [Some] only if [Self::get_supported_opening_directions] returns [Some] with a [Vec] which is not empty
    fn get_open_state(&self) -> Result<Option<OpenState>, OpenCloseError>;

    /// Set the open-close state of the device.
    /// - `percent` Indicates the percentage that a device is opened, where 0 is closed and 100 is fully open.
    /// - `direction` Direction in which to open. Only present if device supports multiple directions, as indicated by the openDirection attribute, and a direction is specified by the user.
    fn set_open(&mut self, percent: f32, direction: Option<OpenDirection>) -> Result<(), OpenCloseError>;

    /// Adjust the open-close state of the device relative to the current state. This command is only available if commandOnlyOpenClose is set to true.
    /// - `relative_percent` The exact percentage to change open-close state. Ambigous relative commands will be converted to an exact percentage parameter (for example, "Open the blinds a little more" vs "Open the blinds by 5%").
    /// - `direction` Direction in which to open. Only present if device supports multiple directions, as indicated by the openDirection attribute, and a direction is specified by the user.
    fn set_open_relative(&mut self, relative_percent: f32, direction: Option<OpenDirection>) -> Result<(), OpenCloseError>;
}
