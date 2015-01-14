extern crate sdl2;
extern crate sdl2_image;

use std::os;
use std::num::Float;
use std::iter::range_step;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks, delay};
use sdl2::rect::Rect;
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

const PLAYER_SPEED_X : f32 = 4.0;
const PLAYER_THRESHOLD_X : f32 = 0.2;

const PLAYER_ACCELERATION_X_START : f32 = 0.02;
const PLAYER_ACCELERATION_X_STOP : f32 = 0.15;
const PLAYER_ACCELERATION_X_CHANGE : f32 = 0.06;


#[derive(Clone)]
enum Tile {
    Empty,
    Floor(Color)
}


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new("Super Matte Bros", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH as isize, SCREEN_HEIGHT as isize, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let floor = Path::new("gfx/floor.png");

    let surface = match sdl2_image::LoadSurface::from_file(&floor) {
        Ok(surface) => surface,
        Err(err) => panic!(format!("failed to load png: {}", err))
    };

    let texture = match renderer.create_texture_from_surface(&surface) {
        Ok(texture) => texture,
        Err(err) => panic!(format!("failed to create surface: {}", err))
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

    let mut player = Player::new(390.0, 390.0);

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

        if keyboard.is_held(KeyCode::Right) && (player.dx >= 0.0 || player.on_ground) {
            let a = if player.dx > 0.0 {
                PLAYER_ACCELERATION_X_START
            } else {
                PLAYER_ACCELERATION_X_CHANGE
            };

            player.dx = a * PLAYER_SPEED_X + (1.0 - a) * player.dx;
        } else if keyboard.is_held(KeyCode::Left) && (player.dx <= 0.0 || player.on_ground) {
            let a = if player.dx < 0.0 {
                PLAYER_ACCELERATION_X_START
            } else {
                PLAYER_ACCELERATION_X_CHANGE
            };

            player.dx = a * -PLAYER_SPEED_X + (1.0 - a) * player.dx;
        } else if player.on_ground {
            player.dx = (1.0 - PLAYER_ACCELERATION_X_STOP) * player.dx;

            if player.dx.abs() <= PLAYER_THRESHOLD_X {
                player.dx = 0.0;
            }
        }

        if player.on_ground {
            if keyboard.was_pressed(KeyCode::Up) {
                player.dy = -8.0;

                player.on_ground = false;
            }
        }

        if keyboard.was_released(KeyCode::Up) {
            if player.dy < -4.0 {
                player.dy = -4.0;
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
                                }
                            } else {
                                break;
                            }

                            x += 1;
                        }
                    }

                    if d > 0.0 {
                        player.x += d;
                    } else if d < 0.0 {
                        player.x += d;
                        player.dx = 0.0;
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
                                }
                            } else {
                                break;
                            }

                            x -= 1;
                        }
                    }

                    if d < 0.0 {
                        player.x += d;
                    } else if d > 0.0 {
                        player.x += d;
                        player.dx = 0.0;
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
                                }
                            } else {
                                break;
                            }

                            y += 1;
                        }
                    }

                    if d > 0.0 {
                        player.y += d;
                    } else if d < 0.0 {
                        player.y += d;
                        player.dy = 0.0;

                        player.on_ground = true;
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
                                }
                            } else {
                                break;
                            }

                            y -= 1;
                        }
                    }

                    if d < 0.0 {
                        player.y += d;
                    } else if d > 0.0 {
                        player.y += d;
                        player.dy = 0.0;
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

        let _ = renderer.copy(&texture, None, Some(Rect::new(0, 0, 32, 32)));

        renderer.present();

        delay(5);
    }

    sdl2_image::quit();
    sdl2::quit();
}

fn camera_relative_rect(camera: &Rect, other: &Rect) -> Rect {
    Rect::new(other.x - camera.x, other.y - camera.y, other.w, other.h)
}
