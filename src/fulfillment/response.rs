use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub payload: ResponsePayload,
}

#[derive(Debug, Serialize)]
pub enum ResponsePayload {
    Sync(sync::Payload),
    Query(query::Payload),
    Execute(execute::Payload),
}

pub mod sync {
    use crate::device_trait::Trait;
    use crate::traits::arm_disarm::AvailableArmLevels;
    use crate::traits::color_setting::{ColorModel, ColorTemperatureRange};
    use crate::traits::cook::{CookingMode, FoodPreset};
    use crate::traits::dispense::{DispenseItem, DispensePreset};
    use crate::traits::energy_storage::UxDistanceUnit;
    use crate::traits::fan_speed::AvailableFanSpeeds;
    use crate::traits::fill::AvailableFillLevels;
    use crate::traits::humidity_setting::HumiditySetPointRange;
    use crate::traits::input_selector::AvailableInput;
    use crate::traits::light_effects::LightEffectType;
    use crate::traits::modes::AvailableMode;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        pub agent_user_id: String,
        pub devices: Vec<Device>,
        pub error_code: Option<String>,
        pub debug_string: Option<String>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        pub id: String,
        #[serde(rename = "type")]
        pub device_type: String,
        pub traits: Vec<Trait>,
        pub name: DeviceName,
        pub will_report_state: bool,
        pub room_hint: Option<String>,
        pub device_info: DeviceInfo,
        pub attributes: SyncAttributes,
    }

    #[derive(Debug, Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct SyncAttributes {
        // TODO appselector
        pub available_arm_levels: Option<AvailableArmLevels>,
        pub command_only_brightness: Option<bool>,
        // TODO camerastream
        // TODO channel
        pub command_only_color_setting: Option<bool>,
        pub color_model: Option<ColorModel>,
        pub color_temperature_range: Option<ColorTemperatureRange>,
        pub supported_cooking_modes: Option<Vec<CookingMode>>,
        pub food_presets: Option<Vec<FoodPreset>>,
        pub supported_dispense_items: Option<Vec<DispenseItem>>,
        pub supported_dispense_presets: Option<Vec<DispensePreset>>,
        pub query_only_energy_storage: Option<bool>,
        #[serde(rename = "energyStorageDistanceUnitForUX")]
        pub energy_storage_distance_unit_for_ux: Option<UxDistanceUnit>,
        pub is_rechargeable: Option<bool>,
        pub reversible: Option<bool>,
        pub command_only_fan_speed: Option<bool>,
        pub available_fan_speeds: Option<AvailableFanSpeeds>,
        pub supports_fan_speed_percent: Option<bool>,
        pub available_fill_levels: Option<AvailableFillLevels>,
        pub humidity_set_point_range: Option<HumiditySetPointRange>,
        pub command_only_humidity_setting: Option<bool>,
        pub query_only_humidity_setting: Option<bool>,
        pub available_inputs: Option<Vec<AvailableInput>>,
        pub command_only_input_selector: Option<bool>,
        pub ordered_inputs: Option<bool>,
        pub default_color_loop_duration: Option<i32>,
        pub default_sleep_duration: Option<i32>,
        pub default_wake_duration: Option<i32>,
        pub supported_effects: Option<Vec<LightEffectType>>,
        pub support_activity_state: Option<bool>,
        pub support_playback_state: Option<bool>,
        pub available_modes: Option<Vec<AvailableMode>>,
        pub command_only_modes: Option<bool>,
        pub query_only_modes: Option<bool>,
        pub supports_enabling_guest_network: Option<bool>,
        pub supports_disabling_guest_network: Option<bool>,
        pub supports_getting_guest_network_password: Option<bool>,
        pub network_profiles: Option<Vec<String>>,
        pub supports_enabling_network_profile: Option<bool>,
        pub supports_disabling_network_profile: Option<bool>,
        pub supports_network_download_speed_test: Option<bool>,
        pub supports_network_upload_speed_test: Option<bool>,
        pub command_only_on_off: Option<bool>,
        pub query_only_on_off: Option<bool>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceInfo {
        pub manufacturer: String,
        pub model: String,
        pub hw_version: String,
        pub sw_version: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceName {
        pub default_names: Vec<String>,
        pub name: String,
        pub nicknames: Vec<String>,
    }
}

pub mod query {
    use crate::traits::color_setting::Color;
    use crate::traits::cook::CookingMode;
    use crate::traits::dispense::DispenseItemState;
    use crate::traits::energy_storage::{CapacityState, CapacityValue};
    use crate::traits::light_effects::LightEffectType;
    use crate::traits::media_state::{ActivityState, PlaybackState};
    use crate::traits::network_control::{DownloadSpeedTestResult, NetworkProfileState, NetworkSettings, UploadSpeedTestResult};
    use crate::traits::SizeUnit;
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Debug, Serialize)]
    pub struct Payload {
        pub error_code: Option<String>,
        pub debug_string: Option<String>,
        pub devices: HashMap<String, QueryDeviceState>,
    }

    #[derive(Debug, Serialize)]
    pub struct QueryDeviceState {
        #[serde(flatten)]
        pub required: RequiredQueryDeviceState,
        #[serde(flatten)]
        pub traits: Option<TraitsQueryDeviceState>,
    }

    #[derive(Debug, Serialize)]
    #[allow(unused)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum QueryStatus {
        Success,
        Offline,
        Exceptions,
        Error,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RequiredQueryDeviceState {
        pub on: bool,
        pub online: bool,
        pub status: QueryStatus,
        pub error_code: Option<String>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraitsQueryDeviceState {
        // TODO appselector
        is_armed: Option<bool>,
        current_arm_level: Option<String>,
        exit_allowance: Option<i32>,
        brightness: Option<i32>,
        // TODO camerastream
        // TODO channel
        color: Option<Color>,
        current_cooking_mode: Option<CookingMode>,
        current_food_preset: Option<String>,
        current_food_quantity: Option<f32>,
        current_food_unit: Option<SizeUnit>,
        dispense_items: Option<Vec<DispenseItemState>>,
        is_docked: Option<bool>,
        descriptive_capacity_remaining: Option<CapacityState>,
        capacity_remaining: Option<CapacityValue>,
        capacity_until_full: Option<CapacityValue>,
        is_charging: Option<bool>,
        is_plugged_in: Option<bool>,
        current_fan_speed_setting: Option<Vec<String>>,
        current_fan_speed_percent: Option<f32>,
        is_filled: Option<bool>,
        current_fill_level: Option<String>,
        current_fill_percent: Option<f32>,
        humidity_setpoint_percent: Option<i32>,
        humidity_ambient_percent: Option<i32>,
        current_input: Option<String>,
        active_light_effect: Option<LightEffectType>,
        light_effect_end_unix_timestamp_sec: Option<i64>,
        is_locked: Option<bool>,
        is_jammed: Option<bool>,
        activity_state: Option<ActivityState>,
        playback_state: Option<PlaybackState>,
        current_mode_setting: Option<HashMap<String, String>>,
        network_enabled: Option<bool>,
        network_settings: Option<NetworkSettings>,
        guest_network_enabled: Option<bool>,
        guest_network_settings: Option<NetworkSettings>,
        num_connected_devices: Option<i32>,
        #[serde(rename = "networkUsageMB")]
        network_usage_mb: Option<f32>,
        #[serde(rename = "networkUsageLimitMB")]
        network_usage_limit_mb: Option<f32>,
        network_usage_unlimited: Option<bool>,
        last_network_download_speed_test: Option<DownloadSpeedTestResult>,
        last_network_upload_speed_test: Option<UploadSpeedTestResult>,
        network_speed_test_in_progress: Option<bool>,
        network_profiles_state: Option<HashMap<String, NetworkProfileState>>,
        on: Option<bool>,
    }
}

pub mod execute {
    use crate::serializable_error::SerializableError;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct Payload {
        pub commands: Vec<Command>,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum CommandStatus {
        Success,
        Pending,
        Offline,
        Exceptions,
        Error,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Command {
        pub ids: Vec<String>,
        pub status: CommandStatus,
        pub states: Option<CommandState>,
        pub error_code: Option<SerializableError>,
        pub debug_string: Option<String>,
    }

    #[derive(Debug, Default, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CommandState {
        pub lock: Option<bool>,
        pub guest_network_password: Option<String>,
    }
}
