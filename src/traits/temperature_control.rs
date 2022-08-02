use crate::traits::{TemperatureRange, TemperatureUnit};
use crate::CombinedDeviceError;

/// Trait for devices (other than thermostats) that support controlling temperature,
/// either within or around the device. This includes devices such as ovens and refrigerators.
///
/// This differs from the [TemperatureSetting](crate::traits::temperature_setting::TemperatureSetting) trait,
/// which is specifically for thermostat-style controls.
/// The [TemperatureSetting](crate::traits::temperature_setting::TemperatureSetting) trait represents ambient (room/outdoor) temperature
/// and should not be used for controlling the temperature of a specific device.
/// In order to control the temperature of a specific device,
/// you must use the [TemperatureControl]) trait.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/temperaturecontrol>
pub trait TemperatureControl {
    /// Supported temperature range of the device.
    fn get_temperature_range(&self) -> Result<TemperatureRange, CombinedDeviceError>;

    /// Specifies the relative temperature step. This is the minimum adjustment interval the device supports.
    /// If unspecified, relative steps are calculated as a percentage of temperatureRange.
    fn get_temperature_step_celsius(&self) -> Result<Option<f32>, CombinedDeviceError> {
        Ok(None)
    }

    /// Temperature unit used in responses to the user.
    fn get_temperature_unit_for_ux(&self) -> Result<TemperatureUnit, CombinedDeviceError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_temperature_control(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Required if the device supports query-only execution. This attribute indicates if the device can only be queried for state information, and cannot be controlled.
    /// Default: false
    fn is_query_only_temperature_control(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// The current temperature setpoint, in degrees Celsius. Must fall within temperatureRange. Required if queryOnlyTemperatureControl set to false
    fn get_temperature_setpoint_celsius(&self) -> Result<f32, CombinedDeviceError>;

    /// The currently observed temperature, in degrees Celsius. Must fall within temperatureRange.
    fn get_temperatuer_ambient_celsius(&self) -> Result<f32, CombinedDeviceError>;

    /// Set the temperature to a specific value.
    /// `temperature` The temperature to set, in degrees Celsius. Must fall within temperatureRange.
    fn set_temperature(&mut self, temperature: f32) -> Result<(), CombinedDeviceError>;
}
