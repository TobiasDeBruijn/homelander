use crate::CombinedDeviceError;

/// This trait belongs to devices that support software updates such as a router.
/// Optionally, these devices may report the time of the last successful update.
pub trait SoftwareUpdate {
    /// The Unix timestamp (number of seconds since the Unix Epoch) of the last successful software update.
    /// The Unix Epoch is 00:00:00, 1 January 1970, UTC.
    fn get_last_software_update_unix_timestamp_sec(&self) -> Result<i64, CombinedDeviceError>;

    /// Update the device.
    fn perform_update(&mut self) -> Result<(), CombinedDeviceError>;
}
