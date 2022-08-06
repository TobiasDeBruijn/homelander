use crate::device_trait::Trait;
use crate::device_type::DeviceType;
use crate::execute_error::ExecuteError;
use crate::fulfillment::response::execute::CommandState;
use crate::traits::app_selector::AppSelector;
use crate::traits::arm_disarm::AvailableArmLevels;
use crate::traits::camera_stream::CameraStream;
use crate::traits::channel::Channel;
use crate::traits::cook::{Cook, CookingConfig};
use crate::traits::dispense::Dispense;
use crate::traits::dock::Dock;
use crate::traits::energy_storage::EnergyStorage;
use crate::traits::fan_speed::FanSpeed;
use crate::traits::fill::Fill;
use crate::traits::humidity_setting::HumiditySetting;
use crate::traits::input_selector::InputSelector;
use crate::traits::light_effects::LightEffects;
use crate::traits::locator::Locator;
use crate::traits::lock_unlock::LockUnlock;
use crate::traits::media_state::MediaState;
use crate::traits::modes::Modes;
use crate::traits::network_control::NetworkControl;
use crate::traits::on_off::OnOff;
use crate::traits::open_close::OpenClose;
use crate::traits::reboot::Reboot;
use crate::traits::rotation::Rotation;
use crate::traits::run_cycle::RunCycle;
use crate::traits::scene::Scene;
use crate::traits::sensor_state::SensorState;
use crate::traits::software_update::SoftwareUpdate;
use crate::traits::start_stop::StartStop;
use crate::traits::status_report::StatusReport;
use crate::traits::temperature_control::TemperatureControl;
use crate::traits::temperature_setting::TemperatureSetting;
use crate::traits::timer::Timer;
use crate::traits::toggles::Toggles;
use crate::traits::transport_control::TransportControl;
use crate::traits::volume::Volume;
use crate::traits::ObjectDetection;
use crate::{fulfillment, ArmDisarm, Brightness, ColorSetting, CommandOutput, CommandStatus, CommandType, GoogleHomeDevice, SerializableError};
use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use tracing::{instrument, trace};

/// A Google Home device with its traits
#[derive(Debug)]
pub struct Device<T: GoogleHomeDevice + Debug + Send + ?Sized + Sync + 'static> {
    pub(crate) id: String,
    device_type: DeviceType,
    device_traits: DeviceTraits,
    traits: Vec<Trait>,
    inner: Rc<RefCell<T>>,
}

impl<T: GoogleHomeDevice + Send + Debug + Sync + 'static> Device<T> {
    pub(crate) fn unsize(self) -> Device<dyn crate::DeviceTraits> {
        let Self {
            id,
            device_type,
            device_traits,
            traits,
            inner,
        } = self;
        Device {
            id,
            device_type,
            device_traits,
            traits,
            inner,
        }
    }

    /// Create a new ID. Note that the `id` has to be persistent.
    pub fn new(device: T, device_type: DeviceType, id: String) -> Self {
        Self {
            id,
            device_type,
            device_traits: DeviceTraits::default(),
            traits: Vec::new(),
            inner: Rc::new(RefCell::new(device)),
        }
    }
}

impl<T: GoogleHomeDevice + Send + Sync + Debug + ?Sized + 'static> Device<T> {
    pub(crate) fn disconnect(&mut self) {
        self.inner.borrow_mut().disconnect();
    }

    /// Execute the QUERY intent
    #[instrument]
    pub(crate) fn query(&self) -> fulfillment::response::query::QueryDeviceState {
        trace!("Running QUERY for device {}", self.id);

        let states = self.query_get_states();
        let states = match states {
            Ok(s) => s,
            Err(e) => {
                return fulfillment::response::query::QueryDeviceState {
                    required: fulfillment::response::query::RequiredQueryDeviceState {
                        status: fulfillment::response::query::QueryStatus::Error,
                        on: false,
                        online: self.inner.borrow().is_online(),
                        error_code: Some(e.to_string()),
                    },
                    traits: None,
                }
            }
        };

        if !self.inner.borrow().is_online() {
            return fulfillment::response::query::QueryDeviceState {
                required: fulfillment::response::query::RequiredQueryDeviceState {
                    status: fulfillment::response::query::QueryStatus::Offline,
                    on: true,
                    online: false,
                    error_code: None,
                },
                traits: None,
            };
        }

        fulfillment::response::query::QueryDeviceState {
            required: fulfillment::response::query::RequiredQueryDeviceState {
                status: fulfillment::response::query::QueryStatus::Success,
                online: true,
                on: true,
                error_code: None,
            },
            traits: Some(states),
        }
    }

    /// Collect the states for all traits supported by the device
    #[instrument]
    fn query_get_states(&self) -> Result<fulfillment::response::query::TraitsQueryDeviceState, Box<dyn Error>> {
        let mut states = fulfillment::response::query::TraitsQueryDeviceState::default();

        if let Some(d) = &self.device_traits.app_selector {
            states.current_application = Some(d.borrow().get_current_application()?);
        }

        if let Some(d) = &self.device_traits.arm_disarm {
            states.is_armed = Some(d.borrow().is_armed()?);
            states.current_arm_level = Some(d.borrow().current_arm_level()?);
            states.exit_allowance = Some(d.borrow().exit_allowance()?);
        }

        if let Some(d) = &self.device_traits.brightness {
            states.brightness = Some(d.borrow().get_brightness()?);
        }

        // TODO CameraStream
        // TODO Channel

        if let Some(d) = &self.device_traits.color_setting {
            states.color = Some(d.borrow().get_color()?);
        }

        if let Some(d) = &self.device_traits.cook {
            states.current_cooking_mode = Some(d.borrow().get_current_cooking_mode()?);
            states.current_food_preset = d.borrow().get_current_food_preset()?;
            states.current_food_unit = d.borrow().get_current_food_unit()?;
        }

        if let Some(d) = &self.device_traits.dispense {
            states.dispense_items = Some(d.borrow().get_dispense_items_state()?);
        }

        if let Some(d) = &self.device_traits.dock {
            states.is_docked = Some(d.borrow().is_docked()?);
        }

        if let Some(d) = &self.device_traits.energy_storage {
            states.descriptive_capacity_remaining = Some(d.borrow().get_descriptive_capacity_remaining()?);
            states.capacity_remaining = d.borrow().get_capacity_remaining()?;
            states.capacity_until_full = d.borrow().get_capacity_until_full()?;
            states.is_charging = d.borrow().is_charging()?;
            states.is_plugged_in = d.borrow().is_plugged_in()?;
        }

        if let Some(d) = &self.device_traits.fan_speed {
            states.current_fan_speed_setting = d.borrow().get_current_fan_speed_setting()?;
            states.current_fan_speed_percent = d.borrow().get_current_fan_speed_percent()?;
        }

        if let Some(d) = &self.device_traits.fill {
            states.is_filled = Some(d.borrow().is_filled()?);
            states.current_fill_level = d.borrow().get_current_fill_level()?;
            states.current_fill_percent = d.borrow().get_current_fill_percent()?;
        }

        if let Some(d) = &self.device_traits.humidity_setting {
            states.humidity_setpoint_percent = Some(d.borrow().get_current_humidity_set_point_range()?);
            states.humidity_ambient_percent = Some(d.borrow().get_current_humidity_ambient_percent()?);
        }

        if let Some(d) = &self.device_traits.input_selector {
            states.current_input = Some(d.borrow().get_current_input()?);
        }

        if let Some(d) = &self.device_traits.light_effects {
            states.active_light_effect = d.borrow().get_active_light_effect()?;
            states.light_effect_end_unix_timestamp_sec = d.borrow().get_light_efccect_end_unix_timestamp_sec()?;
        }

        if let Some(d) = &self.device_traits.lock_unlock {
            states.is_locked = Some(d.borrow().is_locked()?);
            states.is_jammed = Some(d.borrow().is_jammed()?);
        }

        if let Some(d) = &self.device_traits.media_state {
            states.activity_state = d.borrow().get_activity_state()?;
            states.playback_state = d.borrow().get_playback_state()?;
        }

        if let Some(d) = &self.device_traits.modes {
            states.current_mode_setting = Some(d.borrow().get_current_mode_settings()?);
        }

        if let Some(d) = &self.device_traits.network_control {
            states.network_enabled = Some(d.borrow().is_network_enabled()?);
            states.network_settings = Some(d.borrow().get_network_settings()?);
            states.guest_network_enabled = Some(d.borrow().is_guest_network_enabled()?);
            states.guest_network_settings = Some(d.borrow().get_guest_network_settings()?);
            states.num_connected_devices = Some(d.borrow().get_num_connected_devices()?);
            states.network_usage_mb = Some(d.borrow().get_network_usage_mb()?);
            states.network_usage_unlimited = Some(d.borrow().is_network_usage_unlimited()?);
            states.last_network_download_speed_test = Some(d.borrow().get_last_network_download_speed_test()?);
            states.last_network_upload_speed_test = Some(d.borrow().get_last_network_upload_speed_test()?);
            states.network_speed_test_in_progress = d.borrow().is_network_speed_test_in_progress()?;
            states.network_profiles_state = Some(d.borrow().get_network_profiles_state()?);
        }

        if let Some(d) = &self.device_traits.on_off {
            states.on = Some(d.borrow().is_on()?);
        }

        if let Some(d) = &self.device_traits.open_close {
            states.open_percent = d.borrow().get_open_percent()?;
            states.open_state = d.borrow().get_open_state()?;
        }

        if let Some(d) = &self.device_traits.rotation {
            states.rotation_degrees = Some(d.borrow().get_rotation_degrees()?);
            states.rotation_percent = Some(d.borrow().get_rotation_percent()?);
        }

        if let Some(d) = &self.device_traits.run_cycle {
            states.current_run_cycle = Some(d.borrow().get_current_run_cycle()?);
            states.current_total_remaining_time = Some(d.borrow().get_current_total_remaining_time()?);
            states.current_cycle_remaining_time = Some(d.borrow().get_current_cycle_remaining_time()?);
        }

        if let Some(d) = &self.device_traits.sensor_state {
            states.current_sensor_state_data = Some(d.borrow().get_current_sensor_states()?);
        }

        if let Some(d) = &self.device_traits.software_update {
            states.last_software_update_unix_timestamp_sec = Some(d.borrow().get_last_software_update_unix_timestamp_sec()?);
        }

        if let Some(d) = &self.device_traits.start_stop {
            states.is_running = Some(d.borrow().is_running()?);
            states.is_paused = d.borrow().is_paused()?;
            states.active_zones = d.borrow().get_active_zones()?;
        }

        if let Some(d) = &self.device_traits.status_report {
            states.current_status_report = Some(d.borrow().get_current_status_report()?);
        }

        if let Some(d) = &self.device_traits.temperature_control {
            states.temperature_setpoint_celsius = Some(d.borrow().get_temperature_setpoint_celsius()?);
            states.temperature_ambient_celsius = Some(d.borrow().get_temperatuer_ambient_celsius()?);
        }

        if let Some(d) = &self.device_traits.temperature_setting {
            states.active_thermostat_mode = Some(d.borrow().get_active_thermostat_mode()?);
            states.target_temp_reached_estimate_unix_timestamp_sec = d.borrow().get_target_temp_reached_estimate_unix_timestamp_sec()?;
            states.thermostat_humidity_ambient = d.borrow().get_thermostat_humidity_ambient()?;
            states.thermostat_mode = Some(d.borrow().get_thermostat_mode()?);
        }

        if let Some(d) = &self.device_traits.timer {
            // The API requires this to be -1 if there is no timer set
            // Because we want idiomatic Rust, it's wrapped in an Option
            // for if no timer is set
            states.timer_remaining_sec = Some(d.borrow().get_timer_remaining_sec()?.unwrap_or(-1));
            states.timer_paused = d.borrow().is_timer_paused()?;
        }

        if let Some(d) = &self.device_traits.volume {
            states.current_volume = d.borrow().get_current_volume()?;
            states.is_muted = d.borrow().is_muted()?
        }

        if let Some(d) = &self.device_traits.toggles {
            states.current_toggle_settings = Some(d.borrow().get_current_toggle_settings()?);
        }

        Ok(states)
    }

    /// Execute the SYNC intent
    #[instrument]
    pub(crate) fn sync(&self) -> Result<fulfillment::response::sync::Device, Box<dyn Error>> {
        trace!("Running SYNC for device {}", self.id);
        let name = self.inner.borrow().get_device_name();
        let info = self.inner.borrow().get_device_info();

        Ok(fulfillment::response::sync::Device {
            id: self.id.clone(),
            device_type: self.device_type.as_device_type_string(),
            traits: self.traits.clone(),
            name: fulfillment::response::sync::DeviceName {
                name: name.name,
                default_names: name.default_names,
                nicknames: name.nicknames,
            },
            will_report_state: self.inner.borrow().will_report_state(),
            room_hint: self.inner.borrow().get_room_hint(),
            device_info: fulfillment::response::sync::DeviceInfo {
                manufacturer: info.manufacturer,
                model: info.model,
                hw_version: info.hw,
                sw_version: info.sw,
            },
            attributes: self.sync_set_attributes()?,
        })
    }

    /// Collect all attributes for all traits supported by the device
    #[instrument]
    fn sync_set_attributes(&self) -> Result<fulfillment::response::sync::SyncAttributes, Box<dyn Error>> {
        let mut attributes = fulfillment::response::sync::SyncAttributes::default();

        if let Some(d) = &self.device_traits.app_selector {
            attributes.available_applications = Some(d.borrow().get_available_applications()?);
        }

        if let Some(d) = &self.device_traits.arm_disarm {
            attributes.available_arm_levels = Some(AvailableArmLevels {
                levels: d.borrow().get_available_arm_levels()?,
                ordered: d.borrow().is_ordered()?,
            });
        }

        if let Some(d) = &self.device_traits.brightness {
            attributes.command_only_brightness = Some(d.borrow().is_command_only_brightness()?);
        }

        if let Some(d) = &self.device_traits.camera_stream {
            attributes.camera_stream_supported_protocols = Some(d.borrow().get_supported_camera_stream_protocols()?);
            attributes.camera_stream_need_auth_token = Some(d.borrow().need_auth_token()?);
        }

        if let Some(d) = &self.device_traits.channel {
            attributes.available_channels = Some(d.borrow().get_available_channels()?);
            attributes.command_only_channels = d.borrow().is_command_only_channels()?;
        }

        if let Some(d) = &self.device_traits.color_setting {
            attributes.command_only_color_setting = Some(d.borrow().is_command_only_color_setting()?);
            let support = d.borrow().get_color_model_support()?;
            attributes.color_model = support.color_model;
            attributes.color_temperature_range = support.color_temperature_range;
        }

        if let Some(d) = &self.device_traits.cook {
            attributes.supported_cooking_modes = Some(d.borrow().get_supported_cooking_modes()?);
            attributes.food_presets = Some(d.borrow().get_food_presets()?);
        }

        if let Some(d) = &self.device_traits.dispense {
            attributes.supported_dispense_items = Some(d.borrow().get_supported_dispense_items()?);
            attributes.supported_dispense_presets = Some(d.borrow().get_supported_dispense_presets()?);
        }

        if let Some(d) = &self.device_traits.energy_storage {
            attributes.query_only_energy_storage = Some(d.borrow().is_query_only()?);
            attributes.energy_storage_distance_unit_for_ux = Some(d.borrow().get_distance_unit_for_ux()?);
            attributes.is_rechargeable = Some(d.borrow().is_rechargable()?);
        }

        if let Some(d) = &self.device_traits.fan_speed {
            attributes.reversible = d.borrow().is_reversable()?;
            attributes.command_only_fan_speed = d.borrow().is_command_only_fan_speed()?;
            attributes.available_fan_speeds = d.borrow().get_available_fan_speeds()?;
            attributes.supports_fan_speed_percent = d.borrow().is_support_fan_speed_percent()?;
        }

        if let Some(d) = &self.device_traits.fill {
            attributes.available_fill_levels = Some(d.borrow().get_available_fill_levels()?);
        }

        if let Some(d) = &self.device_traits.humidity_setting {
            attributes.humidity_set_point_range = d.borrow().get_humidity_set_point_range_minmax()?;
            attributes.command_only_humidity_setting = d.borrow().is_command_only_humidity_settings()?;
            attributes.query_only_humidity_setting = d.borrow().is_query_only_humidity_setting()?;
        }

        if let Some(d) = &self.device_traits.input_selector {
            attributes.available_inputs = Some(d.borrow().get_available_inputs()?);
            attributes.command_only_input_selector = d.borrow().is_command_only_input_selector()?;
            attributes.ordered_inputs = d.borrow().has_ordered_inputs()?;
        }

        if let Some(d) = &self.device_traits.light_effects {
            attributes.default_color_loop_duration = d.borrow().get_default_color_loop_duration()?;
            attributes.default_sleep_duration = d.borrow().get_default_sleep_duration()?;
            attributes.default_wake_duration = d.borrow().get_default_wake_duration()?;
            attributes.supported_effects = Some(d.borrow().get_supported_effects()?);
        }

        if let Some(d) = &self.device_traits.media_state {
            attributes.support_activity_state = d.borrow().does_support_activity_state()?;
            attributes.support_playback_state = d.borrow().does_support_playback_state()?;
        }

        if let Some(d) = &self.device_traits.modes {
            attributes.available_modes = Some(d.borrow().get_available_modes()?);
            attributes.command_only_modes = d.borrow().is_command_only_modes()?;
            attributes.query_only_modes = d.borrow().is_query_only_modes()?;
        }

        if let Some(d) = &self.device_traits.network_control {
            attributes.network_profiles = d.borrow().get_network_profiles()?;
            attributes.supports_enabling_guest_network = d.borrow().supports_disabling_guest_network()?;
            attributes.supports_disabling_guest_network = d.borrow().supports_disabling_guest_network()?;
            attributes.supports_getting_guest_network_password = d.borrow().supports_getting_guest_network_password()?;
            attributes.supports_enabling_network_profile = d.borrow().supports_enabling_network_profile()?;
            attributes.supports_disabling_network_profile = d.borrow().supports_disabling_network_profile()?;
            attributes.supports_network_download_speed_test = d.borrow().supports_network_download_speed_test()?;
            attributes.supports_network_upload_speed_test = d.borrow().supports_network_upload_speed_test()?;
        }

        if let Some(d) = &self.device_traits.on_off {
            attributes.command_only_on_off = d.borrow().is_command_only()?;
            attributes.query_only_on_off = d.borrow().is_query_only()?;
        }

        if let Some(d) = &self.device_traits.open_close {
            attributes.discrete_only_open_close = d.borrow().is_discrete_only_open_close()?;
            attributes.open_direction = d.borrow().get_supported_opening_directions()?;
            attributes.command_only_open_close = d.borrow().is_command_only_open_close()?;
            attributes.query_only_open_close = d.borrow().is_query_only_open_close()?;
        }

        if let Some(d) = &self.device_traits.rotation {
            attributes.supports_degrees = Some(d.borrow().supports_degrees()?);
            attributes.supports_percent = Some(d.borrow().supports_percent()?);
            attributes.rotation_degrees_range = Some(d.borrow().get_rotation_degree_range()?);
            attributes.supports_continuous_rotation = d.borrow().supports_continuous_rotation()?;
            attributes.command_only_rotation = d.borrow().is_command_only_rotation()?;
        }

        if let Some(d) = &self.device_traits.scene {
            attributes.scene_reversible = d.borrow().is_reversible()?;
        }

        if let Some(d) = &self.device_traits.sensor_state {
            attributes.sensor_states_supported = Some(d.borrow().get_supported_sensor_states()?);
        }

        if let Some(d) = &self.device_traits.start_stop {
            attributes.pausable = d.borrow().is_pausable()?;
            attributes.available_zones = d.borrow().get_available_zones()?;
        }

        if let Some(d) = &self.device_traits.temperature_control {
            attributes.temperature_range = Some(d.borrow().get_temperature_range()?);
            attributes.temperature_step_celsius = d.borrow().get_temperature_step_celsius()?;
            attributes.temperature_unit_for_ux = Some(d.borrow().get_temperature_unit_for_ux()?);
            attributes.command_only_temperature_control = d.borrow().is_command_only_temperature_control()?;
            attributes.query_only_temperature_control = d.borrow().is_query_only_temperature_control()?;
        }

        if let Some(d) = &self.device_traits.temperature_setting {
            attributes.available_thermostat_modes = Some(d.borrow().get_available_thermostat_modes()?);
            attributes.thermostat_temperature_range = d.borrow().get_thermostat_temperature_range()?;
            attributes.thermostat_temperature_unit = Some(d.borrow().get_thermostat_temperature_unit()?);
            attributes.buffer_range_celsius = d.borrow().get_buffer_range_celsius()?;
            attributes.command_only_temperature_setting = d.borrow().is_command_only_temperature_setting()?;
            attributes.query_only_temperature_setting = d.borrow().is_query_only_temperature_setting()?;
        }

        if let Some(d) = &self.device_traits.timer {
            attributes.max_timer_limit_sec = Some(d.borrow().get_max_timer_limit_sec()?);
            attributes.command_only_timer = d.borrow().is_command_only_timer()?;
        }

        if let Some(d) = &self.device_traits.toggles {
            attributes.available_toggles = Some(d.borrow().get_available_toggles()?);
            attributes.command_only_toggles = d.borrow().is_command_only_toggles()?;
            attributes.query_only_toggles = d.borrow().is_query_only_toggles()?;
        }

        if let Some(d) = &self.device_traits.transport_control {
            attributes.transport_control_supported_commands = Some(d.borrow().get_supported_control_commands()?);
        }

        if let Some(d) = &self.device_traits.volume {
            attributes.volume_max_level = Some(d.borrow().get_volume_max_level()?);
            attributes.volume_can_mute_and_unmute = Some(d.borrow().can_mute_and_unmute()?);
            attributes.volume_default_percentage = d.borrow().get_volume_default_percentage()?;
            attributes.level_step_size = d.borrow().get_level_step_size()?;
            attributes.command_only_volume = d.borrow().is_command_only_volume()?;
        }

        Ok(attributes)
    }

    /// Execute the EXECUTE intent. Handles the error handling, delegates to [Self::execute_inner]
    #[instrument]
    pub(crate) fn execute(&mut self, command: CommandType) -> CommandOutput {
        trace!("Running EXECUTE for device {}", self.id);
        match self.execute_inner(command) {
            Ok(state) => CommandOutput {
                id: self.id.clone(),
                status: CommandStatus::Success,
                state: Some(state),
                error: None,
                debug_string: None,
            },
            Err(e) => match e {
                ExecuteError::Serializable(e) => CommandOutput {
                    id: self.id.clone(),
                    status: CommandStatus::Error,
                    state: None,
                    error: Some(SerializableError(e)),
                    debug_string: None,
                },
                ExecuteError::Server(e) => CommandOutput {
                    // TODO: maybe print the error?
                    id: self.id.clone(),
                    status: CommandStatus::Offline,
                    state: None,
                    error: None,
                    debug_string: Some(e.to_string()),
                },
            },
        }
    }

    /// Execute the EXECUTE intent
    #[instrument]
    fn execute_inner(&mut self, command: CommandType) -> Result<CommandState, ExecuteError> {
        let mut state = CommandState::default();

        match command {
            CommandType::AppInstall {
                new_application,
                new_application_name,
            } => {
                let device = match &mut self.device_traits.app_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(key) = new_application {
                    device.borrow_mut().app_install_key(key)?;
                }

                if let Some(name) = new_application_name {
                    device.borrow_mut().app_install_name(name)?;
                }
            }
            CommandType::AppSearch {
                new_application,
                new_application_name,
            } => {
                let device = match &mut self.device_traits.app_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(key) = new_application {
                    device.borrow_mut().app_search_key(key)?;
                }

                if let Some(name) = new_application_name {
                    device.borrow_mut().app_search_name(name)?;
                }
            }
            CommandType::AppSelect {
                new_application,
                new_application_name,
            } => {
                let device = match &mut self.device_traits.app_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(key) = new_application {
                    device.borrow_mut().app_select_key(key)?;
                }

                if let Some(name) = new_application_name {
                    device.borrow_mut().app_select_name(name)?;
                }
            }
            CommandType::ArmDisarm { arm, cancel, arm_level, .. } => {
                let device = match &mut self.device_traits.arm_disarm {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(cancel) = cancel {
                    if cancel {
                        device.borrow_mut().cancel_arm()?;
                    }
                } else {
                    if let Some(level) = arm_level {
                        device.borrow_mut().arm_with_level(arm, level)?;
                    } else {
                        device.borrow_mut().arm(arm)?;
                    }
                }
            }
            CommandType::BrightnessAbsolute { brightness } => {
                let device = match &mut self.device_traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_brightness_absolute(brightness)?;
            }
            CommandType::BrightnessRelative {
                brightness_relative_percent,
                brightness_relative_weight,
            } => {
                let device = match &mut self.device_traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(brightness_relative_percent) = brightness_relative_percent {
                    device.borrow_mut().set_brightness_relative_percent(brightness_relative_percent)?;
                }

                if let Some(brightness_relative_weight) = brightness_relative_weight {
                    device.borrow_mut().set_brightness_relative_weight(brightness_relative_weight)?;
                }
            }
            CommandType::GetCameraStream {
                stream_to_chromecast,
                supported_stream_protocols,
            } => {
                let device = match &mut self.device_traits.camera_stream {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().get_camera_stream(stream_to_chromecast, supported_stream_protocols)?;
            }
            CommandType::SelectChannel {
                channel_code,
                channel_name,
                channel_number,
            } => {
                let device = match &mut self.device_traits.channel {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(code) = channel_code {
                    device.borrow_mut().select_channel_by_id(code, channel_name, channel_number)?;
                } else if let Some(number) = channel_number {
                    device.borrow_mut().select_channel_by_number(number)?;
                }
            }
            CommandType::RelativeChannel { relative_channel_change } => {
                let device = match &mut self.device_traits.channel {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().select_channel_relative(relative_channel_change)?;
            }
            CommandType::ReturnChannel => {
                let device = match &mut self.device_traits.channel {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().return_to_last_channel()?;
            }
            CommandType::ColorAbsolute { color } => {
                let device = match &mut self.device_traits.color_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_color(color)?;
            }
            CommandType::Cook {
                start,
                cooking_mode,
                food_preset,
                quantity,
                unit,
            } => {
                let device = match &mut self.device_traits.cook {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if start {
                    device.borrow_mut().start(CookingConfig {
                        cooking_mode,
                        food_preset,
                        quantity,
                        unit,
                    })?;
                } else {
                    device.borrow_mut().stop()?;
                }
            }
            CommandType::Dispense {
                item,
                amount,
                unit,
                preset_name,
            } => {
                let device = match &mut self.device_traits.dispense {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(item) = item {
                    // Unwraps are safe, specified in Google spec.
                    // https://developers.google.com/assistant/smarthome/traits/dispense#device-commands
                    let unit = unit.unwrap();
                    let amount = amount.unwrap();

                    device.borrow_mut().dispense_amount(item, amount, unit)?;
                } else if let Some(preset_name) = preset_name {
                    device.borrow_mut().dispense_preset(preset_name)?;
                } else {
                    device.borrow_mut().dispense_default()?;
                }
            }
            CommandType::Dock => {
                let device = match &mut self.device_traits.dock {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().dock()?;
            }
            CommandType::Charge { charge } => {
                let device = match &mut self.device_traits.energy_storage {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().charge(charge)?;
            }
            CommandType::SetFanSpeed { fan_speed, fan_speed_percent } => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(fan_speed) = fan_speed {
                    device.borrow_mut().set_fan_speed_setting(fan_speed)?;
                } else if let Some(fan_speed_percent) = fan_speed_percent {
                    device.borrow_mut().set_fan_speed_percent(fan_speed_percent)?;
                }
            }
            CommandType::SetFanSpeedRelative {
                fan_speed_relative_weight,
                fan_speed_relative_percent,
            } => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(weight) = fan_speed_relative_weight {
                    device.borrow_mut().set_fan_speed_relative_weight(weight)?;
                } else if let Some(percent) = fan_speed_relative_percent {
                    device.borrow_mut().set_fan_speed_relative_percent(percent)?;
                }
            }
            CommandType::Reverse => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_fan_reverse()?;
            }
            CommandType::Fill {
                fill,
                fill_level,
                fill_percent,
            } => {
                let device = match &mut self.device_traits.fill {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(fill_level) = fill_level {
                    device.borrow_mut().fill_to_level(fill_level)?;
                } else if let Some(fill_percent) = fill_percent {
                    device.borrow_mut().fill_to_percent(fill_percent)?;
                } else {
                    device.borrow_mut().fill(fill)?;
                }
            }
            CommandType::SetInput { new_input } => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_input(new_input)?;
            }
            CommandType::NextInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_next_input()?;
            }
            CommandType::PreviousInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_previous_input()?;
            }
            CommandType::ColorLoop { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_color_loop(duration)?;
            }
            CommandType::Sleep { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_sleep(duration)?;
            }
            CommandType::StopEffect => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().stop_effect()?;
            }
            CommandType::Wake { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_wake(duration)?;
            }
            CommandType::Locate { silence, lang } => {
                let device = match &mut self.device_traits.locator {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().locate(Some(silence), Some(lang))?;
            }
            CommandType::LockUnlock { lock, .. } => {
                let device = match &mut self.device_traits.lock_unlock {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_locked(lock)?;

                state.lock = Some(device.borrow().is_locked()?);
            }
            CommandType::SetModes { update_mode_settings } => {
                let device = match &mut self.device_traits.modes {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                for (mode_name, setting_name) in update_mode_settings {
                    device.borrow_mut().update_mode(mode_name, setting_name)?;
                }
            }
            CommandType::EnableDisableGuestNetwork { enable } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_guest_network_enabled(enable)?;
            }
            CommandType::EnableDisableNetworkProfile { enable, profile } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_network_profile_enabled(profile, enable)?;
            }
            CommandType::GetGuestNetworkPassword => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                let password = device.borrow_mut().get_guest_network_password()?;
                state.guest_network_password = Some(password)
            }
            CommandType::TestNetworkSpeed {
                test_upload_speed,
                test_download_speed,
                ..
            } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().test_network_speed(test_download_speed, test_upload_speed)?;
            }
            CommandType::OnOff { on } => {
                let device = match &mut self.device_traits.on_off {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_on(on)?;
            }
            CommandType::OpenClose { open_percent, open_direction } => {
                let device = match &mut self.device_traits.open_close {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_open(open_percent, open_direction)?;
            }
            CommandType::OpenCloseRelative {
                open_relative_percent,
                open_direction,
            } => {
                let device = match &mut self.device_traits.open_close {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_open_relative(open_relative_percent, open_direction)?;
            }
            CommandType::Reboot => {
                let device = match &mut self.device_traits.reboot {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().reboot()?;
            }
            CommandType::RotationAbsolute {
                rotation_degrees,
                rotation_percent,
            } => {
                let device = match &mut self.device_traits.rotation {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(deg) = rotation_degrees {
                    device.borrow_mut().set_rotation_degrees(deg)?;
                } else if let Some(per) = rotation_percent {
                    device.borrow_mut().set_rotation_percent(per)?;
                }
            }
            CommandType::ActivateScene { deactivate } => {
                let device = match &mut self.device_traits.scene {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if deactivate {
                    device.borrow_mut().deactivate()?;
                } else {
                    device.borrow_mut().activate()?;
                }
            }
            CommandType::SoftwareUpdate => {
                let device = match &mut self.device_traits.software_update {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().perform_update()?;
            }
            CommandType::StartStop { start, zone, multiple_zones } => {
                let device = match &mut self.device_traits.start_stop {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                let zones = if let Some(zone) = zone { Some(vec![zone]) } else { multiple_zones };

                device.borrow_mut().start_stop(start, zones)?;
            }
            CommandType::PauseUnpause { pause } => {
                let device = match &mut self.device_traits.start_stop {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().pause_unpause(pause)?;
            }
            CommandType::SetTemperature { temperature } => {
                let device = match &mut self.device_traits.temperature_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_temperature(temperature)?;
            }
            CommandType::ThermostatTemperatureSetpoint {
                thermostat_temperature_setpoint,
            } => {
                let device = match &mut self.device_traits.temperature_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_temperature_setpoint(thermostat_temperature_setpoint)?
            }
            CommandType::ThermostatTemperatureSetRange {
                thermostat_temperature_setpoint_high,
                thermostat_temperature_setpoint_low,
            } => {
                let device = match &mut self.device_traits.temperature_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device
                    .borrow_mut()
                    .set_temperature_set_range(thermostat_temperature_setpoint_high, thermostat_temperature_setpoint_low)?;
            }
            CommandType::ThermostatSetMode { thermostat_mode } => {
                let device = match &mut self.device_traits.temperature_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_thermostat_mode(thermostat_mode)?;
            }
            CommandType::TemperatureRelative {
                thermostat_temperature_relative_degree,
                thermostat_temperature_relative_weight,
            } => {
                let device = match &mut self.device_traits.temperature_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(t) = thermostat_temperature_relative_degree {
                    device.borrow_mut().set_temperature_relative_degree(t)?;
                }

                if let Some(w) = thermostat_temperature_relative_weight {
                    device.borrow_mut().set_temperature_relative_weight(w)?;
                }
            }
            CommandType::TimerStart { timer_time_sec } => {
                let device = match &mut self.device_traits.timer {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().start_timer(timer_time_sec)?;
            }
            CommandType::TimerAdjust { timer_time_sec } => {
                let device = match &mut self.device_traits.timer {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().adjust_timer(timer_time_sec)?;
            }
            CommandType::TimerPause => {
                let device = match &mut self.device_traits.timer {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().pause_timer()?;
            }
            CommandType::TimerResume => {
                let device = match &mut self.device_traits.timer {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().resume_timer()?;
            }
            CommandType::TimerCancel => {
                let device = match &mut self.device_traits.timer {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().cancel_timer()?;
            }
            CommandType::SetToggles { update_toggle_settings } => {
                let device = match &mut self.device_traits.toggles {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                for (k, v) in update_toggle_settings {
                    device.borrow_mut().set_toggle(k, v)?;
                }
            }
            CommandType::MediaStop => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_stop()?;
            }
            CommandType::MediaNext => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_next()?;
            }
            CommandType::MediaPrevious => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_previous()?;
            }
            CommandType::MediaPause => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_pause()?;
            }
            CommandType::MediaResume => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_resume()?;
            }
            CommandType::MediaSeekRelative { relative_position_ms } => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_seek_relative(relative_position_ms)?;
            }
            CommandType::MediaSeekToPosition { abs_position_ms } => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_seek_to_position(abs_position_ms)?;
            }
            CommandType::MediaRepeatMode { is_on, is_single } => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_repeat_mode(is_on, is_single.unwrap_or(false))?;
            }
            CommandType::MediaShuffle => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_shuffle()?;
            }
            CommandType::MediaClosedCaptioningOn {
                closed_captioning_language,
                user_query_language,
            } => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device
                    .borrow_mut()
                    .media_closed_captioning_on(closed_captioning_language, user_query_language)?;
            }
            CommandType::MediaClosedCaptioningOff => {
                let device = match &mut self.device_traits.transport_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().media_closed_captioning_off()?;
            }
            CommandType::Mute { mute } => {
                let device = match &mut self.device_traits.volume {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().mute(mute)?;
            }
            CommandType::SetVolume { volume_level } => {
                let device = match &mut self.device_traits.volume {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_volume(volume_level)?;
            }
            CommandType::VolumeRelative { relative_steps } => {
                let device = match &mut self.device_traits.volume {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.borrow_mut().set_volume_relative(relative_steps)?;
            }
            _ => {}
        }
        Ok(state)
    }

    /// Register the [AppSelector] trait
    pub fn set_app_selector(&mut self)
    where
        T: AppSelector + Sized,
    {
        self.device_traits.app_selector = Some(self.inner.clone());
        self.traits.push(Trait::AppSelector);
    }

    /// Register the [ArmDisarm] trait
    pub fn set_arm_disarm(&mut self)
    where
        T: ArmDisarm + Sized,
    {
        self.device_traits.arm_disarm = Some(self.inner.clone());
        self.traits.push(Trait::ArmDisarm);
    }

    /// Register the [Brightness] trait
    pub fn set_brightness(&mut self)
    where
        T: Brightness + Sized,
    {
        self.device_traits.brightness = Some(self.inner.clone());
        self.traits.push(Trait::Brightness);
    }

    /// Register the [CameraStream] trait
    pub fn set_camera_stream(&mut self)
    where
        T: CameraStream + Sized,
    {
        self.device_traits.camera_stream = Some(self.inner.clone());
        self.traits.push(Trait::CameraStream);
    }

    /// Register the [Channel] trait
    pub fn set_channel(&mut self)
    where
        T: Channel + Sized,
    {
        self.device_traits.channel = Some(self.inner.clone());
        self.traits.push(Trait::Channel);
    }

    /// Register the [ColorSetting] trait
    pub fn set_color_setting(&mut self)
    where
        T: ColorSetting + Sized,
    {
        self.device_traits.color_setting = Some(self.inner.clone());
        self.traits.push(Trait::ColorSetting);
    }

    /// Register the [Cook] trait
    pub fn set_cook(&mut self)
    where
        T: Cook + Sized,
    {
        self.device_traits.cook = Some(self.inner.clone());
        self.traits.push(Trait::Cook);
    }

    /// Register the [Dispense] trait
    pub fn set_dispense(&mut self)
    where
        T: Dispense + Sized,
    {
        self.device_traits.dispense = Some(self.inner.clone());
        self.traits.push(Trait::Dispense);
    }

    /// Register the [Dock] trait
    pub fn set_dock(&mut self)
    where
        T: Dock + Sized,
    {
        self.device_traits.dock = Some(self.inner.clone());
        self.traits.push(Trait::Dock);
    }

    /// Register the [EnergyStorage] trait
    pub fn set_energy_storage(&mut self)
    where
        T: EnergyStorage + Sized,
    {
        self.device_traits.energy_storage = Some(self.inner.clone());
        self.traits.push(Trait::EnergyStorage);
    }

    /// Register the [FanSpeed] trait
    pub fn set_fan_speed(&mut self)
    where
        T: FanSpeed + Sized,
    {
        self.device_traits.fan_speed = Some(self.inner.clone());
        self.traits.push(Trait::FanSpeed);
    }

    /// Register the [InputSelector] trait
    pub fn set_input_selector(&mut self)
    where
        T: InputSelector + Sized,
    {
        self.device_traits.input_selector = Some(self.inner.clone());
        self.traits.push(Trait::InputSelector);
    }

    /// Register the [LightEffects] trait
    pub fn set_light_effects(&mut self)
    where
        T: LightEffects + Sized,
    {
        self.device_traits.light_effects = Some(self.inner.clone());
        self.traits.push(Trait::LightEffects);
    }

    /// Register the [Locator] trait
    pub fn set_locator(&mut self)
    where
        T: Locator + Sized,
    {
        self.device_traits.locator = Some(self.inner.clone());
        self.traits.push(Trait::Locator);
    }

    /// Register the [LockUnlock] trait
    pub fn set_lock_unlock(&mut self)
    where
        T: LockUnlock + Sized,
    {
        self.device_traits.lock_unlock = Some(self.inner.clone());
        self.traits.push(Trait::LockUnlock);
    }

    /// Register the [MediaState] trait
    pub fn set_media_state(&mut self)
    where
        T: MediaState + Sized,
    {
        self.device_traits.media_state = Some(self.inner.clone());
        self.traits.push(Trait::MediaState);
    }

    /// Register the [Modes] trait
    pub fn set_modes(&mut self)
    where
        T: Modes + Sized,
    {
        self.device_traits.modes = Some(self.inner.clone());
        self.traits.push(Trait::Modes);
    }

    /// Register the [NetworkControl] trait
    pub fn set_network_control(&mut self)
    where
        T: NetworkControl + Sized,
    {
        self.device_traits.network_control = Some(self.inner.clone());
        self.traits.push(Trait::NetworkControl);
    }

    /// Register the [OnOff] trait
    pub fn set_on_off(&mut self)
    where
        T: OnOff + Sized,
    {
        self.device_traits.on_off = Some(self.inner.clone());
        self.traits.push(Trait::OnOff);
    }

    /// Register the [OpenClose] trait
    pub fn set_open_close(&mut self)
    where
        T: OpenClose + Sized,
    {
        self.device_traits.open_close = Some(self.inner.clone());
        self.traits.push(Trait::OpenClose);
    }

    /// Register the [Reboot] trait
    pub fn set_reboot(&mut self)
    where
        T: Reboot + Sized,
    {
        self.device_traits.reboot = Some(self.inner.clone());
        self.traits.push(Trait::Reboot);
    }

    /// Register the [Rotation] trait
    pub fn set_rotation(&mut self)
    where
        T: Rotation + Sized,
    {
        self.device_traits.rotation = Some(self.inner.clone());
        self.traits.push(Trait::Rotation);
    }

    /// Register the [RunCycle] trait
    pub fn set_run_cycle(&mut self)
    where
        T: RunCycle + Sized,
    {
        self.device_traits.run_cycle = Some(self.inner.clone());
        self.traits.push(Trait::RunCycle);
    }

    /// Register the [Scene] trait
    pub fn set_scene(&mut self)
    where
        T: Scene + Sized,
    {
        self.device_traits.scene = Some(self.inner.clone());
        self.traits.push(Trait::Scene);
    }

    /// Register the [SensorState] trait
    pub fn set_sensor_state(&mut self)
    where
        T: SensorState + Sized,
    {
        self.device_traits.sensor_state = Some(self.inner.clone());
        self.traits.push(Trait::SensorState);
    }

    /// Register the [SoftwareUpdate] trait
    pub fn set_software_update(&mut self)
    where
        T: SoftwareUpdate + Sized,
    {
        self.device_traits.software_update = Some(self.inner.clone());
        self.traits.push(Trait::SoftwareUpdate);
    }

    /// Register the [StartStop] trait
    pub fn set_start_stop(&mut self)
    where
        T: StartStop + Sized,
    {
        self.device_traits.start_stop = Some(self.inner.clone());
        self.traits.push(Trait::StartStop);
    }

    /// Register the [StatusReport] trait
    pub fn set_status_report(&mut self)
    where
        T: StatusReport + Sized,
    {
        self.device_traits.status_report = Some(self.inner.clone());
        self.traits.push(Trait::StatusReport);
    }

    /// Register the [TemperatureControl] trait
    pub fn set_temperature_control(&mut self)
    where
        T: TemperatureControl + Sized,
    {
        self.device_traits.temperature_control = Some(self.inner.clone());
        self.traits.push(Trait::TemperatureControl);
    }

    /// Register the [TemperatureSetting] trait
    pub fn set_temperature_setting(&mut self)
    where
        T: TemperatureSetting + Sized,
    {
        self.device_traits.temperature_setting = Some(self.inner.clone());
        self.traits.push(Trait::TemperatureSetting);
    }

    /// Register the [Timer] trait
    pub fn set_timer(&mut self)
    where
        T: Timer + Sized,
    {
        self.device_traits.timer = Some(self.inner.clone());
        self.traits.push(Trait::Timer);
    }

    /// Register the [Toggles] trait
    pub fn set_toggles(&mut self)
    where
        T: Toggles + Sized,
    {
        self.device_traits.toggles = Some(self.inner.clone());
        self.traits.push(Trait::Toggles);
    }

    /// Register the [TransportControl] trait
    pub fn set_transport_control(&mut self)
    where
        T: TransportControl + Sized,
    {
        self.device_traits.transport_control = Some(self.inner.clone());
        self.traits.push(Trait::TransportControl)
    }

    /// Register the [Volume] trait
    pub fn set_volume(&mut self)
    where
        T: Volume + Sized,
    {
        self.device_traits.volume = Some(self.inner.clone());
        self.traits.push(Trait::Volume);
    }

    // TODO rest of the traits
}

/// Contains all supported device traits.
/// If the [Option] is empty, then the trait is not registered for the [Device]
#[allow(unused)]
#[derive(Default)]
struct DeviceTraits {
    app_selector: Option<Rc<RefCell<dyn AppSelector>>>,
    arm_disarm: Option<Rc<RefCell<dyn ArmDisarm>>>,
    brightness: Option<Rc<RefCell<dyn Brightness + Send + Sync>>>,
    camera_stream: Option<Rc<RefCell<dyn CameraStream + Send + Sync>>>,
    channel: Option<Rc<RefCell<dyn Channel + Send + Sync>>>,
    color_setting: Option<Rc<RefCell<dyn ColorSetting + Send + Sync>>>,
    cook: Option<Rc<RefCell<dyn Cook + Send + Sync>>>,
    dispense: Option<Rc<RefCell<dyn Dispense + Send + Sync>>>,
    dock: Option<Rc<RefCell<dyn Dock + Send + Sync>>>,
    energy_storage: Option<Rc<RefCell<dyn EnergyStorage + Send + Sync>>>,
    fan_speed: Option<Rc<RefCell<dyn FanSpeed + Send + Sync>>>,
    fill: Option<Rc<RefCell<dyn Fill + Send + Sync>>>,
    humidity_setting: Option<Rc<RefCell<dyn HumiditySetting + Send + Sync>>>,
    input_selector: Option<Rc<RefCell<dyn InputSelector + Send + Sync>>>,
    light_effects: Option<Rc<RefCell<dyn LightEffects + Send + Sync>>>,
    locator: Option<Rc<RefCell<dyn Locator + Send + Sync>>>,
    lock_unlock: Option<Rc<RefCell<dyn LockUnlock + Send + Sync>>>,
    media_state: Option<Rc<RefCell<dyn MediaState + Send + Sync>>>,
    modes: Option<Rc<RefCell<dyn Modes + Send + Sync>>>,
    network_control: Option<Rc<RefCell<dyn NetworkControl + Send + Sync>>>,
    object_detection: Option<Rc<RefCell<dyn ObjectDetection + Send + Sync>>>,
    on_off: Option<Rc<RefCell<dyn OnOff + Send + Sync>>>,
    open_close: Option<Rc<RefCell<dyn OpenClose + Send + Sync>>>,
    reboot: Option<Rc<RefCell<dyn Reboot + Send + Sync>>>,
    rotation: Option<Rc<RefCell<dyn Rotation + Send + Sync>>>,
    run_cycle: Option<Rc<RefCell<dyn RunCycle + Send + Sync>>>,
    sensor_state: Option<Rc<RefCell<dyn SensorState + Send + Sync>>>,
    scene: Option<Rc<RefCell<dyn Scene + Send + Sync>>>,
    software_update: Option<Rc<RefCell<dyn SoftwareUpdate + Send + Sync>>>,
    start_stop: Option<Rc<RefCell<dyn StartStop + Send + Sync>>>,
    status_report: Option<Rc<RefCell<dyn StatusReport + Send + Sync>>>,
    temperature_control: Option<Rc<RefCell<dyn TemperatureControl + Send + Sync>>>,
    temperature_setting: Option<Rc<RefCell<dyn TemperatureSetting + Send + Sync>>>,
    timer: Option<Rc<RefCell<dyn Timer + Send + Sync>>>,
    toggles: Option<Rc<RefCell<dyn Toggles + Send + Sync>>>,
    transport_control: Option<Rc<RefCell<dyn TransportControl + Send + Sync>>>,
    volume: Option<Rc<RefCell<dyn Volume + Send + Sync>>>,
}

impl fmt::Debug for DeviceTraits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceTraits {{ .. }}")
    }
}
