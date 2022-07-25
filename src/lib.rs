use std::any::Any;
use std::fmt::Debug;
use crate::traits::{AppSelector, GoogleHomeDevice};
use serde::{Serialize, Deserialize};
use crate::fulfillment::request::execute::CommandType;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;

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

    fn execute(&self, command: DeviceCommand) -> Result<(), Box<dyn std::error::Error>> {

        Ok(())
    }
}

pub struct Device<T: GoogleHomeDevice + ?Sized + 'static> {
    id: String,
    inner: Box<T>,
    objects: Vec<Box<dyn Any>>,
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
            objects: Vec::new(),
        }
    }

    fn take_item<A: 'static>(&mut self) -> Option<&mut Box<A>> {
        let mut items = self.objects
            .iter_mut()
            .filter(|x| x.is::<A>())
            .collect::<Vec<_>>();
        if items.is_empty() {
            return None;
        }

        let item = items.remove(0);
        item.downcast_mut()
    }

    fn execute(&mut self, command: CommandType) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            CommandType::ArmDisarm { follow_up_token, arm, cancel, arm_level} => {
                let device = match self.take_item::<Box<dyn ArmDisarm>>() {
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
        self.objects.push(Box::new(self.inner.clone() as Box<dyn AppSelector>));
    }

    pub fn set_arm_disarm(&mut self) where T: ArmDisarm {
        self.objects.push(Box::new(self.inner.clone() as Box<dyn ArmDisarm>));
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