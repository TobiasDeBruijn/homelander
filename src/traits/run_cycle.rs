use crate::traits::Language;
use crate::CombinedDeviceError;
use serde::Serialize;

/// Contains the synonyms for the current cycle in each supported language.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentRunCycle {
    /// Current cycle being performed.
    pub current_cycle: String,
    /// Optional. Next cycle to perform.
    pub next_cycle: Option<String>,
    /// Language code for the given cycle names
    pub lang: Language,
}

/// This trait represents any device that has an ongoing duration for its operation which can be queried.
/// This includes, but is not limited to, devices that operate cyclically, such as washing machines, dryers, and dishwashers.
pub trait RunCycle {
    /// Contains the synonyms for the current cycle in each supported language.
    fn get_current_run_cycle(&self) -> Result<Vec<CurrentRunCycle>, CombinedDeviceError>;

    /// Time remaining on operation, in seconds.
    fn get_current_total_remaining_time(&self) -> Result<i32, CombinedDeviceError>;

    /// Time remaining on current cycle, in seconds.
    fn get_current_cycle_remaining_time(&self) -> Result<i32, CombinedDeviceError>;
}
