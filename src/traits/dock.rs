use crate::CombinedDeviceError;

/// This trait is designed for self-mobile devices that can be commanded to return for charging.
pub trait Dock {
    /// Whether the device is connected to the docking station or not.
    fn is_docked(&self) -> Result<bool, CombinedDeviceError>;
    /// Dock the device.
    fn dock(&mut self) -> Result<(), CombinedDeviceError>;
}
