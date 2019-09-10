use std::{env, fs, io, path::PathBuf};

use crate::utils;

use ggez::{graphics::Image, Context, GameResult};
use imgui::{self, im_str, ImStr, ImString, Ui};
use serde::{Deserialize, Serialize};
use toml;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub fullscreen: bool,
    pub window_scale: f32,
    pub block_size: i32,
    pub ghost_piece: bool,
    pub animated_background: bool,
    pub skin_id: usize,
}

pub struct State {
    pub skins: Vec<PathBuf>,
    pub skins_imstr: Vec<ImString>,
    pub skin_switched: bool,
}

impl Settings {
    pub fn new(filename: &str) -> Settings {
        if let Ok(settings) = Settings::load(filename) {
            settings
        } else {
            Settings {
                fullscreen: false,
                window_scale: 0.625,
                block_size: 32,
                ghost_piece: true,
                animated_background: true,
                skin_id: 0,
            }
        }
    }

    pub fn save(&self, filename: &str) -> io::Result<()> {
        match toml::to_string_pretty(self) {
            Ok(toml) => {
                let mut path = env::current_exe()?;
                path.pop();
                path.push(filename);
                fs::write(path, toml)?;
            }
            Err(e) => {
                log::error!("Error while saving config: {:?}", e);
            }
        }

        Ok(())
    }

    fn load(filename: &str) -> io::Result<Settings> {
        let mut path = env::current_exe()?;
        path.pop();
        path.push(filename);

        let contents = fs::read_to_string(path)?;
        let settings: Settings = toml::from_str(&contents)?;

        Ok(settings)
    }

    pub fn tileset(&self, ctx: &mut Context, state: &State) -> GameResult<Image> {
        Image::new(
            ctx,
            utils::path(ctx, state.skins[self.skin_id].to_str().unwrap()),
        )
    }

    pub fn draw(&mut self, state: &mut State, ui: &Ui) {
        if let Some(menu) = ui.begin_menu(im_str!("Settings"), true) {
            ui.separator();
            ui.text(im_str!("Fullscreen"));
            let id = ui.push_id(0);
            ui.checkbox(im_str!("Enabled"), &mut self.fullscreen);
            id.pop(&ui);

            ui.separator();
            ui.text(im_str!("Window scale"));
            let id = ui.push_id(1);
            imgui::Slider::new(im_str!(""), 0.25..=2.0)
                .display_format(im_str!("%.2f"))
                .build(&ui, &mut self.window_scale);
            id.pop(&ui);

            ui.separator();
            ui.text(im_str!("Block size"));
            let id = ui.push_id(2);
            imgui::Slider::new(im_str!(""), 16..=44).build(&ui, &mut self.block_size);
            id.pop(&ui);

            ui.separator();
            ui.text(im_str!("Ghost piece"));
            let id = ui.push_id(3);
            ui.checkbox(im_str!("Enabled"), &mut self.ghost_piece);
            id.pop(&ui);

            ui.separator();
            ui.text(im_str!("Animated background"));
            let id = ui.push_id(4);
            ui.checkbox(im_str!("Enabled"), &mut self.animated_background);
            id.pop(&ui);

            ui.separator();
            ui.text(im_str!("Blocks skin"));
            let id = ui.push_id(5);
            let skins: Vec<&ImStr> = state.skins_imstr.iter().map(|s| s.as_ref()).collect();
            if imgui::ComboBox::new(im_str!("")).build_simple_string(&ui, &mut self.skin_id, &skins)
            {
                state.skin_switched = true;
            }
            id.pop(&ui);

            menu.end(ui);
        }
    }
}
