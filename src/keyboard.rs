use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct KeyboardHandler {
    pressed_keys: HashMap<Keycode, bool>,
    released_keys: HashMap<Keycode, bool>,
    held_keys: HashMap<Keycode, bool>
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        KeyboardHandler {
            pressed_keys: HashMap::new(),
            released_keys: HashMap::new(),
            held_keys: HashMap::new()
        }
    }

    pub fn process(&mut self, event: &Event) {
        match *event {
            Event::KeyDown {keycode, repeat, ..} => {
                if repeat == false {
                    self.key_down(keycode.unwrap());
                }
            },
            Event::KeyUp {keycode, ..} => {
                self.key_up(keycode.unwrap());
            },
            _ => (),
        }
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn key_down(&mut self, keycode: Keycode) {
        self.pressed_keys.insert(keycode, true);
        self.held_keys.insert(keycode, true);
    }

    pub fn key_up(&mut self, keycode: Keycode) {
        self.released_keys.insert(keycode, true);
        self.held_keys.insert(keycode, false);
    }

    pub fn was_pressed(&self, keycode: Keycode) -> bool {
        match self.pressed_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }

    pub fn was_released(&self, keycode: Keycode) -> bool {
        match self.released_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }

    pub fn is_held(&self, keycode: Keycode) -> bool {
        match self.held_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }
}
