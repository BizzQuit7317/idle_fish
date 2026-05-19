use macroquad::prelude::*;
use serde::{Serialize, Deserialize}; 

use crate::util;
use crate::game_data;

#[derive(Debug, Serialize, Deserialize)]
pub struct Debugger {
    pub current_fish_debug_index: usize,
    pub store_scroll_offset: usize,
    pub stat_change_direction: bool, //false for negative and true for positive
    pub current_stat_debug_index: usize,
}

impl Debugger {
    pub fn new() -> Debugger {
        Debugger {
            current_fish_debug_index: 0, //default to first fish
            store_scroll_offset: 0, //default to first
            stat_change_direction: false, //start at false
            current_stat_debug_index: 0,
        }
    }
}

pub fn draw_debug_grid() {
    let spacing = 10.0;
    
    // vertical lines
    let mut x = 0.0;
    while x < screen_width() {
        if x == screen_width() / 2.0 {
            draw_line(x, 0.0, x, screen_height(), 1.0, RED); //add a red line half way through the screen for centering
        } else {
            draw_line(x, 0.0, x, screen_height(), 1.0, Color::from_rgba(255, 255, 255, 75));
        }
        x += spacing;
    }
    
    // horizontal lines
    let mut y = 0.0;
    while y < screen_height() {
        if y == screen_height() / 2.0 {
            draw_line(0.0, y, screen_width(), y, 1.0, RED); //add a red line half way through the screen for centering
        } else {
            draw_line(0.0, y, screen_width(), y, 1.0, Color::from_rgba(255, 255, 255, 75));
        }
        y += spacing;
    }

    // right click shows coordinates
    if is_mouse_button_pressed(MouseButton::Right) {
        let (mx, my) = mouse_position();
        println!("[GRID] x: {}, y: {}", mx, my);
    }
}

pub fn draw_debug_zones() {
    //Usable area for Tabs
    util::ui_helper::draw_debug_box(0.0, screen_height() * 0.6, screen_width(), screen_height() * 0.4, "functional space");

    //Area where tabs will go
    util::ui_helper::draw_debug_box(0.0, screen_height() * 0.6, screen_width(), screen_height() * 0.1, "tabs");

    //General stats Area
    util::ui_helper::draw_debug_box(0.0, 0.0, screen_width() / 4.0, screen_height() * 0.6, "stats");

    //Top menues, prestige and settings
    util::ui_helper::draw_debug_box(0.0, screen_height() * 0.025, screen_width(), screen_height() * 0.1, "top tabs");
}

pub fn draw_settings_debug_zones() {
    //Usable area for Tabs
    util::ui_helper::draw_debug_box(screen_width() * 0.025, screen_height() * 0.15, screen_width() * 0.925, screen_height() * game_data::constants::TAB_BUTTON_BOX_SCALE_HEIGHT, "tabs space");

    //Functional Space
    util::ui_helper::draw_debug_box(screen_width() * 0.025, screen_height() * 0.3, screen_width() * 0.925, screen_height() * 0.4, "functional space");
}