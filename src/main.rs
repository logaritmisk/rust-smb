#[deny(trivial_casts, trivial_numeric_casts)]
extern crate sdl2;
extern crate sdl2_image;

use std::path::Path;

use sdl2_image::LoadTexture;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use tile::Layer;
use camera::Camera;
use keyboard::KeyboardHandler;
use sprite::{Sprite, StaticSprite};
use timer::Timer;
use game_object::GameObject;
use player_components::{PlayerPhysicsComponent, PlayerGraphicsComponent};

mod timer;
mod tile;
mod camera;
mod keyboard;
mod sprite;
mod game_object;
mod component;
mod player_components;

const SCREEN_WIDTH : u32 = 960;
const SCREEN_HEIGHT : u32 = 640;

const TILE_WIDTH : u32 = 32;
const TILE_HEIGHT : u32 = 32;

const MS_PER_UPDATE : f64 = 10.0;

const PLAYER_SPEED_X : f32 = 4.0;
const PLAYER_THRESHOLD_X : f32 = 0.2;

const PLAYER_ACCELERATION_X_START : f32 = 0.02;
const PLAYER_ACCELERATION_X_STOP : f32 = 0.15;
const PLAYER_ACCELERATION_X_CHANGE : f32 = 0.06;

#[derive(Clone)]
enum Tile<'a> {
    Empty,
    Static(&'a StaticSprite<'a>, bool),
    Background(Rect),
    Floor(Rect)
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let _ = sdl2_image::init(sdl2_image::INIT_PNG).unwrap();

    let window = video_subsystem.window("Super Matte Bros", SCREEN_WIDTH, SCREEN_HEIGHT).position_centered().build().unwrap();
    let mut renderer = window.renderer().software().build().unwrap();

    let world_sprites = renderer.load_texture(&Path::new("gfx/world.png")).unwrap();

    let floor_sprite = StaticSprite::new(&world_sprites, 16 * 0, 16 * 0);
    let brick_sprite = StaticSprite::new(&world_sprites, 16 * 1, 16 * 0);

    let player_sprites = renderer.load_texture(&Path::new("gfx/mario.png")).unwrap();

    let timer = Timer::new();

    let mut player = GameObject::new(390.0, 390.0, Box::new(PlayerPhysicsComponent), Box::new(PlayerGraphicsComponent::new(&player_sprites)));

    let mut keyboard = KeyboardHandler::new();

    let mut layer = Layer::new(212, 20, TILE_WIDTH, TILE_HEIGHT, Tile::Empty);

    layer.set_tile(2, 15, Tile::Background(Rect::new(16 * 9, 16 * 8, 16, 16)));

    layer.set_tile(1, 16, Tile::Background(Rect::new(16 * 8, 16 * 8, 16, 16)));
    layer.set_tile(2, 16, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(3, 16, Tile::Background(Rect::new(16 * 10, 16 * 8, 16, 16)));

    layer.set_tile(0, 17, Tile::Background(Rect::new(16 * 8, 16 * 8, 16, 16)));
    layer.set_tile(1, 17, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(2, 17, Tile::Background(Rect::new(16 * 9, 16 * 9, 16, 16)));
    layer.set_tile(3, 17, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(4, 17, Tile::Background(Rect::new(16 * 10, 16 * 8, 16, 16)));

    layer.set_tile(11, 17, Tile::Background(Rect::new(16 * 11, 16 * 9, 16, 16)));
    layer.set_tile(12, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(13, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(14, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(15, 17, Tile::Background(Rect::new(16 * 13, 16 * 9, 16, 16)));

    layer.set_tile(16, 14, Tile::Floor(Rect::new(16 * 24, 16 * 0, 16, 16)));

    layer.set_tile(17, 16, Tile::Background(Rect::new(16 * 9, 16 * 8, 16, 16)));

    layer.set_tile(16, 17, Tile::Background(Rect::new(16 * 8, 16 * 8, 16, 16)));
    layer.set_tile(17, 17, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(18, 17, Tile::Background(Rect::new(16 * 10, 16 * 8, 16, 16)));

    layer.set_tile(20, 14, Tile::Static(&brick_sprite, true));
    layer.set_tile(21, 14, Tile::Floor(Rect::new(16 * 24, 16 * 0, 16, 16)));
    layer.set_tile(22, 14, Tile::Static(&brick_sprite, true));
    layer.set_tile(23, 14, Tile::Floor(Rect::new(16 * 24, 16 * 0, 16, 16)));
    layer.set_tile(24, 14, Tile::Static(&brick_sprite, true));

    layer.set_tile(22, 10, Tile::Floor(Rect::new(16 * 24, 16 * 0, 16, 16)));

    layer.set_tile(19, 7, Tile::Floor(Rect::new(16 * 0, 16 * 20, 16, 16)));
    layer.set_tile(20, 7, Tile::Floor(Rect::new(16 * 1, 16 * 20, 16, 16)));
    layer.set_tile(21, 7, Tile::Floor(Rect::new(16 * 2, 16 * 20, 16, 16)));
    layer.set_tile(19, 8, Tile::Floor(Rect::new(16 * 0, 16 * 21, 16, 16)));
    layer.set_tile(20, 8, Tile::Floor(Rect::new(16 * 1, 16 * 21, 16, 16)));
    layer.set_tile(21, 8, Tile::Floor(Rect::new(16 * 2, 16 * 21, 16, 16)));

    layer.set_tile(23, 17, Tile::Background(Rect::new(16 * 11, 16 * 9, 16, 16)));
    layer.set_tile(24, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(25, 17, Tile::Background(Rect::new(16 * 13, 16 * 9, 16, 16)));

    layer.set_tile(28, 16, Tile::Floor(Rect::new(16 * 0, 16 * 8, 16, 16)));
    layer.set_tile(29, 16, Tile::Floor(Rect::new(16 * 1, 16 * 8, 16, 16)));
    layer.set_tile(28, 17, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(29, 17, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));

    layer.set_tile(38, 15, Tile::Floor(Rect::new(16 * 0, 16 * 8, 16, 16)));
    layer.set_tile(39, 15, Tile::Floor(Rect::new(16 * 1, 16 * 8, 16, 16)));
    layer.set_tile(38, 16, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(39, 16, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));
    layer.set_tile(38, 17, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(39, 17, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));

    layer.set_tile(41, 17, Tile::Background(Rect::new(16 * 11, 16 * 9, 16, 16)));
    layer.set_tile(42, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(43, 17, Tile::Background(Rect::new(16 * 12, 16 * 9, 16, 16)));
    layer.set_tile(44, 17, Tile::Background(Rect::new(16 * 13, 16 * 9, 16, 16)));

    layer.set_tile(46, 14, Tile::Floor(Rect::new(16 * 0, 16 * 8, 16, 16)));
    layer.set_tile(47, 14, Tile::Floor(Rect::new(16 * 1, 16 * 8, 16, 16)));
    layer.set_tile(46, 15, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(47, 15, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));
    layer.set_tile(46, 16, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(47, 16, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));
    layer.set_tile(46, 17, Tile::Floor(Rect::new(16 * 0, 16 * 9, 16, 16)));
    layer.set_tile(47, 17, Tile::Floor(Rect::new(16 * 1, 16 * 9, 16, 16)));

    layer.set_tile(50, 15, Tile::Background(Rect::new(16 * 9, 16 * 8, 16, 16)));

    layer.set_tile(49, 16, Tile::Background(Rect::new(16 * 8, 16 * 8, 16, 16)));
    layer.set_tile(50, 16, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(51, 16, Tile::Background(Rect::new(16 * 10, 16 * 8, 16, 16)));

    layer.set_tile(48, 17, Tile::Background(Rect::new(16 * 8, 16 * 8, 16, 16)));
    layer.set_tile(49, 17, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(50, 17, Tile::Background(Rect::new(16 * 9, 16 * 9, 16, 16)));
    layer.set_tile(51, 17, Tile::Background(Rect::new(16 * 8, 16 * 9, 16, 16)));
    layer.set_tile(52, 17, Tile::Background(Rect::new(16 * 10, 16 * 8, 16, 16)));

    for x in 0..212 {
        layer.set_tile(x, 18, Tile::Static(&floor_sprite, true));
        layer.set_tile(x, 19, Tile::Static(&floor_sprite, true));
    }

    let mut camera = Camera::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, layer.to_rect());

    let mut previous = timer.current_time();
    let mut lag = 0.0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main : loop {
        let current = timer.current_time();
        let elapsed = current - previous;

        previous = current;
        lag += elapsed;

        keyboard.clear();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode, repeat, ..} => {
                    if repeat == false {
                        keyboard.key_down(keycode.unwrap());
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    keyboard.key_up(keycode.unwrap());
                },
                _ => (),
            }
        }

        if keyboard.was_pressed(Keycode::Escape) {
            break 'main;
        }

        if keyboard.is_held(Keycode::Right) && (player.dx >= 0.0 || player.on_ground) {
            let a = if player.dx > 0.0 {
                PLAYER_ACCELERATION_X_START
            } else {
                PLAYER_ACCELERATION_X_CHANGE
            };

            player.dx = a * PLAYER_SPEED_X + (1.0 - a) * player.dx;
        } else if keyboard.is_held(Keycode::Left) && (player.dx <= 0.0 || player.on_ground) {
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
            if keyboard.was_pressed(Keycode::Up) {
                player.dy = -8.0;

                player.on_ground = false;
            }
        }

        if keyboard.was_released(Keycode::Up) {
            if player.dy < -4.0 {
                player.dy = -4.0;
            }
        }

        while lag >= MS_PER_UPDATE {
            player.dy += player.gravity;

            if player.dy > 8.0 {
                player.dy = 8.0;
            } else if player.dy < -8.0 {
                player.dy = -8.0;
            }

            player.on_ground = false;

            if let Some(intersect) = layer.find_intersecting(&player.to_rect()) {
                if player.dx > 0.0 {
                    let p = player.x + player.w as f32;
                    let mut d = player.dx;

                    for y in intersect.y()..(intersect.y() + intersect.height() as i32) {
                        let mut x = intersect.x();

                        loop {
                            let t = (x * TILE_WIDTH as i32) as f32 - p;

                            if t > d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.min(t),
                                    Tile::Static(_, solid) => if solid { d.min(t) } else { d },
                                    _ => d
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

                    for y in intersect.y()..(intersect.y() + intersect.height() as i32) {
                        let mut x = intersect.x();

                        loop {
                            let t = (x * TILE_WIDTH as i32 + TILE_WIDTH as i32) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.max(t),
                                    Tile::Static(_, solid) => if solid { d.max(t) } else { d },
                                    _ => d
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

                    for x in intersect.x()..(intersect.x() + intersect.width() as i32) {
                        let mut y = intersect.y();

                        loop {
                            let t = (y * TILE_HEIGHT as i32) as f32 - p;

                            if t > d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.min(t),
                                    Tile::Static(_, solid) => if solid { d.min(t) } else { d },
                                    _ => d
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

                    for x in intersect.x()..(intersect.x() + intersect.width() as i32) {
                        let mut y = intersect.y();

                        loop {
                            let t = (y * TILE_HEIGHT as i32 + TILE_HEIGHT as i32) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.max(t),
                                    Tile::Static(_, solid) => if solid { d.max(t) } else { d },
                                    _ => d
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

            player.update();

            camera.center(&player.to_rect());

            lag -= MS_PER_UPDATE;
        }

        renderer.set_draw_color(Color::RGB(93, 148, 251));
        renderer.clear();

        layer.for_each_intersecting(&camera.to_rect(), |tile: &Tile, position: &Rect| {
            let object = camera.to_relative_rect(position);

            match *tile {
                Tile::Background(src) => {
                    renderer.copy(&world_sprites, Some(src), Some(object));
                },
                Tile::Floor(src) => {
                    renderer.copy(&world_sprites, Some(src), Some(object));
                },
                Tile::Static(ref sprite, _) => sprite.render(lag / MS_PER_UPDATE, &mut renderer, &object),
                _ => ()
            }
        });

        let player_rect = camera.to_relative_rect(&player.to_rect());

        player.render(elapsed, &mut renderer, &player_rect);

        renderer.present();
    }
}
