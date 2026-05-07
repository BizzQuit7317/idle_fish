use macroquad::prelude::*;
use crate::game_state;
use crate::ui_helper as ui;
use crate::constants as con;

pub enum hudAction {
    FeedFish,
    AddFish(usize),
    TestAddFish(usize),
    Save,
    Settings,
    FishStats,
    Store,
    Testing,
    AddPrestige,
    BuyFood,
    BuyTankCap,
    DebugIndexIncrease,
    DebugIndexDecrease,
    None,
}

#[derive(PartialEq)]
pub enum BottomTab {
    FishStats,
    Store,
    Testing, // Add/Feed fish and other testing features
}

fn parameter_colour(value: f64, min: f64, max: f64) -> Color {
    //Alter at somepoint to accoount for 0.00 being ideal
    let margin = (max - min) * 0.1;
    if value < min || value > max {
        RED
    } else if value < min + margin || value > max - margin {
        ORANGE
    } else {
        GREEN
    }
}

pub fn draw_main_hud(gameState: &game_state::GameState, active_tab: &BottomTab) -> hudAction {
    let sw = screen_width();
    let sh = screen_height();

    //Draw the side stat bar
    ui::draw_stat(sw * 0.075, sh * 0.1, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank occupancy: {} ({})", gameState.player.current_fish_owned, gameState.tank.max_fish), BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.20, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Temp: {:.1}°C", gameState.tank.water_parameters.temprature), parameter_colour(
        gameState.tank.water_parameters.temprature,
        gameState.tank.ideal_parameters.temprature_range.min,
        gameState.tank.ideal_parameters.temprature_range.max,
    ));
    ui::draw_stat(sw * 0.075, sh * 0.25, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank PH: {:.1}pH", gameState.tank.water_parameters.ph), parameter_colour(
        gameState.tank.water_parameters.ph,
        gameState.tank.ideal_parameters.ph_range.min,
        gameState.tank.ideal_parameters.ph_range.max,
    ));
    ui::draw_stat(sw * 0.075, sh * 0.30, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank GH: {:.1}°dGH", gameState.tank.water_parameters.gh), parameter_colour(
        gameState.tank.water_parameters.gh,
        gameState.tank.ideal_parameters.gh_range.min,
        gameState.tank.ideal_parameters.gh_range.max,
    ));
    ui::draw_stat(sw * 0.075, sh * 0.35, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Ammonia: {:.1}ppm", gameState.tank.water_parameters.ammonia), parameter_colour(
        gameState.tank.water_parameters.ammonia,
        gameState.tank.ideal_parameters.ammonia_range.min,
        gameState.tank.ideal_parameters.ammonia_range.max,
    ));
    ui::draw_stat(sw * 0.075, sh * 0.40, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Nitrite: {:.1}ppm", gameState.tank.water_parameters.nitrite), parameter_colour(
        gameState.tank.water_parameters.nitrite,
        gameState.tank.ideal_parameters.nitrite_range.min,
        gameState.tank.ideal_parameters.nitrite_range.max,
    ));
    ui::draw_stat(sw * 0.075, sh * 0.45, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank Nitrate: {:.1}ppm", gameState.tank.water_parameters.nitrate), parameter_colour(
        gameState.tank.water_parameters.nitrate,
        gameState.tank.ideal_parameters.nitrate_range.min,
        gameState.tank.ideal_parameters.nitrate_range.max,
    ));

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

    //Draw the store tab
    if ui::draw_button_box(sw * 0.15, sh * 0.6 , sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Store", BLACK) {
        return hudAction::Store;
    }

    //Draw the Testing Tab
    if ui::draw_button_box(sw * 0.275, sh * 0.6 , sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Testing", BLACK) {
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
                ui::draw_stat(init_x, init_y + sh * 0.08, stat_width, stat_height, &format!("Hunger {:.2}", fish.hunger), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.12, stat_width, stat_height, &format!("Status {:?}", fish.status), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.16, stat_width, stat_height, &format!("PPS {}", fish.get_prestige()), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.20, stat_width, stat_height, &format!("Traits {:.5?}", fish.fish_traits[0]), BLACK);
                ui::draw_stat(init_x, init_y + sh * 0.24, stat_width, stat_height, &format!("Mods {:.5?}", fish.moddifiers[0]), BLACK);

                //println!("[DBG]Species {}\nAge {}\nHunger {}\nStatus {:?}\nPPS {} \nTraits coming soon\nModdifiers coming soon", fish.species, fish.age, fish.hunger, fish.status, fish.base_prestige);
                
                init_x += stat_width; //Needs to move 1 box along
            }

        },
        &BottomTab::Store => {
            //Draw text info
            ui::draw_stat(sw * 0.15, sh * 0.7 , sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, "GoldFish", BLACK);
            //Testing button to add prestige to buy things
            if ui::draw_button_box(sw * 0.15, sh * 0.8 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), &format!("{:.2}", gameState.economy.get_cost(&gameState.fish_registry.fish[0])), BLACK) {
                return hudAction::AddFish(0);
            }
            
            //Food upgrades
            ui::draw_stat(sw * 0.45, sh * 0.7 , sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, "Upgrade Food", BLACK);
            //Testing button to add prestige to buy things
            if ui::draw_button_box(sw * 0.45, sh * 0.8 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), &format!("{:.2}", gameState.economy.get_food_cost(gameState.player.current_food_level)), BLACK) {
                return hudAction::BuyFood;
            }

            //Upgrade Tank Capacity
            ui::draw_stat(sw * 0.75, sh * 0.7 , sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, "Upgrade Tank Cap", BLACK);
            //Testing button to add prestige to buy things
            if ui::draw_button_box(sw * 0.75, sh * 0.8 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), &format!("{:.2}", gameState.economy.get_tank_cap_cost(gameState.player.tank_cap_level)), BLACK) {
                return hudAction::BuyTankCap;
            }
            
        }
        &BottomTab::Testing => {
            let selected = &gameState.fish_registry.fish[gameState.debugger.current_fish_debug_index];
            ui::draw_stat(sw * 0.5, sh * 0.68, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT,
                &format!("Selected: {}", selected.species), BLACK);

            if ui::draw_button_box(sw * 0.44, sh * 0.78, sw * 0.04, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT,
                Color::from_rgba(192, 192, 192, 255), "<", BLACK) {
                return hudAction::DebugIndexDecrease;
            }
            if ui::draw_button_box(sw * 0.62, sh * 0.78, sw * 0.04, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT,
                Color::from_rgba(192, 192, 192, 255), ">", BLACK) {
                return hudAction::DebugIndexIncrease;
            }

            if ui::draw_button_box(sw * 0.5, sh * 0.78, sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT,
                Color::from_rgba(192, 192, 192, 255), "Add Fish", BLACK) {
                return hudAction::TestAddFish(gameState.debugger.current_fish_debug_index);
            }

            //Testing button to add prestige to buy things
            if ui::draw_button_box(sw * 0.15, sh * 0.78 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Add Prestige", BLACK) {
                return hudAction::AddPrestige;
            }
            //Draw text info
            ui::draw_stat(sw * 0.15, sh * 0.88 , sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, "Have 1000 prestige.", BLACK);
        },
    }

    //Feed Button always available to player, moved lower to stop flashing
    if ui::draw_button_box(sw * 0.75, sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), &format!("Food lvl {}", &gameState.player.current_food_level), BLACK) {
        return hudAction::FeedFish;
    }
    
    //Dispplay the notification pop up
    if gameState.notification.is_active() {
        ui::draw_centered_text_box(sw * 0.5, sh * 0.55, sw * 0.3, sh * 0.05, Color::from_rgba(0, 0, 0, 180), &gameState.notification.message, WHITE);
    }
    
    hudAction::None
}