use serde_json;

use crate::systems;
use crate::game_data;

pub fn save_game_json(current_game_state: &systems::game_state::GameState) {
    let json = serde_json::to_string(current_game_state).unwrap();
    std::fs::write("saves/save.json", json).unwrap();
}

pub fn load_game_json() -> systems::game_state::GameState {
    let json = std::fs::read_to_string("saves/save.json").expect("Could not load the saved file!");
    let mut state: systems::game_state::GameState = serde_json::from_str(&json).expect("Could not deserialize from save!");

    state.fish_registry = game_data::registry::FishRegistry::load();

    state
}