#![allow(warnings)]

use std::fmt::Debug;
use crate::traits::*;
use serde::{Serialize, Deserialize};
use crate::cook::{Cook, CookingConfig};
use crate::fulfillment::request::execute::CommandType;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;
use crate::traits::color_setting::ColorSetting;

mod traits;
mod fulfillment;
mod serializable_error;

pub use serializable_error::*;
use crate::dispense::Dispense;
use crate::dock::Dock;
use crate::energy_storage::EnergyStorage;
use crate::fan_speed::FanSpeed;
use crate::fill::Fill;
use crate::humidity_setting::HumiditySetting;
use crate::input_selector::InputSelector;
use crate::light_effects::LightEffects;
use crate::locator::Locator;
use crate::lock_unlock::LockUnlock;
use crate::media_state::MediaState;

pub struct Homelander {
    devices: Vec<Device<dyn GoogleHomeDevice>>
}

impl Homelander {
    pub fn add_device<T>(&mut self, device: Device<dyn GoogleHomeDevice>) {
        self.devices.push(device);
    }

    pub fn remove_device(&mut self, id: String) {
        self.devices.retain(|f| f.id.ne(&id));
    }

    fn execute(&self, _command: DeviceCommand) -> Result<(), Box<dyn std::error::Error>> {
        todo!();
        Ok(())
    }
}

pub struct Device<T: GoogleHomeDevice + ?Sized + 'static> {
    id: String,
    inner: Box<T>,
    traits: DeviceTraits,
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

struct DeviceCommand {
    id: String,
    command: CommandType,
}

impl<T: GoogleHomeDevice + Clone + Send + Sync + ?Sized + 'static> Device<T> {
    pub fn new(device: Box<T>, id: String) -> Self {
        Self {
            id,
            inner: device,
            traits: DeviceTraits::default(),
        }
    }

    fn execute(&mut self, command: CommandType) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            // TODO AppSelector
            CommandType::ArmDisarm { arm, cancel, arm_level, ..} => {
                let device = match &mut self.traits.arm_disarm {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                if let Some(cancel) = cancel {
                    if cancel {
                        device.cancel_arm()?;
                        return Ok(())
                    }
                }

                if let Some(level) = arm_level {
                    device.arm_with_level(arm, level)?;
                } else {
                    device.arm(arm)?;
                }
            },
            CommandType::BrightnessAbsolute { brightness } => {
                let device = match &mut self.traits.brightness {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_brightness_absolute(brightness)?;
            },
            CommandType::BrightnessRelative { brightness_relative_percent, brightness_relative_weight } => {
                let device = match &mut self.traits.brightness {
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
                let device = match &mut self.traits.color_setting {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_color(color)?;
            },
            CommandType::Cook { start, cooking_mode, food_preset, quantity, unit } => {
                let device = match &mut self.traits.cook {
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
                let device = match &mut self.traits.dispense {
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
                let device = match &mut self.traits.dock {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.dock()?;
            },
            CommandType::Charge { charge} => {
                let device = match &mut self.traits.energy_storage {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.charge(charge)?;
            },
            CommandType::SetFanSpeed { fan_speed, fan_speed_percent } => {
                let device = match &mut self.traits.fan_speed {
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
                let device = match &mut self.traits.fan_speed {
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
                let device = match &mut self.traits.fan_speed {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_fan_reverse()?;
            },
            CommandType::Fill { fill, fill_level, fill_percent } => {
                let device = match &mut self.traits.fill {
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
                let device = match &mut self.traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_input(new_input)?;
            },
            CommandType::NextInput => {
                let device = match &mut self.traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_next_input()?;
            },
            CommandType::PreviousInput => {
                let device = match &mut self.traits.input_selector {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_previous_input()?;
            },
            CommandType::ColorLoop { duration } => {
                let device = match &mut self.traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_color_loop(duration)?;
            },
            CommandType::Sleep { duration } => {
                let device = match &mut self.traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_sleep(duration)?;
            },
            CommandType::StopEffect => {
                let device = match &mut self.traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.stop_effect()?;
            },
            CommandType::Wake { duration } => {
                let device = match &mut self.traits.light_effects {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_wake(duration)?;
            },
            CommandType::Locate { silence, lang } => {
                let device = match &mut self.traits.locator {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.locate(Some(silence), Some(lang))?;
            },
            CommandType::LockUnlock { lock, follow_up_token } => {
                let device = match &mut self.traits.lock_unlock {
                    Some(x) => x,
                    None => panic!("Unsupported")
                };

                device.set_locked(lock)?;
                // TODO how do we handle the response?
                // https://developers.google.com/assistant/smarthome/traits/lockunlock#action.devices.commands.lockunlock
            }
            _ => {}
        }
        Ok(())
    }

    pub fn set_app_selector(&mut self) where T: AppSelector {
        todo!()
    }

    pub fn set_arm_disarm(&mut self) where T: ArmDisarm {
        self.traits.arm_disarm = Some(self.inner.clone());
    }

    pub fn set_brightness(&mut self) where T: Brightness {
        self.traits.brightness = Some(self.inner.clone());
    }

    pub fn set_camera_stream(&mut self) where T: CameraStream {
        todo!();
    }

    pub fn set_channel(&mut self) where T: Channel {
        todo!();
    }

    pub fn set_color_setting(&mut self) where T: ColorSetting {
        self.traits.color_setting = Some(self.inner.clone());
    }

    pub fn set_cook(&mut self) where T: Cook {
        self.traits.cook = Some(self.inner.clone());
    }

    pub fn set_dispense(&mut self) where T: Dispense {
        self.traits.dispense = Some(self.inner.clone());
    }

    pub fn set_dock(&mut self) where T: Dock {
        self.traits.dock = Some(self.inner.clone());
    }

    pub fn set_charge(&mut self) where T: EnergyStorage {
        self.traits.energy_storage = Some(self.inner.clone());
    }

    pub fn set_fan_speed(&mut self) where T: FanSpeed {
        self.traits.fan_speed = Some(self.inner.clone());
    }

    pub fn set_input_selector(&mut self) where T: InputSelector {
        self.traits.input_selector = Some(self.inner.clone());
    }

    pub fn set_light_effects(&mut self) where T: LightEffects {
        self.traits.light_effects = Some(self.inner.clone());
    }

    pub fn set_locator(&mut self) where T: Locator {
        self.traits.locator = Some(self.inner.clone());
    }

    pub fn set_lock_unlock(&mut self) where T: LockUnlock {
        self.traits.lock_unlock = Some(self.inner.clone());
    }

    pub fn set_media_state(&mut self) where T: MediaState {
        self.traits.media_state = Some(self.inner.clone());
    }
}

#[cfg(test)]
mod test {
    use crate::{ArmDisarm, CommandType, Device, GoogleHomeDevice};
    use crate::traits::arm_disarm::{ArmDisarmError, ArmLevel};
    use crate::traits::DeviceVersion;

    #[test]
    fn test_dynamic_traits() {
        #[derive(Clone)]
        struct Foo;

        impl GoogleHomeDevice for Foo {
            fn get_version(&self) -> DeviceVersion {
                DeviceVersion {
                    hw: String::default(),
                    sw: String::default(),
                }
            }

            fn get_name(&self) -> String {
                String::default()
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

        let mut device = Device::new(Box::new(Foo), String::default());
        device.set_arm_disarm();
        device.execute(CommandType::ArmDisarm {
            arm: true,
            follow_up_token: None,
            cancel: None,
            arm_level: None,
        }).unwrap();
    }
}

#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum Trait {
    AppSelector,
    ArmDisarm,
    Brightness,
    CameraStream,
    Channel,
    ColorSetting,
    Cook,
    Dispense,
    Dock,
    EnergyStorage,
    FanSpeed,
    Fill,
    HumiditySetting,
    InputSelector,
    LightEffects,
    Locator,
    LockUnlock,
    MediaState,
    Modes,
    NetworkControl,
    ObjectDetection,
    OnOff,
    OpenClose,
    Reboot,
    Rotation,
    RunCycle,
    SensorState,
    Scene,
    SoftwareUpdate,
    StartStop,
    StatusReport,
    TemperatureControl,
    TemperatureSetting,
    Timer,
    Toggles,
    TransportControl,
    Volume,
}