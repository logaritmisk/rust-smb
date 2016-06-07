use sdl2::rect::Rect;
use sdl2::render::Renderer;

use component::{Updatable, Renderable};

pub struct GameObject<'a> {
    pub x: f32,
    pub y: f32,
    pub w: u32,
    pub h: u32,
    pub dx: f32,
    pub dy: f32,
    pub gravity: f32,
    pub on_ground: bool,
    physics: Box<Updatable + 'a>,
    graphics: Box<Renderable + 'a>
}

impl<'a> GameObject<'a> {
    pub fn new(x: f32, y: f32, physics: Box<Updatable + 'a>, graphics: Box<Renderable + 'a>) -> GameObject<'a> {
        GameObject {
            x: x,
            y: y,
            w: 32,
            h: 32,
            dx: 0.0,
            dy: 0.0,
            gravity: 0.3,
            on_ground: false,
            physics: physics,
            graphics: graphics
        }
    }

    pub fn update(&self) {
        self.physics.update(self);
    }

    pub fn render(&self, elapsed: f64, renderer: &mut Renderer, destination: &Rect) {
        self.graphics.render(self, elapsed, renderer, destination);
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.x as i32, self.y as i32, 32, 32)
    }
}
