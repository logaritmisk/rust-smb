extern crate sdl2;

use std::num::Float;
use std::iter::range_step;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks, delay};
use sdl2::rect::{Point, Rect};
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;

use tile::Layer;
use camera::Camera;
use player::Player;
use keyboard::KeyboardHandler;

mod tile;
mod camera;
mod player;
mod keyboard;


const SCREEN_WIDTH : i32 = 960;
const SCREEN_HEIGHT : i32 = 640;

const TILE_WIDTH : i32 = 32;
const TILE_HEIGHT : i32 = 32;

const MS_PER_UPDATE : usize = 10;


#[derive(Clone)]
enum Tile {
    Empty,
    Floor(Color)
}


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);

    let window = match Window::new("Super Matte Bros", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH as isize, SCREEN_HEIGHT as isize, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let mut keyboard = KeyboardHandler::new();

    let mut layer = Layer::new(120, 20, TILE_WIDTH, TILE_HEIGHT, Tile::Empty);

    let colors = vec![Color::RGB(0, 0, 255), Color::RGB(0, 128, 255)];

    for x in range(5, 120) {
        layer.set_tile(x, 14, Tile::Floor(colors[(x % 2) as usize]));
    }
    for x in range(11, 20) {
        layer.set_tile(x, 13, Tile::Floor(colors[(x % 2) as usize]));
    }
    for x in range(14, 20) {
        layer.set_tile(x, 11, Tile::Floor(colors[(x % 2) as usize]));
    }

    let mut camera = Camera::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, layer.to_rect());

    let mut player = Player::new(290.0, 390.0);

    let mut current : usize;
    let mut elapsed : usize;
    let mut previous : usize = get_ticks();
    let mut lag : usize = 0;

    'main : loop {
        current = get_ticks();
        elapsed = current - previous;
        previous = current;
        lag += elapsed;

        keyboard.clear();

        'event : loop {
            match poll_event() {
                Event::Quit(_) => break 'main,
                Event::KeyDown(_, _, key, _, _, repeat) => {
                    if repeat == false {
                        keyboard.key_down(key);
                    }
                },
                Event::KeyUp(_, _, key, _, _, _) => {
                    keyboard.key_up(key);
                },
                Event::None => break 'event,
                _ => (),
            }
        }

        if keyboard.was_pressed(KeyCode::Escape) {
            break 'main;
        }

        if keyboard.is_held(KeyCode::Right) {
            player.dx = 4.0;
        }

        if keyboard.is_held(KeyCode::Left) {
            player.dx = -4.0;
        }

        if keyboard.was_pressed(KeyCode::Up) {
            if player.on_ground {
                player.dy = -12.0;

                player.on_ground = false;
            }
        }

        if keyboard.was_released(KeyCode::Right) {
            if player.dx > 0.0 {
                player.dx = 0.0;
            }
        }

        if keyboard.was_released(KeyCode::Left) {
            if player.dx < 0.0 {
                player.dx = 0.0;
            }
        }

        if keyboard.was_released(KeyCode::Up) {
            if player.dy < -6.0 {
                player.dy = -6.0;
            }
        }

        while lag >= MS_PER_UPDATE {
            player.update();

            player.on_ground = false;

            if let Some(intersect) = layer.find_intersecting(&player.to_rect()) {
                if player.dx > 0.0 {
                    let p = player.x + player.w as f32;
                    let mut d = player.dx;

                    for y in range(intersect.y, intersect.y + intersect.h + 1) {
                        let mut x = intersect.x;

                        loop {
                            let t = (x * TILE_WIDTH) as f32 - p;

                            if t > d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Empty => d,
                                    Tile::Floor(_) => d.min(t)
                                };
                            } else {
                                break;
                            }

                            x += 1;
                        }
                    }

                    if d > 0.0 {
                        player.x += d;
                    } else {
                        player.dx = 0.0;
                    }
                } else if player.dx < 0.0 {
                    let p = player.x;
                    let mut d = player.dx;

                    for y in range(intersect.y, intersect.y + intersect.h + 1) {
                        let mut x = intersect.x;

                        loop {
                            let t = (x * TILE_WIDTH + TILE_WIDTH) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Empty => d,
                                    Tile::Floor(_) => d.max(t)
                                };
                            } else {
                                break;
                            }

                            x -= 1;
                        }
                    }

                    if d < 0.0 {
                        player.x += d;
                    } else {
                        player.dx = 0.0;
                    }
                }

                if player.dy > 0.0 {
                    let p = player.y + player.h as f32;
                    let mut d = player.dy;

                    for x in range(intersect.x, intersect.x + intersect.w + 1) {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT) as f32 - p;

                            if t > d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Empty => d,
                                    Tile::Floor(_) => d.min(t)
                                };
                            } else {
                                break;
                            }

                            y += 1;
                        }
                    }

                    if d > 0.0 {
                        player.y += d;
                    } else {
                        player.dy = 0.0;

                        player.on_ground = true;
                    }
                } else if player.dy < 0.0 {
                    let p = player.y;
                    let mut d = player.dy;

                    for x in range(intersect.x, intersect.x + intersect.w + 1) {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT + TILE_HEIGHT) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Empty => d,
                                    Tile::Floor(_) => d.max(t)
                                };
                            } else {
                                break;
                            }

                            y -= 1;
                        }
                    }

                    if d < 0.0 {
                        player.y += d;
                    } else {
                        player.dy = 0.0;
                    }
                }
            }

            camera.center(&player.to_rect());

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        layer.for_each_intersecting(&camera.to_rect(), |tile: &Tile, position: &Rect| {
            let object = camera_relative_rect(&camera.to_rect(), position);

            match *tile {
                Tile::Empty => (),
                Tile::Floor(color) => {
                    let _ = renderer.set_draw_color(color);
                    let _ = renderer.fill_rect(&object);
                }
            }
        });

        let player_rect = camera_relative_rect(&camera.to_rect(), &player.to_rect());

        let _ = renderer.set_draw_color(Color::RGB(0, 255, 0));
        let _ = renderer.fill_rect(&player_rect);

        renderer.present();

        delay(5);
    }

    sdl2::quit();
}

fn camera_relative_rect(camera: &Rect, other: &Rect) -> Rect {
    Rect::new(other.x - camera.x, other.y - camera.y, other.w, other.h)
}
