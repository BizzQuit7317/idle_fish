use macroquad::prelude::*;

use crate::constants as con;

pub enum GamePage {
    MainMenu,
    Game,
    Settings,
}

pub fn draw_centered_text_box(cx: f32, cy: f32, w: f32, h: f32, box_color: Color, text: &str, text_color: Color) {
    let x = cx - w * 0.5;
    let y = cy - h * 0.5;
    draw_rectangle(x, y, w, h, box_color);
    draw_rectangle_lines(x, y, w, h, 2.0, con::BORDER_COLOUR);

    // text centred inside the box
    let font_size = h * 0.5;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, text_color);
}

pub fn draw_tank(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x, y, w, h, con::TANK_COLOUR);
    draw_rectangle_lines(x, y, w, h, 5.0, con::TANK_BORDER_COLOUR);
}

pub fn draw_text_box(x: f32, y: f32, w: f32, h: f32, box_color: Color, text: &str, text_color: Color) {
    draw_rectangle(x, y, w, h, box_color);
    draw_rectangle_lines(x, y, w, h, 2.0, con::BORDER_COLOUR);

    // text centred inside the box
    let font_size = h * 0.5;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, text_color);
}

pub fn draw_stat(x: f32, y: f32, w: f32, h: f32, text: &str, text_color: Color) {
    let font_size = h * 0.5;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, text_color);
}

pub fn draw_centered_button_box(cx: f32, cy: f32, w: f32, h: f32, box_color: Color, text: &str, text_color: Color) -> bool {
    let x = cx - w * 0.5;
    let y = cy - h * 0.5;
    draw_rectangle(x, y, w, h, box_color);
    draw_rectangle_lines(x, y, w, h, 2.0, con::BORDER_COLOUR);

    // text centred inside the box
    let font_size = h * 0.5;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, text_color);

    // hit detection using the same x, y, w, h
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        return mx >= x && mx <= x + w && my >= y && my <= y + h;
    }

    false
}

pub fn draw_button_box(x: f32, y: f32, w: f32, h: f32, box_color: Color, text: &str, text_color: Color) -> bool {
    draw_rectangle(x, y, w, h, box_color);
    draw_rectangle_lines(x, y, w, h, 2.0, con::BORDER_COLOUR);

    // text centred inside the box
    let font_size = h * 0.5;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, text_color);

    // hit detection using the same x, y, w, h
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        return mx >= x && mx <= x + w && my >= y && my <= y + h;
    }

    false
}

pub fn draw_debug_box (x: f32, y: f32, w: f32, h: f32, text: &str) {
    draw_rectangle(x, y, w, h, Color::from_rgba(255, 255, 255, 75));
    draw_rectangle_lines(x, y, w, h, 2.0, RED);

    // text centred inside the box
    let font_size = h * 0.2;
    let text_size = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + w * 0.5 - text_size.width * 0.5;   // centre horizontally
    let text_y = y + h * 0.5 + text_size.height * 0.5;   // centre vertically

    draw_text(text, text_x, text_y, font_size, RED);
}