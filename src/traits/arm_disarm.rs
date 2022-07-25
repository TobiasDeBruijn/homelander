use crate::traits::{DeviceError, DeviceException, Language};
use thiserror::Error;
use std::error::Error;

/// Security level.
pub struct ArmLevel {
    /// The internal name of the security level that is used in commands and states. This name can be non-user-friendly and is shared across all languages.
    level_name: String,
    level_values: Vec<LevelValue>,
}

/// Contains `level_synonym` and `lang`.
pub struct LevelValue {
    /// User-friendly names for the level in each supported language. The first item is treated as the canonical name.
    level_synonym: Vec<String>,
    /// Language code for the level synonyms.
    lang: Language,
}

/// An error occurred arming or disarming the device.
#[derive(Debug, Error)]
pub enum ArmDisarmError<T: Error> {
    AlreadyInState,
    DeviceTampered,
    PassphraseIncorrect,
    PinIncorrect,
    SecurityRestrictions,
    TooManyFailedAttempts,
    UserCancelled,
    DeviceError(DeviceError),
    DeviceException(DeviceException),
    Other(#[from] T)
}

/// This trait supports arming and disarming as used in, for example, security systems.
pub trait ArmDisarm {
    type Error: Error;

    /// Describes the supported security levels of the device. If this attribute is not reported, the device only supports one level.
    fn get_available_arm_levels(&self) -> Result<Option<Vec<ArmLevel>>, ArmDisarmError<Self::Error>>;

    /// If set to true, additional grammar for increase/decrease logic applies,
    /// in the order of the levels array. For example, "Hey Google, increase my security level by 1",
    /// results in the Assistant determining the current security level and then increasing that security level by one.
    /// If this value is set to false, additional grammar for increase/decrease logic is not supported.
    fn is_ordered(&self) -> Result<bool, ArmDisarmError<Self::Error>>;

    /// Indicates if the device is currently armed.
    fn is_armed(&self) -> Result<bool, ArmDisarmError<Self::Error>>;

    /// If multiple security levels exist, indicates the name of the current security level.
    fn current_arm_level(&self) -> Result<String, ArmDisarmError<Self::Error>>;

    /// Indicates the time, in seconds, the user has to leave before `currentArmLevel` takes effect.
    fn exit_allowance(&self) -> Result<i32, ArmDisarmError<Self::Error>>;

    /// Arm or disarm the device. `arm` Is true when the intent is to arm the device, false to disarm
    fn arm(&mut self, arm: bool) -> Result<(), ArmDisarmError<Self::Error>>;

    /// Cancels the arming of the device
    fn cancel_arm(&mut self) -> Result<(), ArmDisarmError<Self::Error>>;

    /// Arm the device. `level` is the `level_name` to arm to.
    fn arm_with_level(&mut self, level: String) -> Result<(), ArmDisarmError<Self::Error>>;
}