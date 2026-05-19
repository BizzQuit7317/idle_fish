use macroquad::prelude::*;

use crate::constants as con;
use crate::ui_helper as ui;

pub enum MenuChoice {
    NewGame,
    Continue,
    Settings,
    None,
}

// ----- Local palette (matches hud.rs / settings.rs) -----
const PANEL_BG: Color        = Color { r: 0.94, g: 0.94, b: 0.96, a: 1.00 };
const PANEL_BG_ALT: Color    = Color { r: 0.88, g: 0.89, b: 0.93, a: 1.00 };
const HEADER_STRIP: Color    = Color { r: 0.20, g: 0.28, b: 0.42, a: 1.00 };
const TOP_BAR_BG: Color      = Color { r: 0.85, g: 0.87, b: 0.92, a: 1.00 };
const BUTTON_NEUTRAL: Color  = Color::new(0.75, 0.75, 0.78, 1.0);
const BUTTON_ACTION: Color   = Color::new(0.55, 0.78, 0.58, 1.0);

pub fn draw_main_menu() -> MenuChoice {
    let sw = screen_width();
    let sh = screen_height();

    // =====================================================================
    // BACKGROUND — flat hud-style panel colour
    // =====================================================================
    clear_background(PANEL_BG);

    // =====================================================================
    // TOP BAR — matches hud/settings: strip with divider line, Settings on right
    // =====================================================================
    let top_bar_h = sh * 0.10;
    draw_rectangle(0.0, 0.0, sw, top_bar_h, TOP_BAR_BG);
    draw_line(0.0, top_bar_h, sw, top_bar_h, 2.0, con::AREA_BORDER_COLOUR);

    let tb_btn_w = sw * 0.09;
    let tb_btn_h = sh * 0.055;
    let tb_btn_y = (top_bar_h - tb_btn_h) * 0.5;

    // Settings button (right)
    if ui::draw_button_box(
        sw - sw * 0.015 - tb_btn_w,
        tb_btn_y,
        tb_btn_w, tb_btn_h,
        BUTTON_NEUTRAL, "Settings", BLACK,
    ) {
        return MenuChoice::Settings;
    }

    // =====================================================================
    // CENTRAL CARD — title strip + buttons, anchored in the middle of the screen
    // =====================================================================
    let card_w = sw * 0.40;
    let card_h = sh * 0.55;
    let card_x = sw * 0.5 - card_w * 0.5;
    let card_y = sh * 0.5 - card_h * 0.5 + top_bar_h * 0.25; // nudged down to sit below top bar visually

    // Card backing
    draw_rectangle(card_x, card_y, card_w, card_h, PANEL_BG_ALT);

    // Card header strip with the title
    let head_h = sh * 0.10;
    draw_rectangle(card_x, card_y, card_w, head_h, HEADER_STRIP);

    let title = "IDLE FISH";
    let title_font = head_h * 0.55;
    let title_size = measure_text(title, None, title_font as u16, 1.0);
    draw_text(
        title,
        card_x + card_w * 0.5 - title_size.width * 0.5,
        card_y + head_h * 0.5 + title_size.height * 0.5,
        title_font,
        WHITE,
    );

    // Subtitle line under the title strip
    let subtitle = "An idle aquarium";
    let sub_font = sh * 0.025;
    let sub_size = measure_text(subtitle, None, sub_font as u16, 1.0);
    draw_text(
        subtitle,
        card_x + card_w * 0.5 - sub_size.width * 0.5,
        card_y + head_h + sh * 0.04,
        sub_font,
        BLACK,
    );

    // Card border
    draw_rectangle_lines(card_x, card_y, card_w, card_h, 3.0, con::AREA_BORDER_COLOUR);

    // ----- Buttons inside the card -----
    let btn_w = card_w * 0.65;
    let btn_h = sh * 0.08;
    let btn_x = card_x + card_w * 0.5 - btn_w * 0.5;

    // Position the two buttons in the lower half of the card
    let buttons_top = card_y + head_h + sh * 0.10;
    let btn_gap = sh * 0.025;

    // Continue button — primary action, green
    if ui::draw_button_box(
        btn_x, buttons_top,
        btn_w, btn_h,
        BUTTON_ACTION, "Continue", BLACK,
    ) {
        return MenuChoice::Continue;
    }

    // New Game button — neutral
    if ui::draw_button_box(
        btn_x, buttons_top + btn_h + btn_gap,
        btn_w, btn_h,
        BUTTON_NEUTRAL, "New Game", BLACK,
    ) {
        return MenuChoice::NewGame;
    }

    //debug::draw_debug_grid(); //adds the grid and right click function for creating and makeing more areas also in main.rs

    MenuChoice::None
}