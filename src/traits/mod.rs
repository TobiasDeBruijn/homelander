use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

pub mod app_selector;
pub mod arm_disarm;
pub mod brightness;
pub mod camera_stream;
pub mod channel;
pub mod color_setting;
pub mod cook;
pub mod dispense;
pub mod dock;
pub mod energy_storage;
pub mod fan_speed;
pub mod fill;
pub mod humidity_setting;
pub mod input_selector;
pub mod light_effects;
pub mod locator;
pub mod lock_unlock;
pub mod media_state;
pub mod modes;
pub mod network_control;
pub mod on_off;
pub mod open_close;
pub mod reboot;
pub mod rotation;
pub mod run_cycle;
pub mod scene;
pub mod sensor_state;
pub mod software_update;
pub mod start_stop;
pub mod status_report;
pub mod temperature_control;
pub mod temperature_setting;
pub mod timer;
pub mod toggles;
pub mod transport_control;
pub mod volume;

#[derive(Debug, PartialEq)]
pub struct DeviceInfo {
    pub model: String,
    pub manufacturer: String,
    pub hw: String,
    pub sw: String,
}

#[derive(Debug, PartialEq)]
pub struct DeviceName {
    pub default_names: Vec<String>,
    pub name: String,
    pub nicknames: Vec<String>,
}

pub trait GoogleHomeDevice {
    fn get_device_info(&self) -> DeviceInfo;

    fn get_room_hint(&self) -> Option<String> {
        None
    }

    fn will_report_state(&self) -> bool;

    fn get_device_name(&self) -> DeviceName;

    /// Indicates if the device is online (that is, reachable) or not.
    fn is_online(&self) -> bool;
}

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum DeviceError {
    // Todo
    // https://developers.google.com/assistant/smarthome/reference/errors-exceptions#error_list
}

#[derive(Debug, PartialEq, Serialize, Error)]
pub enum DeviceException {
    // Todo
    // https://developers.google.com/assistant/smarthome/reference/errors-exceptions#exception_list
}

#[derive(Debug, Serialize, PartialEq, Error)]
pub enum CombinedDeviceError {
    #[error("{0}")]
    DeviceError(DeviceError),
    #[error("{0}")]
    DeviceException(DeviceException),
    #[error("{0}")]
    Other(#[from] crate::SerializableError),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Language {
    #[serde(rename = "da")]
    Danish,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "en")]
    English,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "hi")]
    Hindi,
    #[serde(rename = "id")]
    Indonesian,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "no")]
    Norwegian,
    #[serde(rename = "pt-BR")]
    Portuguese,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "sv")]
    Swedish,
    #[serde(rename = "th")]
    Thai,
    #[serde(rename = "zh-TW")]
    Chinese,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SizeUnit {
    UnknownUnits,
    NoUnits,
    Centimeters,
    Cups,
    Deciliters,
    Feet,
    FluidOunces,
    Gallons,
    Grams,
    Inches,
    Kilograms,
    Liters,
    Meters,
    Milligrams,
    Milliliters,
    Millimeters,
    Ounces,
    Pinch,
    Pints,
    Portion,
    Pounds,
    Quarts,
    Tablespoons,
    Teaspoons,
}

/// Supported temperature range of the device.
#[derive(Debug, PartialEq, Serialize)]
pub struct TemperatureRange {
    /// Minimum temperature for the range, in degrees Celsius.
    pub min_threshold_celsius: f32,
    ///Maximum temperature for the range, in degrees Celsius.
    pub max_threshold_celsius: f32,
}

/// Temperature unit used in responses to the user.
#[derive(Debug, PartialEq, Serialize)]
pub enum TemperatureUnit {
    #[serde(rename = "C")]
    Celsius,
    #[serde(rename = "F")]
    Fahrenheit,
}

/// Name synonyms in each supported language.
#[derive(Debug, PartialEq, Serialize)]
pub struct Synonym {
    /// Synonyms for the preset, should include both singular and plural forms, if applicable.
    pub synonym: Vec<String>,
    /// Language code
    pub lang: Language,
}

/// This trait belongs to devices that can detect objects or people and send a notification to the user.
/// For example, it can be used for doorbells to indicate that a person (named or unnamed) rang the doorbell,
/// as well as for cameras and sensors that can detect movement of objects or people approaching.
pub trait ObjectDetection {
    // TODO
}
