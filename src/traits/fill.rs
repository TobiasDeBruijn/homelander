use serde::{Serialize, Deserialize};
use crate::{CombinedDeviceError};
use crate::traits::Language;

/// Describes different levels of filling the device.
#[derive(Debug, Serialize)]
pub struct AvailableFillLevels {
    /// List of level names and language-specific synonyms.
    levels: Vec<FillLevel>,
    /// If true, the device handles additional grammar for increase or decrease logic, as represented by the levels array (for example, half level).
    ordered: bool,
    /// If true, accept commands for adjusting the level to a percentage.
    supports_fill_percent: bool,
}

/// Level name and its language-specific synonyms.
#[derive(Debug, Serialize)]
pub struct FillLevel {
    /// Internal name of the level. This can be non-user-friendly, and will be shared across all languages.
    level_name: String,
    /// Synonyms of the level in each supported language.
    level_values: Vec<LevelValue>,
}

/// Synonyms of the level in a given language.
#[derive(Debug, Serialize)]
pub struct LevelValue {
    /// Synonym of the level. The first string in this list is used as the canonical name of the level in that language.
    level_synonym: Vec<String>,
    /// Language code
    lang: Language
}


/// This trait applies to devices that support being filled, such as a bathtub.
pub trait Fill {
    /// Describes different levels of filling the device.
    fn get_available_fill_levels(&self) -> Result<AvailableFillLevels, CombinedDeviceError>;

    /// True if the device is filled to any level. False if the device is completly drained.
    fn is_filled(&self) -> Result<bool, CombinedDeviceError>;

    /// Required if availableFillLevels attribute is set. Indicates the current level_name from the availableFillLevels attribute at which the device is filled.
    fn get_current_fill_level(&self) -> Result<Option<String>, CombinedDeviceError>;

    /// Required if supportsFillPercent attribute is set. Indicates the current fill level percentage.
    fn get_current_fill_percent(&self) -> Result<Option<bool>, CombinedDeviceError>;

    /// True to fill, false to drain.
    fn fill(&mut self, fill: bool) -> Result<(), CombinedDeviceError>;

    /// Indicates the level_name from the availableFillLevels attribute to set. If unspecified, fill to the default level.
    fn fill_to_level(&mut self, level: String) -> Result<(), CombinedDeviceError>;

    /// Indicates the requested level percentage.
    fn fill_to_percent(&mut self, percent: f32) -> Result<(), CombinedDeviceError>;
}
