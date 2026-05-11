use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub current_prestige: f64,
    pub all_time_prestige: f64,

    pub current_food_level: f64,
    pub highest_food_level: f64,

    pub water_change_percent: u32, //only needs to track the percent of water to change
    pub water_change_cooldown: f32, //counted in seconds

    pub tank_cap_level: f64,
    pub highest_tank_cap_level: f64,

    pub current_fish_owned: u32,
    pub total_fish_died: u32,
    pub peak_fish_count: u8,
    pub total_species_discovered: u32,

    pub total_time_played: f64,
    pub last_save_time: u64,
    pub first_play_time: u64,

    pub total_rebirths: u32,
    pub rebirth_multiplyer: f64,
}

impl Player {
    pub fn new() -> Player {
        Player {
            current_prestige: 0.0,
            all_time_prestige: 0.0,

            current_food_level: 1.0, //Always resets to 1.0 on rebirth
            highest_food_level: 1.0,

            water_change_percent: 20, //a base of a 20% water change
            water_change_cooldown: 100.0, //start with a base cooldown of 100 seconds

            tank_cap_level: 1.0,
            highest_tank_cap_level: 1.0,

            current_fish_owned: 0,
            total_fish_died: 0,
            peak_fish_count: 0,
            total_species_discovered: 0,
            
            total_time_played: 0.0,
            last_save_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards...").as_secs(), //place holder until a save button is added
            first_play_time: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards...").as_secs(),

            total_rebirths: 0,
            rebirth_multiplyer: 1.0,
        }
    }
}