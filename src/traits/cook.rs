use serde::{Serialize, Deserialize};
use crate::traits::{SizeUnit, Synonym};
use thiserror::Error;
use crate::CombinedDeviceError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CookingMode {
    None,
    UnknownCookingMode,
    Bake,
    Beat,
    Blend,
    Boil,
    Brew,
    Broil,
    ConvectionBake,
    Cook,
    Defrost,
    Dehydrate,
    Ferment,
    Fry,
    Grill,
    Knead,
    Microwave,
    Mix,
    PressureCook,
    Puree,
    Roast,
    Saute,
    SlowCook,
    SousVide,
    Steam,
    Stew,
    Stir,
    Warm,
    Whip
}

/// Food preset.
#[derive(Serialize)]
pub struct FoodPreset {
    /// Internal name of the food preset, which will be used in commands and states. This name can be non-user-friendly, and is shared across all languages.
    pub food_preset_name: String,
    /// Contains all of the units supported by the device for a specific food.
    pub supported_unit: Vec<SizeUnit>,
    /// Food name synonyms for the preset in each supported language.
    pub food_synonyms: Vec<Synonym>,
}

#[derive(Debug, Serialize, Error)]
#[serde(rename = "lowerCamelCase")]
pub enum CookError {
    #[error("DeviceDoorOpen")]
    DeviceDoorOpen,
    #[error("DeviceLidOpen")]
    DeviceLidOpen,
    #[error("FractionalAmountNotSupported")]
    FractionalAmountNotSupported,
    #[error("AmountAboveLimit")]
    AmountAboveLimit,
    #[error("UnknownFoodPreset")]
    UnknownFoodPreset,
    #[error("{0}")]
    Other(#[from] CombinedDeviceError)
}

#[derive(Debug)]
pub struct CookingConfig {
    /// Requested cooking mode for the device, from the supportedCookingModes attribute.
    pub cooking_mode: Option<CookingMode>,
    /// The name of the food preset requested by the user, from foodPresets attribute.
    pub food_preset: Option<String>,
    /// The quantity of the food requested by the user.
    pub quantity: Option<i32>,
    /// The unit associated with the quantity, from supported_units attribute.
    pub unit: Option<SizeUnit>,
}

/// This trait belongs to devices that can cook food according to various food presets and supported cooking modes.
pub trait Cook {
    /// Cooking modes supported by this device.
    fn get_supported_cooking_modes(&self) -> Result<Vec<CookingMode>, CookError>;

    /// Presets for certain types of food.
    fn get_food_presets(&self) -> Result<Vec<FoodPreset>, CookError>;

    /// Describes the current cooking mode set on the device, from the list of supportedCookingModes attribute. Only one mode may be reported.
    /// If no mode is currently selected, this should be set to [CookingMode::None].
    fn get_current_cooking_mode(&self) -> Result<CookingMode, CookError>;

    /// Describes the current food cooking in the device, from the list of foodPresets attribute.
    /// Only one food may be reported. If no food is currently selected, this should be set to NONE.
    fn get_current_food_preset(&self) -> Result<Option<String>, CookError>;

    /// Defines the current amount of food cooking associated with the currentFoodUnit,
    /// if a quantity was specified. Should not be reported if nothing is currently cooking,
    /// or if there is no quantity associated with this food preset.
    fn get_current_food_quantity(&self) -> Result<Option<i32>, CookError>;

    /// The unit associated with the currentFoodQuantity, from the list of supported_units attribute.
    fn get_current_food_unit(&self) -> Result<Option<SizeUnit>, CookError>;

    /// Start cooking with the provided config
    fn start(&mut self, config: CookingConfig) -> Result<(), CookError>;

    /// Stop the current cooking mode
    fn stop(&mut self) -> Result<(), CookError>;
}
