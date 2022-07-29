use homelander::{Device, DeviceType, Homelander, Request, Response};
use homelander::fulfillment::request::execute::{Command, CommandType, Execute};
use homelander::fulfillment::request::Input;
use homelander::fulfillment::response::execute::CommandStatus;
use homelander::fulfillment::response::ResponsePayload;
use homelander::traits::{CombinedDeviceError, DeviceInfo, DeviceName, GoogleHomeDevice};
use homelander::traits::on_off::OnOff;

#[derive(Debug)]
struct UltimateSwitch {
    on: bool,
}

impl GoogleHomeDevice for UltimateSwitch {
    fn get_device_info(&self) -> DeviceInfo {
        DeviceInfo {
            manufacturer: "Array21 Development".to_string(),
            model: "UltimateSwitch".to_string(),
            hw: "0.1.0".to_string(),
            sw: "0.1.0".to_string(),
        }
    }

    fn will_report_state(&self) -> bool {
        false
    }

    fn get_device_name(&self) -> DeviceName {
        DeviceName {
            name: "UltimateSwitch".to_string(),
            nicknames: Vec::new(),
            default_names: Vec::new()
        }
    }

    fn is_online(&self) -> bool {
        true
    }
}

impl OnOff for UltimateSwitch {
    fn is_on(&self) -> Result<bool, CombinedDeviceError> {
        Ok(self.on)
    }

    fn set_on(&mut self, on: bool) -> Result<(), CombinedDeviceError> {
        self.on = on;
        Ok(())
    }
}

fn setup_homelander() -> Homelander {
    let switch = UltimateSwitch { on: false };
    let mut device = Device::new(switch, DeviceType::Switch, "00".to_string());
    device.set_on_off();

    let mut homelander= Homelander::new("01".to_string());
    homelander.add_device(device);

    homelander
}

fn get_request_payload() -> Request {
    Request {
        request_id: "02".to_string(),
        inputs: vec! [
            Input::Execute(Execute {
                commands: vec! [
                    Command {
                        devices: vec! [
                            homelander::fulfillment::request::execute::Device {
                                id: "00".to_string(),
                            }
                        ],
                        execution: vec! [
                            CommandType::OnOff {
                                on: true
                            }
                        ]
                    }
                ]
            })
        ]
    }
}

fn get_response_payload() -> Response {
    Response {
        request_id: "02".to_string(),
        payload: ResponsePayload::Execute(homelander::fulfillment::response::execute::Payload {
            commands: vec! [
                homelander::fulfillment::response::execute::Command {
                    debug_string: None,
                    error_code: None,
                    status: CommandStatus::Success,
                    ids: vec! [
                        "00".to_string()
                    ],
                    states: None,
                }
            ]
        })
    }
}

fn main() {
    let mut homelander = setup_homelander();
    let response = homelander.handle_request(get_request_payload());
    assert_eq!(response, get_response_payload());
}