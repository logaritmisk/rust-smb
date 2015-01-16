extern crate sdl2;
extern crate sdl2_image;


use std::num::Float;

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
    Background(Rect),
    Floor(Rect)
}


trait Sprite {
    fn update(&mut self, usize) {}
    fn render(&self, &sdl2::render::Renderer, &Rect);
}


struct StaticSprite<'a> {
    texture: &'a sdl2::render::Texture,
    x: i32,
    y: i32
}

impl<'a> StaticSprite<'a> {
    fn new(texture: &'a sdl2::render::Texture, x: i32, y: i32) -> StaticSprite<'a> {
        StaticSprite {
            texture: texture,
            x: x,
            y: y
        }
    }
}

impl<'a> Sprite for StaticSprite<'a> {
    fn render(&self, renderer: &sdl2::render::Renderer, destination: &Rect) {
        let _ = renderer.copy(self.texture, Some(Rect::new(80 + (16 * self.x), 16 * self.y, 16, 16)), Some(*destination));
    }
}


struct AnimatedSprite<'a> {
    texture: &'a sdl2::render::Texture,
    x: i32,
    y: i32,
    frame: i32,
    frames: i32,
    time: usize,
    frame_time: usize
}

impl<'a> AnimatedSprite<'a> {
    fn new(texture: &'a sdl2::render::Texture, x: i32, y: i32, frames: i32, fps: i32) -> AnimatedSprite<'a> {
        AnimatedSprite {
            texture: texture,
            x: x,
            y: y,
            frame: 0,
            frames: frames,
            time: 0,
            frame_time: 1000 / fps as usize
        }
    }
}

impl<'a> Sprite for AnimatedSprite<'a> {
    fn update(&mut self, elapsed: usize) {
        self.time += elapsed;

        if self.time > self.frame_time {
            self.frame += 1;
            self.time = 0;

            if self.frame < self.frames {
                self.x += 16;
            } else {
                self.x -= 16 * (self.frames - 1) as i32;

                self.frame = 0;
            }
        }
    }

    fn render(&self, renderer: &sdl2::render::Renderer, destination: &Rect) {
        let _ = renderer.copy(self.texture, Some(Rect::new(self.x, self.y, 16, 16)), Some(*destination));
    }
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

    let world_surface = match sdl2_image::LoadSurface::from_file(&Path::new("gfx/world.png")) {
        Ok(surface) => surface,
        Err(err) => panic!("failed to load png: {}", err)
    };

    let world_sprites = match renderer.create_texture_from_surface(&world_surface) {
        Ok(texture) => texture,
        Err(err) => panic!("failed to create surface: {}", err)
    };

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


    for x in range(0, 212) {
        layer.set_tile(x, 18, Tile::Floor(Rect::new(16 * 0, 16 * 0, 16, 16)));
        layer.set_tile(x, 19, Tile::Floor(Rect::new(16 * 0, 16 * 0, 16, 16)));
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
                                    Tile::Floor(_) => d.min(t),
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

                    for y in range(intersect.y, intersect.y + intersect.h + 1) {
                        let mut x = intersect.x;

                        loop {
                            let t = (x * TILE_WIDTH + TILE_WIDTH) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.max(t),
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

                    for x in range(intersect.x, intersect.x + intersect.w + 1) {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT) as f32 - p;

                            if t > d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.min(t),
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

                    for x in range(intersect.x, intersect.x + intersect.w + 1) {
                        let mut y = intersect.y;

                        loop {
                            let t = (y * TILE_HEIGHT + TILE_HEIGHT) as f32 - p;

                            if t < d {
                                break;
                            }

                            if let Some(tile) = layer.get_tile(x, y) {
                                d = match *tile {
                                    Tile::Floor(_) => d.max(t),
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

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        layer.for_each_intersecting(&camera.to_rect(), |tile: &Tile, position: &Rect| {
            let object = camera_relative_rect(&camera.to_rect(), position);

            match *tile {
                Tile::Empty => (),
                Tile::Background(src) => {
                    let _ = renderer.copy(&world_sprites, Some(src), Some(object));
                }
                Tile::Floor(src) => {
                    let _ = renderer.copy(&world_sprites, Some(src), Some(object));
                }
            }
        });

        let player_rect = camera_relative_rect(&camera.to_rect(), &player.to_rect());

        player_sprite.render(&renderer, &player_rect);

        renderer.present();

        delay(5);
    }

    sdl2_image::quit();
    sdl2::quit();
}

fn camera_relative_rect(camera: &Rect, other: &Rect) -> Rect {
    Rect::new(other.x - camera.x, other.y - camera.y, other.w, other.h)
}
