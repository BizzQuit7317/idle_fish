use macroquad::prelude::*;
use crate::game_state;
use crate::ui_helper as ui;
use crate::constants as con;

pub enum hudAction {
    AddFish,
    Save,
    Settings,
    None,
}

pub fn draw_main_hud(gameState: &game_state::GameState) -> hudAction {
    let sw = screen_width();
    let sh = screen_height();

    //Draw the prestige amount
    ui::draw_centered_text_box(sw * 0.5, sh * 0.025 + (sh * con::PRESTIGE_BOX_SCALE_HEIGHT * 0.5), sw * con::PRESTIGE_BOX_SCALE_WIDTH, sh * con::PRESTIGE_BOX_SCALE_HEIGHT, Color::from_rgba(0, 0, 128, 255), &format!("Prestige: {}", gameState.player.current_prestige), WHITE);

    //Draw the settings button
    if ui::draw_button_box(sw * 0.975 - (sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH), sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Settings", BLACK) {
        return hudAction::Settings;
    }

    //Draw the save button
    if ui::draw_button_box(sw * 0.025, sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Save", BLACK) {
        return hudAction::Save;
    }

    //Testing button to add more fish
    if ui::draw_centered_button_box(sw * 0.5, sh * 0.75 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Add Fish", BLACK) {
        return hudAction::AddFish;
    }

    //Draw the side stat bar
    //Draw tank ooccupancy
    //draw_text(&format!("Tank occupancy: {}", gameState.player.current_fish_owned), 0.0, sh * 0.2, 30.0, BLACK);
    ui::draw_stat(sw * 0.075, sh * 0.2, sw * con::STAT_WIDTH, sh * con::STAT_HEIGHT, &format!("Tank occupancy: {}", gameState.player.current_fish_owned), BLACK);

    /*
    draw_text(&format!("Prestige: {}", gameState.player.current_prestige), 100.0, 100.0, 40.0, WHITE);
    draw_text(&format!("Fish in tank: {}", gameState.player.current_fish_owned), 100.0, 500.0, 40.0, WHITE);

    // draw the button rectangle
    draw_rectangle(100.0, 100.0, 200.0, 50.0, GRAY); //vars(x, y, width, height, colour)
    // draw text on top of it
    draw_text("Add Fish", 150.0, 132.0, 24.0, BLACK);

    // draw save button
    draw_rectangle(500.0, 100.0, 200.0, 50.0, GRAY);
    //add text
    draw_text("Save", 550.0, 132.0, 24.0, BLACK);



    // check if add fish was clicked
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if mx >= 100.0 && mx <= 300.0 && my >= 100.0 && my <= 150.0 {
            return hudAction::AddFish;
        }
    }

    //check if save was clicked
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if mx >= 500.0 && mx <= 700.0 && my >= 100.0 && my <= 150.0 {
            return hudAction::Save;
        }
    }
    */
    
    hudAction::None
}