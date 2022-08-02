use crate::CombinedDeviceError;

/// Starting and stopping a device serves a similar function to turning it on and off.
/// Devices that inherit this trait function differently when turned on and when started.
/// Certain washing machines, for instance, are able to be turned on and have their settings modified before actually starting operation.
///
/// Unlike devices that simply have an on and off state,
/// some devices that can start and stop are also able to pause while performing an operation.
/// Devices that can pause will cease operation, but upon resume will continue in the same state as when they were paused.
/// Unpausing differs from starting/restarting as regardless of the current state of the device,
/// this will begin operation from the beginning.
///
/// Some devices may support running in certain zones. For example,
/// a sprinkler may have various watering zones and support the ability to water particular zones separately,
/// while a vacuum may support cleaning specific rooms.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/startstop>
pub trait StartStop {
    /// Indicates whether the device can be paused during operation.
    /// Default: false
    fn is_pausable(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates supported zone names. Strings should be localized as set by the user.
    /// This list is not exclusive; users can report any names they want.
    fn get_available_zones(&self) -> Result<Option<Vec<String>>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device is currently in operation.
    fn is_running(&self) -> Result<bool, CombinedDeviceError>;

    /// Indicates if the device is explicitly paused. If this value is true, it implies isRunning is false but can be resumed.
    fn is_paused(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates zones in which the device is currently running, from list of availableZones.
    fn get_active_zones(&self) -> Result<Option<Vec<String>>, CombinedDeviceError> {
        Ok(None)
    }

    /// Start or stop the device.
    /// `start` True to start device operation, false to stop.
    /// `zones` The zone or zones in which to start or stop
    fn start_stop(&mut self, start: bool, zones: Option<Vec<String>>) -> Result<(), CombinedDeviceError>;

    /// Pause or unpause device operation.
    /// `pause` True to pause, false to unpause.
    /// Only called if [Self::is_pausable] returns `true`
    fn pause_unpause(&mut self, pause: bool) -> Result<(), CombinedDeviceError>;
}
