use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub request_id: String,
    pub payload: ResponsePayload
}

#[derive(Debug, Serialize)]
pub enum ResponsePayload {
    Sync(sync::Payload),
    Execute(execute::Payload,),
}

pub mod sync {
    use serde::Serialize;
    use crate::device_trait::Trait;

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Payload {
        pub agent_user_id: String,
        pub devices: Vec<Device>
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Device {
        pub id: String,
        #[serde(rename = "type")]
        pub device_type: String,
        pub traits: Vec<Trait>,
        pub name: DeviceName,
        pub will_report_state: bool,
        pub room_hint: Option<String>,
        pub device_info: DeviceInfo,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceInfo {
        pub manufacturer: String,
        pub model: String,
        pub hw_version: String,
        pub sw_version: String,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeviceName {
        pub default_names: Vec<String>,
        pub name: String,
        pub nicknames: Vec<String>,
    }
}

pub mod execute {
    use serde::Serialize;
    use crate::serializable_error::SerializableError;

    #[derive(Debug, Serialize)]
    pub struct Payload {
        pub commands: Vec<Command>
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum CommandStatus {
        Success,
        Error,
    }

    #[derive(Debug, Serialize)]
    pub struct Command {
        pub ids: Vec<String>,
        pub status: CommandStatus,
        pub states: Option<CommandState>,
        pub error_code: Option<SerializableError>,
    }

    #[derive(Debug, Serialize)]
    pub struct CommandState {

    }
}