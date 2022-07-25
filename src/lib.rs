use std::fmt::Debug;
use rand::Rng;
use crate::traits::{AppSelector, GoogleHomeDevice};
use serde::{Serialize, Deserialize};
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;

mod traits;

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
}

fn generate_id() -> String {
    rand::thread_rng().sample_iter(rand::distributions::Alphanumeric).take(16).map(char::from).collect()
}

pub struct Device<T: GoogleHomeDevice + ?Sized> {
    id: String,
    inner: Box<T>,
    traits: Vec<Trait>,
}

impl<T: GoogleHomeDevice + ?Sized> Device<T> {
    pub fn new(device: Box<T>) -> Self {
        Self {
            id: generate_id(),
            inner: device,
            traits: Vec::new(),
        }
    }

    pub fn set_app_selector(&mut self) where T: AppSelector {
        self.traits.push(Trait::AppSelector);
    }

    pub fn set_arm_disarm(&mut self) where T: ArmDisarm {
        self.traits.push(Trait::ArmDisarm);
    }

    pub fn set_brightness(&mut self) where T: Brightness {
        self.traits.push(Trait::Brightness);
    }

    pub fn
}

#[non_exhaustive]
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