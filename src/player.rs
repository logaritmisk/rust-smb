extern crate sdl2;


use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use vec;


pub struct Player {
    pub position: vec::Vec2,
    pub velocity: vec::Vec2,
    pub gravity: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player { position: vec::Vec2 { x: x, y: y }, velocity: vec::Vec2 { x: 0.0, y: 0.0 }, gravity: 0.3 }
    }

    pub fn update(&mut self) {
        self.velocity.y += self.gravity;

        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
    }

    pub fn render(&self, renderer: &Renderer) {
        let player = Rect::new(self.position.x as i32, self.position.y as i32, 10, 10);

        let _ = renderer.set_draw_color(Color::RGB(0, 255, 0));
        let _ = renderer.fill_rect(&player);
    }
}
