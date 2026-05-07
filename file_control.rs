use serde_json;

use crate::game_state;
use crate::registry;

pub fn save_game_json(gameState: &game_state::GameState) {
    let json = serde_json::to_string(gameState).unwrap();
    std::fs::write("saves/save.json", json).unwrap();
}

pub fn load_game_json() -> game_state::GameState {
    let json = std::fs::read_to_string("saves/save.json").expect("Could not load the saved file!");
    let mut state: game_state::GameState = serde_json::from_str(&json).expect("Could not deserialize from save!");

    state.fish_registry = registry::FishRegistry::load();

    state
}