mod game_state;
mod tank;
mod fish;
mod traits;
mod constants;
mod player;
mod registry;
//mod hud;
//mod menu;
mod sprites;
mod file_control;
mod debug;
mod ui_helper;
//mod settings;
mod economy;
mod offline_report;
mod lights;
mod algea;

mod screens;

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
    let mut current_tab = &screens::hud::BottomTab::FishStats; //set FishStats as the dedfault tab for now
    let mut setting_state = screens::settings::SettingsState::new();
    let mut current_settings_tab = &screens::settings::SettingTab::Game; //set Game tab as the default

    loop {
        match current_page {
            ui_helper::GamePage::MainMenu => {
                match screens::menu::draw_main_menu() {
                    screens::menu::MenuChoice::NewGame => {
                        game_state = Some(game_state::GameState::new());
                        if let Some(gs) = &mut game_state {
                            gs.tank.apply_traits();
                            gs.tank.update_ideal_parameters();
                            file_control::save_game_json(gs);
                            tank_sprites.sync(gs.tank.fish.len());
                            gs.player.current_prestige = 25.0;
                            gs.player.all_time_prestige = 25.0;
                        }
                        current_page = ui_helper::GamePage::Game;
                    },
                    screens::menu::MenuChoice::Continue => {
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

                            //Update the tanks ideal parameters before starting the game to account for any changes like dead fish
                            gs.tank.update_ideal_parameters();

                            gs.notification.set(&format!("Offline report:  Time away {} seconds, {} Prestige gained", gs.offline_report.seconds_passed, gs.offline_report.prestige_gained), 5.0);
                            //println!("[DBG]Offline report:  Time away {} seconds, {} Prestige gained", gs.offline_report.seconds_passed, gs.offline_report.prestige_gained);

                            tank_sprites.sync(gs.tank.fish.len());
                        }
                        current_page = ui_helper::GamePage::Game;
                    },
                    screens::menu::MenuChoice::Settings => {
                            current_page = ui_helper::GamePage::Settings;
                        },
                    screens::menu::MenuChoice::None => {}
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
                    gs.tank.tick_cooldown(get_frame_time());
                    gs.tank.lighting.tick_cooldown(get_frame_time());
                }

                clear_background(DARKGREEN);
                
                if let Some(gs) = &mut game_state {
                    match screens::hud::draw_main_hud(gs, current_tab) {
                        screens::hud::hudAction::FeedFish => {
                            for fish in &mut gs.tank.fish {
                                fish.eat(gs.player.current_food_level);
                            }
                        },
                        screens::hud::hudAction::AddFish(index) => {
                            if let Some(species) = gs.fish_registry.fish.get(index) {
                                if gs.tank.fish.len() < gs.tank.max_fish as usize {
                                    if gs.economy.can_afford(gs.player.current_prestige, species) {
                                        gs.player.current_prestige -= gs.economy.get_cost(species);
                                        gs.tank.fish.push(fish::Fish::new(species));
                                        gs.economy.record_purchase(species);
                                        gs.notification.set("Fish Purchased!", 3.0);
                                        gs.tank.apply_traits(); //Update once fish is added
                                        gs.tank.update_ideal_parameters(); // Update after adding
                                        //println!("could afford fish bought!");
                                    } else {
                                        gs.notification.set("your a peasant who can't buy a goldfish. Begone naeve", 3.0);
                                        //println!("your a peasant who can't buy a goldfish, begone naeve")
                                    }
                                } else {
                                    gs.notification.set("your tank is already at capacity greedy guts!", 3.0);
                                }
                                
                                //println!("[DBG] purchase count: {:?}", gs.economy.purchase_counts);
                                //tank_sprites.sync(gs.tank.fish.len());
                            }
                        },
                        screens::hud::hudAction::TestAddFish(index) => {
                            //same as above except no cost
                            if let Some(species) = gs.fish_registry.fish.get(index) {
                                if gs.tank.fish.len() < gs.tank.max_fish as usize {
                                    gs.tank.fish.push(fish::Fish::new(species));
                                    gs.notification.set("Fish spawned in by God I guess!", 3.0);
                                    gs.tank.apply_traits(); //Update once fish is added
                                    gs.tank.update_ideal_parameters(); // Update after adding
                                    //println!("could afford fish bought!");
                                } else {
                                    gs.notification.set("your tank is already at capacity greedy guts!", 3.0);
                                }
                                
                                //println!("[DBG] purchase count: {:?}", gs.economy.purchase_counts);
                                //tank_sprites.sync(gs.tank.fish.len());
                            }
                        },
                        screens::hud::hudAction::BuyFood => {
                            if gs.player.current_prestige > gs.economy.get_food_cost(gs.player.current_food_level) {
                                gs.player.current_prestige -= gs.economy.get_food_cost(gs.player.current_food_level); //do this before upgrading the food level 
                                gs.player.current_food_level += 1.0;
                                if gs.player.current_food_level > gs.player.highest_food_level {
                                    gs.player.highest_food_level = gs.player.current_food_level; //update the highesst food level reached
                                }
                            } else {
                                gs.notification.set("Get your hands out of the cookie jar scum.", 3.0);
                            }
                            
                        }
                        screens::hud::hudAction::AddPrestige => {
                            gs.player.current_prestige += 1000.0;
                        },
                        screens::hud::hudAction::Save => {
                            file_control::save_game_json(gs);
                            gs.player.last_save_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards...").as_secs();
                        },
                        screens::hud::hudAction::Settings => {
                            current_page = ui_helper::GamePage::Settings;
                        },
                        screens::hud::hudAction::FishStats => {
                            current_tab = &screens::hud::BottomTab::FishStats;
                        },
                        screens::hud::hudAction::Store => {
                            current_tab = &screens::hud::BottomTab::Store;
                        },
                        screens::hud::hudAction::Testing => {
                            current_tab = &screens::hud::BottomTab::Testing;
                        },
                        
                        screens::hud::hudAction::BuyTankCap => {
                            if gs.player.current_prestige > gs.economy.get_tank_cap_cost(gs.player.tank_cap_level) {
                                gs.player.current_prestige -= gs.economy.get_tank_cap_cost(gs.player.tank_cap_level); //do this before upgrading the tank level
                                gs.player.tank_cap_level += 1.0;
                                if gs.player.tank_cap_level > gs.player.highest_tank_cap_level {
                                    gs.player.highest_tank_cap_level = gs.player.tank_cap_level;
                                }
                                gs.tank.max_fish += 1;
                            } else {
                                gs.notification.set("GET OUT OF HERE QUINTON!!!!.", 3.0);
                            }
                        },
                        screens::hud::hudAction::DebugIndexIncrease => {
                            let max = gs.fish_registry.fish.len() - 1;
                            gs.debugger.current_fish_debug_index = (gs.debugger.current_fish_debug_index + 1).min(max);
                        },
                        screens::hud::hudAction::DebugIndexDecrease => {
                            if gs.debugger.current_fish_debug_index > 0 {
                                gs.debugger.current_fish_debug_index -= 1;
                            }
                        },
                        screens::hud::hudAction::StoreScrollUp => {
                            if gs.debugger.store_scroll_offset > 0 {
                                gs.debugger.store_scroll_offset -= 1;
                            }
                        },
                        screens::hud::hudAction::StoreScrollDown => {
                            let max_scroll = (gs.fish_registry.fish.len() / 3).saturating_sub(1);
                            if gs.debugger.store_scroll_offset < max_scroll {
                                gs.debugger.store_scroll_offset += 1;
                            }
                        },
                        screens::hud::hudAction::ChangeWater => {
                            if gs.tank.can_change_water() {
                                gs.tank.water_change(gs.player.water_change_percent, gs.player.water_change_cooldown);
                                gs.notification.set("Water change complete!", 2.0);
                            } else {
                                gs.notification.set(&format!("Water change on cooldown {} seconds", gs.player.water_change_cooldown), 2.0);
                            }
                        },
                        screens::hud::hudAction::TestChangeStat(stat, direction) => {
                            if direction {
                                match stat {
                                    tank::WaterParameter::temprature => {gs.tank.water_parameters.temprature += 1.0},
                                    tank::WaterParameter::ph => {gs.tank.water_parameters.ph += 1.0;},
                                    tank::WaterParameter::gh => {gs.tank.water_parameters.gh += 1.0;},
                                    tank::WaterParameter::nitrate => {gs.tank.water_parameters.nitrate += 1.0;},
                                    tank::WaterParameter::nitrite => {gs.tank.water_parameters.nitrite += 1.0;},
                                    tank::WaterParameter::ammonia => {gs.tank.water_parameters.ammonia += 1.0;},
                                }
                            } else {
                                match stat {
                                    tank::WaterParameter::temprature => {gs.tank.water_parameters.temprature -= 1.0;},
                                    tank::WaterParameter::ph => {gs.tank.water_parameters.ph -= 1.0;},
                                    tank::WaterParameter::gh => {gs.tank.water_parameters.gh -= 1.0;},
                                    tank::WaterParameter::nitrate => {gs.tank.water_parameters.nitrate -= 1.0;},
                                    tank::WaterParameter::nitrite => {gs.tank.water_parameters.nitrite -= 1.0;},
                                    tank::WaterParameter::ammonia => {gs.tank.water_parameters.ammonia -= 1.0;},
                                }
                            }
                            
                        },
                        screens::hud::hudAction::DebugShiftStatLeft => {
                            if gs.debugger.current_stat_debug_index == 0 {
                                gs.debugger.current_stat_debug_index = tank::WaterParameter::ALL.len() - 1;
                            } else {
                                gs.debugger.current_stat_debug_index -= 1;
                            }
                        },
                        screens::hud::hudAction::DebugShiftStatRight => {
                            gs.debugger.current_stat_debug_index = 
                                (gs.debugger.current_stat_debug_index + 1) % tank::WaterParameter::ALL.len();
                        },
                        screens::hud::hudAction::DebugShiftStatPositive => {
                            gs.debugger.stat_change_direction = true;
                        },
                        screens::hud::hudAction::DebugShiftStatNegative => {
                            gs.debugger.stat_change_direction = false;
                        },
                        screens::hud::hudAction::TestToggleLight => {
                            //Only need to count a toggle when the witch is turned on
                            if gs.tank.lighting.can_turn_on() {
                                gs.tank.lighting.cooldown = gs.tank.lighting.on_period;
                                gs.tank.lighting.on = true;
                                gs.tank.lighting.toggles_used += 1;
                            } else {
                               gs.notification.set("Run out of switch toggles", 3.0) 
                            }
                        },
                        screens::hud::hudAction::None => {}
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

                match screens::settings::draw_settings_menu(&last_page, &mut setting_state, current_settings_tab, game_state.as_ref()) {
                    screens::settings::settingChoice::MainMenu => {
                        current_page = ui_helper::GamePage::MainMenu;
                    },
                    screens::settings::settingChoice::GameMenu => {
                        current_page = ui_helper::GamePage::Game;
                    },
                    screens::settings::settingChoice::Game => {
                        current_settings_tab = &screens::settings::SettingTab::Game;
                    },
                    screens::settings::settingChoice::PlayerStats => {
                        current_settings_tab = &screens::settings::SettingTab::PlayerStats;
                    }, 
                    screens::settings::settingChoice::None => {},
                }
            }
        }

        next_frame().await;
    }
}
