use crate::CombinedDeviceError;
use serde::Serialize;

/// List of objects describing available media channels for this particular device. Each item describes a channel the user can select on this device.
#[derive(Debug, PartialEq, Serialize)]
pub struct AvailableChannel {
    /// Unique identifier for this channel. Not exposed to users.
    pub key: String,
    /// List of user-visible names for this channel.
    pub names: Vec<String>,
    /// Optional numeric identifier for this channel.
    pub number: Option<String>,
}

///This trait belongs to devices that support TV channels on a media device.
///
/// The available channels should be shared as a list, per user or device,
/// during SYNC via the availableChannels attribute.
/// This list should comprise of all top or popular channels that the user or device is subscribed to.
/// To ensure a low query latency, we recommend that you keep the channel list small (to 30 channels or less).
pub trait Channel {
    /// List of objects describing available media channels for this particular device.
    /// Each item describes a channel the user can select on this device.
    fn get_available_channels(&self) -> Result<Vec<AvailableChannel>, CombinedDeviceError>;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication.
    /// Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    ///
    /// Default: false
    fn is_command_only_channels(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Set the current channel to a specific value by it's ID.
    /// - `code` Unique identifier for the requested channel, matching one of the availableChannels.
    /// - `name` User-friendly name of the requested channel.
    /// - `number` Numeric identifier for the requested channel.
    fn select_channel_by_id(&mut self, code: String, name: Option<String>, number: Option<String>) -> Result<(), CombinedDeviceError>;

    /// Set the current channel to a specific value by it's channel number.
    /// - `number` Numeric identifier for the requested channel.
    fn select_channel_by_number(&mut self, number: String) -> Result<(), CombinedDeviceError>;

    /// Adjust the current channel by a relative amount.
    /// - `change` The number of channels to increase or decrease.
    fn select_channel_relative(&mut self, change: i32) -> Result<(), CombinedDeviceError>;

    /// Return to the last/previous channel the user was on.
    fn return_to_last_channel(&mut self) -> Result<(), CombinedDeviceError>;
}
