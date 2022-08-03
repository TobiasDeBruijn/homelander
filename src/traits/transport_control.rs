use crate::CombinedDeviceError;
use serde::Serialize;

/// Supported commands.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupportedCommand {
    CaptionControl,
    Next,
    Pause,
    Previous,
    Resume,
    SeekRelative,
    SeekAbsolute,
    SetRepeat,
    Shuffle,
    Stop,
}

/// This trait is used for devices which are able to control media playback (for example, resuming music while it is paused).
///
/// Note: If the device can store or report device state information, it should do so using the [MediaState](crate::traits::media_state::MediaState) trait.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/transportcontrol>
pub trait TransportControl {
    /// A list of strings describing supported transport control commands on this device.
    fn get_supported_control_commands(&self) -> Result<Vec<SupportedCommand>, CombinedDeviceError>;

    /// Pause media playback.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Stop]
    fn media_stop(&mut self) -> Result<(), CombinedDeviceError>;

    /// Skip to next media item.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Next]
    fn media_next(&mut self) -> Result<(), CombinedDeviceError>;

    /// Skip to previous media item.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Previous]
    fn media_previous(&mut self) -> Result<(), CombinedDeviceError>;

    /// Pause media playback.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Pause]
    fn media_pause(&mut self) -> Result<(), CombinedDeviceError>;

    /// Resume media playback.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Resume]
    fn media_resume(&mut self) -> Result<(), CombinedDeviceError>;

    /// Seek to a relative position.
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::SeekRelative]
    /// - `relative_position_ms` Milliseconds of the forward (positive int) or backward (negative int) amount to seek.
    fn media_seek_relative(&mut self, relative_position_ms: i32) -> Result<(), CombinedDeviceError>;

    /// Seek to an absolute position
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::SeekToPosition]
    /// - `abs_position_ms` Millisecond of the absolute position to seek to.
    fn media_seek_to_position(&mut self, abs_position_ms: i32) -> Result<(), CombinedDeviceError>;

    /// Set repeat playback mode
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::SetRepeat]
    /// - `is_on` True to turn on repeat mode, false to turn off repeat mode.
    /// - `is_single` True means turning on single-item repeat mode, false means turning on normal repeat mode (for example a playlist).
    fn media_repeat_mode(&mut self, is_on: bool, single_mode: bool) -> Result<(), CombinedDeviceError>;

    /// Shuffle the current playlist
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::Shuffle]
    fn media_shuffle(&mut self) -> Result<(), CombinedDeviceError>;

    /// Turn captions on
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::CaptionControl]
    /// - `cc_lang` Language or locale for closed captioning.
    /// - `user_query_lang`
    fn media_closed_captioning_on(&mut self, cc_lang: String, user_query_lang: String) -> Result<(), CombinedDeviceError>;

    /// Turn captions off
    /// Only called if [Self::get_supported_control_commands] returns [SupportedCommands::CaptionControl]
    fn media_closed_captioning_off(&mut self) -> Result<(), CombinedDeviceError>;
}
