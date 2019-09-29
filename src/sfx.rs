use std::collections::HashMap;

use ggez::{
    audio::{SoundSource, Source},
    Context, GameResult,
};

use crate::utils;

#[derive(Default)]
pub struct Sfx {
    sounds: HashMap<&'static str, Option<Source>>,
    volume: u32,
}

impl Sfx {
    pub fn load(ctx: &mut Context, volume: u32) -> GameResult<Sfx> {
        let sounds = [
            "ready", "go", "gameover", "levelup", "move", "rotate", "harddrop", "hold", "lock",
            "erase1", "erase2", "erase3", "erase4", "tspin1", "tspin2", "tspin3",
        ]
        .iter()
        .map(|&s| (s, Sfx::source(ctx, s, volume)))
        .collect();

        Ok(Sfx { sounds, volume })
    }

    pub fn play(&mut self, name: &'static str) {
        if let Some(Some(sound)) = self.sounds.get_mut(name) {
            sound
                .play_detached()
                .unwrap_or_else(|e| log::error!("Unable to play {}: {:?}", name, e));
        } else {
            log::warn!("Sound doesn't exist: {}", name);
        }
    }

    pub fn volume(&self) -> u32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: u32) {
        for (_, sound) in self.sounds.iter_mut() {
            if let Some(sound) = sound {
                sound.set_volume(volume as f32 / 100.0);
            }
        }
    }

    fn source(ctx: &mut Context, name: &'static str, volume: u32) -> Option<Source> {
        let path = String::from("sfx/") + name + ".wav";
        match Source::new(ctx, utils::path(ctx, &path)) {
            Ok(mut s) => {
                log::debug!("Loaded {}", path);
                s.set_volume(volume as f32 / 100.0);
                Some(s)
            }
            Err(e) => {
                log::error!("Unable to load {}: {:?}", path, e);
                None
            }
        }
    }
}
