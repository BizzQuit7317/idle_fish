use macroquad::prelude::*;
use crate::util;
use crate::game_data;
use crate::systems;

pub enum SettingsChoice {
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

// ----- Local palette (matches hud.rs) -----
const PANEL_BG: Color        = Color { r: 0.94, g: 0.94, b: 0.96, a: 1.00 };
const PANEL_BG_ALT: Color    = Color { r: 0.88, g: 0.89, b: 0.93, a: 1.00 };
const HEADER_STRIP: Color    = Color { r: 0.20, g: 0.28, b: 0.42, a: 1.00 };
const TOP_BAR_BG: Color      = Color { r: 0.85, g: 0.87, b: 0.92, a: 1.00 };
const BUTTON_NEUTRAL: Color  = Color::new(0.75, 0.75, 0.78, 1.0);
const TAB_INACTIVE: Color    = Color::new(0.78, 0.80, 0.84, 1.0);
const TAB_ACTIVE: Color      = Color::new(0.30, 0.45, 0.70, 1.0);
const ROW_STRIPE: Color      = Color { r: 0.90, g: 0.91, b: 0.94, a: 1.00 };

pub fn draw_settings_menu(
    last_page: &util::ui_helper::GamePage,
    state: &mut SettingsState,
    active_tab: &SettingTab,
    current_game_state: Option<&systems::game_state::GameState>,
) -> SettingsChoice {
    let sw = screen_width();
    let sh = screen_height();

    // =====================================================================
    // TOP BAR — title centered, Back button on the left
    // =====================================================================
    let top_bar_h = sh * 0.10;
    draw_rectangle(0.0, 0.0, sw, top_bar_h, TOP_BAR_BG);
    draw_line(0.0, top_bar_h, sw, top_bar_h, 2.0, game_data::constants::AREA_BORDER_COLOUR);

    let tb_btn_w = sw * 0.09;
    let tb_btn_h = sh * 0.055;
    let tb_btn_y = (top_bar_h - tb_btn_h) * 0.5;

    // Back button (left) — routes based on what page sent us here
    if util::ui_helper::draw_button_box(sw * 0.015, tb_btn_y, tb_btn_w, tb_btn_h, BUTTON_NEUTRAL, "Back", BLACK) {
        match last_page {
            &util::ui_helper::GamePage::MainMenu => return SettingsChoice::MainMenu,
            &util::ui_helper::GamePage::Game     => return SettingsChoice::GameMenu,
            &util::ui_helper::GamePage::Settings => {} // nowhere to go back to
        }
    }

    // Centered title badge
    let title_w = sw * 0.20;
    let title_h = sh * 0.07;
    util::ui_helper::draw_centered_text_box(
        sw * 0.5,
        top_bar_h * 0.5,
        title_w, title_h,
        HEADER_STRIP,
        "Settings",
        WHITE,
    );

    // =====================================================================
    // TAB STRIP — centered, with active highlight (matches hud.rs)
    // =====================================================================
    let tab_strip_top = top_bar_h;
    let tab_strip_h = sh * 0.08;
    draw_rectangle(0.0, tab_strip_top, sw, tab_strip_h, PANEL_BG_ALT);
    draw_line(0.0, tab_strip_top + tab_strip_h, sw, tab_strip_top + tab_strip_h,
              2.0, game_data::constants::AREA_BORDER_COLOUR);

    let tab_w = sw * 0.14;
    let tab_h = tab_strip_h * 0.72;
    let tab_y = tab_strip_top + (tab_strip_h - tab_h) * 0.5;
    let tab_gap = sw * 0.01;
    let total_tabs_w = tab_w * 2.0 + tab_gap;
    let tabs_start_x = sw * 0.5 - total_tabs_w * 0.5;

    let tab_specs: [(SettingTab, &str); 2] = [
        (SettingTab::Game,        "Game"),
        (SettingTab::PlayerStats, "Player Stats"),
    ];

    for (i, (tab, label)) in tab_specs.iter().enumerate() {
        let tx = tabs_start_x + i as f32 * (tab_w + tab_gap);
        let is_active = active_tab == tab;
        let (bg, fg) = if is_active { (TAB_ACTIVE, WHITE) } else { (TAB_INACTIVE, BLACK) };
        if util::ui_helper::draw_button_box(tx, tab_y, tab_w, tab_h, bg, label, fg) {
            return match tab {
                SettingTab::Game        => SettingsChoice::Game,
                SettingTab::PlayerStats => SettingsChoice::PlayerStats,
            };
        }
    }

    // =====================================================================
    // CONTENT AREA
    // =====================================================================
    let content_top = tab_strip_top + tab_strip_h;
    let content_h = sh - content_top;
    draw_rectangle(0.0, content_top, sw, content_h, PANEL_BG);

    // Scrollbar only spans the content area
    let scrollbar_w = sw * 0.01;
    let scrollbar_x = sw - scrollbar_w - sw * 0.005;

    match active_tab {
        &SettingTab::Game => {
            // Reset scroll on this tab so it doesn't carry over
            state.scroll_target = 0.0;
            state.scroll_offset = 0.0;

            // Centered "coming soon" card
            let card_w = sw * 0.5;
            let card_h = sh * 0.18;
            let card_cx = sw * 0.5;
            let card_cy = content_top + content_h * 0.45;

            // Card backing
            draw_rectangle(
                card_cx - card_w * 0.5,
                card_cy - card_h * 0.5,
                card_w, card_h,
                PANEL_BG_ALT,
            );

            // Header strip on the card
            let head_h = sh * 0.035;
            draw_rectangle(
                card_cx - card_w * 0.5,
                card_cy - card_h * 0.5,
                card_w, head_h,
                HEADER_STRIP,
            );
            let head_text = "Game Settings";
            let hf = head_h * 0.55;
            let hts = measure_text(head_text, None, hf as u16, 1.0);
            draw_text(
                head_text,
                card_cx - hts.width * 0.5,
                card_cy - card_h * 0.5 + head_h * 0.5 + hts.height * 0.5,
                hf,
                WHITE,
            );

            // Body text
            let body = "Coming soon brother, please be patient... Maybe for Cicero?";
            let bf = sh * 0.025;
            let bts = measure_text(body, None, bf as u16, 1.0);
            draw_text(
                body,
                card_cx - bts.width * 0.5,
                card_cy + head_h * 0.2 + bts.height * 0.5,
                bf,
                BLACK,
            );

            // Card border
            draw_rectangle_lines(
                card_cx - card_w * 0.5,
                card_cy - card_h * 0.5,
                card_w, card_h,
                3.0, game_data::constants::AREA_BORDER_COLOUR,
            );
        },

        &SettingTab::PlayerStats => {
            if let Some(gs) = current_game_state {
                let p = &gs.player;

                // ---- Build sectioned content ----
                // Each "section" has a title + a Vec<String> of stat lines.
                // Lines are laid out in a 2-column grid inside their section.
                let sections: Vec<(&str, Vec<String>)> = vec![
                    ("Prestige", vec![
                        format!("Current Prestige: {:.2}", p.current_prestige),
                        format!("Max Prestige: {:.2}",     p.all_time_prestige),
                    ]),
                    ("Food", vec![
                        format!("Current Food Level: {:.2}", p.current_food_level),
                        format!("Highest Food Level: {:.2}", p.highest_food_level),
                    ]),
                    ("Water", vec![
                        format!("Water Change %: {}",            p.water_change_percent),
                        format!("Water Change Cooldown: {:.1}s", p.water_change_cooldown),
                    ]),
                    ("Tank", vec![
                        format!("Tank Cap Level: {:.2}",         p.tank_cap_level),
                        format!("Highest Tank Cap Level: {:.2}", p.highest_tank_cap_level),
                    ]),
                    ("Fish", vec![
                        format!("Current Fish Owned: {}", p.current_fish_owned),
                        format!("Total Fish Died: {}",    p.total_fish_died),
                        format!("Peak Fish Count: {}",    p.peak_fish_count),
                        format!("Species Discovered: {}", p.total_species_discovered),
                    ]),
                    ("Time", vec![
                        format!("Total Time Played: {:.1}s", p.total_time_played),
                        format!("Last Save Time: {}",        p.last_save_time),
                        format!("First Play Time: {}",       p.first_play_time),
                    ]),
                    ("Rebirth", vec![
                        format!("Total Rebirths: {}",         p.total_rebirths),
                        format!("Rebirth Multiplier: {:.2}x", p.rebirth_multiplyer),
                    ]),
                ];

                // Layout metrics for the sectioned 2-column view
                let outer_pad_x = sw * 0.02;
                let inner_pad_y = sh * 0.015;
                let content_area_x = outer_pad_x;
                let content_area_w = sw - outer_pad_x * 2.0 - scrollbar_w - sw * 0.01;

                let section_header_h = sh * 0.04;
                let row_h = sh * 0.045;
                let section_gap = sh * 0.025;

                // Pre-compute total content height so we can size the scrollbar.
                let mut total_h = inner_pad_y;
                for (_, rows) in &sections {
                    let row_count_in_section = ((rows.len() as f32) / 2.0).ceil() as f32; // 2 cols
                    total_h += section_header_h + row_count_in_section * row_h + section_gap;
                }
                total_h += inner_pad_y;

                let content_visible_h = content_h;
                let max_scroll = (total_h - content_visible_h).max(0.0);

                // Smooth scroll
                let scroll_delta = mouse_wheel().1;
                state.scroll_target -= scroll_delta * sh * 0.15;
                state.scroll_target = state.scroll_target.clamp(0.0, max_scroll);
                let smoothing = 0.18;
                state.scroll_offset +=
                    (state.scroll_target - state.scroll_offset) * smoothing;

                // ---- Draw sections ----
                let mut y = content_top + inner_pad_y - state.scroll_offset;
                let col_gap = sw * 0.01;
                let col_w = (content_area_w - col_gap) * 0.5;
                let row_text_h = row_h * 0.9; // util::ui_helper::draw_text_box uses h*0.5 for font

                for (title, rows) in &sections {
                    let row_count_in_section = ((rows.len() as f32) / 2.0).ceil() as f32;
                    let section_h = section_header_h + row_count_in_section * row_h;

                    // Cull sections fully outside the viewport
                    let sec_bottom = y + section_h;
                    if sec_bottom < content_top || y > content_top + content_visible_h {
                        y += section_h + section_gap;
                        continue;
                    }

                    // Section background card
                    draw_rectangle(content_area_x, y, content_area_w, section_h, PANEL_BG_ALT);

                    // Section header strip
                    draw_rectangle(content_area_x, y, content_area_w, section_header_h, HEADER_STRIP);
                    let hf = section_header_h * 0.55;
                    let hts = measure_text(title, None, hf as u16, 1.0);
                    draw_text(
                        title,
                        content_area_x + sw * 0.012,
                        y + section_header_h * 0.5 + hts.height * 0.5,
                        hf,
                        WHITE,
                    );

                    // Rows in 2 columns
                    let rows_top = y + section_header_h;
                    for (i, line) in rows.iter().enumerate() {
                        let col = (i % 2) as f32;
                        let row = (i / 2) as f32;
                        let rx = content_area_x + col * (col_w + col_gap);
                        let ry = rows_top + row * row_h;

                        // Stripe alternate rows for readability
                        if row as usize % 2 == 1 {
                            draw_rectangle(
                                rx + 2.0, ry,
                                col_w - 4.0, row_h,
                                ROW_STRIPE,
                            );
                        }

                        // Use draw_text_box for the stat tile (gives it bg + border)
                        util::ui_helper::draw_text_box(
                            rx + sw * 0.005,
                            ry + sh * 0.005,
                            col_w - sw * 0.01,
                            row_text_h,
                            WHITE,
                            line,
                            BLACK,
                        );
                    }

                    // Section card border
                    draw_rectangle_lines(content_area_x, y, content_area_w, section_h,
                        3.0, game_data::constants::AREA_BORDER_COLOUR);

                    y += section_h + section_gap;
                }

                // Scrollbar (drawn last, on top, only within content area)
                draw_rectangle(scrollbar_x, content_top, scrollbar_w, content_visible_h,
                    Color::from_rgba(80, 80, 80, 200));
                if max_scroll > 0.0 {
                    let thumb_h = (content_visible_h * (content_visible_h / total_h))
                        .max(sh * 0.03);
                    let thumb_y = content_top
                        + (state.scroll_offset / max_scroll) * (content_visible_h - thumb_h);
                    draw_rectangle(scrollbar_x, thumb_y, scrollbar_w, thumb_h,
                        Color::from_rgba(180, 180, 180, 220));
                }
            } else {
                // No save loaded — centered message card
                let card_w = sw * 0.35;
                let card_h = sh * 0.12;
                util::ui_helper::draw_centered_text_box(
                    sw * 0.5,
                    content_top + content_h * 0.4,
                    card_w, card_h,
                    PANEL_BG_ALT,
                    "No save loaded",
                    BLACK,
                );
            }
        },
    }

    SettingsChoice::None
}