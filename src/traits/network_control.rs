use crate::CombinedDeviceError;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    /// An error occurred while attempting to control the given network profile.
    #[error("NetworkProfileNotRecognized")]
    NetworkProfileNotRecognized,
    /// An error occurred while attempting to request a speed test.
    #[error("NetworkSpeedTestInProgress")]
    NetworkSpeedTestInProgress,
}

#[derive(Debug, Error)]
pub enum NetworkControlError {
    #[error("{0}")]
    Device(#[from] DeviceError),
    #[error("{0}")]
    Other(#[from] CombinedDeviceError),
}

#[derive(Debug, Serialize)]
pub struct NetworkSettings {
    /// Network SSID.
    pub ssid: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SpeedTestStatus {
    Success,
    Failure,
}

/// Contains the results of the most recent network download speed test.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSpeedTestResult {
    /// The download speed in Mbps (megabits per second) of the last network speed test.
    pub download_speed_mbps: f32,
    /// The Unix timestamp (number of seconds since the Unix Epoch) of when the last network download speed test was run.
    pub unix_timestamp_sec: i64,
    /// Indicates whether the last network download speed test succeeded or failed.
    pub status: SpeedTestStatus,
}

/// Contains the results of the most recent network upload speed test.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadSpeedTestResult {
    /// The upload speed in Mbps (megabits per second) of the last network speed test.
    pub upload_speed_mbps: f32,
    /// The Unix timestamp (number of seconds since the Unix Epoch) of when the last network upload speed test was run.
    pub unix_timestamp_sec: i64,
    /// Indicates whether the last network upload speed test succeeded or failed.
    pub status: SpeedTestStatus,
}

/// An object storing the state of an individual network profile.
/// The value of the key should be the name of one of the network profiles in the networkProfiles attribute.
#[derive(Debug, Serialize)]
pub struct NetworkProfileState {
    /// The current enabled/disabled state of the network profile.
    pub enabled: bool,
}

/// This trait belongs to devices that support reporting network data and performing network specific operations.
pub trait NetworkControl {
    /// Set to true if the guest network can be enabled.
    /// Default: false
    fn supports_enabling_guest_network(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if the guest network can be disabled
    /// Default: false
    fn supports_disabling_guest_network(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if the guest network password can be obtained via the GetGuestNetworkPassword command.
    /// Default: false
    fn supports_getting_guest_network_password(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if network profiles can be enabled.
    /// Default: false
    fn supports_enabling_network_profile(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if network profiles can be disabled.
    /// Default: false
    fn supports_disabling_network_profile(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if a download speed test can be run.
    /// Default: false
    fn supports_network_download_speed_test(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Set to true if an upload speed test can be run.
    /// Default: false
    fn supports_network_upload_speed_test(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// Indicates the supported network profile names.
    /// Default: No network profiles
    fn get_network_profiles(&self) -> Result<Option<Vec<String>>, NetworkControlError> {
        Ok(None)
    }

    /// Whether the main network is enabled.
    fn is_network_enabled(&self) -> Result<bool, NetworkControlError>;

    /// Contains the SSID of the main network.
    fn get_network_settings(&self) -> Result<NetworkSettings, NetworkControlError>;

    /// Whether the guest network is enabled.
    fn is_guest_network_enabled(&self) -> Result<bool, NetworkControlError>;

    /// Contains the SSID of the guest network.
    fn get_guest_network_settings(&self) -> Result<NetworkSettings, NetworkControlError>;

    /// The number of devices connected to the network.
    fn get_num_connected_devices(&self) -> Result<i32, NetworkControlError>;

    /// The network usage in MB (megabytes).
    /// The network usage is within the current billing period,
    /// which can be useful to monitor with respect to a billing period network usage limit.
    fn get_network_usage_mb(&self) -> Result<f32, NetworkControlError>;

    /// The network usage limit in MB (megabytes). The network usage limit is within the current billing period.
    fn get_network_usage_limit_mb(&self) -> Result<f32, NetworkControlError>;

    /// Whether the network usage is unlimited. The device state networkUsageLimitMB will be ignored if this is set to true.
    fn is_network_usage_unlimited(&self) -> Result<bool, NetworkControlError>;

    /// Contains the results of the most recent network download speed test.
    fn get_last_network_download_speed_test(&self) -> Result<DownloadSpeedTestResult, NetworkControlError>;

    /// Contains the results of the most recent network upload speed test.
    fn get_last_network_upload_speed_test(&self) -> Result<UploadSpeedTestResult, NetworkControlError>;

    /// Whether a speed test is currently being run.
    /// Default: false
    fn is_network_speed_test_in_progress(&self) -> Result<Option<bool>, NetworkControlError> {
        Ok(None)
    }

    /// State for network profiles.
    /// This top level object should contain key value pairs where the key is the name of one of the
    /// network profiles listed in the networkProfiles attribute and the value should be that profile's corresponding state.
    fn get_network_profiles_state(&self) -> Result<HashMap<String, NetworkProfileState>, NetworkControlError>;

    /// Enable or disable the guest network.
    /// Only called if both [Self::supports_enabling_guest_network] and [Self::supports_disabling_guest_network] return `true`.
    /// - `enable` True to enable the guest network, false to disable the guest network.
    fn set_guest_network_enabled(&mut self, enable: bool) -> Result<(), NetworkControlError>;

    /// Enable or disable a network profile.
    /// Only called if both [Self::supports_enabling_network_profile] and [Self::supports_disabling_network_profile] return `true`
    /// - `profile` The profile name from networkProfiles attribute.
    /// - `enable` True to enable the profile, false to disable the profile.
    fn set_network_profile_enabled(&mut self, profile: String, enable: bool) -> Result<(), NetworkControlError>;

    /// Get the guest network password.
    /// Only called if [Self::supports_getting_guest_network_password] returns `true`
    fn get_guest_network_password(&self) -> Result<String, NetworkControlError>;

    /// Test the network download and upload speed.
    /// Only called if [Self::supports_network_download_speed_test] and [Self::supports_network_uploadd_speed_test] both return `true`
    /// - `download` Indicates whether the download speed should be tested.
    /// - `upload` Indicates whether the upload speed should be tested.
    fn test_network_speed(&mut self, download: bool, upload: bool) -> Result<(), NetworkControlError>;
}
