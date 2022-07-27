use serde::{Serialize, Serializer};
use std::fmt;
use std::error::Error;
use crate::CombinedDeviceError;

pub trait ToStringError: Error + ToString {}

impl<T: Error + ToString> ToStringError for T {}

pub struct SerializableError(pub(crate) Box<dyn ToStringError>);

impl From<CombinedDeviceError> for SerializableError {
    fn from(x: CombinedDeviceError) -> Self {
        Self(x as Box<dyn ToStringError>)
    }
}

impl Serialize for SerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
    {
        let self_string = self.0.to_string();
        serializer.serialize_str(&self_string)
    }
}

impl fmt::Display for SerializableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for SerializableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl Error for SerializableError {}