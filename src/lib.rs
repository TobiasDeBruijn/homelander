use std::fmt::Debug;
use crate::traits::*;
use serde::{Serialize, Deserialize};
use crate::fulfillment::request::execute::CommandType;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;
use crate::traits::color_setting::ColorSetting;

mod traits;
mod fulfillment;

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
            _ => {}
        }
        Ok(())
    }

    pub fn set_app_selector(&mut self) where T: AppSelector {
        self.traits.app_selector = Some(self.inner.clone() as Box<dyn AppSelector>);
    }

    pub fn set_arm_disarm(&mut self) where T: ArmDisarm {
        self.traits.arm_disarm = Some(self.inner.clone() as Box<dyn ArmDisarm>);
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