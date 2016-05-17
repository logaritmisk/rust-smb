use std::cell::RefCell;

use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;


pub trait Sprite {
    fn render(&self, f64, &mut Renderer, &Rect);
}


pub struct StaticSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool
}

impl<'a> StaticSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32) -> StaticSprite<'a> {
        StaticSprite {
            texture: texture,
            x: x,
            y: y,
            flip_horizontal: false,
            flip_vertical: false
        }
    }
}

impl<'a> Sprite for StaticSprite<'a> {
    fn render(&self, elapsed: f64, drawer: &mut Renderer, destination: &Rect) {
        drawer.copy_ex(self.texture, Some(Rect::new(self.x, self.y, 16, 16)), Some(*destination), 0.0, None, self.flip_horizontal, self.flip_vertical);
    }
}


pub struct AnimatedSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    frames: u32,
    frame_time: f64,
    frame: RefCell<u32>,
    time: RefCell<f64>
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32, frames: u32, fps: f32) -> AnimatedSprite<'a> {
        AnimatedSprite {
            texture: texture,
            x: x,
            y: y,
            flip_horizontal: false,
            flip_vertical: false,
            frame: RefCell::new(0),
            frames: frames,
            time: RefCell::new(0.0),
            frame_time: 1000.0 / fps as f64
        }
    }
}

impl<'a> Sprite for AnimatedSprite<'a> {
    fn render(&self, elapsed: f64, drawer: &mut Renderer, destination: &Rect) {
        let mut time = self.time.borrow_mut();
        let mut frame = self.frame.borrow_mut();

        *time += 10.0 + elapsed;
        *frame += (*time / self.frame_time) as u32;
        *time %= self.frame_time;
        *frame %= self.frames;

        let x = self.x + (*frame * 16) as i32;

        drawer.copy_ex(self.texture, Some(Rect::new(x, self.y, 16, 16)), Some(*destination), 0.0, None, self.flip_horizontal, self.flip_vertical);
    }
}
