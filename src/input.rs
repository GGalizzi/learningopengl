use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct Input {
    pressed_keys: HashSet<Keycode>,
    mouse_rel: (i32, i32),
}

impl Input {
    pub fn new() -> Input {
        Input {
            pressed_keys: HashSet::new(),
            mouse_rel: (0, 0),
        }
    }

    pub fn set_mouse(&mut self, xrel: i32, yrel: i32) {
        self.mouse_rel = (xrel, yrel);
    }

    pub fn press(&mut self, key: Keycode) {
        self.pressed_keys.insert(key);
    }

    pub fn release(&mut self, key: Keycode) {
        self.pressed_keys.remove(&key);
    }

    pub fn is_pressed(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn mouse_x(&self) -> f32 {
        self.mouse_rel.0 as f32
    }

    pub fn mouse_y(&self) -> f32 {
        self.mouse_rel.1 as f32
    }
}
