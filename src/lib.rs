//! # Homelander
//! Homelander is a Google Home integration framework. It provides serialization and deserialization for fulfillment requests.
//! It also handles translation between Google Home traits and Rust traits. Furthermore it provides error handling and translating between
//! Rust errors and errors accepted by Google Home.
//!
//! Homelander does *not* provide an OAuth2 server or a web server.
//!
//! ## Getting started
//! To get started, you'll first have to create your own OAuth2 server or use an existing implementation.
//! Refer to [the Google documentation](https://developers.google.com/assistant/smarthome/concepts/account-linking) for details.
//!
//! After you've done this, you've presumably also configured your web server. You can then easily get started with Homelander.
//! Create a Device like so:
//! ```
//! use std::sync::{Arc, Mutex};
//! use homelander::{Device, DeviceType, Homelander};
//! use homelander::traits::{CombinedDeviceError, DeviceInfo, DeviceName, GoogleHomeDevice};
//! use homelander::traits::on_off::OnOff;
//!
//! #[derive(Debug)]
//! struct MyDevice(bool);
//!
//! // Implement the basic GoogleHomeDevice trait,
//! // This gives the basic information required for every device
//! impl GoogleHomeDevice for MyDevice {
//!     fn get_device_info(&self) -> DeviceInfo {
//!         DeviceInfo {
//!             model: "mydevice".to_string(),
//!             manufacturer: "mydevice company".to_string(),
//!             hw: "0.1.0".to_string(),
//!             sw: "0.1.0".to_string(),
//!         }
//!     }
//!
//!     fn will_report_state(&self) -> bool {
//!         false
//!     }
//!
//!     fn get_device_name(&self) -> DeviceName {
//!         DeviceName {
//!             name: "MyDevice".to_string(),
//!             default_names: Vec::new(),
//!             nicknames: Vec::new(),
//!         }
//!     }
//!
//!     fn is_online(&self) -> bool {
//!            true
//!     }
//! }
//!
//! // Implement a device specific trait. E.g. OnOff
//! impl OnOff for MyDevice {
//!     fn is_on(&self) -> Result<bool, CombinedDeviceError> {
//!         Ok(self.0)
//!     }
//!
//!     fn set_on(&mut self, on: bool) -> Result<(), CombinedDeviceError> {
//!         self.0 = on;
//!         Ok(())
//!     }
//! }
//!
//! // Create the device
//! let mut device = Device::new(MyDevice(false), DeviceType::Outlet, "my_id".to_string());
//! // Register the OnOff traitr
//! device.set_on_off();
//!
//! // Create the Homelander struct
//! let mut homelander = Homelander::new("my_user_id".to_string());
//! homelander.add_device(device);
//! ```
//! This will create a basic setup. You can now register a fulfillment route with your webserver.
//! This route should take a JSON payload: [Request]. This request can then be passed to Homelander:
//! ```
//! # use std::sync::{Arc, Mutex};
//! # use homelander::{Device, DeviceTraits, DeviceType, Homelander, Request};
//! # use homelander::fulfillment::request::Input;
//! # use homelander::traits::{CombinedDeviceError, DeviceInfo, DeviceName, GoogleHomeDevice};
//! # use homelander::traits::on_off::OnOff;
//! #
//! # fn get_homelander(_: String) -> Homelander {
//! #    let mut homelander = Homelander::new("my_user_id".to_string());
//! #    let mut device = Device::new(MyDevice(false), DeviceType::Outlet, "my_id".to_string());
//! #    device.set_on_off();
//! #    homelander.add_device(device);
//! #    homelander
//! # }
//! #
//! # #[derive(Debug)]
//! # struct MyDevice(bool);
//! #
//! # impl GoogleHomeDevice for MyDevice {
//! #    fn get_device_info(&self) -> DeviceInfo {
//! #        DeviceInfo {
//! #            model: "mydevice".to_string(),
//! #            manufacturer: "mydevice company".to_string(),
//! #            hw: "0.1.0".to_string(),
//! #            sw: "0.1.0".to_string(),
//! #        }
//! #    }
//! #
//! #    fn will_report_state(&self) -> bool {
//! #        false
//! #    }
//! #
//! #    fn get_device_name(&self) -> DeviceName {
//! #        DeviceName {
//! #            name: "MyDevice".to_string(),
//! #            default_names: Vec::new(),
//! #            nicknames: Vec::new(),
//! #        }
//! #    }
//! #
//! #    fn is_online(&self) -> bool {
//! #           true
//! #    }
//! # }
//! #
//! # impl OnOff for MyDevice {
//! #    fn is_on(&self) -> Result<bool, CombinedDeviceError> {
//! #        Ok(self.0)
//! #    }
//! #
//! #    fn set_on(&mut self, on: bool) -> Result<(), CombinedDeviceError> {
//! #        self.0 = on;
//! #        Ok(())
//! #    }
//! # }
//! #
//! # fn get_incoming_request() -> Request {
//! #    Request {
//! #        request_id: String::default(),
//! #        inputs: vec![
//! #            Input::Sync
//! #        ]
//! #    }
//! # }
//!
//! // Retrieve the Homelander for the user,
//! // The user can be identified through the OAuth2 token provided by Google
//! let mut homelander = get_homelander("my_user_id".to_string());
//! // Let homelander handle the request and create a response
//! // The response can then be returned to Google as JSON
//! let the_request = get_incoming_request(); // Usually you'd get this from your web framework
//! let response = homelander.handle_request(the_request);
//! ```
//!

use crate::fulfillment::request::execute::CommandType;
use crate::fulfillment::request::Input;
use crate::fulfillment::response::execute::CommandStatus;
use crate::traits::arm_disarm::ArmDisarm;
use crate::traits::brightness::Brightness;
use crate::traits::color_setting::ColorSetting;
use crate::traits::{CombinedDeviceError, GoogleHomeDevice};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use tracing::{instrument, trace};

mod device;
mod device_trait;
mod device_type;
mod execute_error;
#[doc(hidden)]
pub mod fulfillment;
mod serializable_error;
pub mod traits;

pub use device::Device;
pub use device_type::DeviceType;
pub use fulfillment::request::Request;
pub use fulfillment::response::Response;
pub use serializable_error::*;

/// The output of an EXECUTE command
struct CommandOutput {
    id: String,
    status: CommandStatus,
    state: Option<fulfillment::response::execute::CommandState>,
    error: Option<SerializableError>,
    debug_string: Option<String>,
}

pub trait DeviceTraits: GoogleHomeDevice + Send + Sync + Debug + 'static {}

impl<T: GoogleHomeDevice + Send + Debug + Sync + 'static> DeviceTraits for T {}

/// Keeps track of all devices owned by a specific user.
#[derive(Debug)]
pub struct Homelander {
    agent_user_id: String,
    devices: Vec<Device<dyn crate::DeviceTraits>>,
}

impl Homelander {
    pub fn new(user_id: String) -> Self {
        Self {
            agent_user_id: user_id,
            devices: Vec::new(),
        }
    }

    /// Add a device
    pub fn add_device<T: DeviceTraits>(&mut self, device: Device<T>) {
        self.devices.push(device.unsize());
    }

    /// Remove a device with ID `id`
    pub fn remove_device<S: AsRef<str>>(&mut self, id: S) {
        self.devices.retain(|f| f.id.ne(id.as_ref()));
    }

    /// Handle an incomming fulfillment request from Google and create a response for it
    #[instrument]
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
                                debug_string: output.debug_string,
                            },
                            CommandStatus::Error => fulfillment::response::execute::Command {
                                ids: vec![output.id],
                                status: CommandStatus::Error,
                                states: None,
                                error_code: output.error,
                                debug_string: output.debug_string,
                            },
                            CommandStatus::Offline | CommandStatus::Pending => fulfillment::response::execute::Command {
                                ids: vec![output.id],
                                status: output.status,
                                states: None,
                                error_code: None,
                                debug_string: output.debug_string,
                            },
                        })
                        .collect::<Vec<_>>();

                    fulfillment::response::ResponsePayload::Execute(fulfillment::response::execute::Payload { commands })
                }
                Input::Sync => fulfillment::response::ResponsePayload::Sync(self.sync()),
                Input::Query(payload) => fulfillment::response::ResponsePayload::Query(self.query(payload)),
            })
            .collect::<Vec<_>>()
            .remove(0);

        fulfillment::response::Response {
            request_id: request.request_id,
            payload,
        }
    }

    /// QUERY all devices specified in `payload`
    #[instrument]
    fn query(&self, payload: fulfillment::request::query::Payload) -> fulfillment::response::query::Payload {
        trace!("Running QUERY operation");

        let device_states = payload
            .devices
            .into_iter()
            .map(|device| device.id)
            .map(|device_id| {
                (
                    device_id.clone(),
                    self.devices
                        .iter()
                        .filter(|device| device.id.eq(&device_id))
                        .map(|device| device.query())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(_, device_states)| !device_states.is_empty())
            .map(|(id, mut device_state)| (id, device_state.remove(0)))
            .collect::<HashMap<_, _>>();

        fulfillment::response::query::Payload {
            devices: device_states,
            error_code: None,
            debug_string: None,
        }
    }

    /// SYNC all devices
    #[instrument]
    fn sync(&self) -> fulfillment::response::sync::Payload {
        trace!("Running SYNC operation");
        let devices = self.devices.iter().map(|x| x.sync()).collect::<Result<Vec<_>, Box<dyn Error>>>();

        struct PayloadContent {
            devices: Vec<fulfillment::response::sync::Device>,
            error_code: Option<String>,
            debug_string: Option<String>,
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
                debug_string: Some(e.to_string()),
            },
        };

        fulfillment::response::sync::Payload {
            agent_user_id: self.agent_user_id.clone(),
            devices: content.devices,
            error_code: content.error_code,
            debug_string: content.debug_string,
        }
    }

    /// EXECUTE `command` on `device_id`
    #[instrument]
    fn execute(&mut self, device_id: &str, command: CommandType) -> Option<CommandOutput> {
        trace!("Running EXECUTE intent");
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
    use crate::{ArmDisarm, CommandType, Device, Homelander};

    #[derive(Clone, Debug)]
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

    #[test]
    fn add_device() {
        let mut device = Device::new(Foo, DeviceType::AcUnit, String::default());
        device.set_arm_disarm();

        let mut homelander = Homelander::new(String::default());
        homelander.add_device(device);
    }

    #[test]
    fn test_dynamic_traits() {
        let mut device = Device::new(Foo, DeviceType::AcUnit, String::default());
        device.set_arm_disarm();
        device.execute(CommandType::ArmDisarm {
            arm: true,
            follow_up_token: None,
            cancel: None,
            arm_level: None,
        });
    }
}
