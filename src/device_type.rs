use convert_case::{Case, Casing};
use serde::Serialize;
use strum_macros::AsRefStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    AcUnit,
    Aircooler,
    Airfreshener,
    Airpurifier,
    AudioVideoReceiver,
    Awning,
    Bathtub,
    Bed,
    Blender,
    Blinds,
    Boiler,
    Camera,
    CarbonMonoxideDetector,
    Charger,
    Closet,
    CoffeeMaker,
    Cooktop,
    Curtain,
    Dehumidifier,
    Dehydrator,
    Dishwasher,
    Door,
    Doorbell,
    Drawer,
    Dryer,
    Fan,
    Faucet,
    Fireplace,
    Freezer,
    Fryer,
    Garage,
    Gate,
    Grill,
    Heater,
    Hood,
    Humidifier,
    Kettle,
    Light,
    Lock,
    Microwave,
    Mop,
    Mower,
    Multicooker,
    Network,
    Outlet,
    Oven,
    Pergola,
    Petfeeder,
    Pressurecooker,
    Radiator,
    Refrigerator,
    Remotecontrol,
    Router,
    Scene,
    SecuritySystem,
    Settop,
    Shower,
    Shutter,
    SmokeDetector,
    Soundbar,
    Sousvide,
    Speaker,
    Sprinkler,
    Standmixer,
    StreamingBox,
    StreamingSoundbar,
    StreamingStick,
    Switch,
    Thermostat,
    Tv,
    Vacuum,
    Valve,
    Washer,
    Waterheater,
    Waterpurifier,
    Watersoftener,
    Window,
    Yogurtmaker,
}

const DEVICE_TYPE_PREFIX: &str = "action.devices.types.";

impl DeviceType {
    pub(crate) fn as_device_type_string(&self) -> String {
        let as_string = self.as_ref();
        let cased = as_string.to_case(Case::ScreamingSnake);
        format!("{DEVICE_TYPE_PREFIX}{cased}")
    }
}

#[cfg(test)]
mod test {
    use super::DeviceType;

    #[test]
    fn test_as_device_type_string() {
        assert_eq!("action.devices.types.OUTLET", DeviceType::Outlet.as_device_type_string());
        assert_eq!("action.devices.types.AC_UNIT", DeviceType::AcUnit.as_device_type_string());
    }
}