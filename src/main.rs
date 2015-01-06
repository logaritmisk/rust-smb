extern crate sdl2;


use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks, delay};
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;

use tile::Layer;
use player::Player;
use camera::Camera;


mod vec;
mod tile;
mod player;
mod camera;


const SCREEN_WIDTH : i32 = 960;
const SCREEN_HEIGHT : i32 = 640;

const TILE_WIDTH : i32 = 32;
const TILE_HEIGHT : i32 = 32;

const MS_PER_UPDATE : uint = 10;


#[derive(Clone)]
enum Tile {
    Empty,
    Floor(Color)
}


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("Super Matte Bros", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH as int, SCREEN_HEIGHT as int, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut camera = Camera::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut layer = Layer::new(120, 20, TILE_WIDTH, TILE_HEIGHT, Tile::Empty);

    camera.x = 16;

    let colors = vec![Color::RGB(0, 0, 255), Color::RGB(0, 128, 255)];

    for x in range(0, 120) {
        layer.set_tile(x, 14, Tile::Floor(colors[(x % 2) as uint]));
    }

    let mut player = Player::new(290.0, 390.0);

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
            Event::KeyDown(_, _, key, _, _, repeat) => {
                if key == KeyCode::Escape {
                    break;
                } else if key == KeyCode::Right {
                    player.velocity.x = 4.0;
                } else if key == KeyCode::Left {
                    player.velocity.x = -4.0;
                } else if key == KeyCode::Up {
                    if player.on_ground && repeat == false {
                        player.velocity.y = -12.0;

                        player.on_ground = false;
                    }
                }
            },
            Event::KeyUp(_, _, key, _, _, _) => {
                if key == KeyCode::Right {
                    if player.velocity.x > 0.0 {
                        player.velocity.x = 0.0;
                    }
                } else if key == KeyCode::Left {
                    if player.velocity.x < 0.0 {
                        player.velocity.x = 0.0;
                    }
                } else if key == KeyCode::Up {
                    if player.velocity.y < -6.0 {
                        player.velocity.y = -6.0;
                    }
                }
            },
            _ => (),
        }

        while lag >= MS_PER_UPDATE {
            player.update();

            layer.for_each_intersecting(&player.get_rect(), |tile: &Tile, position: &Rect| {
                match *tile {
                    Tile::Empty => (),
                    Tile::Floor(_) => {
                        if position.has_intersection(&player.get_rect()) {
                            match position.intersection(&player.get_rect()) {
                                Some(intersect) => {
                                    if intersect.w >= intersect.h {
                                        if player.velocity.y < 0.0 {
                                            player.position.y += intersect.h as f32;
                                        } else {
                                            player.position.y -= intersect.h as f32;
                                        }

                                        player.velocity.y = 0.0;

                                        player.on_ground = true;
                                    } else {
                                        if player.velocity.x < 0.0 {
                                            player.position.x += intersect.w as f32;
                                        } else {
                                            player.position.x -= intersect.w as f32;
                                        }

                                        player.velocity.x = 0.0;
                                    }
                                },
                                None => (),
                            }
                        }
                    }
                }
            });

            player.on_ground = false;

            let mut ground = player.get_rect();

            ground.y += ground.h;
            ground.h = 1;

            layer.for_each_intersecting(&ground, |tile: &Tile, position: &Rect| {
                match *tile {
                    Tile::Empty => (),
                    Tile::Floor(_) => {
                        if position.has_intersection(&ground) {
                            player.on_ground = true;
                        }
                    }
                }
            });

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        layer.for_each_intersecting(&camera.get_rect(), |tile: &Tile, position: &Rect| {
            match *tile {
                Tile::Empty => (),
                Tile::Floor(c) => {
                    let _ = renderer.set_draw_color(c);
                    let _ = renderer.fill_rect(position);
                }
            }
        });

        player.render(&renderer);

        renderer.present();

        delay(5);
    }

    sdl2::quit();
}
