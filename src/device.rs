use std::error::Error;
use std::sync::{Arc, Mutex};
use crate::device_trait::Trait;
use crate::device_type::DeviceType;
use crate::execute_error::ExecuteError;
use crate::fulfillment::response::execute::CommandState;
use crate::{ArmDisarm, Brightness, ColorSetting, CommandOutput, CommandStatus, CommandType, fulfillment, GoogleHomeDevice, SerializableError};
use crate::traits::{AppSelector, CameraStream, Channel, ObjectDetection, OpenClose, Reboot, Rotation, RunCycle, Scene, SensorState, SoftwareUpdate, StartStop, StatusReport, TemperatureControl, TemperatureSetting, Timer, TransportControl, Volume};
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
use crate::traits::arm_disarm::AvailableArmLevels;

/// A Google Home device with its traits
pub struct Device<T: GoogleHomeDevice + Send + Sync + ?Sized + 'static> {
    pub(crate) id: String,
    device_type: DeviceType,
    inner: Arc<Mutex<T>>,
    device_traits: DeviceTraits,
    traits: Vec<Trait>,
}

impl<T: GoogleHomeDevice + Send + Sync + ?Sized + 'static> Device<T> {
    /// Create a new ID. Note that the `id` has to be persistent.
    pub fn new(device: Arc<Mutex<T>>, device_type: DeviceType, id: String) -> Self {

        Self {
            id,
            device_type,
            inner: device,
            device_traits: DeviceTraits::default(),
            traits: Vec::new(),
        }
    }

    /// Execute the QUERY intent
    pub(crate) fn query(&self) -> fulfillment::response::query::QueryDeviceState {
        let states = self.query_get_states();
        let states = match states {
            Ok(s) => s,
            Err(e) => return fulfillment::response::query::QueryDeviceState {
                required: fulfillment::response::query::RequiredQueryDeviceState {
                    status: fulfillment::response::query::QueryStatus::Error,
                    on: false,
                    online: self.inner.lock().unwrap().is_online(),
                    error_code: Some(e.to_string()),
                },
                traits: None
            }
        };

        if !self.inner.lock().unwrap().is_online() {
            return fulfillment::response::query::QueryDeviceState {
                required: fulfillment::response::query::RequiredQueryDeviceState {
                    status: fulfillment::response::query::QueryStatus::Offline,
                    on: true,
                    online: false,
                    error_code: None,
                },
                traits: None
            }
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
    fn query_get_states(&self) -> Result<fulfillment::response::query::TraitsQueryDeviceState, Box<dyn Error>> {
        todo!()
    }

    /// Execute the SYNC intent
    pub(crate) fn sync(&self) -> Result<fulfillment::response::sync::Device, Box<dyn Error>> {
        let name = self.inner.lock().unwrap().get_device_name();
        let info = self.inner.lock().unwrap().get_device_info();



        Ok(fulfillment::response::sync::Device {
            id: self.id.clone(),
            device_type: self.device_type.as_device_type_string(),
            traits: self.traits.clone(),
            name: fulfillment::response::sync::DeviceName {
                name: name.name,
                default_names: name.default_names,
                nicknames: name.nicknames,
            },
            will_report_state: self.inner.lock().unwrap().will_report_state(),
            room_hint: self.inner.lock().unwrap().get_room_hint(),
            device_info: fulfillment::response::sync::DeviceInfo {
                manufacturer: info.manufacturer,
                model: info.model,
                hw_version: info.hw,
                sw_version: info.sw,
            },
            attributes: self.sync_set_attributes()?
        })
    }

    /// Collect all attributes for all traits supported by the device
    fn sync_set_attributes(&self) -> Result<fulfillment::response::sync::SyncAttributes, Box<dyn Error>> {
        let mut attributes = fulfillment::response::sync::SyncAttributes::default();

        // TODO appselector

        if let Some(d) = &self.device_traits.arm_disarm {
            attributes.available_arm_levels = Some(AvailableArmLevels {
                levels: d.lock().unwrap().get_available_arm_levels()?,
                ordered: d.lock().unwrap().is_ordered()?
            });
        }

        if let Some(d) = &self.device_traits.brightness {
            attributes.command_only_brightness = Some(d.lock().unwrap().is_command_only_brightness()?);
        }

        // TODO camerastream
        // TODO channel

        if let Some(d) = &self.device_traits.color_setting {
            attributes.command_only_color_setting = Some(d.lock().unwrap().is_command_only_color_setting()?);
            let support = d.lock().unwrap().get_color_model_support()?;
            attributes.color_model = support.color_model;
            attributes.color_temperature_range = support.color_temperature_range;
        }

        if let Some(d) = &self.device_traits.cook {
            attributes.supported_cooking_modes = Some(d.lock().unwrap().get_supported_cooking_modes()?);
            attributes.food_presets = Some(d.lock().unwrap().get_food_presets()?);
        }

        if let Some(d) = &self.device_traits.dispense {
            attributes.supported_dispense_items = Some(d.lock().unwrap().get_supported_dispense_items()?);
            attributes.supported_dispense_presets = Some(d.lock().unwrap().get_supported_dispense_presets()?);
        }

        // Dock has no attributes

        if let Some(d) = &self.device_traits.energy_storage {
            attributes.query_only_energy_storage = Some(d.lock().unwrap().is_query_only()?);
            attributes.energy_storage_distance_unit_for_ux = Some(d.lock().unwrap().get_distance_unit_for_ux()?);
            attributes.is_rechargeable = Some(d.lock().unwrap().is_rechargable()?);
        }

        if let Some(d) = &self.device_traits.fan_speed {
            attributes.reversible = d.lock().unwrap().is_reversable()?;
            attributes.command_only_fan_speed = d.lock().unwrap().is_command_only_fan_speed()?;
            attributes.available_fan_speeds = d.lock().unwrap().get_available_fan_speeds()?;
            attributes.supports_fan_speed_percent = d.lock().unwrap().is_support_fan_speed_percent()?;
        }

        if let Some(d) = &self.device_traits.fill {
            attributes.available_fill_levels = Some(d.lock().unwrap().get_available_fill_levels()?);
        }

        if let Some(d) = &self.device_traits.humidity_setting {
            attributes.humidity_set_point_range = d.lock().unwrap().get_humidity_set_point_range_minmax()?;
            attributes.command_only_humidity_setting = d.lock().unwrap().is_command_only_humidity_settings()?;
            attributes.query_only_humidity_setting = d.lock().unwrap().is_query_only_humidity_setting()?;
        }

        if let Some(d) = &self.device_traits.input_selector {
            attributes.available_inputs = Some(d.lock().unwrap().get_available_inputs()?);
            attributes.command_only_input_selector = d.lock().unwrap().is_command_only_input_selector()?;
            attributes.ordered_inputs = d.lock().unwrap().has_ordered_inputs()?;
        }

        if let Some(d) = &self.device_traits.light_effects {
            attributes.default_color_loop_duration = d.lock().unwrap().get_default_color_loop_duration()?;
            attributes.default_sleep_duration = d.lock().unwrap().get_default_sleep_duration()?;
            attributes.default_wake_duration = d.lock().unwrap().get_default_wake_duration()?;
            attributes.supported_effects = Some(d.lock().unwrap().get_supported_effects()?);
        }

        // Locator has no attributes
        // LockUnlock has no attributes

        if let Some(d) = &self.device_traits.media_state {
            attributes.support_activity_state = d.lock().unwrap().does_support_activity_state()?;
            attributes.support_playback_state = d.lock().unwrap().does_support_playback_state()?;
        }

        if let Some(d) = &self.device_traits.modes {
            attributes.available_modes = Some(d.lock().unwrap().get_available_modes()?);
            attributes.command_only_modes = d.lock().unwrap().is_command_only_modes()?;
            attributes.query_only_modes = d.lock().unwrap().is_query_only_modes()?;
        }

        if let Some(d) = &self.device_traits.network_control {
            attributes.network_profiles = d.lock().unwrap().get_network_profiles()?;
            attributes.supports_enabling_guest_network = d.lock().unwrap().supports_disabling_guest_network()?;
            attributes.supports_disabling_guest_network = d.lock().unwrap().supports_disabling_guest_network()?;
            attributes.supports_getting_guest_network_password = d.lock().unwrap().supports_getting_guest_network_password()?;
            attributes.supports_enabling_network_profile = d.lock().unwrap().supports_enabling_network_profile()?;
            attributes.supports_disabling_network_profile = d.lock().unwrap().supports_disabling_network_profile()?;
            attributes.supports_network_download_speed_test = d.lock().unwrap().supports_network_download_speed_test()?;
            attributes.supports_network_upload_speed_test = d.lock().unwrap().supports_network_upload_speed_test()?;
        }

        // ObjectDetection has no attributes

        if let Some(d) = &self.device_traits.on_off {
            attributes.command_only_on_off = d.lock().unwrap().is_command_only()?;
            attributes.query_only_on_off = d.lock().unwrap().is_query_only()?;
        }

        // TODO the rest of the traits

        Ok(attributes)
    }

    /// Execute the EXECUTE intent. Handles the error handling, delegates to [Self::execute_inner]
    pub(crate) fn execute(&mut self, command: CommandType) -> CommandOutput {
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
                    debug_string: Some(e.to_string())
                },
            },
        }
    }

    /// Execute the EXECUTE intent
    fn execute_inner(&mut self, command: CommandType) -> Result<CommandState, ExecuteError> {
        let mut state = CommandState::default();

        match command {
            // TODO AppSelector
            CommandType::ArmDisarm { arm, cancel, arm_level, .. } => {
                let device = match &mut self.device_traits.arm_disarm {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(cancel) = cancel {
                    if cancel {
                        device.lock().unwrap().cancel_arm()?;
                    }
                } else {
                    if let Some(level) = arm_level {
                        device.lock().unwrap().arm_with_level(arm, level)?;
                    } else {
                        device.lock().unwrap().arm(arm)?;
                    }
                }
            }
            CommandType::BrightnessAbsolute { brightness } => {
                let device = match &mut self.device_traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_brightness_absolute(brightness)?;
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
                    device.lock().unwrap().set_brightness_relative_percent(brightness_relative_percent)?;
                }

                if let Some(brightness_relative_weight) = brightness_relative_weight {
                    device.lock().unwrap().set_brightness_relative_weight(brightness_relative_weight)?;
                }
            }
            // TODO CameraStream
            // TODO Channel
            CommandType::ColorAbsolute { color } => {
                let device = match &mut self.device_traits.color_setting {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_color(color)?;
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
                    device.lock().unwrap().start(CookingConfig {
                        cooking_mode,
                        food_preset,
                        quantity,
                        unit,
                    })?;
                } else {
                    device.lock().unwrap().stop()?;
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

                    device.lock().unwrap().dispense_amount(item, amount, unit)?;
                } else if let Some(preset_name) = preset_name {
                    device.lock().unwrap().dispense_preset(preset_name)?;
                } else {
                    device.lock().unwrap().dispense_default()?;
                }
            }
            CommandType::Dock => {
                let device = match &mut self.device_traits.dock {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().dock()?;
            }
            CommandType::Charge { charge } => {
                let device = match &mut self.device_traits.energy_storage {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().charge(charge)?;
            }
            CommandType::SetFanSpeed { fan_speed, fan_speed_percent } => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                if let Some(fan_speed) = fan_speed {
                    device.lock().unwrap().set_fan_speed_setting(fan_speed)?;
                } else if let Some(fan_speed_percent) = fan_speed_percent {
                    device.lock().unwrap().set_fan_speed_percent(fan_speed_percent)?;
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
                    device.lock().unwrap().set_fan_speed_relative_weight(weight)?;
                } else if let Some(percent) = fan_speed_relative_percent {
                    device.lock().unwrap().set_fan_speed_relative_percent(percent)?;
                }
            }
            CommandType::Reverse => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_fan_reverse()?;
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
                    device.lock().unwrap().fill_to_level(fill_level)?;
                } else if let Some(fill_percent) = fill_percent {
                    device.lock().unwrap().fill_to_percent(fill_percent)?;
                } else {
                    device.lock().unwrap().fill(fill)?;
                }
            }
            CommandType::SetInput { new_input } => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_input(new_input)?;
            }
            CommandType::NextInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_next_input()?;
            }
            CommandType::PreviousInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_previous_input()?;
            }
            CommandType::ColorLoop { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_color_loop(duration)?;
            }
            CommandType::Sleep { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_sleep(duration)?;
            }
            CommandType::StopEffect => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().stop_effect()?;
            }
            CommandType::Wake { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_wake(duration)?;
            }
            CommandType::Locate { silence, lang } => {
                let device = match &mut self.device_traits.locator {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().locate(Some(silence), Some(lang))?;
            }
            CommandType::LockUnlock { lock, .. } => {
                let device = match &mut self.device_traits.lock_unlock {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_locked(lock)?;

                state.lock = Some(device.lock().unwrap().is_locked()?);
            }
            CommandType::SetModes { update_mode_settings } => {
                let device = match &mut self.device_traits.modes {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                for (mode_name, setting_name) in update_mode_settings {
                    device.lock().unwrap().update_mode(mode_name, setting_name)?;
                }
            }
            CommandType::EnableDisableGuestNetwork { enable } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_guest_network_enabled(enable)?;
            }
            CommandType::EnableDisableNetworkProfile { enable, profile } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_network_profile_enabled(profile, enable)?;
            }
            CommandType::GetGuestNetworkPassword => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                let password = device.lock().unwrap().get_guest_network_password()?;
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

                device.lock().unwrap().test_network_speed(test_download_speed, test_upload_speed)?;
            }
            CommandType::OnOff { on } => {
                let device = match &mut self.device_traits.on_off {
                    Some(x) => x,
                    None => panic!("Unsupported"),
                };

                device.lock().unwrap().set_on(on)?;
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
        todo!()
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
        todo!();
    }

    /// Register the [Channel] trait
    pub fn set_channel(&mut self)
    where
        T: Channel + Sized,
    {
        todo!();
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

    // TODO rest of the traits
}

/// Contains all supported device traits.
/// If the [Option] is empty, then the trait is not registered for the [Device]
#[allow(unused)]
#[derive(Default)]
struct DeviceTraits {
    app_selector: Option<Arc<Mutex<dyn AppSelector>>>,
    arm_disarm: Option<Arc<Mutex<dyn ArmDisarm>>>,
    brightness: Option<Arc<Mutex<dyn Brightness + Send + Sync>>>,
    camera_stream: Option<Arc<Mutex<dyn CameraStream + Send + Sync>>>,
    channel: Option<Arc<Mutex<dyn Channel + Send + Sync>>>,
    color_setting: Option<Arc<Mutex<dyn ColorSetting + Send + Sync>>>,
    cook: Option<Arc<Mutex<dyn Cook + Send + Sync>>>,
    dispense: Option<Arc<Mutex<dyn Dispense + Send + Sync>>>,
    dock: Option<Arc<Mutex<dyn Dock + Send + Sync>>>,
    energy_storage: Option<Arc<Mutex<dyn EnergyStorage + Send + Sync>>>,
    fan_speed: Option<Arc<Mutex<dyn FanSpeed + Send + Sync>>>,
    fill: Option<Arc<Mutex<dyn Fill + Send + Sync>>>,
    humidity_setting: Option<Arc<Mutex<dyn HumiditySetting + Send + Sync>>>,
    input_selector: Option<Arc<Mutex<dyn InputSelector + Send + Sync>>>,
    light_effects: Option<Arc<Mutex<dyn LightEffects + Send + Sync>>>,
    locator: Option<Arc<Mutex<dyn Locator + Send + Sync>>>,
    lock_unlock: Option<Arc<Mutex<dyn LockUnlock + Send + Sync>>>,
    media_state: Option<Arc<Mutex<dyn MediaState + Send + Sync>>>,
    modes: Option<Arc<Mutex<dyn Modes + Send + Sync>>>,
    network_control: Option<Arc<Mutex<dyn NetworkControl + Send + Sync>>>,
    object_detection: Option<Arc<Mutex<dyn ObjectDetection + Send + Sync>>>,
    on_off: Option<Arc<Mutex<dyn OnOff + Send + Sync>>>,
    open_close: Option<Arc<Mutex<dyn OpenClose + Send + Sync>>>,
    reboot: Option<Arc<Mutex<dyn Reboot + Send + Sync>>>,
    rotation: Option<Arc<Mutex<dyn Rotation + Send + Sync>>>,
    run_cycle: Option<Arc<Mutex<dyn RunCycle + Send + Sync>>>,
    sensor_state: Option<Arc<Mutex<dyn SensorState + Send + Sync>>>,
    scene: Option<Arc<Mutex<dyn Scene + Send + Sync>>>,
    software_update: Option<Arc<Mutex<dyn SoftwareUpdate + Send + Sync>>>,
    start_stop: Option<Arc<Mutex<dyn StartStop + Send + Sync>>>,
    status_report: Option<Arc<Mutex<dyn StatusReport + Send + Sync>>>,
    temperature_control: Option<Arc<Mutex<dyn TemperatureControl + Send + Sync>>>,
    temperature_setting: Option<Arc<Mutex<dyn TemperatureSetting + Send + Sync>>>,
    timer: Option<Arc<Mutex<dyn Timer + Send + Sync>>>,
    transport_control: Option<Arc<Mutex<dyn TransportControl + Send + Sync>>>,
    volume: Option<Arc<Mutex<dyn Volume + Send + Sync>>>,
}
