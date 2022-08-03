use crate::CombinedDeviceError;

/// This trait belongs to devices which are able to change volume (for example, setting the volume to a certain level, mute, or unmute).
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/volume>
pub trait Volume {
    /// The maximum volume level, assuming a baseline of 0 (mute).
    /// Assistant will adjust adverbial commands (e.g. 'make the tv a little louder') accordingly.
    fn get_volume_max_level(&self) -> Result<i32, CombinedDeviceError>;

    /// Indicates if the device can mute and unmute the volume.
    /// Mute is a separate option as the 'mute' behavior takes the volume to 0 while remembering the previous volume,
    /// so that unmute restores it. This is reflected in volume state—if volume is 5,
    /// and the user mutes, the volume remains 5 and isMuted is true.
    fn can_mute_and_unmute(&self) -> Result<bool, CombinedDeviceError>;

    /// The volume (in percentage) for the default volume defined by user or manufacturer. The scale must be 0-100.
    /// Default: 40
    fn get_volume_default_percentage(&self) -> Result<Option<i32>, CombinedDeviceError> {
        Ok(None)
    }

    /// The default step size for relative volume queries like 'volume up on <device_name>.
    /// Default: 1
    fn get_level_step_size(&self) -> Result<Option<i32>, CombinedDeviceError> {
        Ok(None)
    }

    /// Indicates if the device operates using one-way (true) or two-way (false) communication.
    /// For example, if the controller can confirm the new device state after sending the request, this field would be false.
    /// If it's not possible to confirm if the request is successfully executed or
    /// to get the state of the device (for example, if the device is a traditional infrared remote), set this field to true.
    /// Default: false
    fn is_command_only_volume(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// The current volume percentage. It must be between >0 and volumeMaxLevel.
    /// If [Self::is_command_only_volume] is `true`, this **must** be [Some]
    fn get_current_volume(&self) -> Result<Option<i32>, CombinedDeviceError>;

    /// True if the device is muted; false otherwise. If isMuted is true,
    /// the device still returns currentVolume for the remembered point.
    /// If [Self::can_mute_and_unmute] is `true`, this **must** return [Some]
    fn is_muted(&self) -> Result<Option<bool>, CombinedDeviceError>;

    /// Mutes (sets the volume to 0) or unmutes the device.
    /// - `mute` Whether to mute a device or unmute a device.
    ///
    /// This function *should* only be called if [Self::can_mute_and_unmute] returns `Some(true)`.
    /// However the Google documentation does not specify this explicitly!
    fn mute(&mut self, mute: bool) -> Result<(), CombinedDeviceError>;

    /// Set volume to the requested level, based on volumeMaxLevel.
    /// - `volume_level` New volume, from 0 to volumeMaxLevel.
    fn set_volume(&mut self, volume_level: i32) -> Result<(), CombinedDeviceError>;

    /// Set volume up or down n steps, based on volumeMaxLevel. For commands that use a relative scale,
    /// the Assistant will select `n` appropriately to scale to the available steps.
    /// For example, Make the TV much louder will set a higher number of steps than Make the TV a tiny bit louder.
    fn set_volume_relative(&mut self, relative_steps: i32) -> Result<(), CombinedDeviceError>;
}
