use crate::CombinedDeviceError;
use serde::Serialize;

/// Each object represents sensor state capabilities supported by this specific device.
/// Each sensor must have at least a descriptive or numeric capability.
/// Sensors can also report both, in which case the numeric value will be preferred.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SupportedSensorState {
    /// Supported sensor type.
    /// Refer to [the Google docs](https://developers.google.com/assistant/smarthome/traits/sensorstate#supported-sensors) for supported values.
    pub name: String,
    /// A description of the sensor's capabilities.
    pub descriptive_capabilities: Option<DescriptiveCapabilities>,
    /// Describes the possible numerical values that the sensor can report.
    pub numeric_capabilities: Option<NumericCapabilities>,
}

/// A description of the sensor's capabilities.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptiveCapabilities {
    /// List of the available states for the device. The "unknown" state is implicitly supported when the sensor does not return a value.
    /// Requires at least 1 item.
    /// Refer to [the Google docs](https://developers.google.com/assistant/smarthome/traits/sensorstate#supported-sensors) for supported values.
    pub available_states: Vec<String>,
}

/// Describes the possible numerical values that the sensor can report.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericCapabilities {
    /// Supported numerical unit.
    /// Refer to [the Google docs](https://developers.google.com/assistant/smarthome/traits/sensorstate#supported-sensors) for supported values.
    pub raw_value_unit: String,
}

/// Current sensor state.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentSensorState {
    /// Sensor state name. Matches a value from sensorStatesSupported.
    pub name: String,
    /// Current descriptive state value. Matches a value from sensorStatesSupported.
    pub current_sensor_state: Option<String>,
    /// Current numeric sensor value.
    pub raw_value: Option<f32>,
}

/// This trait covers both quantitative measurement (for example,
/// air quality index or smoke level) and qualitative state (for example, whether the air quality is healthy
/// or whether the smoke level is low or high).
///
/// ## See also
/// <https://developers.google.com/assistant/smarthome/traits/sensorstate>
pub trait SensorState {
    /// Each object represents sensor state capabilities supported by this specific device.
    /// Each sensor must have at least a descriptive or numeric capability.
    /// Sensors can also report both, in which case the numeric value will be preferred.
    fn get_supported_sensor_states(&self) -> Result<Vec<SupportedSensorState>, CombinedDeviceError>;

    /// List of current sensor states.
    fn get_current_sensor_states(&self) -> Result<Vec<CurrentSensorState>, CombinedDeviceError>;
}
