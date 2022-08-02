use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub payload: ResponsePayload,
}

#[derive(Debug, PartialEq, Serialize)]
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
    use crate::traits::open_close::OpenDirection;
    use crate::traits::rotation::RotationDegreeRange;
    use crate::traits::sensor_state::SupportedSensorState;
    use crate::traits::temperature_setting::ThermostatMode;
    use crate::traits::{TemperatureRange, TemperatureUnit};
    use serde::Serialize;

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        pub agent_user_id: String,
        pub devices: Vec<Device>,
        pub error_code: Option<String>,
        pub debug_string: Option<String>,
    }

    #[derive(Debug, PartialEq, Serialize)]
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

    #[derive(Debug, PartialEq, Serialize, Default)]
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
        pub discrete_only_open_close: Option<bool>,
        pub open_direction: Option<Vec<OpenDirection>>,
        pub command_only_open_close: Option<bool>,
        pub query_only_open_close: Option<bool>,
        pub supports_degrees: Option<bool>,
        pub supports_percent: Option<bool>,
        pub rotation_degrees_range: Option<RotationDegreeRange>,
        pub supports_continuous_rotation: Option<bool>,
        pub command_only_rotation: Option<bool>,
        pub scene_reversible: Option<bool>,
        pub sensor_states_supported: Option<Vec<SupportedSensorState>>,
        pub pausable: Option<bool>,
        pub available_zones: Option<Vec<String>>,
        pub temperature_range: Option<TemperatureRange>,
        pub temperature_step_celsius: Option<f32>,
        #[serde(rename = "temperatureUnitForUX")]
        pub temperature_unit_for_ux: Option<TemperatureUnit>,
        pub command_only_temperature_control: Option<bool>,
        pub query_only_temperature_control: Option<bool>,
        pub available_thermostat_modes: Option<Vec<ThermostatMode>>,
        pub thermostat_temperature_range: Option<TemperatureRange>,
        pub thermostat_temperature_unit: Option<TemperatureUnit>,
        pub buffer_range_celsius: Option<f32>,
        pub command_only_temperature_setting: Option<bool>,
        pub query_only_temperature_setting: Option<bool>,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceInfo {
        pub manufacturer: String,
        pub model: String,
        pub hw_version: String,
        pub sw_version: String,
    }

    #[derive(Debug, PartialEq, Serialize)]
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
    use crate::traits::open_close::OpenState;
    use crate::traits::run_cycle::CurrentRunCycle;
    use crate::traits::sensor_state::CurrentSensorState;
    use crate::traits::status_report::CurrentStatusReport;
    use crate::traits::temperature_setting::{QueryThermostatMode, ThermostatMode};
    use crate::traits::SizeUnit;
    use serde::Serialize;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize)]
    pub struct Payload {
        pub error_code: Option<String>,
        pub debug_string: Option<String>,
        pub devices: HashMap<String, QueryDeviceState>,
    }

    #[derive(Debug, PartialEq, Serialize)]
    pub struct QueryDeviceState {
        #[serde(flatten)]
        pub required: RequiredQueryDeviceState,
        #[serde(flatten)]
        pub traits: Option<TraitsQueryDeviceState>,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[allow(unused)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum QueryStatus {
        Success,
        Offline,
        Exceptions,
        Error,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RequiredQueryDeviceState {
        pub on: bool,
        pub online: bool,
        pub status: QueryStatus,
        pub error_code: Option<String>,
    }

    #[derive(Debug, Default, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TraitsQueryDeviceState {
        // TODO appselector
        pub is_armed: Option<bool>,
        pub current_arm_level: Option<String>,
        pub exit_allowance: Option<i32>,
        pub brightness: Option<i32>,
        // TODO camerastream
        // TODO channel
        pub color: Option<Color>,
        pub current_cooking_mode: Option<CookingMode>,
        pub current_food_preset: Option<String>,
        pub current_food_quantity: Option<f32>,
        pub current_food_unit: Option<SizeUnit>,
        pub dispense_items: Option<Vec<DispenseItemState>>,
        pub is_docked: Option<bool>,
        pub descriptive_capacity_remaining: Option<CapacityState>,
        pub capacity_remaining: Option<Vec<CapacityValue>>,
        pub capacity_until_full: Option<Vec<CapacityValue>>,
        pub is_charging: Option<bool>,
        pub is_plugged_in: Option<bool>,
        pub current_fan_speed_setting: Option<String>,
        pub current_fan_speed_percent: Option<f32>,
        pub is_filled: Option<bool>,
        pub current_fill_level: Option<String>,
        pub current_fill_percent: Option<f32>,
        pub humidity_setpoint_percent: Option<i32>,
        pub humidity_ambient_percent: Option<i32>,
        pub current_input: Option<String>,
        pub active_light_effect: Option<LightEffectType>,
        pub light_effect_end_unix_timestamp_sec: Option<i64>,
        pub is_locked: Option<bool>,
        pub is_jammed: Option<bool>,
        pub activity_state: Option<ActivityState>,
        pub playback_state: Option<PlaybackState>,
        pub current_mode_setting: Option<HashMap<String, String>>,
        pub network_enabled: Option<bool>,
        pub network_settings: Option<NetworkSettings>,
        pub guest_network_enabled: Option<bool>,
        pub guest_network_settings: Option<NetworkSettings>,
        pub num_connected_devices: Option<i32>,
        #[serde(rename = "networkUsageMB")]
        pub network_usage_mb: Option<f32>,
        #[serde(rename = "networkUsageLimitMB")]
        pub network_usage_limit_mb: Option<f32>,
        pub network_usage_unlimited: Option<bool>,
        pub last_network_download_speed_test: Option<DownloadSpeedTestResult>,
        pub last_network_upload_speed_test: Option<UploadSpeedTestResult>,
        pub network_speed_test_in_progress: Option<bool>,
        pub network_profiles_state: Option<HashMap<String, NetworkProfileState>>,
        pub on: Option<bool>,
        pub open_percent: Option<f32>,
        pub open_state: Option<Vec<OpenState>>,
        pub rotation_degrees: Option<f32>,
        pub rotation_percent: Option<f32>,
        pub current_run_cycle: Option<Vec<CurrentRunCycle>>,
        pub current_total_remaining_time: Option<i32>,
        pub current_cycle_remaining_time: Option<i32>,
        pub current_sensor_state_data: Option<Vec<CurrentSensorState>>,
        pub last_software_update_unix_timestamp_sec: Option<i64>,
        pub is_running: Option<bool>,
        pub is_paused: Option<bool>,
        pub active_zones: Option<Vec<String>>,
        pub current_status_report: Option<Vec<CurrentStatusReport>>,
        pub temperature_setpoint_celsius: Option<f32>,
        pub temperature_ambient_celsius: Option<f32>,
        pub active_thermostat_mode: Option<ThermostatMode>,
        pub target_temp_reached_estimate_unix_timestamp_sec: Option<i64>,
        pub thermostat_humidity_ambient: Option<f32>,
        #[serde(flatten)]
        pub thermostat_mode: Option<QueryThermostatMode>,
    }
}

pub mod execute {
    use crate::serializable_error::SerializableError;
    use serde::Serialize;

    #[derive(Debug, PartialEq, Serialize)]
    pub struct Payload {
        pub commands: Vec<Command>,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum CommandStatus {
        Success,
        Pending,
        Offline,
        Exceptions,
        Error,
    }

    #[derive(Debug, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Command {
        pub ids: Vec<String>,
        pub status: CommandStatus,
        pub states: Option<CommandState>,
        pub error_code: Option<SerializableError>,
        pub debug_string: Option<String>,
    }

    #[derive(Debug, Default, PartialEq, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CommandState {
        pub lock: Option<bool>,
        pub guest_network_password: Option<String>,
    }
}
