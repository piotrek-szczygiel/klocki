use std::{
    collections::VecDeque,
    fs,
    io::{Read, Write},
    path::Path,
    time::Duration,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use ggez::{timer, Context, GameResult};
use serde::{Deserialize, Serialize};

use crate::{action::Action, gameplay::Gameplay, global::Global};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct TimedAction {
    action: Action,
    duration: Duration,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplayData {
    pub seed: [u8; 32],
    pub actions: VecDeque<TimedAction>,
}

impl ReplayData {
    pub fn new(seed: &[u8; 32]) -> ReplayData {
        let mut seed_clone = [0; 32];
        seed_clone.clone_from_slice(seed);

        ReplayData {
            actions: VecDeque::new(),
            seed: seed_clone,
        }
    }

    pub fn add(&mut self, action: Action, duration: Duration) {
        self.actions.push_back(TimedAction { action, duration });
    }

    pub fn current_duration(&self) -> Option<Duration> {
        if let Some(action) = self.actions.get(0) {
            Some(action.duration)
        } else {
            None
        }
    }

    pub fn pop_action(&mut self) -> Action {
        if let Some(action) = self.actions.pop_front() {
            action.action
        } else {
            Action::GameOver
        }
    }

    pub fn save(&self, path: &Path) {
        let mut writer = GzEncoder::new(Vec::new(), Compression::best());
        let bytes = bincode::serialize(&self).unwrap();
        writer.write_all(&bytes).unwrap();

        if let Err(e) = fs::write(path, writer.finish().unwrap()) {
            log::error!("Unable to save replay: {:?}", e)
        } else {
            log::info!("Saved replay in {:?}", path);
        }
    }

    pub fn load(path: &Path) -> Option<ReplayData> {
        match fs::read(path) {
            Err(e) => log::error!("Unable to load replay: {:?}", e),
            Ok(bytes) => {
                let mut reader = GzDecoder::new(&bytes[..]);
                let mut bytes: Vec<u8> = vec![];

                match reader.read_to_end(&mut bytes) {
                    Err(e) => log::error!("Unable to decompress replay: {:?}", e),
                    Ok(_) => {
                        let replay_data: Result<ReplayData, _> = bincode::deserialize(&bytes);

                        match replay_data {
                            Err(e) => log::error!("Unable to deserialize replay: {:?}", e),
                            Ok(replay_data) => {
                                log::info!("Loaded replay from {:?}", path,);
                                return Some(replay_data);
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

pub struct Replay {
    replay_data: ReplayData,
    action_duration: Duration,
    pub gameplay: Gameplay,
}

impl Replay {
    pub fn new(ctx: &mut Context, g: &mut Global, replay_data: ReplayData) -> GameResult<Replay> {
        Ok(Replay {
            gameplay: Gameplay::new(ctx, g, false, &replay_data.seed)?,
            replay_data,
            action_duration: Duration::new(0, 0),
        })
    }

    pub fn update(&mut self, ctx: &mut Context) {
        self.action_duration += timer::delta(ctx);

        while let Some(duration) = self.replay_data.current_duration() {
            if self.action_duration >= duration {
                self.gameplay.action(self.replay_data.pop_action());
                self.action_duration = Duration::new(0, 0);
            } else {
                break;
            }
        }
    }
}
