use std::collections::HashMap;
use serde::{Serialize, Deserialize}; 

//use crate::registry;
use crate::game_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Economy {
    pub purchase_counts: HashMap<String, u32>,
}

impl Economy {
    pub fn new() -> Economy{
        Economy {
            purchase_counts: HashMap::new(),
        }
    }

    pub fn record_purchase(&mut self, species: &game_data::registry::FishSpecies) {
        let count = self.purchase_counts.entry(species.species.clone()).or_insert(0);
        *count += 1;
    }

    pub fn get_cost(&self, species: &game_data::registry::FishSpecies) -> f64 {
        let count = self.purchase_counts.get(&species.species).copied().unwrap_or(0);
        // each purchase increases cost by 1% compounding
        species.base_cost * (2.25_f64.powi(count as i32))
    }

    pub fn can_afford(&self, player_prestige: f64, species: &game_data::registry::FishSpecies) -> bool {
        player_prestige >= self.get_cost(species)
    }

    pub fn get_food_cost(&self, player_food_level: f64) -> f64 {
        game_data::constants::BASE_FOOD_PRICE * player_food_level.powf(2.0)
    }

    pub fn get_tank_cap_cost(&self, current_tank_level: f64) -> f64 {
        game_data::constants::BASE_TANK_CAP_PRICE * current_tank_level.powf(2.5)
    }
}