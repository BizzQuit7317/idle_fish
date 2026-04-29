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
mod settings;
mod economy;
mod offline_report;

use macroquad::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
//use crate::sprites;

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
    let mut game_state: Option<game_state::GameState> = None; //initiate the game
    let mut tank_sprites = sprites::TankSprites::new(); //allow the fishes to roam freely and beautifully
    let mut current_page = ui_helper::GamePage::MainMenu;
    let mut last_page = ui_helper::GamePage::MainMenu;
    let mut tick_timer = 0.0f32; //control the frame speed

    loop {
        match current_page {
            ui_helper::GamePage::MainMenu => {
                match menu::draw_main_menu() {
                    menu::MenuChoice::NewGame => {
                        game_state = Some(game_state::GameState::new());
                        if let Some(gs) = &game_state {
                            file_control::save_game_json(gs);
                            tank_sprites.sync(gs.tank.fish.len());
                        }
                        current_page = ui_helper::GamePage::Game;
                    },
                    menu::MenuChoice::Continue => {
                        game_state = Some(file_control::load_game_json());
                        if let Some(gs) = &mut game_state {
                            let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards...").as_secs();

                            let offline_seconds = now.saturating_sub(gs.player.last_save_time);
                            let mut offline_prestige = gs.player.current_prestige;
                            for _ in 0..offline_seconds {
                                gs.tick();
                            }
                            offline_prestige = gs.player.current_prestige - offline_prestige;

                            gs.offline_report.seconds_passed = offline_seconds as u32;
                            gs.offline_report.prestige_gained = offline_prestige;

                            println!("[DBG]Offline report:  Time away {} seconds, {} Prestige gained", gs.offline_report.seconds_passed, gs.offline_report.prestige_gained);

                            tank_sprites.sync(gs.tank.fish.len());
                        }
                        current_page = ui_helper::GamePage::Game;
                    },
                    menu::MenuChoice::Settings => {
                            current_page = ui_helper::GamePage::Settings;
                        },
                    menu::MenuChoice::None => {}
                }
            },
            ui_helper::GamePage::Game => {
                last_page = ui_helper::GamePage::Game;

                tick_timer += get_frame_time();
                if tick_timer >= 1.0 {
                    if let Some(gs) = &mut game_state {
                        gs.tick();
                    }
                    tick_timer = 0.0;
                }
                if let Some(gs) = &mut game_state {
                    gs.notification.tick(get_frame_time());
                }

                clear_background(DARKGREEN);
                
                if let Some(gs) = &mut game_state {
                    match hud::draw_main_hud(gs) {
                        hud::hudAction::FeedFish => {
                            for fish in &mut gs.tank.fish {
                                fish.eat();
                            }
                        },
                        hud::hudAction::AddFish => {
                            if let Some(species) = gs.fish_registry.fish.iter().find(|s| s.species == "Goldfish") {
                                if gs.economy.can_afford(gs.player.current_prestige, species) {
                                    gs.player.current_prestige -= gs.economy.get_cost(species);
                                    gs.tank.fish.push(fish::Fish::new(species));
                                    gs.economy.record_purchase(species);
                                    gs.notification.set("Fish Purchased!", 3.0);
                                    println!("could afford fish bought!");
                                } else {
                                    gs.notification.set("your a peasant who can't buy a goldfish. Begone naeve", 3.0);
                                    println!("your a peasant who can't buy a goldfish, begone naeve")
                                }
                                
                                println!("[DBG] purchase count: {:?}", gs.economy.purchase_counts);
                                //tank_sprites.sync(gs.tank.fish.len());
                            }
                        },
                        hud::hudAction::Save => {
                            file_control::save_game_json(gs);
                            gs.player.last_save_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards...").as_secs();
                        },
                        hud::hudAction::Settings => {
                            current_page = ui_helper::GamePage::Settings;
                        },
                        hud::hudAction::None => {}
                    }

                    tank_sprites.sync(gs.tank.fish.len());
                }

                tank_sprites.update();
                tank_sprites.draw();
                //debug::draw_debug_zones();
                //debug::draw_debug_grid();
            },
            ui_helper::GamePage::Settings => {
                clear_background(BLUE);
                match settings::draw_settings_menu(&last_page) {
                    settings::settingChoice::MainMenu => {
                        current_page = ui_helper::GamePage::MainMenu;
                    },
                    settings::settingChoice::GameMenu => {
                        current_page = ui_helper::GamePage::Game;
                    },
                    settings::settingChoice::None => {},
                }

                //debug::draw_debug_grid();
            }
        }

        next_frame().await;
    }
}