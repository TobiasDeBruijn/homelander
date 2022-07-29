use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Request {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub inputs: Vec<Input>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "intent", content = "payload")]
pub enum Input {
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(execute::Execute),
    #[serde(rename = "action.devices.QUERY")]
    Query(query::Payload),
    #[serde(rename = "action.devices.SYNC")]
    Sync,
}

pub mod query {
    use serde::Deserialize;

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct Payload {
        pub devices: Vec<Device>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct Device {
        pub id: String,
    }
}

pub mod execute {
    use crate::traits::color_setting::ColorCommand;
    use crate::traits::cook::CookingMode;
    use crate::traits::open_close::OpenDirection;
    use crate::traits::{Language, SizeUnit};
    use serde::Deserialize;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct Execute {
        pub commands: Vec<Command>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    pub struct Command {
        pub devices: Vec<Device>,
        pub execution: Vec<CommandType>,
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    pub struct Device {
        pub id: String,
    }

    fn locate_default_lang() -> Language {
        Language::English
    }

    #[derive(Clone, Debug, PartialEq, Deserialize)]
    #[serde(tag = "command", content = "params")]
    pub enum CommandType {
        // TODO AppSelector
        /// Set the alarm level of this device.
        #[serde(rename = "action.devices.commands.ArmDisarm")]
        ArmDisarm {
            /// Google-provided token for follow-up response.
            #[serde(rename = "followUpToken")]
            follow_up_token: Option<String>,
            /// True when command is to arm. False to disarm.
            arm: bool,
            /// True when command is to cancel the arm value.
            cancel: Option<bool>,
            /// The level_name to arm to.
            #[serde(rename = "armLevel")]
            arm_level: Option<String>,
        },
        /// Adjust device absolute brightness.
        #[serde(rename = "action.devices.commands.BrightnessAbsolute")]
        BrightnessAbsolute {
            /// New absolute brightness percentage.
            brightness: i32,
        },
        /// Adjust device relative brightness.
        #[serde(rename = "action.devices.commands.BrightnessRelative")]
        BrightnessRelative {
            /// The exact percentage of brightness to change.
            #[serde(rename = "brightnessRelativePercent")]
            brightness_relative_percent: Option<i32>,
            /// This indicates the ambiguous amount of the brightness change. From small amount to large amount,
            /// this param will be scaled to integer 0 to 5, with the sign to indicate direction.
            #[serde(rename = "brightnessRelativeWeight")]
            brightness_relative_weight: Option<i32>,
        },
        // TODO CameraStream
        // TODO Channel
        /// Set the absolute color value.
        #[serde(rename = "action.devices.commands.ColorAbsolute")]
        ColorAbsolute {
            /// Color to set.
            color: ColorCommand,
        },
        /// Start or stop cooking.
        #[serde(rename = "action.devices.commands.Cook")]
        Cook {
            /// True to start cooking, false to stop current cooking mode.
            start: bool,
            /// Requested cooking mode for the device, from the supportedCookingModes attribute.
            #[serde(rename = "cookingMode")]
            cooking_mode: Option<CookingMode>,
            /// The name of the food preset requested by the user, from foodPresets attribute.
            #[serde(rename = "foodPreset")]
            food_preset: Option<String>,
            /// The quantity of the food requested by the user.
            quantity: Option<i32>,
            /// The unit associated with the quantity, from supported_units attribute.
            unit: Option<SizeUnit>,
        },
        /// Dispense items.
        #[serde(rename = "action.devices.commands.Dispense")]
        Dispense {
            /// Name of the item to dispense, from the item_name attribute.
            item: Option<String>,
            /// Amount to dispense.
            amount: Option<i32>,
            /// Unit for the amount, from the supported_units attribute.
            unit: Option<SizeUnit>,
            /// Name of the preset to dispense, from the preset_name attribute.
            #[serde(rename = "presetName")]
            preset_name: Option<String>,
        },
        /// Dock the device.
        #[serde(rename = "action.devices.commands.Dock")]
        Dock,
        /// Start or stop charging.
        #[serde(rename = "action.devices.commands.Charge")]
        Charge {
            /// True to start charging, false to stop charging.
            charge: bool,
        },
        /// Set speed.
        #[serde(rename = "action.devices.commands.SetFanSpeed")]
        SetFanSpeed {
            /// The requested speed settings of the fan.
            #[serde(rename = "fanSpeed")]
            fan_speed: Option<String>,
            /// The requested speed setting percentage.
            #[serde(rename = "fanSpeedPercent")]
            fan_speed_percent: Option<f32>,
        },
        /// Set relative speed.
        #[serde(rename = "action.devices.commands.SetFanSpeedRelative")]
        SetFanSpeedRelative {
            /// This value indicates the relative amount of the speed change.
            /// The absolute value indicates the scaled amount while the numerical sign indicates the direction of the change.
            #[serde(rename = "fanSpeedRelativeWeight")]
            fan_speed_relative_weight: Option<i32>,
            /// This value represents the percentage of speed to change.
            #[serde(rename = "fanSpeedRelativePercent")]
            fan_speed_relative_percent: Option<f32>,
        },
        /// Reverse fan direction.
        #[serde(rename = "action.devices.commands.Reverse")]
        Reverse,
        /// Fill or drain the device.
        #[serde(rename = "action.devices.commands.Fill")]
        Fill {
            /// True to fill, false to drain.
            fill: bool,
            /// Indicates the level_name from the availableFillLevels attribute to set. If unspecified, fill to the default level.
            #[serde(rename = "fillLevel")]
            fill_level: Option<String>,
            /// Indicates the requested level percentage.
            #[serde(rename = "fillPercent")]
            fill_percent: Option<f32>,
        },
        /// Set the humidity level to an absolute value.
        #[serde(rename = "action.devices.commands.SetHumidity")]
        SetHumidity {
            /// Setpoint humidity percentage. Must fall within humiditySetpointRange.
            humidity: i32,
        },
        /// Adjust the humidity level relative to the current value.
        #[serde(rename = "action.devices.commands.HumidityRelative")]
        HumidityRelative {
            /// The percentage value to adjust the humidity level.
            #[serde(rename = "humidityRelativePercent")]
            humidity_relative_percent: Option<i32>,
            /// Indicates the amount of ambiguous humidity change from a small amount ("a little") to a large amount ("a lot").
            #[serde(rename = "humidityRelativeWeight")]
            humidity_relative_weight: Option<i32>,
        },
        /// Set the media input.
        #[serde(rename = "action.devices.commands.SetInput")]
        SetInput {
            /// Key of the new input
            #[serde(rename = "newInput")]
            new_input: String,
        },
        /// Select the next input. Only applicable when the orderedInputs attribute is set to true.
        #[serde(rename = "action.devices.commands.NextInput")]
        NextInput,
        /// Select the previous input. Only applicable when the orderedInputs attribute is set to true.
        #[serde(rename = "action.devices.commands.PreviousInput")]
        PreviousInput,
        /// Request the device to cycle through a set of colors.
        #[serde(rename = "action.devices.commands.ColorLoop")]
        ColorLoop {
            /// Duration for the color loop command, in seconds.
            duration: Option<i32>,
        },
        /// Gradually lower the device's brightness and, optionally, adjusts the color temperature over a duration of time.
        #[serde(rename = "action.devices.commands.Sleep")]
        Sleep {
            /// Duration for the sleep command, in seconds.
            duration: Option<i32>,
        },
        /// Stop the current light effect.
        #[serde(rename = "action.devices.commands.StopEffect")]
        StopEffect,
        /// Gradually increase the device's brightness and, optionally, adjusts the color temperature over a duration of time.
        #[serde(rename = "actin.devices.commands.Wake")]
        Wake {
            /// Duration for the wake command, in seconds.
            duration: Option<i32>,
        },
        /// Locate the target device by generating a local alert.
        #[serde(rename = "action.devices.commands.Locate")]
        Locate {
            /// For use on devices that make an audible response for local alerts. If set to true, the device should silence any in-progress alarms.
            /// Default: false
            #[serde(default)]
            silence: bool,
            /// Current language of query or display, for return of localized location strings if needed. See [supported languages](https://developers.google.com/assistant/smarthome/traits#supported-languages).
            /// Default: "en"
            #[serde(default = "locate_default_lang")]
            lang: Language,
        },
        /// Lock or unlock the device.
        #[serde(rename = "action.devices.commands.LockUnlock")]
        LockUnlock {
            /// True when command is to lock, false to unlock.
            lock: bool,
            /// Google-provided token for follow-up response.
            #[serde(rename = "followUpToken")]
            follow_up_token: String,
        },
        /// Update mode settings.
        #[serde(rename = "action.devices.commands.SetModes")]
        SetModes {
            /// Key/value pair with the mode name of the device as the key, and the new setting_name as the value.
            #[serde(rename = "updateModeSettings")]
            update_mode_settings: HashMap<String, String>,
        },
        /// Enable or disable the guest network.
        #[serde(rename = "action.devices.commands.EnableDisableGuestNetwork")]
        EnableDisableGuestNetwork {
            /// True to enable the guest network, false to disable the guest network.
            enable: bool,
        },
        /// Enable or disable a network profile.
        #[serde(rename = "action.devices.commands.EnableDisableNetworkProfile")]
        EnableDisableNetworkProfile {
            /// The profile name from networkProfiles attribute.
            profile: String,
            /// True to enable the profile, false to disable the profile.
            enable: bool,
        },
        /// Get the guest network password.
        #[serde(rename = "action.devices.commands.GetGuestNetworkPassword")]
        GetGuestNetworkPassword,
        /// Test the network download and upload speed.
        #[serde(rename = "action.devices.commands.TestNetworkSpeed")]
        TestNetworkSpeed {
            /// Indicates whether the download speed should be tested.
            #[serde(rename = "testDownloadSpeed")]
            test_download_speed: bool,
            /// Indicates whether the upload speed should be tested.
            #[serde(rename = "testUploadSpeed")]
            test_upload_speed: bool,
            /// Google-provided token for follow-up response.
            #[serde(rename = "followUpToken")]
            follow_up_token: String,
        },
        /// Turn the device on or off.
        #[serde(rename = "action.devices.commands.OnOff")]
        OnOff {
            /// Whether to turn the device on or off.
            on: bool,
        },
        /// Set the open-close state of the device.
        #[serde(rename = "action.devices.commands.OpenClose")]
        OpenClose {
            /// Indicates the percentage that a device is opened, where 0 is closed and 100 is fully open.
            #[serde(rename = "openPercent")]
            open_percent: f32,
            /// Direction in which to open. Only present if device supports multiple directions, as indicated by the openDirection attribute, and a direction is specified by the user.
            #[serde(rename = "openDirection")]
            open_direction: Option<OpenDirection>,
        },
        /// Adjust the open-close state of the device relative to the current state.
        #[serde(rename = "action.devices.commands.OpenCloseRelative")]
        OpenCloseRelative {
            /// The exact percentage to change open-close state. Ambigous relative commands will be converted to an exact percentage parameter (for example, "Open the blinds a little more" vs "Open the blinds by 5%").
            #[serde(rename = "oopenRelativePercent")]
            open_relative_percent: f32,
            /// Direction in which to open. Only present if device supports multiple directions, as indicated by the openDirection attribute, and a direction is specified by the user.
            #[serde(rename = "openDirection")]
            open_direction: Option<OpenDirection>,
        },
    }
}

#[cfg(test)]
mod test {
    use crate::fulfillment::request::{Input, Request};
    use crate::CommandType::OnOff;

    #[test]
    fn test_execute_payload() {
        use crate::fulfillment::request::execute::{Command, Device, Execute};

        let payload = r#"
            {
              "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
              "inputs": [
                {
                  "intent": "action.devices.EXECUTE",
                  "payload": {
                    "commands": [
                      {
                        "devices": [
                          {
                            "id": "123",
                            "customData": {
                              "fooValue": 74,
                              "barValue": true,
                              "bazValue": "sheepdip"
                            }
                          },
                          {
                            "id": "456"
                          }
                        ],
                        "execution": [
                          {
                            "command": "action.devices.commands.OnOff",
                            "params": {
                              "on": true
                            }
                          }
                        ]
                      }
                    ]
                  }
                }
              ]
            }
        "#;

        let request = Request {
            request_id: "ff36a3cc-ec34-11e6-b1a0-64510650abcf".to_string(),
            inputs: vec![Input::Execute(Execute {
                commands: vec![Command {
                    devices: vec![Device { id: "123".to_string() }, Device { id: "456".to_string() }],
                    execution: vec![OnOff { on: true }],
                }],
            })],
        };

        let deserialized = serde_json::from_str::<Request>(payload);
        let payload = deserialized.unwrap();
        assert_eq!(request, payload);
    }
}
