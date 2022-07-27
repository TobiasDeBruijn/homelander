use crate::traits::arm_disarm::ArmDisarmError;
use crate::traits::cook::CookError;
use crate::traits::dispense::DispenseError;
use crate::traits::energy_storage::EnergyStorageError;
use crate::traits::fan_speed::FanSpeedError;
use crate::traits::input_selector::InputSelectorError;
use crate::traits::lock_unlock::LockUnlockError;
use crate::traits::network_control::NetworkControlError;
use crate::{CombinedDeviceError, ToStringError};
use std::error::Error;

#[derive(Debug)]
pub enum ExecuteError {
    Serializable(Box<dyn ToStringError>),
    Server(Box<dyn Error>),
}

macro_rules! impl_execute_error {
    ($ty:ty) => {
        impl From<$ty> for ExecuteError {
            fn from(t: $ty) -> Self {
                Self::Serializable(Box::new(t))
            }
        }
    };
}

impl From<CombinedDeviceError> for ExecuteError {
    fn from(x: CombinedDeviceError) -> Self {
        match x {
            CombinedDeviceError::Other(x) => Self::Server(Box::new(x)),
            CombinedDeviceError::DeviceError(e) => Self::Serializable(Box::new(e)),
            CombinedDeviceError::DeviceException(e) => Self::Serializable(Box::new(e)),
        }
    }
}

impl_execute_error!(ArmDisarmError);
impl_execute_error!(CookError);
impl_execute_error!(DispenseError);
impl_execute_error!(EnergyStorageError);
impl_execute_error!(FanSpeedError);
impl_execute_error!(InputSelectorError);
impl_execute_error!(LockUnlockError);
impl_execute_error!(NetworkControlError);
