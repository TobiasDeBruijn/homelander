use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Request {
    #[serde(rename = "requestId")]
    request_id: String,
    inputs: Vec<Input>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
#[serde(tag = "intent", content = "payload")]
pub enum Input {
    #[serde(rename = "action.devices.EXECUTE")]
    Execute(execute::Execute)
}

#[cfg(test)]
mod test {
    use crate::fulfillment::request::{Input, Request};

    #[test]
    fn test_execute_payload() {
        use crate::fulfillment::request::execute::{Command, Execute, Device, CommandType};

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
            inputs: vec! [
                Input::Execute(Execute {
                    commands: vec! [
                        Command {
                            devices: vec! [
                                Device {
                                    id: "123".to_string(),
                                },
                                Device {
                                    id: "456".to_string()
                                }
                            ],
                            execution: vec! [
                            ]
                        }
                    ]
                })
            ]
        };

        let deserialized = serde_json::from_str::<Request>(payload);
        let payload = deserialized.unwrap();
        assert_eq!(request, payload);
    }
}

pub mod sync {

}

pub mod query {

}

pub mod execute {
    use serde::Deserialize;
    use crate::traits::color_setting::ColorCommand;

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    pub struct Execute {
        pub commands: Vec<Command>,
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    pub struct Command {
        pub devices: Vec<Device>,
        pub execution: Vec<CommandType>
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    pub struct Device {
        pub id: String,
    }

    #[derive(Debug, PartialEq, Eq, Deserialize)]
    #[serde(tag = "command", content = "params")]
    pub enum CommandType {
        // TODO AppSelector
        #[serde(rename = "action.devices.commands.ArmDisarm")]
        ArmDisarm {
            #[serde(rename = "followUpToken")]
            follow_up_token: Option<String>,
            arm: bool,
            cancel: Option<bool>,
            #[serde(rename = "armLevel")]
            arm_level: Option<String>,
        },
        #[serde(rename = "action.devices.commands.BrightnessAbsolute")]
        Brightness {
            brightness: i32,
        },
        // TODO CameraStream
        // TODO Channel
        #[serde(rename = "action.devices.commands.colorAbsolute")]
        ColorSetting {
            color: ColorCommand
        },
    }
}