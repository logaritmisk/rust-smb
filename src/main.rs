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

    let ground1 = Rect::new(0, 400, 325, 5);
    let ground2 = Rect::new(475, 400, 325, 5);

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

            if player.position.y >= 390.0 {
                player.position.y = 390.0;
                player.velocity.y = 0.0;

                on_ground = true;
            }

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 255));
        let _ = renderer.fill_rect(&ground1);
        let _ = renderer.fill_rect(&ground2);

        player.render(&renderer);

        renderer.present();
    }

    sdl2::quit();
}
