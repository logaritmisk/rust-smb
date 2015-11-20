use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;


pub trait Sprite {
    fn update(&mut self, u64) {}
    fn render(&self, &mut Renderer, &Rect);
}


pub struct StaticSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
    pub flip: (bool, bool)
}

impl<'a> StaticSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32) -> StaticSprite<'a> {
        StaticSprite {
            texture: texture,
            x: x,
            y: y,
            flip: (false, false)
        }
    }
}

impl<'a> Sprite for StaticSprite<'a> {
    fn render(&self, drawer: &mut Renderer, destination: &Rect) {
        drawer.copy_ex(self.texture, Some(Rect::new_unwrap(self.x, self.y, 16, 16)), Some(*destination), 0.0, None, self.flip);
    }
}


pub struct AnimatedSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
    pub flip: (bool, bool),
    frame: u32,
    frames: u32,
    time: u64,
    frame_time: u64
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32, frames: u32, fps: u32) -> AnimatedSprite<'a> {
        AnimatedSprite {
            texture: texture,
            x: x,
            y: y,
            flip: (false, false),
            frame: 0,
            frames: frames,
            time: 0,
            frame_time: 1000 / fps as u64
        }
    }
}

impl<'a> Sprite for AnimatedSprite<'a> {
    fn update(&mut self, elapsed: u64) {
        self.time += elapsed;
        self.frame += (self.time / self.frame_time) as u32;
        self.frame %= self.frames;
        self.time %= self.frame_time;
    }

    fn render(&self, drawer: &mut Renderer, destination: &Rect) {
        let x = self.x + (self.frame * 16) as i32;

        drawer.copy_ex(self.texture, Some(Rect::new_unwrap(x, self.y, 16, 16)), Some(*destination), 0.0, None, self.flip);
    }
}
