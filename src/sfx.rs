use std::collections::HashMap;

use ggez::{
    audio::{SoundSource, Source},
    Context, GameResult,
};

use crate::utils;

#[derive(Default)]
pub struct Sfx {
    sounds: HashMap<&'static str, Option<Source>>,
    volume: f32,
}

impl Sfx {
    pub fn load(ctx: &mut Context, volume: f32) -> GameResult<Sfx> {
        let sounds = [
            "move", "rotate", "softdrop", "harddrop", "hold", "holdfail", "lock", "linefall",
            "gameover", "erase1", "erase2", "erase3", "erase4", "tspin0", "tspin1", "tspin2",
            "tspin3",
        ]
        .iter()
        .map(|&s| (s, Sfx::source(ctx, s, volume)))
        .collect();

        Ok(Sfx { sounds, volume })
    }

    pub fn play(&mut self, name: &'static str) {
        if let Some(Some(sound)) = self.sounds.get_mut(name) {
            sound
                .play()
                .unwrap_or_else(|e| log::error!("Unable to play {}: {:?}", name, e));
        } else {
            log::warn!("Sound doesn't exist: {}", name);
        }
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) {
        for (_, sound) in self.sounds.iter_mut() {
            if let Some(sound) = sound {
                sound.set_volume(volume);
            }
        }
    }

    fn source(ctx: &mut Context, name: &'static str, volume: f32) -> Option<Source> {
        let path = String::from("sfx/") + name + ".wav";
        match Source::new(ctx, utils::path(ctx, &path)) {
            Ok(mut s) => {
                log::debug!("Loaded {}", path);
                s.set_volume(volume);
                Some(s)
            }
            Err(e) => {
                log::error!("Unable to load {}: {:?}", path, e);
                None
            }
        }
    }
}
