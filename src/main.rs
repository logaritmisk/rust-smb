extern crate sdl2;


use sdl2::video::{Window, WindowPos, OPENGL};
use sdl2::event::{poll_event, Event};
use sdl2::timer::{get_ticks};
use sdl2::rect::{Rect};


const SCREEN_WIDTH : int = 800;
const SCREEN_HEIGHT : int = 600;

const MS_PER_UPDATE : uint = 10;


fn main() {
    sdl2::init(sdl2::INIT_EVERYTHING);
    
    let window = match Window::new("SMB", WindowPos::PosCentered, WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, OPENGL) {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err)     => panic!("failed to create renderer: {}", err)
    };
        
    let mut player = Rect::new(0, 290, 10, 10);
    
    let mut x : f32 = 390.0;
    let mut y : f32 = 290.0;

    let mut v_x : f32 = 0.0;
    let mut v_y : f32 = 0.0;

    let mut on_ground = true;
    let mut gravity : f32 = 0.3;
    
    let mut current : uint = 0;
    let mut elapsed : uint = 0;
    let mut lag : uint = 0;
    
    let mut previous : uint = get_ticks();

    'event : loop {
        current = get_ticks();
        elapsed = current - previous;
        previous = current;
        lag += elapsed;

        match poll_event() {
            Event::Quit(_) => break 'event,
            Event::KeyDown(_, _, key, _, _, _) => {
                if key == sdl2::keycode::KeyCode::Escape {
                    break 'event;
                } else if key == sdl2::keycode::KeyCode::Right {
                    v_x = 4.0;
                } else if key == sdl2::keycode::KeyCode::Left {
                    v_x = -4.0;
                } else if key == sdl2::keycode::KeyCode::Up {
                    if on_ground {
                        v_y = -8.0;
                        
                        on_ground = false;
                    }
                }
            },
            Event::KeyUp(_, _, key, _, _, _) => {
                if key == sdl2::keycode::KeyCode::Right {
                    v_x = 0.0;
                } else if key == sdl2::keycode::KeyCode::Left {
                    v_x = 0.0;
                } else if key == sdl2::keycode::KeyCode::Up {
                    if v_y < -4.0 {
                        v_y = -4.0;
                    }
                }
            },
            _ => (),
        }
        
        while lag >= MS_PER_UPDATE {
            v_y += gravity;
            
            x += v_x;
            y += v_y;

            if y > 290.0 {
                y = 290.0;
                v_y = 0.0;
                
                on_ground = true;
            }

            lag -= MS_PER_UPDATE;
        }

        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        let _ = renderer.clear();

        player.x = x as i32;
        player.y = y as i32;
        
        let _ = renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 0));
        let _ = renderer.fill_rect(&player);
        
        renderer.present();
    }

    sdl2::quit();
}
