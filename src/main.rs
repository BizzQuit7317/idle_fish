mod game_state;
mod tank;
mod fish;
mod traits;
mod constants;
mod player;
mod registry;
mod hud;
mod menu;
mod sprites;

use macroquad::prelude::*;
//use crate::sprites;

#[macroquad::main("Idle Fish")]
async fn main() {
    let mut game_state: Option<game_state::GameState> = None;
    let mut tank_sprites = sprites::TankSprites::new();
    let mut in_menu = true;
    let mut tick_timer = 0.0f32;

    loop {
        if in_menu {
            match menu::draw_main_menu() {
                menu::MenuChoice::NewGame => {
                    game_state = Some(game_state::GameState::new());
                    in_menu = false;
                },
                menu::MenuChoice::Continue => {
                    // load from save later, for now same as new game
                    game_state = Some(game_state::GameState::new());
                    in_menu = false;
                },
                menu::MenuChoice::None => {}
            }
        } else {
            tick_timer += get_frame_time();
            if tick_timer >= 1.0 {
                if let Some(gs) = &mut game_state {
                    gs.tick();
                }
                tick_timer = 0.0;
            }

            //Drawing
            clear_background(DARKGREEN);
            tank_sprites.update();
            tank_sprites.draw();

            if let Some(gs) = &mut game_state {
                match hud::draw_main_hud(gs) {
                    hud::hudAction::AddFish => {
                        if let Some(species) = gs.fish_registry.fish.iter().find(|s| s.species == "Goldfish") {
                            gs.tank.fish.push(fish::Fish::new(species));
                            tank_sprites.sync(gs.tank.fish.len()); // sync after tick so count is accurate
                            
                        }
                    },
                    hud::hudAction::None => {}
                }
            }
  
        }

        next_frame().await;
    }
}