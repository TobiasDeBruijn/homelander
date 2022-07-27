use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::{CombinedDeviceError};
use crate::traits::Language;

/// Available mode.
#[derive(Debug, Serialize)]
pub struct AvailableMode {
    /// Internal name of the mode, which will be used in commands and states. This can be non-user-friendly, and will be shared across all languages.
    pub name: String,
    /// Synonyms of the mode in each supported languages.
    pub name_values: Vec<NameValue>,
    /// Supported settings for this mode.
    /// Requires at least 2 items.
    pub settings: Vec<Setting>,
    /// If this is set to true, additional grammar for increase/decrease logic will apply, in the order (increasing) of the settings array.
    pub ordered: bool,
}

/// Supported setting.
#[derive(Debug, Serialize)]
pub struct Setting {
    /// Internal name of the mode setting, which will be used in commands and states. This can be non-user-friendly, and will be shared across all languages.
    pub setting_name: String,
    /// Synonyms of the setting in each supported languages.
    pub setting_values: Vec<SettingValue>,
}

/// Synonyms of the setting in a given language.
#[derive(Debug, Serialize)]
pub struct SettingValue {
    /// Synonyms of the setting. The first string in this list is used as the canonical name of the level in that language.
    pub setting_synonym: Vec<String>,
    /// Language code
    pub lang: Language
}

/// Synonyms of the mode in a given language.
#[derive(Debug, Serialize)]
pub struct NameValue {
    /// Synonyms of the mode. The first string in this list is used as the canonical name of the level in that language.
    pub name_synonym: Vec<String>,
    /// Language code
    pub lang: Language,
}

/// This trait belongs to any devices with an arbitrary number of "n-way" modes in which
/// the modes and settings for each mode are arbitrary and unique to each device or device type.
/// Each mode has multiple possible settings,
/// but only one can be selected at a time; a dryer cannot be in "delicate," "normal," and
/// "heavy duty" mode simultaneously. A setting that simply can be turned on or off belongs in the [crate::traits::Toggles] trait.
///
/// For instance, a washing machine can have settings for load size and temperature.
/// These would both be modes because they are independent of each other,
/// but each can be in only one state at a time.
/// The user can set a mode such as temperature explicitly with a command like Set the washer’s temperature to cold.
///
/// Some modes are "ordered" and can also be adjusted with up/down, increase/decrease verbiage.
/// For example, load size (small, medium, large) and temperature are clearly ordered
/// (note that temperature is not an actual thermostat with a numeric target, as on other devices),
/// but load type (delicates, normal, wool, etc) may not be.
///
/// This trait covers one or more individual modes which users can set. In general,
/// these modes should be used for functionality that is unlinked from other device behavior.
/// Linked behavior, such as turning the device itself on or off,
/// should use more specific traits (for example, the thermostatMode in the trait [crate::traits::TemperatureSetting]).
pub trait Modes {
    /// List of available modes.
    fn get_available_modes(&self) -> Result<Vec<AvailableMode>, CombinedDeviceError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    /// Default: false
    fn is_command_only_modes(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Required if the device supports query-only execution.
    /// This attribute indicates if the device can only be queried for state information, and cannot be controlled.
    /// Default: false
    fn is_query_only_modes(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Key/value pair with the mode name of the device as the key, and the current setting_name as the value.
    fn get_current_mode_settings(&self) -> Result<HashMap<String, String>, CombinedDeviceError>;

    /// Update mode settings.
    fn update_mode(&self, mode_name: String, setting_name: String) -> Result<(), CombinedDeviceError>;
}