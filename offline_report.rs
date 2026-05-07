use serde::{Serialize, Deserialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct OfflineReport {
    pub seconds_passed: u32,
    pub prestige_gained: f64,
}

impl OfflineReport {
    pub fn new() -> OfflineReport {
        OfflineReport {
            seconds_passed: 0,
            prestige_gained: 0.0,
        }
    }
}