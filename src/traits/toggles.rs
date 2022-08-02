use crate::traits::Language;
use crate::CombinedDeviceError;
use serde::Serialize;
use std::collections::HashMap;

/// Available toggle.
#[derive(Debug, PartialEq, Serialize)]
pub struct AvailableToggle {
    /// Internal name of the toggle, which will be used in commands and states. This can be non-user-friendly, and will be shared across all languages.
    pub name: String,
    /// Synonyms of the toggle in each supported languages.
    pub name_values: Vec<NameValue>,
}

/// Synonyms of the toggle in a given language.
#[derive(Debug, PartialEq, Serialize)]
pub struct NameValue {
    /// Synonyms of the toggle. The first string in this list is used as the canonical name of the level in that language.
    pub name_synonym: Vec<String>,
    /// Language code
    pub lang: Language,
}

/// This trait belongs to any devices with settings that can only exist in one of two states.
///
/// These settings can represent a physical button with an on/off or active/inactive state,
/// a checkbox in HTML, or any other sort of specifically enabled/disabled element.
/// If the setting has more than two states, or has a state in which neither of the binary options is selected,
/// it is better represented as a Modes trait, which equates to multi-state dials,
/// radio buttons (physical or HTML),
/// or binary states that are not explicitly on/off (for example, "AM/FM" or "hot/cold").
///
/// This trait covers one or more individual toggles which users can set.
/// In general, these toggles should be used for functionality that is unlinked from other device behavior.
/// Linked behavior, such as turning the device itself on or off,
/// should use more specific traits (for example, the thermostatMode in the trait [TemperatureSetting](crate::traits::temperature_setting::TemperatureSetting)).
pub trait Toggles {
    /// List of available toggles.
    fn get_available_toggles(&self) -> Result<Vec<AvailableToggle>, CombinedDeviceError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    fn is_command_only_toggles(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Required if the device supports query-only execution. This attribute indicates if the device can only be queried for state information, and cannot be controlled.
    fn is_query_only_toggles(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Key/value pair with the toggle name of the device as the key, and the current state as the value.
    fn get_current_toggle_settings(&self) -> Result<HashMap<String, bool>, CombinedDeviceError>;

    /// Set a given toggle state.
    /// - `name` The name of the toggle
    /// - `value` The new state of the toggle
    fn set_toggle(&mut self, name: String, value: bool) -> Result<(), CombinedDeviceError>;
}
