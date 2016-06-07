use sdl2::rect::Rect;
use sdl2::render::Renderer;

use game_object::GameObject;

pub trait Updatable {
    fn update(&self, &GameObject);
}

pub trait Renderable {
    fn render(&self, &GameObject, f64, &mut Renderer, &Rect);
}
