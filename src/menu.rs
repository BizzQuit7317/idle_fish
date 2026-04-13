use macroquad::prelude::*;

use crate::constants as con;
use crate::ui_helper as ui;

pub enum MenuChoice {
    NewGame,
    Continue,
    Settings,
    None,
}

pub fn draw_main_menu() -> MenuChoice {
    let sw = screen_width();
    let sh = screen_height();

    // Background
    clear_background(Color::from_rgba(0, 128, 128, 255)); // classic win98 teal

    // Title box
    ui::draw_centered_text_box(sw * 0.5, sh * 0.2, sw * con::TITLE_BOX_SCALE_WIDTH, sh * con::TITLE_BOX_SCALE_HEIGHT, Color::from_rgba(0, 0, 128, 255), "IDLE FISH", WHITE);

    // New Game button
    if ui::draw_centered_button_box(sw * 0.5, sh * 0.35, sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "New Game", BLACK) {
        return MenuChoice::NewGame;
    }

    // Continue button
    if ui::draw_centered_button_box(sw * 0.5, sh * 0.5, sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Continue", BLACK) {
        return MenuChoice::Continue;
    }

    //Draw the settings button
    if ui::draw_button_box(sw * 0.975 - (sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH), sh * 0.025 , sw * con::SETTING_BUTTON_BOX_SCALE_WIDTH, sh * con::SETTING_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Settings", BLACK) {
        return MenuChoice::Settings;
    }

    //debug::draw_debug_grid(); //adds the grid and right click function for creating and makeing more areas also in main.rs

    MenuChoice::None
}