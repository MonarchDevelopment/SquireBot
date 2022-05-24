use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct TimerWarnings {
    pub five_min: bool,
    pub one_min: bool,
    pub time_up: bool,
}