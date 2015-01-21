use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;


pub trait Sprite {
    fn update(&mut self, usize) {}
    fn render(&self, &Renderer, &Rect);
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
    fn render(&self, renderer: &Renderer, destination: &Rect) {
        let _ = renderer.copy(self.texture, Some(Rect::new(self.x, self.y, 16, 16)), Some(*destination));
    }
}


pub struct AnimatedSprite<'a> {
    texture: &'a Texture,
    x: i32,
    y: i32,
    frame: i32,
    frames: i32,
    time: usize,
    frame_time: usize
}

impl<'a> AnimatedSprite<'a> {
    pub fn new(texture: &'a Texture, x: i32, y: i32, frames: i32, fps: i32) -> AnimatedSprite<'a> {
        AnimatedSprite {
            texture: texture,
            x: x,
            y: y,
            frame: 0,
            frames: frames,
            time: 0,
            frame_time: 1000 / fps as usize
        }
    }
}

impl<'a> Sprite for AnimatedSprite<'a> {
    fn update(&mut self, elapsed: usize) {
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

    fn render(&self, renderer: &Renderer, destination: &Rect) {
        let _ = renderer.copy(self.texture, Some(Rect::new(self.x, self.y, 16, 16)), Some(*destination));
    }
}
