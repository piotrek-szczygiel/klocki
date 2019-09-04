use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

use ggez::{self, input::keyboard::KeyCode};

const MAX_KEYCODES: usize = 161;

#[derive(Debug, Clone)]
pub enum Action {
    HardFall,
    SoftFall,
}

#[derive(Debug, Clone)]
struct KeyBind {
    keycode: KeyCode,
    actions: Vec<Action>,
}

pub struct Input {
    key_pressed: Vec<bool>,
    last_press: Vec<Option<Instant>>,
    key_binds: HashMap<KeyCode, Vec<Action>>,
    actions: VecDeque<Action>,
}

impl Input {
    pub fn new() -> Self {
        let mut key_pressed = Vec::with_capacity(MAX_KEYCODES);
        let mut last_press = Vec::with_capacity(MAX_KEYCODES);

        for _ in 0..MAX_KEYCODES {
            key_pressed.push(false);
            last_press.push(None);
        }

        Input {
            key_pressed,
            last_press,
            key_binds: HashMap::new(),
            actions: VecDeque::new(),
        }
    }

    pub fn bind_key(&mut self, keycode: KeyCode, action: Action) -> &mut Self {
        match self.key_binds.get_mut(&keycode) {
            None => {
                self.key_binds.insert(keycode, vec![action]);
            }
            Some(bind) => {
                bind.push(action);
            }
        };

        self
    }

    fn key_active(&mut self, _keycode: KeyCode) -> bool {
        true
    }

    pub fn update(&mut self) {
        // for (key, binds) in &mut self.key_binds {
        // 	if self.key_active(*key) {

        // 	}
        // }
    }

    pub fn key_down(&mut self, keycode: KeyCode) {
        self.key_pressed[keycode as usize] = true;
    }

    pub fn key_up(&mut self, keycode: KeyCode) {
        self.key_pressed[keycode as usize] = false;
        self.last_press[keycode as usize] = None;
    }
}
