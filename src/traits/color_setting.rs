use std::error::Error;
use crate::traits::CombinedDeviceError;
use serde::{Serialize, Deserialize};

/// Color model support. At least one of the fields has to be [Some]
#[derive(Debug, Serialize)]
pub struct ColorModelSupport {
    /// Full spectrum color model supported by the device.
    #[serde(rename = "colorModel")]
    color_model: Option<ColorModel>,
    /// Supported color temperature range in Kelvin.
    #[serde(rename = "colorTemperatureRange")]
    color_temperature_range: Option<ColorTemperature>,
}

/// Supported color temperature range in Kelvin.
#[derive(Debug, Serialize)]
pub struct ColorTemperature {
    /// Minimum supported color temperature in Kelvin.
    #[serde(rename = "temperatureMinK")]
    temperature_min_k: i32,
    /// Maximum supported color temperature in Kelvin.
    #[serde(rename = "temperatureMaxK")]
    temperature_max_k: i32,
}

/// Full spectrum color model supported by the device.
#[derive(Debug, Serialize)]
pub enum ColorModel {
    #[serde(rename = "rgb")]
    Rgb,
    #[serde(rename = "hsv")]
    Hsv,
}

#[derive(Debug, Serialize)]
pub struct Color {
    #[serde(rename = "temperatureK")]
    temperature_k: Option<i32>,
    #[serde(rename = "spectrumRgb")]
    spectrum_rgb: Option<i32>,
    #[serde(rename = "spectrumHsv")]
    spectrum_hsv: Option<SpectrumHsv>,
}

/// Coloor to set
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct SpectrumHsv {
    hue: i32,
    saturation: i32,
    value: i32,
}

/// This trait applies to devices, such as smart lights, that can change color or color temperature.
pub trait ColorSetting {
    type Error: Error;

    /// Indicates if the device supports using one-way (true) or two-way (false) communication. Set this attribute to true if the device cannot respond to a QUERY intent or Report State for this trait.
    fn is_command_only_color_setting(&self) -> Result<bool, CombinedDeviceError<Self::Error>>;

    /// Color model support.
    fn get_color_model_support(&self) -> Result<ColorModelSupport, CombinedDeviceError<Self::Error>>;

    /// The current color setting currently being used on the device.
    fn get_color(&self) -> Result<Color, CombinedDeviceError<Self::Error>>;

    /// Set a color
    fn set_color(&mut self, command: ColorCommand) -> Result<(), CombinedDeviceError<Self::Error>>;
}