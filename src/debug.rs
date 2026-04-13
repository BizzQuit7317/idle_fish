use macroquad::prelude::*;
use crate::ui_helper as ui;

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
    ui::draw_debug_box(0.0, screen_height() * 0.6, screen_width(), screen_height() * 0.4, "functional space");

    //Area where tabs will go
    ui::draw_debug_box(0.0, screen_height() * 0.6, screen_width(), screen_height() * 0.1, "tabs");

    //General stats Area
    ui::draw_debug_box(0.0, 0.0, screen_width() / 4.0, screen_height() * 0.6, "stats");

    //Top menues, prestige and settings
    ui::draw_debug_box(0.0, screen_height() * 0.025, screen_width(), screen_height() * 0.1, "top tabs");
}

