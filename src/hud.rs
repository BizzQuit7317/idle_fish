use macroquad::prelude::*;
use crate::game_state;

pub enum hudAction {
    AddFish,
    None,
}

pub fn draw_main_hud(gameState: &game_state::GameState) -> hudAction {
    draw_text(&format!("Prestige: {}", gameState.player.current_prestige), 100.0, 100.0, 40.0, WHITE);
    draw_text(&format!("Fish in tank: {}", gameState.player.current_fish_owned), 100.0, 500.0, 40.0, WHITE);

    // draw the button rectangle
    draw_rectangle(100.0, 100.0, 200.0, 50.0, GRAY);

    // draw text on top of it
    draw_text("Add Fish", 150.0, 132.0, 24.0, BLACK);

    // check if it was clicked
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if mx >= 100.0 && mx <= 300.0 && my >= 100.0 && my <= 150.0 {
            return hudAction::AddFish;
        }
    }

    hudAction::None
}