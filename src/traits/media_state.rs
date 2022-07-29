use crate::CombinedDeviceError;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum ActivityState {
    Inactive,
    Standby,
    Active,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename = "SCREAMING_SNAKE_CASE")]
pub enum PlaybackState {
    Paused,
    Playing,
    FastForwarding,
    Rewinding,
    Buffering,
    Stopped,
}

/// This trait is used for devices which are able to report media states.
pub trait MediaState {
    fn does_support_activity_state(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    fn does_support_playback_state(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicate whether the device is active and the user is actively interacting with it.
    ///
    /// Only called if [Self::does_support_activity_state] returns `Some(true)`
    fn get_activity_state(&self) -> Result<Option<ActivityState>, CombinedDeviceError>;

    /// Indicate the current state when playing media.
    ///
    /// Only called if [Self::does_support_playback_state] returns `Some(true)`
    fn get_playback_state(&self) -> Result<Option<PlaybackState>, CombinedDeviceError>;
}
