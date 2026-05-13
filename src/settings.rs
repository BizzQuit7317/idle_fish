use macroquad::prelude::*;
use crate::ui_helper as ui;
use crate::constants as con;
use crate::debug;

pub enum settingChoice {
    MainMenu,
    GameMenu,
    Game,
    PlayerStats,
    None,
}

#[derive(PartialEq)]
pub enum SettingTab {
    Game,
    PlayerStats,
}

pub struct SettingsState {
    pub scroll_offset: f32,
}

impl SettingsState {
    pub fn new() -> Self {
        Self { scroll_offset: 0.0 }
    }
}

pub fn draw_settings_menu(last_page: &ui::GamePage, state: &mut SettingsState, active_tab: &SettingTab) -> settingChoice {
    let sw = screen_width();
    let sh = screen_height();

    // Scrollbar
    let scrollbar_w = sw * 0.01;
    let scrollbar_x = sw - scrollbar_w - sw * 0.005;

    // Content and scroll setup
    let content_total_h = sh * 3.0; // how tall the scrollable content is, increase as you add elements
    let content_visible_h = sh * 0.9; // everything above the back button
    let max_scroll = (content_total_h - content_visible_h).max(0.0);

    let scroll_delta = mouse_wheel().1;
    state.scroll_offset -= scroll_delta * sh * 0.03;
    state.scroll_offset = state.scroll_offset.clamp(0.0, max_scroll);

    // Scrollbar track + thumb
    draw_rectangle(scrollbar_x, 0.0, scrollbar_w, content_visible_h, Color::from_rgba(80, 80, 80, 200));
    if max_scroll > 0.0 {
        let thumb_h = (content_visible_h * (content_visible_h / content_total_h)).max(sh * 0.03);
        let thumb_y = (state.scroll_offset / max_scroll) * (content_visible_h - thumb_h);
        draw_rectangle(scrollbar_x, thumb_y, scrollbar_w, thumb_h, Color::from_rgba(180, 180, 180, 220));
    }

    // --- ADD YOUR SETTINGS ELEMENTS HERE ---
    //Keep tabs visible for now add
    //Add Game settings
    if ui::draw_button_box(sw * 0.025, sh * 0.15, sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Game", BLACK) {
        return settingChoice::Game;
    }

    //Add Player Stats Button
    if ui::draw_button_box(sw * 0.15, sh * 0.15, sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Player Stats", BLACK) {
        return settingChoice::PlayerStats;
    }

    // Back button pinned to top left corner
    match last_page {
        &ui::GamePage::MainMenu => {
            if ui::draw_button_box(sw * 0.025 + ( con::BUTTON_BOX_SCALE_WIDTH * 0.5 ), sh * 0.025 + ( con::BUTTON_BOX_SCALE_HEIGHT * 0.5 ), sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::MainMenu;
            }
        },
        &ui::GamePage::Game => {
            if ui::draw_button_box(sw * 0.025 + ( con::BUTTON_BOX_SCALE_WIDTH * 0.5 ), sh * 0.025 + ( con::BUTTON_BOX_SCALE_HEIGHT * 0.5 ), sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::GameMenu;
            }
        },
        &ui::GamePage::Settings => {},
    }

    settingChoice::None
}