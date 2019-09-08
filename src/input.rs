use std::{
    collections::{HashMap, VecDeque},
    time::{Duration, Instant},
};

use ggez::{self, input::keyboard::KeyCode, Context};

const MAX_KEYCODES: usize = 161;

#[derive(Copy, Clone, Debug)]
pub enum Action {
    MoveRight,
    MoveLeft,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
    HardFall,
    SoftFall,
    HoldPiece,
}

#[derive(Debug)]
pub struct Repeat {
    delay: Duration,
    interval: Duration,
}

#[derive(Debug)]
struct KeyBind {
    actions: Vec<Action>,
    repeat: Option<Repeat>,
}

pub struct Input {
    key_activated: Vec<Option<Instant>>,
    key_repeated: Vec<Option<Instant>>,
    key_binds: HashMap<KeyCode, KeyBind>,
    actions: VecDeque<Action>,
}

impl Input {
    pub fn new() -> Input {
        let mut key_activated = Vec::with_capacity(MAX_KEYCODES);
        let mut key_repeated = Vec::with_capacity(MAX_KEYCODES);

        for _ in 0..MAX_KEYCODES {
            key_activated.push(None);
            key_repeated.push(None);
        }

        Input {
            key_activated,
            key_repeated,
            key_binds: HashMap::new(),
            actions: VecDeque::new(),
        }
    }

    pub fn bind(
        &mut self,
        keycode: KeyCode,
        action: Action,
        repeat: Option<(u64, u64)>,
    ) -> &mut Input {
        match self.key_binds.get_mut(&keycode) {
            None => {
                let repeat = match repeat {
                    None => None,
                    Some(repeat) => Some(Repeat {
                        delay: Duration::from_millis(repeat.0),
                        interval: Duration::from_millis(repeat.1),
                    }),
                };

                self.key_binds.insert(
                    keycode,
                    KeyBind {
                        actions: vec![action],
                        repeat,
                    },
                );
            }
            Some(bind) => {
                bind.actions.push(action);
            }
        };

        self
    }

    pub fn update(&mut self, ctx: &Context) {
        let now = Instant::now();

        let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);

        for (keycode, bind) in &self.key_binds {
            let key = *keycode as usize;

            if !pressed_keys.contains(keycode) {
                self.key_activated[key] = None;
                self.key_repeated[key] = None;
                continue;
            }

            let mut active = false;

            match self.key_activated[key] {
                None => {
                    self.key_activated[key] = Some(now);
                    active = true;
                }
                Some(key_activated) => {
                    if let Some(repeat) = &bind.repeat {
                        if now - key_activated >= repeat.delay {
                            match self.key_repeated[key] {
                                None => {
                                    self.key_repeated[key] = Some(now);
                                    active = true;
                                }
                                Some(key_repeated) => {
                                    if now - key_repeated >= repeat.interval {
                                        self.key_repeated[key] = Some(now);
                                        active = true;
                                    }
                                }
                            };
                        }
                    }
                }
            };

            if active {
                self.actions.extend(&bind.actions);
            }
        }
    }

    pub fn action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }
}
