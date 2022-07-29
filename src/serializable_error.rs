use serde::{Serialize, Serializer};
use std::error::Error;
use std::fmt;

pub trait ToStringError: Error + ToString + 'static {}

impl<T: Error + ToString + 'static> ToStringError for T {}

pub struct SerializableError(pub(crate) Box<dyn ToStringError>);

impl PartialEq for SerializableError {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_string().eq(&other.0.to_string())
    }
}

impl Serialize for SerializableError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
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
