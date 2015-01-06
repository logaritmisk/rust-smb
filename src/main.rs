extern crate sdl2;


use std::cmp::{max, min};

use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks, delay};
use sdl2::rect::Rect;
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color;

use tile::Layer;
use player::Player;


mod vec;
mod tile;
mod player;


const SCREEN_WIDTH : i32 = 960;
const SCREEN_HEIGHT : i32 = 640;

const TILE_WIDTH : i32 = 32;
const TILE_HEIGHT : i32 = 32;

const MS_PER_UPDATE : uint = 10;


#[derive(Clone, Show)]
enum Tile {
    Empty,
    Floor
}


struct Camera {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Camera {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Camera {
        Camera {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }

    fn get_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
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

    let camera = Camera::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut layer = Layer::new(30, 20, TILE_WIDTH, TILE_HEIGHT, Tile::Empty);

    for x in range(0, 14) {
        layer.set_tile(x, 14, Tile::Floor);
    }

    for x in range(17, 30) {
        layer.set_tile(x, 13, Tile::Floor);
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
            Event::KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                } else if key == KeyCode::Right {
                    player.velocity.x = 4.0;
                } else if key == KeyCode::Left {
                    player.velocity.x = -4.0;
                } else if key == KeyCode::Up {
                    if player.on_ground {
                        player.velocity.y = -12.0;

                        player.on_ground = false;
                    }
                }
            },
            Event::KeyUp(_, _, key, _, _, _) => {
                if key == KeyCode::Right && player.velocity.x > 0.0 {
                    player.velocity.x = 0.0;
                } else if key == KeyCode::Left && player.velocity.x < 0.0 {
                    player.velocity.x = 0.0;
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

            layer.for_each_intersecting(&player.get_rect(), |tile: &Tile, x: i32, y: i32| {
                match *tile {
                    Tile::Empty => (),
                    Tile::Floor => {
                        let object = Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT);

                        if collision_detection(&object, &player.get_rect()) {
                            let intersect = collision_intersect(&object, &player.get_rect());

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
                        }
                    }
                }
            });

            player.on_ground = false;

            let mut ground = player.get_rect();

            ground.y += ground.h;
            ground.h = 1;

            layer.for_each_intersecting(&ground, |tile: &Tile, x: i32, y: i32| {
                match *tile {
                    Tile::Empty => (),
                    Tile::Floor => {
                        let object = Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT);

                        if collision_detection(&object, &ground) {
                            player.on_ground = true;
                        }
                    }
                }
            });

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        layer.for_each_intersecting(&camera.get_rect(), |tile: &Tile, x: i32, y: i32| {
            match *tile {
                Tile::Empty => (),
                Tile::Floor => {
                    let _ = renderer.set_draw_color(Color::RGB(0, 0, 255));
                    let _ = renderer.fill_rect(&Rect::new(x, y, TILE_WIDTH, TILE_HEIGHT));
                }
            }
        });

        player.render(&renderer);

        renderer.present();

        delay(5);
    }

    sdl2::quit();
}

fn collision_detection(lhs: &Rect, rhs: &Rect) -> bool {
    if lhs.x + lhs.w <= rhs.x || rhs.x + rhs.w <= lhs.x {
        false
    }
    else if lhs.y + lhs.h <= rhs.y || rhs.y + rhs.h <= lhs.y {
        false
    }
    else {
        true
    }
}

fn collision_intersect(lhs: &Rect, rhs: &Rect) -> Rect {
    let x = max(lhs.x, rhs.x);
    let y = max(lhs.y, rhs.y);

    Rect::new(x, y, min(lhs.x + lhs.w, rhs.x + rhs.w) - x, min(lhs.y + lhs.h, rhs.y + rhs.h) - y)
}
