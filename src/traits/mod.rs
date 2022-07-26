use std::any::TypeId;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};
use thiserror::Error;
use serde::{Serialize, Deserialize, Serializer};
use crate::Trait;

pub mod arm_disarm;
pub mod brightness;
pub mod color_setting;
pub mod cook;
pub mod dispense;

pub struct DeviceVersion {
    pub hw: String,
    pub sw: String,
}

pub trait GoogleHomeDevice {
    fn get_version(&self) -> DeviceVersion;
    fn get_name(&self) -> String;
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
    Other(#[from] crate::SerializableError)
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Language {
    Danish,
    Dutch,
    English,
    French,
    German,
    Hindi,
    Indonesian,
    Italian,
    Japanese,
    Korean,
    Norwegian,
    Portuguese,
    Spanish,
    Swedish,
    Thai,
    Chinese
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

/// This trait is designed for self-mobile devices that can be commanded to return for charging.
pub trait Dock {

}

/// This trait belongs to devices that can store energy in a battery and potentially recharge, or devices that can charge another device.
/// The trait supports starting and stopping charging, and checking the current charge level,
/// capacity remaining, and capacity until full values.
pub trait EnergyStorage {

}

/// This trait belongs to devices that support setting the speed of a fan (that is, blowing air from the device at various levels,
/// which may be part of an air conditioning or heating unit, or in a car), with settings such as low, medium, and high.
pub trait FanSpeed {

}

/// This trait applies to devices that support being filled, such as a bathtub.
pub trait Fill {

}

/// This trait belongs to devices that support humidity settings such as humidifiers and dehumidifiers.
pub trait HumiditySetting {

}

/// Trait for devices that can change media inputs. These inputs can have dynamic names per device, and may represent audio or video feeds, hardwired or networked.
pub trait InputSelector {

}

/// This trait belongs to devices that can support complex lighting commands to change state, such as looping through various colors.
pub trait LightEffects {

}

/// This trait is used for devices that can be "found". This includes phones,
/// robots (including vacuums and mowers), drones, and tag-specific products that attach to other devices.
pub trait Locator {

}

/// This trait belongs to any devices that support locking and unlocking, and/or reporting a locked state.
pub trait LockUnlock {

}

/// This trait is used for devices which are able to report media states.
pub trait MediaState {

}

/// This trait belongs to any devices with an arbitrary number of "n-way" modes in which
/// the modes and settings for each mode are arbitrary and unique to each device or device type.
/// Each mode has multiple possible settings,
/// but only one can be selected at a time; a dryer cannot be in "delicate," "normal," and
/// "heavy duty" mode simultaneously. A setting that simply can be turned on or off belongs in the [Toggles] trait.
pub trait Modes {

}

/// This trait belongs to devices that support reporting network data and performing network specific operations.
pub trait NetworkControl {

}

/// This trait belongs to devices that can detect objects or people and send a notification to the user.
/// For example, it can be used for doorbells to indicate that a person (named or unnamed) rang the doorbell,
/// as well as for cameras and sensors that can detect movement of objects or people approaching.
pub trait ObjectDetection {

}

/// The basic on and off functionality for any device that has binary on and off, including plugs and switches as well as many future devices.
pub trait OnOff {

}

/// This trait belongs to devices that support opening and closing,
/// and in some cases opening and closing partially or potentially in more
/// than one direction. For example, some blinds may open either to the left or to the right.
/// In some cases, opening certain devices may be a security sensitive action which can
/// require two-factor authentication authentication. See [Two-factor authentication](https://developers.google.com/assistant/smarthome/two-factor-authentication).
pub trait OpenClose {

}

/// This trait belongs to devices that support rebooting, such as routers. The device needs to support rebooting as a single action.
pub trait Reboot {

}

/// This trait belongs to devices that support rotation, such as blinds with rotatable slats.
pub trait Rotation {

}

/// This trait represents any device that has an ongoing duration for its operation which can be queried.
/// This includes, but is not limited to, devices that operate cyclically, such as washing machines, dryers, and dishwashers.
pub trait RunCycle {

}

/// This trait covers both quantitative measurement (for example,
/// air quality index or smoke level) and qualitative state (for example, whether the air quality is healthy
/// or whether the smoke level is low or high).
pub trait SensorState {

}

/// In the case of scenes, the type maps 1:1 to the trait, as scenes don't combine with other traits to form composite devices.
pub trait Scene {

}

/// This trait belongs to devices that support software updates such as a router. Optionally, these devices may report the time of the last successful update.
pub trait SoftwareUpdate {

}

/// Starting and stopping a device serves a similar function to turning it on and off. Devices that inherit this trait function differently when
/// turned on and when started. Unlike devices that simply have an on and off state,
/// some devices that can start and stop are also able to pause while performing operation.
pub trait StartStop {

}

/// This trait reports the current status or state of a specific device or a connected group of devices.
pub trait StatusReport {

}

/// Trait for devices (other than thermostats) that support controlling temperature,
/// either within or around the device. This includes devices such as ovens and refrigerators.
pub trait TemperatureControl {

}

/// This trait covers handling both temperature point and modes.
pub trait TemperatureSetting {

}

/// The Timer trait represents a timer on a device, primarily kitchen appliances such as ovens and microwaves, but not limited to them.
pub trait Timer {

}

/// This trait belongs to any devices with settings that can only exist in one of two states.
/// These settings can represent a physical button with an on/off or active/inactive state,
/// a checkbox in HTML, or any other sort of specifically enabled/disabled element.
pub trait Toggles {

}

/// his trait supports media devices which are able to control media playback (for example, resuming music that's paused).
pub trait TransportControl {

}

/// This trait belongs to devices which are able to change volume (for example, setting the volume to a certain level, mute, or unmute).
pub trait Volume {

}