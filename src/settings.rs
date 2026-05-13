use macroquad::prelude::*;
use crate::ui_helper as ui;
use crate::constants as con;
use crate::debug;
use crate::game_state;

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
    pub scroll_target: f32,
}

impl SettingsState {
    pub fn new() -> Self {
        Self { scroll_offset: 0.0, scroll_target: 0.0 }
    }
}

pub fn draw_settings_menu(last_page: &ui::GamePage, state: &mut SettingsState, active_tab: &SettingTab, gameState: Option<&game_state::GameState>) -> settingChoice {
    let sw = screen_width();
    let sh = screen_height();

    // Scrollbar
    let scrollbar_w = sw * 0.01;
    let scrollbar_x = sw - scrollbar_w - sw * 0.005;

    // Layout constants for stats rows (used for both content sizing and drawing)
    let stats_x = sw * 0.025;
    let stats_box_w = sw * 0.475; // ends roughly at the middle of the screen
    let stats_box_h = sh * 0.05;
    let stats_row_gap = sh * 0.06;
    let stats_start_y = sh * 0.3;
    let stats_row_count: f32 = 17.0; // update if you add/remove rows

    // Content and scroll setup
    let content_visible_h = sh * 0.9;
    // Actual content height = where the last row ends, plus a little padding
    let content_total_h = (stats_start_y + stats_row_gap * stats_row_count + sh * 0.05).max(content_visible_h);
    let max_scroll = (content_total_h - content_visible_h).max(0.0);

    // Update scroll target from wheel, then smoothly approach it
    let scroll_delta = mouse_wheel().1;
    state.scroll_target -= scroll_delta * sh * 0.15;
    state.scroll_target = state.scroll_target.clamp(0.0, max_scroll);

    // Smooth interpolation toward target (lerp)
    let smoothing = 0.18; // higher = snappier, lower = smoother. tweak to taste
    state.scroll_offset += (state.scroll_target - state.scroll_offset) * smoothing;

    // Scrollbar track + thumb
    draw_rectangle(scrollbar_x, 0.0, scrollbar_w, content_visible_h, Color::from_rgba(80, 80, 80, 200));
    if max_scroll > 0.0 {
        let thumb_h = (content_visible_h * (content_visible_h / content_total_h)).max(sh * 0.03);
        let thumb_y = (state.scroll_offset / max_scroll) * (content_visible_h - thumb_h);
        draw_rectangle(scrollbar_x, thumb_y, scrollbar_w, thumb_h, Color::from_rgba(180, 180, 180, 220));
    }

    // Tabs
    if ui::draw_button_box(sw * 0.025, sh * 0.15, sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Game", BLACK) {
        return settingChoice::Game;
    }
    if ui::draw_button_box(sw * 0.15, sh * 0.15, sw * con::TAB_BUTTON_BOX_SCALE_WIDTH, sh * con::TAB_BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Player Stats", BLACK) {
        return settingChoice::PlayerStats;
    }

    match active_tab {
        &SettingTab::Game => {
            ui::draw_centered_text_box(sw, sh, sw * 0.05, sh * 0.05, GREEN, "Coming soon brother please be patient....... Maybe for Cicero?", BLACK);
        },
        &SettingTab::PlayerStats => {
            if let Some(gs) = gameState {
                let p = &gs.player;
                let y0 = stats_start_y - state.scroll_offset;

                // Prestige
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 0.0,  stats_box_w, stats_box_h, WHITE, &format!("Current Prestige: {:.2}", p.current_prestige), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 1.0,  stats_box_w, stats_box_h, WHITE, &format!("Max Prestige: {:.2}", p.all_time_prestige), BLACK);

                // Food
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 2.0,  stats_box_w, stats_box_h, WHITE, &format!("Current Food Level: {:.2}", p.current_food_level), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 3.0,  stats_box_w, stats_box_h, WHITE, &format!("Highest Food Level: {:.2}", p.highest_food_level), BLACK);

                // Water
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 4.0,  stats_box_w, stats_box_h, WHITE, &format!("Water Change %: {}", p.water_change_percent), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 5.0,  stats_box_w, stats_box_h, WHITE, &format!("Water Change Cooldown: {:.1}s", p.water_change_cooldown), BLACK);

                // Tank
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 6.0,  stats_box_w, stats_box_h, WHITE, &format!("Tank Cap Level: {:.2}", p.tank_cap_level), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 7.0,  stats_box_w, stats_box_h, WHITE, &format!("Highest Tank Cap Level: {:.2}", p.highest_tank_cap_level), BLACK);

                // Fish
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 8.0,  stats_box_w, stats_box_h, WHITE, &format!("Current Fish Owned: {}", p.current_fish_owned), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 9.0,  stats_box_w, stats_box_h, WHITE, &format!("Total Fish Died: {}", p.total_fish_died), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 10.0, stats_box_w, stats_box_h, WHITE, &format!("Peak Fish Count: {}", p.peak_fish_count), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 11.0, stats_box_w, stats_box_h, WHITE, &format!("Species Discovered: {}", p.total_species_discovered), BLACK);

                // Time
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 12.0, stats_box_w, stats_box_h, WHITE, &format!("Total Time Played: {:.1}s", p.total_time_played), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 13.0, stats_box_w, stats_box_h, WHITE, &format!("Last Save Time: {}", p.last_save_time), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 14.0, stats_box_w, stats_box_h, WHITE, &format!("First Play Time: {}", p.first_play_time), BLACK);

                // Rebirth
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 15.0, stats_box_w, stats_box_h, WHITE, &format!("Total Rebirths: {}", p.total_rebirths), BLACK);
                ui::draw_text_box(stats_x, y0 + stats_row_gap * 16.0, stats_box_w, stats_box_h, WHITE, &format!("Rebirth Multiplier: {:.2}x", p.rebirth_multiplyer), BLACK);
            } else {
                ui::draw_text_box(sw * 0.025, sh * 0.3, sw * 0.2, sh * 0.1, WHITE, "No save loaded", BLACK);
            }
        },
    }

    // Back button
    match last_page {
        &ui::GamePage::MainMenu => {
            if ui::draw_button_box(sw * 0.025 + (con::BUTTON_BOX_SCALE_WIDTH * 0.5), sh * 0.025 + (con::BUTTON_BOX_SCALE_HEIGHT * 0.5), sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::MainMenu;
            }
        },
        &ui::GamePage::Game => {
            if ui::draw_button_box(sw * 0.025 + (con::BUTTON_BOX_SCALE_WIDTH * 0.5), sh * 0.025 + (con::BUTTON_BOX_SCALE_HEIGHT * 0.5), sw * con::BUTTON_BOX_SCALE_WIDTH, sh * con::BUTTON_BOX_SCALE_HEIGHT, Color::from_rgba(192, 192, 192, 255), "Back", BLACK) {
                return settingChoice::GameMenu;
            }
        },
        &ui::GamePage::Settings => {},
    }

    settingChoice::None
}