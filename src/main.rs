extern crate sdl2;
extern crate sdl2_image;


use std::path::Path;

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::timer::{get_ticks, delay};
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;

use tile::Layer;
use camera::Camera;
use player::Player;
use keyboard::KeyboardHandler;
use sprite::{Sprite, StaticSprite, AnimatedSprite};


mod tile;
mod camera;
mod player;
mod keyboard;
mod sprite;


const SCREEN_WIDTH : i32 = 960;
const SCREEN_HEIGHT : i32 = 640;

const TILE_WIDTH : i32 = 32;
const TILE_HEIGHT : i32 = 32;

const MS_PER_UPDATE : u32 = 10;

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
    let sdl_context = sdl2::init(sdl2::INIT_EVERYTHING).unwrap();
    sdl2_image::init(sdl2_image::INIT_PNG);

    let window = match Window::new(&sdl_context, "Super Matte Bros", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("failed to create renderer: {}", err)
    };

    let world_surface = match sdl2_image::LoadSurface::from_file(&Path::new("gfx/world.png")) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load png: {}", err)
    };

    let world_sprites = match renderer.create_texture_from_surface(&world_surface) {
        Ok(texture) => texture,
        Err(err) => panic!("failed to create surface: {}", err)
    };

    let floor_sprite = StaticSprite::new(&world_sprites, 16 * 0, 16 * 0);
    let brick_sprite = StaticSprite::new(&world_sprites, 16 * 1, 16 * 0);

    let player_surface = match sdl2_image::LoadSurface::from_file(&Path::new("gfx/mario.png")) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load png: {}", err)
    };

    let player_sprites = match renderer.create_texture_from_surface(&player_surface) {
        Ok(texture) => texture,
        Err(err) => panic!("failed to create surface: {}", err)
    };

    let mut player_sprite = AnimatedSprite::new(&player_sprites, 96, 32, 3, 15);

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

    let mut player = Player::new(390.0, 390.0);

    let mut current : u32;
    let mut elapsed : u32;
    let mut previous : u32 = get_ticks();
    let mut lag : u32 = 0;

    let mut drawer = renderer.drawer();
    let mut event_pump = sdl_context.event_pump();

    'main : loop {
        current = get_ticks();
        elapsed = current - previous;
        previous = current;
        lag += elapsed;

        keyboard.clear();

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} => break 'main,
                Event::KeyDown {keycode, repeat, ..} => {
                    if repeat == false {
                        keyboard.key_down(keycode);
                    }
                },
                Event::KeyUp {keycode, ..} => {
                    keyboard.key_up(keycode);
                },
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

                    for y in intersect.y..intersect.y + intersect.h + 1 {
                        let mut x = intersect.x;

                        loop {
                            let t = (x * TILE_WIDTH) as f32 - p;

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

                    for y in intersect.y..intersect.y + intersect.h + 1 {
                        let mut x = intersect.x;

                        loop {
                            let t = (x * TILE_WIDTH + TILE_WIDTH) as f32 - p;

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

                    for x in intersect.x..intersect.x + intersect.w + 1 {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT) as f32 - p;

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

                    for x in intersect.x..intersect.x + intersect.w + 1 {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT + TILE_HEIGHT) as f32 - p;

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

            player_sprite.update(elapsed);

            camera.center(&player.to_rect());

            lag -= MS_PER_UPDATE;
        }

        drawer.set_draw_color(Color::RGB(93, 148, 251));
        drawer.clear();

        layer.for_each_intersecting(&camera.to_rect(), |tile: &Tile, position: &Rect| {
            let object = camera_relative_rect(&camera.to_rect(), position);

            match *tile {
                Tile::Background(src) => {
                    let _ = drawer.copy(&world_sprites, Some(src), Some(object));
                },
                Tile::Floor(src) => {
                    let _ = drawer.copy(&world_sprites, Some(src), Some(object));
                },
                Tile::Static(ref sprite, _) => sprite.render(&mut drawer, &object),
                _ => ()
            }
        });

        let player_rect = camera_relative_rect(&camera.to_rect(), &player.to_rect());

        player_sprite.render(&mut drawer, &player_rect);

        drawer.present();

        delay(5);
    }

    sdl2_image::quit();
}

fn camera_relative_rect(camera: &Rect, other: &Rect) -> Rect {
    Rect::new(other.x - camera.x, other.y - camera.y, other.w, other.h)
}
