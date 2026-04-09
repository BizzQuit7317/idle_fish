mod game_state;
mod tank;
mod fish;
mod traits;
mod constants;
mod player;
mod registry;

fn main() {
    let mut gameState = game_state::GameState::new();

    loop {
        gameState.tick(); //runs the tick logic for the game to apply changes
        std::thread::sleep(std::time::Duration::from_millis(1000)); //adds a timer so the ticks come in exact regular intervals
    }
}
