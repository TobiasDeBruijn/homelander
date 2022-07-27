use crate::traits::CombinedDeviceError;
use serde::{Serialize, Deserialize};

/// Color model support. At least one of the fields has to be [Some]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorModelSupport {
    /// Full spectrum color model supported by the device.
    #[serde(rename = "colorModel")]
    color_model: Option<ColorModel>,
    /// Supported color temperature range in Kelvin.
    #[serde(rename = "colorTemperatureRange")]
    color_temperature_range: Option<ColorTemperature>,
}

/// Supported color temperature range in Kelvin.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorTemperature {
    /// Minimum supported color temperature in Kelvin.
    #[serde(rename = "temperatureMinK")]
    pub temperature_min_k: i32,
    /// Maximum supported color temperature in Kelvin.
    #[serde(rename = "temperatureMaxK")]
    pub temperature_max_k: i32,
}

/// Full spectrum color model supported by the device.
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorModel {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "hsv")]
    Hsv,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    #[serde(rename = "temperatureK")]
    pub temperature_k: Option<i32>,
    #[serde(rename = "spectrumRgb")]
    pub spectrum_rgb: Option<i32>,
    #[serde(rename = "spectrumHsv")]
    pub spectrum_hsv: Option<SpectrumHsv>,
}

/// Coloor to set
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorCommand {
    /// Temperature value in Kelvin
    #[serde(rename = "temperature")]
    Temperature(i32),
    /// Spectrum value as a decimal integer
    #[serde(rename = "spectrumRGB")]
    SpectrumRgb(i32),
    /// Spectrum HSV value
    #[serde(rename = "spectrumHSV")]
    SpectrumHsv(SpectrumHsv),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpectrumHsv {
    pub hue: i32,
    pub saturation: i32,
    pub value: i32,
}

/// This trait applies to devices, such as smart lights, that can change color or color temperature.
pub trait ColorSetting {
    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    fn is_command_only_color_setting(&self) -> Result<bool, CombinedDeviceError>;

    /// Color model support.
    fn get_color_model_support(&self) -> Result<ColorModelSupport, CombinedDeviceError>;

    /// The current color setting currently being used on the device.
    fn get_color(&self) -> Result<Color, CombinedDeviceError>;

    /// Set a color
    fn set_color(&mut self, command: ColorCommand) -> Result<(), CombinedDeviceError>;
}