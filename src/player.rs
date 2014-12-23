extern crate sdl2;


use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use vec;


pub struct Player {
    pub position: vec::Vec2,
    pub velocity: vec::Vec2,
    pub gravity: f32,
    pub on_ground: bool
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player { position: vec::Vec2 { x: x, y: y }, velocity: vec::Vec2 { x: 0.0, y: 0.0 }, gravity: 0.3, on_ground: false }
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

    pub fn render(&self, renderer: &Renderer) {
        let _ = renderer.set_draw_color(Color::RGB(0, 255, 0));
        let _ = renderer.fill_rect(&self.get_rect());
    }

    pub fn get_rect(&self) -> Rect {
        Rect::new(self.position.x as i32 - 5, self.position.y as i32 - 5, 10, 10)
    }
}
