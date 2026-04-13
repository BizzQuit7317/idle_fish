use macroquad::prelude::*;
use ::rand::Rng;

// sprites.rs
pub struct FishSprite {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

pub struct TankSprites {
    pub fish: Vec<FishSprite>,
}

impl TankSprites {
    pub fn new() -> TankSprites {
        TankSprites { fish: vec![] }
    }

    pub fn sync(&mut self, fish_count: usize) {
        // add sprites if fish were added
        while self.fish.len() < fish_count {
            let mut rng = ::rand::thread_rng();
            self.fish.push(FishSprite { 
                x: rng.gen_range(screen_width() * 0.25..screen_width() - 20.0), y: rng.gen_range(screen_height() * 0.125..screen_height() * 0.575), 
                dx: rng.gen_range(-2.0..2.0), dy: rng.gen_range(-2.0..2.0) 
            });
        }
        // remove sprites if fish died
        self.fish.truncate(fish_count);
    }

    pub fn update(&mut self) {
        for sprite in &mut self.fish {
            sprite.x += sprite.dx;
            sprite.y += sprite.dy;

            // bounce off edges
            if sprite.x <= screen_width() * 0.25 || sprite.x >= screen_width() - 20.0 {
                sprite.dx *= -1.0;
            }
            if sprite.y <= screen_height() * 0.125 || sprite.y >= screen_height() * 0.575 {
                sprite.dy *= -1.0;
            }
        }
    }

    pub fn draw(&self) {
        for sprite in &self.fish {
            draw_rectangle(sprite.x, sprite.y, 20.0, 10.0, Color::from_rgba(255, 140, 0, 255));
        }
    }
}