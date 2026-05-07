use macroquad::prelude::*;
use crate::ui_helper as ui;
use crate::constants as con;

pub enum settingChoice {
    MainMenu,
    GameMenu,
    None,
}

pub fn draw_settings_menu(last_page: &ui::GamePage) -> settingChoice {
    let sw = screen_width();
    let sh = screen_height();

    match last_page {
        &ui::GamePage::MainMenu => {
            if ui::draw_centered_button_box(sw * 0.5, sh * 0.5, sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::MainMenu;
            }
        },
        &ui::GamePage::Game => {
            if ui::draw_centered_button_box(sw * 0.5, sh * 0.5, sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::GameMenu;
            }
        },
        &ui::GamePage::Settings => {},
    }

    settingChoice::None 
}