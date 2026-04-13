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
mod file_control;
mod debug;
mod ui_helper;

use macroquad::prelude::*;
//use crate::sprites;

pub enum GamePage {
    MainMenu,
    Game,
    Options,
}

//#[macroquad::main("Idle Fish")]
fn window_conf() -> Conf {
    Conf {
        window_title: "Idle Fish".to_string(),
        window_width: constants::WINDOWS_DEFAULT_WIDTH,
        window_height: constants::WINDOWS_DEFAULT_LENGTH,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
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
                    if let Some(gs) = &game_state {
                        file_control::save_game_json(gs); //reset or create a new file in the saves
                    } 
                    in_menu = false;  
                },
                menu::MenuChoice::Continue => {
                    // load from save later, for now same as new game
                    game_state = Some(file_control::load_game_json());
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
                    hud::hudAction::Save => {
                        file_control::save_game_json(gs);
                    },
                    hud::hudAction::None => {}
                }
            }

            debug::draw_debug_grid(); //adds the grid and right click function for creating and makeing more areas also in main menu.rs
  
        }

        next_frame().await;
    }
}
