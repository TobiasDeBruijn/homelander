#![allow(warnings)]

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::process::Output;
use crate::traits::*;
use serde::{Serialize, Deserialize};
use crate::cook::{Cook, CookingConfig};
use crate::fulfillment::request::execute::CommandType;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;
use crate::traits::color_setting::ColorSetting;
use thiserror::Error;

mod traits;
mod fulfillment;
mod serializable_error;
mod device_type;
mod device_trait;

pub use serializable_error::*;
use crate::arm_disarm::ArmDisarmError;
use crate::device_trait::Trait;
use crate::device_type::DeviceType;
use crate::dispense::Dispense;
use crate::dock::Dock;
use crate::energy_storage::EnergyStorage;
use crate::fan_speed::FanSpeed;
use crate::fill::Fill;
use crate::fulfillment::request::Input;
use crate::fulfillment::response::execute::{CommandState, CommandStatus};
use crate::fulfillment::response::Response;
use crate::humidity_setting::HumiditySetting;
use crate::input_selector::InputSelector;
use crate::light_effects::LightEffects;
use crate::locator::Locator;
use crate::lock_unlock::LockUnlock;
use crate::media_state::MediaState;
use crate::modes::Modes;
use crate::network_control::NetworkControl;
use crate::on_off::OnOff;

pub struct Homelander<T: GoogleHomeDevice + Clone + Send + Sync + 'static> {
    agent_user_id: String,
    devices: Vec<Device<T>>
}

struct CommandOutput {
    id: String,
    status: crate::fulfillment::response::execute::CommandStatus,
    state: Option<crate::fulfillment::response::execute::CommandState>,
    error: Option<SerializableError>,
}

type BoxResult<T> = Result<T, Box<dyn Error>>;

impl<T: GoogleHomeDevice + Clone + Send + Sync + 'static> Homelander<T> {
    pub fn add_device(&mut self, device: Device<T>) {
        self.devices.push(device);
    }

    pub fn remove_device<S: AsRef<str>>(&mut self, id: S) {
        self.devices.retain(|f| f.id.ne(id.as_ref()));
    }

    pub fn handle_request(&mut self, request: fulfillment::request::Request) -> Result<fulfillment::response::Response, Box<dyn Error>> {
        let payload = request.inputs.into_iter()
            .map(|input| match input {
                Input::Execute(execute) => {
                    let commands = execute.commands.into_iter()
                        .map(|command| Ok(command.devices.into_iter()
                                .map(|device| device.id)
                                .map(|device_id| Ok(command.execution.iter()
                                    .map(|command_type| self.execute(&device_id, command_type.clone()))
                                    .collect::<BoxResult<Vec<_>>>()?
                                    .into_iter()
                                    .filter_map(|command_output| command_output)
                                    .collect::<Vec<_>>()
                                ))
                                .collect::<BoxResult<Vec<_>>>()?
                                .into_iter()
                                .flatten()
                                .collect::<Vec<_>>()
                        ))
                        .collect::<BoxResult<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|output| {
                            match output.status {
                                CommandStatus::Success => fulfillment::response::execute::Command {
                                    ids: vec![output.id],
                                    status: CommandStatus::Success,
                                    states: output.state,
                                    error_code: None,
                                },
                                CommandStatus::Error => {
                                    fulfillment::response::execute::Command {
                                        ids: vec![output.id],
                                        status: CommandStatus::Error,
                                        states: None,
                                        error_code: output.error,
                                    }
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    Ok(fulfillment::response::ResponsePayload::Execute(fulfillment::response::execute::Payload {
                        commands
                    }))
                },
                Input::Sync => {
                    Ok(fulfillment::response::ResponsePayload::Sync(self.sync()))
                },
                // TODO QUERY
            })
            .collect::<BoxResult<Vec<_>>>()?
            .remove(0);

        Ok(fulfillment::response::Response {
            request_id: request.request_id,
            payload
        })
    }

    fn sync(&self) -> fulfillment::response::sync::Payload {
        let devices = self.devices.iter()
            .map(|x| x.sync())
            .collect::<Vec<_>>();

        fulfillment::response::sync::Payload {
            agent_user_id: self.agent_user_id.clone(),
            devices
        }
    }

    fn execute(&mut self, device_id: &str, command: CommandType) -> Result<Option<CommandOutput>, Box<dyn Error>> {
        let mut output = self.devices.iter_mut()
            .filter(|x| x.id.eq(device_id))
            .map(|device| device.execute(command.clone()))
            .collect::<Vec<_>>();

        if output.is_empty() {
            Ok(None)
        } else {
            Ok(Some(output.remove(0)?))
        }
    }
}

pub struct Device<T: GoogleHomeDevice + Clone + Send + Sync + ?Sized + 'static> {
    id: String,
    device_type: DeviceType,
    inner: Box<T>,
    device_traits: DeviceTraits,
    traits: Vec<Trait>,
}

#[derive(Default)]
pub struct DeviceTraits {
    app_selector: Option<Box<dyn AppSelector>>,
    arm_disarm: Option<Box<dyn ArmDisarm>>,
    brightness: Option<Box<dyn Brightness>>,
    camera_stream: Option<Box<dyn CameraStream>>,
    channel: Option<Box<dyn Channel>>,
    color_setting: Option<Box<dyn ColorSetting>>,
    cook: Option<Box<dyn Cook>>,
    dispense: Option<Box<dyn Dispense>>,
    dock: Option<Box<dyn Dock>>,
    energy_storage: Option<Box<dyn EnergyStorage>>,
    fan_speed: Option<Box<dyn FanSpeed>>,
    fill: Option<Box<dyn Fill>>,
    humidity_setting: Option<Box<dyn HumiditySetting>>,
    input_selector: Option<Box<dyn InputSelector>>,
    light_effects: Option<Box<dyn LightEffects>>,
    locator: Option<Box<dyn Locator>>,
    lock_unlock: Option<Box<dyn LockUnlock>>,
    media_state: Option<Box<dyn MediaState>>,
    modes: Option<Box<dyn Modes>>,
    network_control: Option<Box<dyn NetworkControl>>,
    object_detection: Option<Box<dyn ObjectDetection>>,
    on_off: Option<Box<dyn OnOff>>,
    open_close: Option<Box<dyn OpenClose>>,
    reboot: Option<Box<dyn Reboot>>,
    rotation: Option<Box<dyn Rotation>>,
    run_cycle: Option<Box<dyn RunCycle>>,
    sensor_state: Option<Box<dyn SensorState>>,
    scene: Option<Box<dyn Scene>>,
    software_update: Option<Box<dyn SoftwareUpdate>>,
    start_stop: Option<Box<dyn StartStop>>,
    status_report: Option<Box<dyn StatusReport>>,
    temperature_control: Option<Box<dyn TemperatureControl>>,
    temperature_setting: Option<Box<dyn TemperatureSetting>>,
    timer: Option<Box<dyn Timer>>,
    transport_control: Option<Box<dyn TransportControl>>,
    volume: Option<Box<dyn Volume>>,
}

#[derive(Debug, Error)]
pub enum ExecuteError {
    #[error("{0}")]
    Serializable(#[from] SerializableError),
    #[error("{0}")]
    Server(#[from] Box<dyn Error>),
}

impl<T: GoogleHomeDevice + Clone + Send + Sync + ?Sized + 'static> Device<T> {
    pub fn new(device: Box<T>, device_type: DeviceType, id: String) -> Self {
        Self {
            id,
            device_type,
            inner: device,
            device_traits: DeviceTraits::default(),
            traits: Vec::new(),
        }
    }

    pub(crate) fn sync(&self) -> fulfillment::response::sync::Device {
        let name = self.inner.get_device_name();
        let info = self.inner.get_device_info();

        fulfillment::response::sync::Device {
            id: self.id.clone(),
            device_type: self.device_type.as_device_type_string(),
            traits: self.traits.clone(),
            name: fulfillment::response::sync::DeviceName {
                name: name.name,
                default_names: name.default_names,
                nicknames: name.nicknames
            },
            will_report_state: self.inner.will_report_state(),
            room_hint: self.inner.get_room_hint(),
            device_info: fulfillment::response::sync::DeviceInfo {
                manufacturer: info.manufacturer,
                model: info.model,
                hw_version: info.hw,
                sw_version: info.sw,
            },
        }
    }

    pub(crate) fn execute(&mut self, command: CommandType) -> Result<CommandOutput, Box<dyn Error>> {
        match self.execute_inner(command) {
            Ok(state) => Ok(CommandOutput {
                id: self.id.clone(),
                status: CommandStatus::Success,
                state: Some(state),
                error: None,
            }),
            Err(e) => match e {
                ExecuteError::Serializable(e) => Ok(CommandOutput {
                    id: self.id.clone(),
                    status: CommandStatus::Error,
                    state: None,
                    error: Some(e)
                }),
                ExecuteError::Server(e) => Err(e)
            }
        }
    }

    fn execute_inner(&mut self, command: CommandType) -> Result<CommandState, ExecuteError> {
        match command {
            // TODO AppSelector
            CommandType::ArmDisarm { arm, cancel, arm_level, ..} => {
                let device = match &mut self.device_traits.arm_disarm {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(cancel) = cancel {
                    if cancel {
                        match device.cancel_arm() {
                            Ok(_) => {},
                            Err(e) => match e {
                                ArmDisarmError::Other(e) => return Err(ExecuteError::Server(e as Box<dyn Error>)),
                                _ => return Err(ExecuteError::Serializable(SerializableError(e as Box<dyn ToStringError>)))
                            }
                        }
                        return Ok(())
                        // TODO
                    }
                }

                if let Some(level) = arm_level {
                    device.arm_with_level(arm, level)?;
                } else {
                    device.arm(arm)?;
                }
            },
            CommandType::BrightnessAbsolute { brightness } => {
                let device = match &mut self.device_traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_brightness_absolute(brightness)?;
            },
            CommandType::BrightnessRelative { brightness_relative_percent, brightness_relative_weight } => {
                let device = match &mut self.device_traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(brightness_relative_percent) = brightness_relative_percent {
                    device.set_brightness_relative_percent(brightness_relative_percent)?;
                }

                if let Some(brightness_relative_weight) = brightness_relative_weight {
                    device.set_brightness_relative_weight(brightness_relative_weight)?;
                }
            },
            // TODO CameraStream
            // TODO Channel
            CommandType::ColorAbsolute { color } => {
                let device = match &mut self.device_traits.color_setting {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_color(color)?;
            },
            CommandType::Cook { start, cooking_mode, food_preset, quantity, unit } => {
                let device = match &mut self.device_traits.cook {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if start {
                    device.start(CookingConfig {
                        cooking_mode,
                        food_preset,
                        quantity,
                        unit,
                    })?;
                } else {
                    device.stop()?;
                }
            },
            CommandType::Dispense { item, amount, unit, preset_name } => {
                let device = match &mut self.device_traits.dispense {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(item) = item {
                    // Unwraps are safe, specified in Google spec.
                    // https://developers.google.com/assistant/smarthome/traits/dispense#device-commands
                    let unit = unit.unwrap();
                    let amount = amount.unwrap();

                    device.dispense_amount(item, amount, unit)?;
                } else if let Some(preset_name) = preset_name {
                    device.dispense_preset(preset_name)?;
                } else {
                    device.dispense_default()?;
                }
            },
            CommandType::Dock => {
                let device = match &mut self.device_traits.dock {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.dock()?;
            },
            CommandType::Charge { charge} => {
                let device = match &mut self.device_traits.energy_storage {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.charge(charge)?;
            },
            CommandType::SetFanSpeed { fan_speed, fan_speed_percent } => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(fan_speed) = fan_speed {
                    device.set_fan_speed_setting(fan_speed)?;
                } else if let Some(fan_speed_percent) = fan_speed_percent {
                    device.set_fan_speed_percent(fan_speed_percent)?;
                }
            },
            CommandType::SetFanSpeedRelative { fan_speed_relative_weight, fan_speed_relative_percent } => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(weight) = fan_speed_relative_weight {
                    device.set_fan_speed_relative_weight(weight)?;
                } else if let Some(percent) = fan_speed_relative_percent {
                    device.set_fan_speed_relative_percent(percent)?;
                }
            },
            CommandType::Reverse => {
                let device = match &mut self.device_traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_fan_reverse()?;
            },
            CommandType::Fill { fill, fill_level, fill_percent } => {
                let device = match &mut self.device_traits.fill {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(fill_level) = fill_level {
                    device.fill_to_level(fill_level)?;
                } else if let Some(fill_percent) = fill_percent {
                    device.fill_to_percent(fill_percent)?;
                } else {
                    device.fill(fill)?;
                }
            },
            CommandType::SetInput { new_input } => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_input(new_input)?;
            },
            CommandType::NextInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_next_input()?;
            },
            CommandType::PreviousInput => {
                let device = match &mut self.device_traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_previous_input()?;
            },
            CommandType::ColorLoop { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_color_loop(duration)?;
            },
            CommandType::Sleep { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_sleep(duration)?;
            },
            CommandType::StopEffect => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.stop_effect()?;
            },
            CommandType::Wake { duration } => {
                let device = match &mut self.device_traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_wake(duration)?;
            },
            CommandType::Locate { silence, lang } => {
                let device = match &mut self.device_traits.locator {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.locate(Some(silence), Some(lang))?;
            },
            CommandType::LockUnlock { lock, follow_up_token } => {
                let device = match &mut self.device_traits.lock_unlock {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_locked(lock)?;
                // TODO how do we handle the response?
                // https://developers.google.com/assistant/smarthome/traits/lockunlock#action.devices.commands.lockunlock
            },
            CommandType::SetModes { update_mode_settings } => {
                let device = match &mut self.device_traits.modes {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                for (mode_name, setting_name) in update_mode_settings {
                    device.update_mode(mode_name, setting_name)?;
                }
            },
            CommandType::EnableDisableGuestNetwork { enable } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_guest_network_enabled(enable)?;
            },
            CommandType::EnableDisableNetworkProfile { enable, profile } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_network_profile_enabled(profile, enable)?;
            },
            CommandType::GetGuestNetworkPassword => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                let password = device.get_guest_network_password()?;
                // TODO handle response
            },
            CommandType::TestNetworkSpeed { test_upload_speed, test_download_speed, .. } => {
                let device = match &mut self.device_traits.network_control {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.test_network_speed(test_download_speed, test_upload_speed)?;
            },
            CommandType::OnOff { on } => {
                let device = match &mut self.device_traits.on_off {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_on(on)?;
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_app_selector(&mut self) where T: AppSelector {
        todo!()
    }

    pub fn set_arm_disarm(&mut self) where T: ArmDisarm {
        self.device_traits.arm_disarm = Some(self.inner.clone());
        self.traits.push(Trait::ArmDisarm);
    }

    pub fn set_brightness(&mut self) where T: Brightness {
        self.device_traits.brightness = Some(self.inner.clone());
        self.traits.push(Trait::Brightness);
    }

    pub fn set_camera_stream(&mut self) where T: CameraStream {
        todo!();
    }

    pub fn set_channel(&mut self) where T: Channel {
        todo!();
    }

    pub fn set_color_setting(&mut self) where T: ColorSetting {
        self.device_traits.color_setting = Some(self.inner.clone());
        self.traits.push(Trait::ColorSetting);
    }

    pub fn set_cook(&mut self) where T: Cook {
        self.device_traits.cook = Some(self.inner.clone());
        self.traits.push(Trait::Cook);
    }

    pub fn set_dispense(&mut self) where T: Dispense {
        self.device_traits.dispense = Some(self.inner.clone());
        self.traits.push(Trait::Dispense);
    }

    pub fn set_dock(&mut self) where T: Dock {
        self.device_traits.dock = Some(self.inner.clone());
        self.traits.push(Trait::Dock);
    }

    pub fn set_energy_storage(&mut self) where T: EnergyStorage {
        self.device_traits.energy_storage = Some(self.inner.clone());
        self.traits.push(Trait::EnergyStorage);
    }

    pub fn set_fan_speed(&mut self) where T: FanSpeed {
        self.device_traits.fan_speed = Some(self.inner.clone());
        self.traits.push(Trait::FanSpeed);
    }

    pub fn set_input_selector(&mut self) where T: InputSelector {
        self.device_traits.input_selector = Some(self.inner.clone());
        self.traits.push(Trait::InputSelector);
    }

    pub fn set_light_effects(&mut self) where T: LightEffects {
        self.device_traits.light_effects = Some(self.inner.clone());
        self.traits.push(Trait::LightEffects);
    }

    pub fn set_locator(&mut self) where T: Locator {
        self.device_traits.locator = Some(self.inner.clone());
        self.traits.push(Trait::Locator);
    }

    pub fn set_lock_unlock(&mut self) where T: LockUnlock {
        self.device_traits.lock_unlock = Some(self.inner.clone());
        self.traits.push(Trait::LockUnlock);
    }

    pub fn set_media_state(&mut self) where T: MediaState {
        self.device_traits.media_state = Some(self.inner.clone());
        self.traits.push(Trait::MediaState);
    }

    pub fn set_modes(&mut self) where T: Modes {
        self.device_traits.modes = Some(self.inner.clone());
        self.traits.push(Trait::Modes);
    }

    pub fn set_network_control(&mut self) where T: NetworkControl {
        self.device_traits.network_control = Some(self.inner.clone());
        self.traits.push(Trait::NetworkControl);
    }

    pub fn set_on_off(&mut self) where T: OnOff {
        self.device_traits.on_off = Some(self.inner.clone());
        self.traits.push(Trait::OnOff);
    }
}

#[cfg(test)]
mod test {
    use crate::{ArmDisarm, CommandType, Device, DeviceName, DeviceType, GoogleHomeDevice};
    use crate::traits::arm_disarm::{ArmDisarmError, ArmLevel};
    use crate::traits::DeviceInfo;

    #[test]
    fn test_dynamic_traits() {
        #[derive(Clone)]
        struct Foo;

        impl GoogleHomeDevice for Foo {
            fn get_device_info(&self) -> DeviceInfo {
                DeviceInfo {
                    manufacturer: String::default(),
                    model: String::default(),
                    hw: String::default(),
                    sw: String::default(),
                }
            }

            fn will_report_state(&self) -> bool {
                false
            }

            fn get_device_name(&self) -> DeviceName {
                DeviceName {
                    nicknames: Vec::new(),
                    default_names: Vec::new(),
                    name: String::default(),
                }
            }
        }

        impl ArmDisarm for Foo {
            fn get_available_arm_levels(&self) -> Result<Option<Vec<ArmLevel>>, ArmDisarmError> {
                Ok(None)
            }

            fn is_ordered(&self) -> Result<bool, ArmDisarmError> {
                Ok(true)
            }

            fn is_armed(&self) -> Result<bool, ArmDisarmError> {
                Ok(true)
            }

            fn current_arm_level(&self) -> Result<String, ArmDisarmError> {
                Ok(String::default())
            }

            fn exit_allowance(&self) -> Result<i32, ArmDisarmError> {
                Ok(0)
            }

            fn arm(&mut self, arm: bool) -> Result<(), ArmDisarmError> {
                Ok(())
            }

            fn cancel_arm(&mut self) -> Result<(), ArmDisarmError> {
                Ok(())
            }

            fn arm_with_level(&mut self, arm: bool, level: String) -> Result<(), ArmDisarmError> {
                Ok(())
            }
        }

        let mut device = Device::new(Box::new(Foo), DeviceType::AcUnit, String::default());
        device.set_arm_disarm();
        device.execute(CommandType::ArmDisarm {
            arm: true,
            follow_up_token: None,
            cancel: None,
            arm_level: None,
        });
    }
}