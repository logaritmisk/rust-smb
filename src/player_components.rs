use std::cell::{Cell, RefCell};

use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};

use sprite::{Sprite, StaticSprite, AnimatedSprite};
use game_object::GameObject;
use component::{Updatable, Renderable};

pub struct PlayerPhysicsComponent;

impl Updatable for PlayerPhysicsComponent {
    fn update(&self, _: &GameObject) {
    }
}

pub struct PlayerGraphicsComponent<'a> {
    flip_horizontal: Cell<bool>,
    sprite_standing: RefCell<StaticSprite<'a>>,
    sprite_running: RefCell<AnimatedSprite<'a>>
}

impl<'a> PlayerGraphicsComponent<'a> {
    pub fn new(texture: &'a Texture) -> PlayerGraphicsComponent<'a> {
        PlayerGraphicsComponent {
            flip_horizontal: Cell::new(false),
            sprite_standing: RefCell::new(StaticSprite::new(&texture, 80, 32)),
            sprite_running: RefCell::new(AnimatedSprite::new(&texture, 96, 32, 3, 10.0))
        }
    }
}

impl<'a> Renderable for PlayerGraphicsComponent<'a> {
    fn render(&self, object: &GameObject, elapsed: f64, renderer: &mut Renderer, destination: &Rect) {
        if object.dx == 0.0 {
            let mut sprite = self.sprite_standing.borrow_mut();

            sprite.flip_horizontal = self.flip_horizontal.get();

            sprite.render(elapsed, renderer, destination);
        } else {
            let mut sprite = self.sprite_running.borrow_mut();

            if object.dx < 0.0 {
                sprite.flip_horizontal = true;
                self.flip_horizontal.set(true);
            } else if object.dx > 0.0 {
                sprite.flip_horizontal = false;
                self.flip_horizontal.set(false);
            }

            sprite.render(elapsed, renderer, destination);
        };
    }
}
