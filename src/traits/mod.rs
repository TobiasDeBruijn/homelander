use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use thiserror::Error;

pub mod arm_disarm;
pub mod brightness;
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

pub struct DeviceInfo {
    pub model: String,
    pub manufacturer: String,
    pub hw: String,
    pub sw: String,
}

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

#[derive(Debug, Serialize, Error)]
pub enum DeviceError {
    // Todo
    // https://developers.google.com/assistant/smarthome/reference/errors-exceptions#error_list
}

#[derive(Debug, Serialize, Error)]
pub enum DeviceException {
    // Todo
    // https://developers.google.com/assistant/smarthome/reference/errors-exceptions#exception_list
}

#[derive(Debug, Serialize, Error)]
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

/// Name synonyms in each supported language.
#[derive(Debug, Serialize)]
pub struct Synonym {
    /// Synonyms for the preset, should include both singular and plural forms, if applicable.
    pub synonym: Vec<String>,
    /// Language code
    pub lang: Language,
}

/// This trait belongs to devices that support media applications, typically from third parties.
pub trait AppSelector {
    // TODO
}

/// This trait belongs to devices which have the capability to stream video feeds to third party screens,
/// Chromecast-connected screens, or smartphones. By and large, these are security cameras or baby cameras.
/// But this trait also applies to more complex devices which have a camera on them
/// (for example, video-conferencing devices or a vacuum robot with a camera on it).
pub trait CameraStream {
    // TODO
}

/// This trait belongs to devices that support TV channels on a media device.
pub trait Channel {
    // TODO
}

/// This trait belongs to devices that can detect objects or people and send a notification to the user.
/// For example, it can be used for doorbells to indicate that a person (named or unnamed) rang the doorbell,
/// as well as for cameras and sensors that can detect movement of objects or people approaching.
pub trait ObjectDetection {
    // TODO
}

/// This trait belongs to devices that support opening and closing,
/// and in some cases opening and closing partially or potentially in more
/// than one direction. For example, some blinds may open either to the left or to the right.
/// In some cases, opening certain devices may be a security sensitive action which can
/// require two-factor authentication authentication. See [Two-factor authentication](https://developers.google.com/assistant/smarthome/two-factor-authentication).
pub trait OpenClose {}

/// This trait belongs to devices that support rebooting, such as routers. The device needs to support rebooting as a single action.
pub trait Reboot {}

/// This trait belongs to devices that support rotation, such as blinds with rotatable slats.
pub trait Rotation {}

/// This trait represents any device that has an ongoing duration for its operation which can be queried.
/// This includes, but is not limited to, devices that operate cyclically, such as washing machines, dryers, and dishwashers.
pub trait RunCycle {}

/// This trait covers both quantitative measurement (for example,
/// air quality index or smoke level) and qualitative state (for example, whether the air quality is healthy
/// or whether the smoke level is low or high).
pub trait SensorState {}

/// In the case of scenes, the type maps 1:1 to the trait, as scenes don't combine with other traits to form composite devices.
pub trait Scene {}

/// This trait belongs to devices that support software updates such as a router. Optionally, these devices may report the time of the last successful update.
pub trait SoftwareUpdate {}

/// Starting and stopping a device serves a similar function to turning it on and off. Devices that inherit this trait function differently when
/// turned on and when started. Unlike devices that simply have an on and off state,
/// some devices that can start and stop are also able to pause while performing operation.
pub trait StartStop {}

/// This trait reports the current status or state of a specific device or a connected group of devices.
pub trait StatusReport {}

/// Trait for devices (other than thermostats) that support controlling temperature,
/// either within or around the device. This includes devices such as ovens and refrigerators.
pub trait TemperatureControl {}

/// This trait covers handling both temperature point and modes.
pub trait TemperatureSetting {}

/// The Timer trait represents a timer on a device, primarily kitchen appliances such as ovens and microwaves, but not limited to them.
pub trait Timer {}

/// This trait belongs to any devices with settings that can only exist in one of two states.
/// These settings can represent a physical button with an on/off or active/inactive state,
/// a checkbox in HTML, or any other sort of specifically enabled/disabled element.
pub trait Toggles {}

/// his trait supports media devices which are able to control media playback (for example, resuming music that's paused).
pub trait TransportControl {}

/// This trait belongs to devices which are able to change volume (for example, setting the volume to a certain level, mute, or unmute).
pub trait Volume {}
