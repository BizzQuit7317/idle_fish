use macroquad::prelude::*;
use crate::game_state;
use crate::ui_helper as ui;
use crate::constants as con;

pub enum hudAction {
    FeedFish,
    AddFish,
    Save,
    Settings,
    FishStats,
    Testing,
    None,
}

#[derive(PartialEq)]
pub enum BottomTab {
    FishStats,
    Testing, // Add/Feed fish and other testing features
}

pub fn draw_main_hud(gameState: &game_state::GameState, active_tab: &BottomTab) -> hudAction {
    let sw = screen_width();
    let sh = screen_height();

    //Draw the side stat bar
    ui::draw_stat(sw * 0.075, sh * 0.1, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank occupancy: {}", gameState.player.current_fish_owned), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.20, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Temp: {:.1}°C", gameState.tank.water_parameters.temprature), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.25, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank PH: {:.1}pH", gameState.tank.water_parameters.ph), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.30, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank GH: {:.1}°dGH", gameState.tank.water_parameters.gh), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.35, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Nitrate: {:.1}ppm", gameState.tank.water_parameters.nitrate), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.40, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Nitrite: {:.1}ppm", gameState.tank.water_parameters.nitrite), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.45, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Ammonia: {:.1}ppm", gameState.tank.water_parameters.ammonia), BLACK);

    //Draw the prestige amount
    ui::draw_centered_text_box(sw * 0.5, sh * 0.025 + (sh * con::PRESTIGE_BOX_SCALE_HEIGHT * 0.5), sw * con::PRESTIGE_BOX_SCALE_WIDTH, sh * con::PRESTIGE_BOX_SCALE_HEIGHT, Color::from_rgba(0, 0, 128, 255), &format!("Prestige: {:.2}", gameState.player.current_prestige), WHITE);

    //Draw the settings button
    if ui::draw_button_box(sw * 0.975 - (sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH), sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Settings", BLACK) {
        return hudAction::Settings;
    }

    //Draw the save button
    if ui::draw_button_box(sw * 0.025, sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Save", BLACK) {
        return hudAction::Save;
    }

    //Feed Button always available to player
    if ui::draw_button_box(sw * 0.75, sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Feed Fish", BLACK) {
        return hudAction::FeedFish;
    }

    //Add Tank Area where fish can swim
    ui::draw_tank(sw * 0.25, sh * 0.125, sw * con::TANK_WIDTH, sh * con::TANK_HEIGHT);

    //Add bottom areas and tab and stats box
    draw_rectangle_lines(0.0, sh * 0.6, sw, sh * con::BOTTOM_AREA_HEIGHT, 5.0, con::AREA_BORDER_COLOUR);
    draw_rectangle_lines(0.0, sh * 0.6, sw, sh * con::BOTTOM_TAB_AREA_HEIGHT, 5.0, con::AREA_BORDER_COLOUR);
    draw_rectangle_lines(0.0, sh * 0.125, sw * con::STAT_AREA_WIDTH, sh * con::STAT_AREA_HEIGHT, 5.0, con::AREA_BORDER_COLOUR);

    //Add Tab button here, each tab shoul step up 0.125 on the x-axis per tab, change later for better looks
    //Draw FishStats button, furthest to the left
    if ui::draw_button_box(sw * 0.025, sh * 0.6 , sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "FishStats", BLACK) {
        return hudAction::FishStats;
    }

    //Draw the Testing Tab
    if ui::draw_button_box(sw * 0.15, sh * 0.6 , sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Testing", BLACK) {
        return hudAction::Testing;
    }

    match active_tab {
        &BottomTab::FishStats => {
            //Need to loop through all the fish and print out their stats
            let mut init_x = 0.0;
            let mut init_y = sh * 0.7;
            let stat_width = sw / gameState.tank.fish.len() as f32;
            let stat_height = (sh * con::FISH_STAT_AREA_HEIGHT) / 7.0; //7 i the number of stats to display

            for fish in  &gameState.tank.fish {
                //Draw borders
                draw_rectangle_lines(init_x, init_y, stat_width, sh * con::FISH_STAT_AREA_HEIGHT, 5.0, con::AREA_BORDER_COLOUR);

                //Draw stats, things in () are max ranges
                ui::draw_stat(init_x, init_y, stat_width, stat_height, &format!("Species {}", fish.species), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.04, stat_width, stat_height, &format!("Age {} ({})", fish.age, fish.max_age), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.08, stat_width, stat_height, &format!("Hunger {}", fish.hunger), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.12, stat_width, stat_height, &format!("Status {:?}", fish.status), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.16, stat_width, stat_height, &format!("PPS {}", fish.base_prestige), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.20, stat_width, stat_height, &format!("Traits Coming Soon"), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.24, stat_width, stat_height, &format!("Mods Coming Soon"), BLACK);

                //println!("[DBG]Species {}\nAge {}\nHunger {}\nStatus {:?}\nPPS {} \nTraits coming soon\nModdifiers coming soon", fish.species, fish.age, fish.hunger, fish.status, fish.base_prestige);

                init_x += stat_width; //Needs to move 1 box along
            }

        },
        &BottomTab::Testing => {
            //Testing button to add more fish, uses the cot of teh first fish in index
            if ui::draw_button_box(sw * 0.5, sh * 0.75 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Add Fish", BLACK) {
                return hudAction::AddFish;
            }
            //Draw text box for price underneath
            ui::draw_stat(sw * 0.5, sh * 0.82 , sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Cost: {:.2} prestige", gameState.economy.get_cost(&gameState.fish_registry.fish[0])), BLACK);

        },
    }
    
    //Dispplay the notification pop up
    if gameState.notification.is_active() {
        ui::draw_centered_text_box(sw * 0.5, sh * 0.55, sw * 0.3, sh * 0.05, Color::from_rgba(0, 0, 0, 180), &gameState.notification.message, WHITE);
    }
    
    hudAction::None
}