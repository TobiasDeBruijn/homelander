use crate::CombinedDeviceError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceError {
    #[error("RemoteSetDisabled")]
    RemoteSetDisabled,
    #[error("DeviceJammingDetected")]
    DeviceJammingDetected,
    #[error("NotSupported")]
    NotSupported,
    #[error("AlreadyLocked")]
    AlreadyLocked,
    #[error("AlreadyUnlocked")]
    AlreadyUnlocked,
}

#[derive(Debug, PartialEq, Error)]
pub enum LockUnlockError {
    #[error("{0}")]
    Device(DeviceError),
    #[error("{0}")]
    Other(CombinedDeviceError),
}

/// This trait belongs to any devices that support locking and unlocking, and/or reporting a locked state.
pub trait LockUnlock {
    /// Whether the device is currently locked.
    fn is_locked(&self) -> Result<bool, CombinedDeviceError>;

    /// Whether the device is currently jammed and therefore its locked state cannot be determined.
    fn is_jammed(&self) -> Result<bool, CombinedDeviceError>;

    /// Lock or unlock the device.
    /// - `lock` True when command is to lock, false to unlock.
    fn set_locked(&mut self, lock: bool) -> Result<(), LockUnlockError>;
}
