use crate::CombinedDeviceError;

/// This trait belongs to devices that support rebooting, such as routers. The device needs to support rebooting as a single action.
pub trait Reboot {
    /// Reboots the device.
    fn reboot(&mut self) -> Result<(), CombinedDeviceError>;
}