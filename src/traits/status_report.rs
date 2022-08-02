use crate::CombinedDeviceError;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentStatusReport {
    /// True if the error or current status is blocking further commands executions.
    pub blocking: bool,
    /// The ID of the target device.
    pub device_target: String,
    /// Specifies the priority of this status. The lower the value, the higher the priority,
    /// with the highest priority being 0.
    /// Google reports the error or exception status from the highest to lowest priority.
    /// Depending on the surface, Google may report only high priority errors or exceptions.
    pub priority: i32,
    /// The current status of the device. See the full list of [errors and exceptions](https://developers.google.com/assistant/smarthome/reference/errors-exceptions?).
    pub status_code: Option<String>,
}

/// This trait reports the current status or state of a specific device or a connected group of devices.
///
/// A specific device can report its current status as well as the status of any related devices in a group.
/// For example, the target device could be a security system with the related devices representing individual sensors.
/// StatusReport serves as an aggregation for reporting collective status,
/// but does not replace individual addressing. Any
/// device that can be accessed by Google Assistant should be reported as a separate device in the SYNC response.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/statusreport>
pub trait StatusReport {
    /// Current error or exception statuses of the device and any related device IDs.
    fn get_current_status_report(&self) -> Result<Vec<CurrentStatusReport>, CombinedDeviceError>;
}
