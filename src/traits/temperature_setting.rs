use crate::traits::{TemperatureRange, TemperatureUnit};
use crate::CombinedDeviceError;
use serde::{Deserialize, Serialize};

/// Name of the supported mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThermostatMode {
    None,
    /// Heating/Cooling activity disabled
    Off,
    /// Device supports heating
    Heat,
    /// Device supports cooling
    Cool,
    /// Restore the previous mode of the device.
    /// The on mode does not appear in the mode selection screen because the on mode,
    /// by design, is used to restore the previous mode of the device.
    On,
    /// Maintaining heating/cooling target as a range
    Heatcool,
    /// Automatic mode with temperature set by a schedule or learned behavior
    Auto,
    /// Fan running without heat/cool activity
    FanOnly,
    /// Purifying mode
    Purifier,
    /// Energy-saving mode
    Eco,
    /// Dry mode
    Dry,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum QueryThermostatMode {
    Fixed(QueryThermostatModeFixed),
    Range(QueryThermostatModeRange),
}

/// States for fixed set point.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryThermostatModeFixed {
    /// Current mode of the device, from the list of availableThermostatModes.
    pub thermostat_mode: ThermostatMode,
    /// Current observed temperature, in degrees Celsius.
    pub thermostat_temperature_ambient: f32,
    /// Current temperature set point (single target), in degrees Celsius.
    pub thermostat_temperature_setpoint: f32,
}

/// States for set point range.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryThermostatModeRange {
    /// Current mode of the device, from the list of availableThermostatModes.
    pub thermostat_mode: ThermostatMode,
    /// Current observed temperature, in degrees Celsius.
    pub thermostat_temperature_ambient: f32,
    /// Current high point if in heatcool mode, for a range.
    pub thermostat_temperature_setpoint_high: f32,
    /// Current low point if in heatcool mode, for a range.
    pub thermostat_temperature_setpoint_low: f32,
}

/// This trait covers handling both temperature point and modes.
pub trait TemperatureSetting {
    /// Describes the thermostat modes this device can support.
    fn get_available_thermostat_modes(&self) -> Result<Vec<ThermostatMode>, CombinedDeviceError>;

    /// Contains two float values that indicate the supported temperature range for this device (in degrees Celsius).
    fn get_thermostat_temperature_range(&self) -> Result<Option<TemperatureRange>, CombinedDeviceError> {
        Ok(None)
    }

    /// The display unit the device is set to by default. Google reports temperature information using the display unit.
    fn get_thermostat_temperature_unit(&self) -> Result<TemperatureUnit, CombinedDeviceError>;

    /// Specifies the minimum offset between heat-cool setpoints in degrees Celsius, if heatcool mode is supported.
    /// Default: 2
    fn get_buffer_range_celsius(&self) -> Result<Option<f32>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_temperature_setting(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Required if the device supports query-only execution. This attribute indicates if the device can only be queried for state information, and cannot be controlled.
    /// Default: false
    fn is_query_only_temperature_setting(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Currently active mode of the device, from the list of availableThermostatModes. If no mode is currently active, set to none.
    fn get_active_thermostat_mode(&self) -> Result<ThermostatMode, CombinedDeviceError>;

    /// A timestamp representing the estimated time when the target temperature will be reached.
    fn get_target_temp_reached_estimate_unix_timestamp_sec(&self) -> Result<Option<i64>, CombinedDeviceError> {
        Ok(None)
    }

    /// Represents the relative level of the ambient humidity, if supported by the device.
    fn get_thermostat_humidity_ambient(&self) -> Result<Option<f32>, CombinedDeviceError> {
        Ok(None)
    }

    /// Get the fixed set point, or the set point range
    fn get_thermostat_mode(&self) -> Result<QueryThermostatMode, CombinedDeviceError>;

    /// Set the target temperature for a thermostat device.
    /// `setpoint` Target temperature setpoint. Supports up to one decimal place.
    fn set_temperature_setpoint(&mut self, setpoint: f32) -> Result<(), CombinedDeviceError>;

    /// Set a target temperature range for a thermostat device.
    /// Requires the device to support [ThermostatMode::Heatcool].
    /// - `setpoint_high` High target setpoint for the range.
    /// - `setpoint_low` Low target setpoint for the range.
    fn set_temperature_set_range(&mut self, setpoint_high: f32, setpoint_low: f32) -> Result<(), CombinedDeviceError>;

    /// Set the target operating mode for a thermostat device.
    /// - `mode` Target mode, from the list of [Self::get_available_thermostat_modes].
    fn set_thermostat_mode(&mut self, mode: ThermostatMode) -> Result<(), CombinedDeviceError>;

    /// Adjust the target temperature relative to the current state.
    /// Only called if [Self::is_command_only_temperature_setting] returns `true`
    /// - `relative_degrees` The exact number of degrees for the temperature to change (for example, "Turn down 5 degrees").
    fn set_temperature_relative_degree(&mut self, relative_degrees: f32) -> Result<(), CombinedDeviceError>;

    /// Adjust the target temperature relative to the current state.
    /// Only called if [Self::is_command_only_temperature_setting] returns `true`
    /// - `weight` This indicates the amount of ambiguous temperature change from a small amount ("Turn down a little"), to a large amount ("A lot warmer").
    fn set_temperature_relative_weight(&mut self, weight: f32) -> Result<(), CombinedDeviceError>;
}
