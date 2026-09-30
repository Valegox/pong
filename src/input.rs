use std::collections::HashMap;
use minifb::Key::{self};

pub struct InputState {
    state: HashMap<Key, bool>
}

impl InputState {
    pub fn new(listened_keys: Vec<Key>) -> InputState {
        let mut state: HashMap<Key, bool> = HashMap::new();
        for key in listened_keys {
            state.insert(key, false);
        }
        InputState { state }
    }

    pub fn update(&mut self, pressed_keys: Vec<Key>, released_keys: Vec<Key>) {
        for key in pressed_keys {
            if self.state.contains_key(&key) {
                self.state.insert(key, true);
            }
        }
        for key in released_keys {
            if self.state.contains_key(&key) {
                self.state.insert(key, false);
            }
        }
    }

    pub fn is_pressed(&self, key: &Key) -> bool {
        match self.state.get(key) {
            Some(value) => *value,
            None => false
        }
    }

    pub fn get_dir(&self, key_a: &Key, key_b: &Key) -> i8 {
        self.is_pressed(key_b) as i8 - self.is_pressed(key_a) as i8
    }
}

