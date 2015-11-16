use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;


pub trait Sprite {
    fn update(&mut self, u64) {}
    fn render(&self, &mut Renderer, &Rect);
}


pub struct StaticSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32
}

impl<'a> StaticSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32) -> StaticSprite<'a> {
        StaticSprite {
            texture: texture,
            x: x,
            y: y
        }
    }
}

impl<'a> Sprite for StaticSprite<'a> {
    fn render(&self, drawer: &mut Renderer, destination: &Rect) {
        drawer.copy(self.texture, Some(Rect::new_unwrap(self.x, self.y, 16, 16)), Some(*destination));
    }
}


pub struct AnimatedSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
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

        if self.time > self.frame_time {
            self.frame += 1;
            self.time = 0;

            if self.frame < self.frames {
                self.x += 16;
            } else {
                self.x -= 16 * (self.frames - 1) as i32;

                self.frame = 0;
            }
        }
    }

    fn render(&self, drawer: &mut Renderer, destination: &Rect) {
        drawer.copy(self.texture, Some(Rect::new_unwrap(self.x, self.y, 16, 16)), Some(*destination));
    }
}
