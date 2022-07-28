use std::collections::HashMap;
use std::error::Error;
use crate::fulfillment::request::execute::CommandType;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;
use crate::traits::color_setting::ColorSetting;

mod device;
mod device_trait;
mod device_type;
mod execute_error;
#[doc(hidden)]
mod fulfillment;
mod serializable_error;
pub mod traits;

use crate::fulfillment::request::Input;
use crate::traits::{CombinedDeviceError, GoogleHomeDevice};
use crate::fulfillment::response::execute::CommandStatus;

pub use serializable_error::*;
pub use device::Device;

pub struct Homelander<T: GoogleHomeDevice + Clone + Send + Sync + 'static> {
    agent_user_id: String,
    devices: Vec<Device<T>>,
}

struct CommandOutput {
    id: String,
    status: CommandStatus,
    state: Option<fulfillment::response::execute::CommandState>,
    error: Option<SerializableError>,
}

impl<T: GoogleHomeDevice + Clone + Send + Sync + 'static> Homelander<T> {
    pub fn add_device(&mut self, device: Device<T>) {
        self.devices.push(device);
    }

    pub fn remove_device<S: AsRef<str>>(&mut self, id: S) {
        self.devices.retain(|f| f.id.ne(id.as_ref()));
    }

    pub fn handle_request(&mut self, request: fulfillment::request::Request) -> fulfillment::response::Response {
        let payload = request
            .inputs
            .into_iter()
            .map(|input| match input {
                Input::Execute(execute) => {
                    let commands = execute
                        .commands
                        .into_iter()
                        .map(|command| {
                            command
                                .devices
                                .into_iter()
                                .map(|device| device.id)
                                .map(|device_id| {
                                    command
                                        .execution
                                        .iter()
                                        .map(|command_type| self.execute(&device_id, command_type.clone()))
                                        .filter_map(|command_output| command_output)
                                        .collect::<Vec<_>>()
                                })
                                .flatten()
                                .collect::<Vec<_>>()
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                        .into_iter()
                        .map(|output| match output.status {
                            CommandStatus::Success | CommandStatus::Exceptions => fulfillment::response::execute::Command {
                                ids: vec![output.id],
                                status: output.status,
                                states: output.state,
                                error_code: None,
                            },
                            CommandStatus::Error => fulfillment::response::execute::Command {
                                ids: vec![output.id],
                                status: CommandStatus::Error,
                                states: None,
                                error_code: output.error,
                            },
                            CommandStatus::Offline | CommandStatus::Pending => fulfillment::response::execute::Command {
                                ids: vec![output.id],
                                status: output.status,
                                states: None,
                                error_code: None,
                            },
                        })
                        .collect::<Vec<_>>();

                    fulfillment::response::ResponsePayload::Execute(fulfillment::response::execute::Payload { commands })
                }
                Input::Sync => fulfillment::response::ResponsePayload::Sync(self.sync()),
                Input::Query(payload) => {
                    let device_states = payload.devices.into_iter()
                        .map(|device| device.id)
                        .map(|device_id| (device_id.clone(), self.devices.iter()
                            .filter(|device| device.id.eq(&device_id))
                            .map(|device| device.query())
                            .collect::<Vec<_>>())
                        )
                        .filter(|(_, device_states)| !device_states.is_empty())
                        .map(|(id, mut device_state)| (id, device_state.remove(0)))
                        .collect::<HashMap<_, _>>();

                    fulfillment::response::ResponsePayload::Query(fulfillment::response::query::Payload {
                        devices: device_states,
                        error_code: None,
                        debug_string: None,
                    })
                }
            })
            .collect::<Vec<_>>()
            .remove(0);

        fulfillment::response::Response {
            request_id: request.request_id,
            payload,
        }
    }

    fn sync(&self) -> fulfillment::response::sync::Payload {
        let devices = self.devices.iter()
            .map(|x| x.sync())
            .collect::<Result<Vec<_>, Box<dyn Error>>>();

        struct PayloadContent {
            devices: Vec<fulfillment::response::sync::Device>,
            error_code: Option<String>,
            debug_string: Option<String,>
        }

        let content = match devices {
            Ok(d) => PayloadContent {
                devices: d,
                error_code: None,
                debug_string: None,
            },
            Err(e) => PayloadContent {
                devices: Vec::with_capacity(0),
                error_code: Some("deviceOffline".to_string()),
                debug_string: Some(e.to_string())
            }
        };
        
        fulfillment::response::sync::Payload {
            agent_user_id: self.agent_user_id.clone(),
            devices: content.devices,
            error_code: content.error_code,
            debug_string: content.debug_string,
        }
    }

    fn execute(&mut self, device_id: &str, command: CommandType) -> Option<CommandOutput> {
        let mut output = self
            .devices
            .iter_mut()
            .filter(|x| x.id.eq(device_id))
            .map(|device| device.execute(command.clone()))
            .collect::<Vec<_>>();

        if output.is_empty() {
            None
        } else {
            Some(output.remove(0))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::device_type::DeviceType;
    use crate::traits::arm_disarm::{ArmDisarmError, ArmLevel};
    use crate::traits::{DeviceInfo, DeviceName, GoogleHomeDevice};
    use crate::{ArmDisarm, CommandType, Device};

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

            fn is_online(&self) -> bool {
                true
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

            fn arm(&mut self, _arm: bool) -> Result<(), ArmDisarmError> {
                Ok(())
            }

            fn cancel_arm(&mut self) -> Result<(), ArmDisarmError> {
                Ok(())
            }

            fn arm_with_level(&mut self, _arm: bool, _level: String) -> Result<(), ArmDisarmError> {
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
