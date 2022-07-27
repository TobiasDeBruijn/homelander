use crate::traits::Language;
use thiserror::Error;
use serde::{Serialize, Deserialize};
use crate::CombinedDeviceError;

/// Security level.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmLevel {
    /// The internal name of the security level that is used in commands and states. This name can be non-user-friendly and is shared across all languages.
    level_name: String,
    level_values: Vec<LevelValue>,
}

/// Contains `level_synonym` and `lang`.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LevelValue {
    /// User-friendly names for the level in each supported language. The first item is treated as the canonical name.
    level_synonym: Vec<String>,
    /// Language code for the level synonyms.
    lang: Language,
}

/// An error occurred arming or disarming the device.
#[derive(Debug, Error)]
pub enum ArmDisarmError {
    #[error("alreadyInState")]
    AlreadyInState,
    #[error("deviceTampered")]
    DeviceTampered,
    #[error("passphraseIncorrect")]
    PassphraseIncorrect,
    #[error("pinIncorrect")]
    PinIncorrect,
    #[error("securityRestrictions")]
    SecurityRestrictions,
    #[error("tooManyFailedAttempts")]
    TooManyFailedAttempts,
    #[error("userCancelled")]
    UserCancelled,
    #[error("{0}")]
    Other(CombinedDeviceError),
}

/// This trait supports arming and disarming as used in, for example, security systems.
pub trait ArmDisarm {
    /// Describes the supported security levels of the device. If this attribute is not reported, the device only supports one level.
    fn get_available_arm_levels(&self) -> Result<Option<Vec<ArmLevel>>, ArmDisarmError>;

    /// If set to true, additional grammar for increase/decrease logic applies,
    /// in the order of the levels array. For example, "Hey Google, increase my security level by 1",
    /// results in the Assistant determining the current security level and then increasing that security level by one.
    /// If this value is set to false, additional grammar for increase/decrease logic is not supported.
    fn is_ordered(&self) -> Result<bool, ArmDisarmError>;

    /// Indicates if the device is currently armed.
    fn is_armed(&self) -> Result<bool, ArmDisarmError>;

    /// If multiple security levels exist, indicates the name of the current security level.
    fn current_arm_level(&self) -> Result<String, ArmDisarmError>;

    /// Indicates the time, in seconds, the user has to leave before `currentArmLevel` takes effect.
    fn exit_allowance(&self) -> Result<i32, ArmDisarmError>;

    /// Arm or disarm the device. `arm` Is true when the intent is to arm the device, false to disarm
    fn arm(&mut self, arm: bool) -> Result<(), ArmDisarmError>;

    /// Cancels the arming of the device
    fn cancel_arm(&mut self) -> Result<(), ArmDisarmError>;

    /// Arm the device. `level` is the `level_name` to arm to.
    fn arm_with_level(&mut self, arm: bool, level: String) -> Result<(), ArmDisarmError>;
}