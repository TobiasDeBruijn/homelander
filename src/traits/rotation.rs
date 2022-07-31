use serde::Serialize;
use crate::CombinedDeviceError;

/// Represent the range in degrees that a device can rotate.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RotationDegreeRange {
    /// Minimum rotation in degrees.
    pub rotation_degree_min: f32,
    /// Maximum rotation in degrees.
    pub rotation_degree_max: f32,
}

/// This trait belongs to devices that support rotation, such as blinds with rotatable slats.
pub trait Rotation {
    /// Set to true if the device allows rotation by degree.
    fn supports_degrees(&self) -> Result<bool, CombinedDeviceError>;

    /// Set to true if device allows rotation by percent
    fn supports_percent(&self) -> Result<bool, CombinedDeviceError>;

    /// Represent the range in degrees that a device can rotate.
    fn get_rotation_degree_range(&self) -> Result<RotationDegreeRange, CombinedDeviceError>;

    /// Set to true if the device allows continuous rotation. When given a relative query, the RotateAbsolute command will wrap around the supported rotation range.
    /// Default: false
    fn supports_continuous_rotation(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_rotation(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Current level within rotationDegreesRange that indicates how many degrees the device is rotated. This value should always be relative to the clockwise rotation.
    fn get_rotation_degrees(&self) -> Result<f32, CombinedDeviceError>;

    /// Current level that indicates what percent the device is currently rotated. 0.0 corresponds to closed and 100.0 to open.
    fn get_rotation_percent(&self) -> Result<f32, CombinedDeviceError>;

    /// An absolute value, in degrees, that specifies the final clockwise rotation of the device. Value must fall within rotationDegreesRange attribute.
    fn set_rotation_degrees(&mut self, degrees: f32) -> Result<(), CombinedDeviceError>;

    /// An absolute value, in percentage, that specifies the final rotation of the device.
    fn set_rotation_percent(&mut self, percent: f32) -> Result<(), CombinedDeviceError>;
}