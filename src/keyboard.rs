use std::collections::HashMap;

use sdl2::keycode::KeyCode;


pub struct KeyboardHandler {
    pressed_keys: HashMap<KeyCode, bool>,
    released_keys: HashMap<KeyCode, bool>,
    held_keys: HashMap<KeyCode, bool>
}

impl KeyboardHandler {
    pub fn new() -> KeyboardHandler {
        KeyboardHandler {
            pressed_keys: HashMap::new(),
            released_keys: HashMap::new(),
            held_keys: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
        self.released_keys.clear();
    }

    pub fn key_down(&mut self, keycode: KeyCode) {
        self.pressed_keys.insert(keycode, true);
        self.held_keys.insert(keycode, true);
    }

    pub fn key_up(&mut self, keycode: KeyCode) {
        self.released_keys.insert(keycode, true);
        self.held_keys.insert(keycode, false);
    }

    pub fn was_pressed(&self, keycode: KeyCode) -> bool {
        match self.pressed_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }

    pub fn was_released(&self, keycode: KeyCode) -> bool {
        match self.released_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }

    pub fn is_held(&self, keycode: KeyCode) -> bool {
        match self.held_keys.get(&keycode) {
            Some(state) => *state,
            None => false
        }
    }
}
