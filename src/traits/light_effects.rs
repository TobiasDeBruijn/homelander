use crate::CombinedDeviceError;
use serde::Serialize;

/// Supported light effect.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LightEffectType {
    /// Loops through various colors randomly.
    ColorLoop,
    /// Gradually lowers brightness and, optionally, adjusts the color temperature over a period of time.
    Sleep,
    /// Gradually increases brightness and, optionally, adjusts the color temperature over a period of time.
    Wake,
}

/// This trait belongs to devices that can support complex lighting commands to change state, such as looping through various colors.
///
/// ## Light effects emulation
/// If your device implements the Brightness trait but not the LightEffects trait,
/// the platform supports emulated "sleep" and "wake" effects, without the need for additional code.
/// The platform emulates the effects by sending a series of EXECUTE intents.
pub trait LightEffects {
    /// The default duration, in seconds, for the effect triggered by the action.devices.commands.ColorLoop command.
    /// Default: 1800
    fn get_default_color_loop_duration(&self) -> Result<Option<i32>, CombinedDeviceError> {
        Ok(None)
    }

    /// The default duration, in seconds, for the effect triggered by the action.devices.commands.Sleep command.
    /// Default: 1800
    fn get_default_sleep_duration(&self) -> Result<Option<i32>, CombinedDeviceError> {
        Ok(None)
    }

    /// The default duration, in seconds, for the effect triggered by the action.devices.commands.Wake command.
    /// Default: 1800
    fn get_default_wake_duration(&self) -> Result<Option<i32>, CombinedDeviceError> {
        Ok(None)
    }

    /// List of the effects that the device supports.
    fn get_supported_effects(&self) -> Result<Vec<LightEffectType>, CombinedDeviceError>;

    /// Currently active light effect if any. One of supportedEffects.
    fn get_active_light_effect(&self) -> Result<Option<LightEffectType>, CombinedDeviceError>;

    /// Unix timestamp when the effect is expected to end, if the effect ends on its own.
    fn get_light_efccect_end_unix_timestamp_sec(&self) -> Result<Option<i64>, CombinedDeviceError>;

    /// Request the device to cycle through a set of colors.
    /// - `duration` Duration for the color loop command, in seconds.
    /// Only called if [LightEffectType::ColorLoop] is among the supported effects
    fn set_color_loop(&mut self, duration: Option<i32>) -> Result<(), CombinedDeviceError>;

    /// Gradually lower the device's brightness and, optionally, adjusts the color temperature over a duration of time.
    /// - `duration` Duration for the sleep command, in seconds.
    /// Only called if [LightEffectType::Sleep] is among the supported effects
    fn set_sleep(&mut self, duration: Option<i32>) -> Result<(), CombinedDeviceError>;

    /// Stop the current light effect.
    fn stop_effect(&mut self) -> Result<(), CombinedDeviceError>;

    /// Gradually increase the device's brightness and, optionally, adjusts the color temperature over a duration of time.
    /// - `duration` Duration for the sleep command, in seconds.
    /// Only called if [LightEffectType::Wake] is among the supported effects
    fn set_wake(&mut self, duration: Option<i32>) -> Result<(), CombinedDeviceError>;
}
