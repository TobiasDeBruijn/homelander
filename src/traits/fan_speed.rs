use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::{CombinedDeviceError, Language};

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    /// The device is already set to the maximum speed.
    #[error("MaxSpeedReached")]
    MaxSpeedReached,
    /// The device is already set to the minimum speed.
    #[error("MinSpeedReached")]
    MinSpeedReached,
}

#[derive(Debug, Error)]
pub enum FanSpeedError {
    #[error("{0}")]
    Device(DeviceError),
    #[error("{0}")]
    Other(CombinedDeviceError),
}

/// Speed settings supported by the device.
pub struct AvailableFanSpeeds {
    /// If set to true, additional grammar for increase or decrease logic will apply, in the order (increasing) of the speeds array.
    speeds: Vec<FanSpeedItem>,
    /// If set to true, additional grammar for increase or decrease logic will apply, in the order (increasing) of the speeds array.
    ordered: bool,
}

/// Speed setting.
#[derive(Debug, Serialize)]
pub struct FanSpeedItem {
    /// Internal name of the speed setting. This can be non-user-friendly, and will be shared across all languages.
    speed_name: String,
    /// Synonyms for the speed setting in each supported languages.
    speed_values: Vec<FanSpeedValue>
}

/// Synonym for the speed setting in a given language.
#[derive(Debug, Serialize)]
pub struct FanSpeedValue {
    /// Synonyms for the speed setting, should include both singular and plural forms, if applicable.
    /// The first synonym in the list will be considered the canonical name of the speed setting.
    speed_synonym: Vec<String>,
    /// Language code
    lang: Language
}

/// This trait belongs to devices that support setting the speed of a fan (that is, blowing air from the device at various levels,
/// which may be part of an air conditioning or heating unit, or in a car), with settings such as low, medium, and high.
pub trait FanSpeed {
    /// If set to true, this device supports blowing the fan in both directions and can accept the command to reverse fan direction.
    /// Default: false
    fn is_reversable(&self) -> Result<Option<bool>, FanSpeedError> {
        Ok(None)
    }

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_fan_speed(&self) -> Result<Option<bool>, FanSpeedError> {
        Ok(None)
    }

    /// Speed settings supported by the device.
    ///
    /// Note: Either this function must return [Some] or [Self::is_support_fan_speed_percent] must return [Some], or both.
    fn get_available_fan_speeds(&self) -> Result<Option<AvailableFanSpeeds>, FanSpeedError>;

    /// If set to true, this device will accept commands for adjusting the speed using a percentage from 0.0 to 100.0.
    ///
    /// Note: Either this function must return [Some], or [Self::get_available_fan_speeds] must return [Some], or both.
    fn is_support_fan_speed_percent(&self) -> Result<Option<bool>, FanSpeedError>;

    /// This represents the internal name of the current speed setting from the availableFanSpeeds attribute.
    ///
    /// If [Self::get_available_fan_speeds] returns [Some], so must this function
    fn get_current_fan_speed_setting(&self) -> Result<Option<String>, FanSpeedError>;

    /// Indicates the current fan speed by percentage. Required if supportsFanSpeedPercent attribute is set to true
    ///
    /// If [Self::is_support_fan_speed_percent] returns [Some], so must this function
    fn get_current_fan_speed_percent(&self) -> Result<Option<i32>, FanSpeedError>;

    /// Set speed.
    ///
    /// If [Self::get_available_fan_speeds] returns [Some], this function will be called to set the speed
    fn set_fan_speed_setting(&self, name: String) -> Result<(), FanSpeedError>;

    /// Set speed.
    ///
    /// If [Self::is_support_fan_speed_percent] returns [Some], this function will be called to set the speed
    fn set_fan_speed_percent(&self, percent: f32) -> Result<(), FanSpeedError>;

    /// This value indicates the relative amount of the speed change. The absolute value indicates the scaled amount while the numerical sign indicates the direction of the change.
    ///
    /// Only called if [Self::is_command_only_fan_speed] returns `Some(true)`
    fn set_fan_speed_relative_weight(&self, weight: i32) -> Result<(), FanSpeedError>;

    /// This value represents the percentage of speed to change.
    ///
    /// Only called if [Self::is_command_only_fan_speed] returns `Some(true)`
    fn set_fan_speed_relative_percent(&self, percent: f32) -> Result<(), FanSpeedError>;

    /// Reverse fan direction.
    ///
    /// Only called if [Self::is_reversable] returns `Some(true)`
    fn set_fan_reverse(&self) -> Result<(), FanSpeedError>;
}