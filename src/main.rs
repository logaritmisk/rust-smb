extern crate sdl2;


use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks};
use sdl2::rect::{Rect};


const SCREEN_WIDTH : int = 800;
const SCREEN_HEIGHT : int = 600;

const MS_PER_UPDATE : uint = 10;


struct Player {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    gravity: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Player {
        Player { x: x, y: y, vel_x: 0.0, vel_y: 0.0, gravity: 0.3 }
    }

    pub fn update(&mut self) {
        self.vel_y += self.gravity;

        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    pub fn render(&self, renderer: &sdl2::render::Renderer) {
        let player = Rect::new(self.x as i32, self.y as i32, 10, 10);

        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        let _ = renderer.fill_rect(&player);
    }
}


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("SMB", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut player = Player::new(390.0, 290.0);

    let mut on_ground = true;

    let mut current : uint = 0;
    let mut elapsed : uint = 0;
    let mut lag : uint = 0;

    let mut previous : uint = get_ticks();

    loop {
        current = get_ticks();
        elapsed = current - previous;
        previous = current;
        lag += elapsed;

        match poll_event() {
            Event::Quit(_) => break,
            Event::KeyDown(_, _, key, _, _, _) => {
                if key == sdl2::keycode::KeyCode::Escape {
                    break;
                } else if key == sdl2::keycode::KeyCode::Right {
                    player.vel_x = 4.0;
                } else if key == sdl2::keycode::KeyCode::Left {
                    player.vel_x = -4.0;
                } else if key == sdl2::keycode::KeyCode::Up {
                    if on_ground {
                        player.vel_y = -8.0;

                        on_ground = false;
                    }
                }
            },
            Event::KeyUp(_, _, key, _, _, _) => {
                if key == sdl2::keycode::KeyCode::Right {
                    player.vel_x = 0.0;
                } else if key == sdl2::keycode::KeyCode::Left {
                    player.vel_x = 0.0;
                } else if key == sdl2::keycode::KeyCode::Up {
                    if player.vel_y < -4.0 {
                        player.vel_y = -4.0;
                    }
                }
            },
            _ => (),
        }

        while lag >= MS_PER_UPDATE {
            player.update();

            if player.y > 290.0 {
                player.y = 290.0;
                player.vel_y = 0.0;

                on_ground = true;
            }

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        player.render(&renderer);

        renderer.present();
    }

    sdl2::quit();
}
