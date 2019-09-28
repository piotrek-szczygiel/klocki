use std::{collections::HashMap, time::Duration};

use ggez::{self, input::keyboard::KeyCode, timer, Context};

use crate::action::Action;

const MAX_KEYCODES: usize = 161;

struct KeyBind {
    actions: Vec<Action>,
    repeat: bool,
}

pub struct Input {
    key_activated: Vec<Option<Duration>>,
    key_repeated: Vec<Option<Duration>>,
    key_binds: HashMap<KeyCode, KeyBind>,
    actions: Vec<Action>,
    exclusions: HashMap<KeyCode, Vec<KeyCode>>,
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
            actions: vec![],
            exclusions: HashMap::new(),
        }
    }

    pub fn bind(&mut self, keycode: KeyCode, action: Action, repeat: bool) -> &mut Input {
        match self.key_binds.get_mut(&keycode) {
            None => {
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

    pub fn exclude(&mut self, keycode: KeyCode, excludes: KeyCode) -> &mut Input {
        if let Some(exclusions) = self.exclusions.get_mut(&keycode) {
            exclusions.push(excludes);
        } else {
            self.exclusions.insert(keycode, vec![excludes]);
        }

        self
    }

    pub fn update(&mut self, ctx: &Context, das: u32, arr: u32, paused: bool) {
        let das = Duration::from_millis(das.into());
        let arr = Duration::from_millis(arr.into());

        let pressed_keys = ggez::input::keyboard::pressed_keys(ctx);
        let zero = Duration::new(0, 0);
        let dt = timer::delta(ctx);

        let mut ignore: Vec<KeyCode> = vec![];
        for exclusion in &self.exclusions {
            if pressed_keys.contains(&exclusion.0) {
                ignore.extend(exclusion.1);
            }
        }

        for (keycode, bind) in &self.key_binds {
            let key = *keycode as usize;

            if !pressed_keys.contains(keycode) {
                self.key_activated[key] = None;
                self.key_repeated[key] = None;
                continue;
            }

            if ignore.contains(keycode) {
                continue;
            }

            let mut active = false;

            match self.key_activated[key].as_mut() {
                None => {
                    if paused {
                        self.key_activated[key] = Some(das);
                    } else {
                        self.key_activated[key] = Some(zero);
                        active = true;
                    }
                }
                Some(key_activated) => {
                    *key_activated += dt;

                    if bind.repeat && *key_activated >= das {
                        match self.key_repeated[key].as_mut() {
                            None => {
                                if !paused {
                                    self.key_repeated[key] = Some(zero);
                                    active = true;
                                }
                            }
                            Some(key_repeated) => {
                                *key_repeated += dt;

                                if *key_repeated >= arr {
                                    *key_repeated = zero;
                                    active = true;
                                }
                            }
                        };
                    }
                }
            };

            if active {
                self.actions.extend(&bind.actions);
            }
        }
    }

    pub fn actions(&mut self) -> Vec<Action> {
        self.actions.drain(..).collect()
    }
}
