use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::CombinedDeviceError;
use crate::traits::{SizeUnit, Synonym};

#[derive(Debug, Serialize)]
pub struct DispenseItem {
    /// Internal name for the dispensed item. This can be non-user-friendly, and is shared across all languages.
    item_name: String,
    /// Synonyms names for the dispensed in each supported language.
    item_name_synonyms: Vec<Synonym>,
    /// Set of units the device supports for that item.
    supported_units: Vec<SizeUnit>,
    /// Typical amount of the item that may be dispensed.
    default_portion: DispenseAmount,
}

#[derive(Debug, Serialize)]
pub struct DispenseAmount {
    /// Dispensed amount.
    amount: i32,
    /// Dispensed unit.
    unit: SizeUnit
}

/// Preset.
#[derive(Debug, Serialize)]
pub struct DispensePreset {
    /// Internal name for the preset. This name can be non-user-friendly, and is shared across all languages.
    preset_name: String,
    /// Synonym names for the preset in each supported language.
    preset_name_synonyms: Vec<Synonym>,
}

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    /// The user tried to dispense an item or amount from a device that does not have enough amount remaining.
    #[error("DispenseAmountRemainingExceeded")]
    DispenseAmountRemainingExceeded,
    /// The user tried to dispense an amount that is beyond the limits of what
    /// they can ask for in a single query. This is to prevent accidentally
    /// dispensing an excessive amount (for example, 500,000 cups of water).
    #[error("DispenseAmountAboveLimit")]
    DispenseAmountAboveLimit,
    /// The user tried to dispense an item or amount from the device the device that is below the minimum amount it can dispense.
    #[error("DispenseAmountBelowLimit")]
    DispenseAmountBelowLimit,
    /// The user tried to dispense a fractional amount of an item that the
    /// device cannot split (for example, countable items like dog treats may not be divisible by the device).
    #[error("DispenseFractionalAmountNotSupported")]
    DispenseFractionalAmountNotSupported,
    /// The user tries to dispense from a device without specifying an item or preset,
    /// but the device does not support such functionality (for example, a default dispense action).
    #[error("GenericDispenseNotSupported")]
    GenericDispenseNotSupported,
    /// The user tries to dispense from a device with a unit not supported for that case
    /// (for example, the item is not provided so supported_unit validation was skipped).
    #[error("DispenseNotSupported")]
    DispenseNotSupported,
    /// The user tried to dispense a fractional amount of an item which can be split but
    /// not for the particular unit specified (for example, a faucet may be able to dispense 2.7 cups but not 2.7 mL).
    #[error("DispenseFractionalUnitNotSupported")]
    DispenseFractionalUnitNotSupported,
    /// The users tries to dispense an item but the device is already dispensing.
    #[error("DeviceCurrentlyDispensing")]
    DeviceCurrentlyDispensing,
    /// The users tries to dispense an item but the device is clogged.
    #[error("DeviceClogged")]
    DeviceClogged,
    /// The users tries to dispense an item but the device is busy (generic).
    #[error("DeviceBusy")]
    DeviceBusy,
}

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceException {
    /// The user dispenses an item or amount from the device which brings the
    /// amount remaining to a low level. You are responsible for defining what constitutes a "low" level.
    #[error("AmountRemainingLow")]
    AmountRemainingLow,
    /// When the user has to wait before the requested item or amount can
    /// be successfully dispensed (for example, a faucet will dispense hot water
    /// but the user needs to wait for the water to heat up before it begins dispensing).
    #[error("userNeedsToWait")]
    UserNeedsToWait,
}

#[derive(Debug, Error)]
pub enum DispenseError {
    #[error("{0}")]
    Error(DeviceError),
    #[error("{0}")]
    Exception(DeviceException),
    #[error("{0}")]
    Other(CombinedDeviceError)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DispenseItemState {
    /// Name of the item from the item_name attribute.
    item_name: String,
    /// Amount of that item remaining in the device. If the device is currently dispensing,
    /// this reports the remaining amount or what the amount will be once the device finishes dispensing.
    amount_remaining: DispenseAmount,
    /// Amount of that item that the device most recently dispensed. If the device is currently dispensing,
    /// this should report the amount it dispensed prior to the current dispensing amount.
    amount_last_dispensed: DispenseAmount,
    /// Indicates if the device is currently dispensing this item.
    is_currently_dispensing: bool,
}

/// This trait belongs to devices that support dispensing a specified amount of one or more physical items.
/// For example, a dog treat dispenser may dispense a number of treats,
/// a faucet may dispense cups of water, and a pet feeder may dispense both water and pet food.
pub trait Dispense {
    /// Contains information on all the items that the device can dispense.
    fn get_supported_dispense_items(&self) -> Result<Vec<DispenseItem>, DispenseError>;

    /// Presets supported by the device.
    fn get_supported_dispense_presets(&self) -> Result<Vec<DispensePreset>, DispenseError>;

    /// States of the items that the device can dispense.
    fn get_dispense_items_state(&self) -> Result<Vec<DispenseItemState>, DispenseError>;

    /// Dispense by amount.
    fn dispense_amount(&self, item: String, amount: i32, unit: SizeUnit) -> Result<(), DispenseError>;

    /// Dispense by preset.
    fn dispense_preset(&self, preset: String) -> Result<(), DispenseError>;

    /// Dispense without parameters.
    fn dispense_default(&self) -> Result<(), DispenseError>;
}