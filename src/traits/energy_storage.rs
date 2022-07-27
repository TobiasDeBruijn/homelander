use serde::Serialize;
use thiserror::Error;
use crate::CombinedDeviceError;

#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    /// The user tried to charge a device that is not plugged in.
    #[error("DeviceUnplugged")]
    DeviceUnplugged,
}

#[derive(Debug, Error)]
pub enum EnergyStorageError {
    #[error("{0}")]
    Device(DeviceError),
    #[error("{0}")]
    Other(CombinedDeviceError),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UxDistanceUnit {
    Kilometers,
    Miles,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapacityState {
    CriticallyLow,
    Low,
    Medium,
    High,
    Full,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapacityUnit {
    Seconds,
    Miles,
    Kilometers,
    Percentage,
    KilowattHours
}

pub struct CapacityValue {
    /// The capacity value.
    pub raw_value: i32,
    /// The capacity unit.
    pub unit: CapacityUnit
}

/// This trait belongs to devices that can store energy in a battery and potentially recharge, or devices that can charge another device.
/// The trait supports starting and stopping charging, and checking the current charge level,
/// capacity remaining, and capacity until full values.
pub trait EnergyStorage {
    /// True if this device only supports queries about the stored energy levels and,
    /// optionally, active charging state (dependent on isRechargeable attribute),
    /// but does not support starting and stopping charging.
    fn is_query_only(&self) -> Result<bool, EnergyStorageError>;

    /// Will be used in responses to the user.
    fn get_distance_unit_for_ux(&self) -> Result<UxDistanceUnit, EnergyStorageError>;

    /// Set to true if this device is rechargeable.
    /// This indicates the device may report capacityUntilFull, isCharging,
    /// and optionally isPluggedIn state, and can accept the Charge command.
    fn is_rechargable(&self) -> Result<bool, EnergyStorageError>;

    /// A qualitative description of the energy capacity level.
    /// Note this is for when there's no numeric capacity data.
    /// If numeric capacity data is also available, it will be preferred over descriptive when possible.
    fn get_descriptive_capacity_remaining(&self) -> Result<CapacityState, EnergyStorageError>;

    /// Array of unit/value pairs that hold information on the energy capacity the device currently holds.
    /// For example: How many miles does my <device> currently have or What percentage charge does my <device> have
    fn get_capacity_remaining(&self) -> Result<Option<Vec<CapacityValue>>, EnergyStorageError> {
        Ok(None)
    }

    /// Array of unit/value pairs that hold information on the capacity until
    /// the device is fully charged. For example: How much time until <device> is fully charged.
    fn get_capacity_until_full(&self) -> Result<Option<Vec<CapacityValue>>, EnergyStorageError> {
        Ok(None)
    }

    /// Whether the device is currently charging.
    fn is_charging(&self) -> Result<Option<bool>, EnergyStorageError> {
        Ok(None)
    }

    /// Whether the device is currently plugged in. The device can be plugged in, but not actively charging.
    fn is_plugged_in(&self) -> Result<Option<bool>, EnergyStorageError> {
        Ok(None)
    }

    /// Start or stop charging.
    /// If the device is not rechargable, this function will never be called.
    /// - `charge` True to start charging, false to stop charging.
    fn charge(&mut self, charge: bool) -> Result<(), EnergyStorageError>;
}
