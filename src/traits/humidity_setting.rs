use serde::{Serialize, Deserialize};
use crate::CombinedDeviceError;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HumiditySetPointRange {
    /// Represents the minimum humdity level as a percentage.
    /// Default: 0
    min_percent: Option<i32>,
    /// Represents the maximum humidity level as a percentage.
    /// Default: 100
    max_percent: Option<i32>,
}

/// This trait belongs to devices that support humidity settings such as humidifiers and dehumidifiers.
pub trait HumiditySetting {
    /// Contains the minimum and maximum humidity levels as percentages.
    fn get_humidity_set_point_range_minmax(&self) -> Result<Option<HumiditySetPointRange>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_humidity_settings(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Required if the device supports query-only execution. This attribute indicates if the device can only be queried for state information, and cannot be controlled.
    /// Default: true
    fn is_query_only_humidity_setting(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates the current target humidity percentage of the device. Must fall within humiditySetpointRange.
    fn get_current_humidity_set_point_range(&self) -> Result<i32, CombinedDeviceError>;

    /// Indicates the current ambient humidity reading of the device as a percentage.
    fn get_current_humidity_ambient_percent(&self) -> Result<i32, CombinedDeviceError>;

    /// Set the humidity level to an absolute value.
    /// - `humidity` Setpoint humidity percentage. Must fall within humiditySetpointRange.
    fn set_humidity(&mut self, humidity: i32) -> Result<(), CombinedDeviceError>;

    /// Adjust the humidity level relative to the current value.
    /// - `percent` The percentage value to adjust the humidity level.
    fn set_humidity_relative_percent(&mut self, percent: i32) -> Result<(), CombinedDeviceError>;

    /// Adjust the humidity level relative to the current value.
    /// - `weight` Indicates the amount of ambiguous humidity change from a small amount ("a little") to a large amount ("a lot").
    fn set_humidity_relative_weight(&mut self, weight: i32) -> Result<(), CombinedDeviceError>;
}