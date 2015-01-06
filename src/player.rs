use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use vec::Vec2;


pub struct Player {
    pub position: Vec2<f32>,
    pub velocity: Vec2<f32>,
    pub gravity: f32,
    pub on_ground: bool
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            position: Vec2 { x: x, y: y },
            velocity: Vec2 { x: 0.0, y: 0.0 },
            gravity: 0.3,
            on_ground: false
        }
    }

    pub fn update(&mut self) {
        self.velocity.y += self.gravity;

        if self.velocity.y > 8.0 {
            self.velocity.y = 8.0;
        } else if self.velocity.y < -8.0 {
            self.velocity.y = -8.0;
        }

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.position.x as i32 - 16, self.position.y as i32 - 16, 32, 32)
    }
}
