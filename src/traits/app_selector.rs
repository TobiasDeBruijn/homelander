use crate::traits::Language;
use crate::CombinedDeviceError;
use serde::Serialize;

/// Application that users of this device can interact with.
#[derive(Debug, PartialEq, Serialize)]
pub struct AvailableApplication {
    /// Unique key for the application which is not exposed to users in speech or response.
    pub key: String,
    /// Name of each application and its language-specific synonyms.
    pub names: Vec<Name>,
}

/// Application synonyms.
#[derive(Debug, PartialEq, Serialize)]
pub struct Name {
    /// User-friendly synonyms for the application name for a given language. The first synonym is used in the response.
    pub name_synonyms: Vec<String>,
    /// Language code
    pub lang: Language,
}

/// This trait belongs to devices that support media applications, typically from third parties.
///
/// Note: This trait currently supports only user queries and commands in the en-US language.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/appselector>
pub trait AppSelector {
    /// A list of applications. Each application has one or more synonyms in each supported language. The first synonym is used in the response.
    fn get_available_applications(&self) -> Result<Vec<AvailableApplication>, CombinedDeviceError>;

    /// Key value of the current application that is active in the foreground.
    fn get_current_application(&self) -> Result<String, CombinedDeviceError>;

    /// Install the given application.
    /// `key` Key of the application to install.
    fn app_install_key(&mut self, key: String) -> Result<(), CombinedDeviceError>;

    /// Install the given application.
    /// `name` Name of the application to install.
    fn app_install_name(&mut self, name: String) -> Result<(), CombinedDeviceError>;

    /// Search for the given application.
    /// - `key` Key of the application to search for.
    fn app_search_key(&mut self, key: String) -> Result<(), CombinedDeviceError>;

    /// Search for the given application.
    /// - `name` Name of the application to search for.
    fn app_search_name(&mut self, name: String) -> Result<(), CombinedDeviceError>;

    /// Select the given application.
    /// - `key` Key of the application to select.
    fn app_select_key(&mut self, key: String) -> Result<(), CombinedDeviceError>;

    /// Select the given application.
    /// - `name` Name of the application to select.
    fn app_select_name(&mut self, name: String) -> Result<(), CombinedDeviceError>;
}
