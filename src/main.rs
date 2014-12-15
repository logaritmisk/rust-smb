extern crate sdl2;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::get_ticks;
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;


mod vec;
mod player;


const SCREEN_WIDTH : int = 800;
const SCREEN_HEIGHT : int = 600;

const MS_PER_UPDATE : uint = 10;


struct Object {
    position: vec::Vec2,
    color: Color,
    w: f32,
    h: f32,
}

impl Object {
    fn new(x: f32, y: f32, w: f32, h: f32, color: Color) -> Object {
        Object {
            position: vec::Vec2 { x: x, y: y },
            color: color,
            w: w,
            h: h
        }
    }

    fn render(&self, renderer: &sdl2::render::Renderer) {
        let _ = renderer.set_draw_color(self.color);
        let _ = renderer.fill_rect(&self.get_rect());
    }

    fn get_rect(&self) -> Rect {
        Rect::new((self.position.x - (self.w / 2.0)) as i32, (self.position.y - (self.h / 2.0)) as i32, self.w as i32, self.h as i32)
    }
}


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("Super Matte Bros", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut player = player::Player::new(290.0, 390.0);
    let mut on_ground = true;

    let ground1 = Object::new(162.5, 400.0, 325.0, 5.0, Color::RGB(0, 0, 255));
    let ground2 = Object::new(637.5, 395.0, 325.0, 5.0, Color::RGB(0, 0, 255));

    let mut current : uint;
    let mut elapsed : uint;
    let mut previous : uint = get_ticks();
    let mut lag : uint = 0;

    loop {
        current = get_ticks();
        elapsed = current - previous;
        previous = current;
        lag += elapsed;

        match poll_event() {
            Event::Quit(_) => break,
            Event::KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                } else if key == KeyCode::Right {
                    player.velocity.x = 4.0;
                } else if key == KeyCode::Left {
                    player.velocity.x = -4.0;
                } else if key == KeyCode::Up {
                    if on_ground {
                        player.velocity.y = -8.0;

                        on_ground = false;
                    }
                }
            },
            Event::KeyUp(_, _, key, _, _, _) => {
                if key == KeyCode::Right {
                    player.velocity.x = 0.0;
                } else if key == KeyCode::Left {
                    player.velocity.x = 0.0;
                } else if key == KeyCode::Up {
                    if player.velocity.y < -4.0 {
                        player.velocity.y = -4.0;
                    }
                }
            },
            _ => (),
        }

        while lag >= MS_PER_UPDATE {
            player.update();

            collision_detection(&player.get_rect(), &ground2.get_rect());

            if player.position.y >= 390.0 {
                player.position.y = 390.0;
                player.velocity.y = 0.0;

                on_ground = true;
            }

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        ground1.render(&renderer);
        ground2.render(&renderer);

        player.render(&renderer);

        renderer.present();
    }

    sdl2::quit();
}

fn collision_detection(lhs: &Rect, rhs: &Rect) -> bool {
    if lhs.x + lhs.w < rhs.x || rhs.x + rhs.w < lhs.x {
        false
    }
    else if lhs.y + lhs.h < rhs.y || rhs.y + rhs.h < lhs.y {
        false
    }
    else {
        true
    }
}
