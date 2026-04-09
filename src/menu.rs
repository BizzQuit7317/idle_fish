use macroquad::prelude::*;

pub enum MenuChoice {
    NewGame,
    Continue,
    None,
}

pub fn draw_main_menu() -> MenuChoice {
    let sw = screen_width();
    let sh = screen_height();

    // Background
    clear_background(Color::from_rgba(0, 128, 128, 255)); // classic win98 teal

    // Title box
    draw_rectangle(sw / 2.0 - 150.0, 80.0, 300.0, 60.0, Color::from_rgba(0, 0, 128, 255));
    draw_rectangle_lines(sw / 2.0 - 150.0, 80.0, 300.0, 60.0, 2.0, WHITE);
    draw_text("IDLE FISH", sw / 2.0 - 70.0, 120.0, 40.0, WHITE);

    // New Game button
    let ng_x = sw / 2.0 - 100.0;
    let ng_y = sh / 2.0 - 60.0;
    draw_rectangle(ng_x, ng_y, 200.0, 40.0, Color::from_rgba(192, 192, 192, 255));
    draw_rectangle_lines(ng_x, ng_y, 200.0, 40.0, 2.0, WHITE);
    draw_text("New Game", ng_x + 45.0, ng_y + 26.0, 24.0, BLACK);

    // Continue button
    let c_x = sw / 2.0 - 100.0;
    let c_y = sh / 2.0 + 20.0;
    draw_rectangle(c_x, c_y, 200.0, 40.0, Color::from_rgba(192, 192, 192, 255));
    draw_rectangle_lines(c_x, c_y, 200.0, 40.0, 2.0, WHITE);
    draw_text("Continue", c_x + 50.0, c_y + 26.0, 24.0, BLACK);

    // Mouse click detection
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();

        if mx >= ng_x && mx <= ng_x + 200.0 && my >= ng_y && my <= ng_y + 40.0 {
            return MenuChoice::NewGame;
        }
        if mx >= c_x && mx <= c_x + 200.0 && my >= c_y && my <= c_y + 40.0 {
            return MenuChoice::Continue;
        }
    }

    MenuChoice::None
}