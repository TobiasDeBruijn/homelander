use serde::{Serialize, Deserialize};
use crate::{CombinedDeviceError};
use crate::traits::Language;

/// This trait is used for devices that can be "found". This includes phones,
/// robots (including vacuums and mowers), drones, and tag-specific products that attach to other devices.
pub trait Locator {
    /// Locate the target device by generating a local alert.
    /// - `silence` For use on devices that make an audible response for local alerts. If set to true, the device should silence any in-progress alarms.
    /// - `lang` Current language of query or display, for return of localized location strings if needed.
    fn locate(&mut self, silence: Option<bool>, lang: Option<Language>) -> Result<(), CombinedDeviceError>;
}