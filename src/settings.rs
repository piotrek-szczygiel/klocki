use std::{env, fs, io, path::PathBuf};

use crate::utils;

use ggez::{conf::NumSamples, graphics::Image, Context, GameResult};
use imgui::{self, im_str, ImStr, ImString, Ui};
use serde::{Deserialize, Serialize};
use toml;

const CONFIG_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub width: f32,
    pub height: f32,
    pub fullscreen: bool,
    pub multi_sampling: NumSamples,
    pub block_size: i32,
    pub ghost_piece: bool,
    pub animated_background: bool,
    pub skin_id: usize,
    pub music_volume: f32,
}

#[derive(Default)]
pub struct SettingsState {
    pub skins: Vec<PathBuf>,
    pub skins_imstr: Vec<ImString>,
    pub skin_switched: bool,
    pub quit: bool,
}

static SAMPLINGS: [NumSamples; 6] = [
    NumSamples::Zero,
    NumSamples::One,
    NumSamples::Two,
    NumSamples::Four,
    NumSamples::Eight,
    NumSamples::Sixteen,
];

impl Settings {
    pub fn new() -> Settings {
        if let Some(settings) = Settings::load(CONFIG_FILENAME) {
            settings
        } else {
            Settings {
                width: 1280.0,
                height: 720.0,
                fullscreen: false,
                multi_sampling: NumSamples::Zero,
                block_size: 32,
                ghost_piece: true,
                animated_background: true,
                skin_id: 0,
                music_volume: 0.2,
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        if let Ok(toml) = toml::to_string_pretty(self) {
            let mut path = env::current_exe()?;
            path.pop();
            path.push(CONFIG_FILENAME);
            fs::write(path, toml)?;
            log::info!("Config saved successfully");
            Ok(())
        } else {
            Err(io::Error::from(io::ErrorKind::InvalidData))
        }
    }

    fn load(filename: &str) -> Option<Settings> {
        let path = env::current_exe();
        if path.is_err() {
            log::error!("Unable to get executable directory");
            return None;
        }

        let mut path = path.unwrap();
        path.pop();
        path.push(filename);

        if let Ok(contents) = fs::read_to_string(path) {
            if let Ok(settings) = toml::from_str(&contents) {
                return Some(settings);
            } else {
                log::error!("Error while reading config file");
            }
        } else {
            log::warn!("Unable to find config file");
        }

        None
    }

    pub fn tileset(&self, ctx: &mut Context, state: &SettingsState) -> GameResult<Image> {
        Image::new(
            ctx,
            utils::path(ctx, state.skins[self.skin_id].to_str().unwrap()),
        )
    }

    pub fn draw(&mut self, state: &mut SettingsState, ui: &Ui) {
        let pos = 120.0;
        let header_color = [0.0, 1.0, 1.0, 1.0];

        if let Some(menu) = ui.begin_menu(im_str!("Settings"), true) {
            ui.separator();
            ui.text_colored(header_color, im_str!("Graphics"));
            ui.separator();

            {
                ui.text(im_str!("Fullscreen"));
                ui.same_line(pos);
                let id = ui.push_id("fullscreen");
                ui.checkbox(im_str!(""), &mut self.fullscreen);
                id.pop(&ui);

                let mut sampling_id = SAMPLINGS
                    .iter()
                    .position(|&s| s == self.multi_sampling)
                    .unwrap();

                ui.text(im_str!("Multi-Sampling"));
                ui.same_line(pos);
                let mut open_popup = false;
                let id = ui.push_id(im_str!("sampling"));
                if imgui::ComboBox::new(im_str!("")).build_simple_string(
                    &ui,
                    &mut sampling_id,
                    &[
                        im_str!("Off"),
                        im_str!("1x"),
                        im_str!("2x"),
                        im_str!("4x"),
                        im_str!("8x"),
                        im_str!("16x"),
                    ],
                ) {
                    self.multi_sampling = SAMPLINGS[sampling_id];
                    open_popup = true;
                }
                id.pop(&ui);

                if open_popup {
                    ui.open_popup(im_str!("Restart needed"));
                }
            }

            ui.separator();
            ui.text_colored(header_color, im_str!("Gameplay"));
            ui.separator();

            {
                ui.text(im_str!("Ghost piece"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("ghost_piece"));
                ui.checkbox(im_str!(""), &mut self.ghost_piece);
                id.pop(&ui);

                ui.text(im_str!("Background"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("background"));
                ui.checkbox(im_str!(""), &mut self.animated_background);
                id.pop(&ui);

                ui.text(im_str!("Block size"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("block_size"));
                imgui::Slider::new(im_str!(""), 16..=44).build(&ui, &mut self.block_size);
                id.pop(&ui);

                ui.text(im_str!("Skin"));
                ui.same_line(pos);
                let skins: Vec<&ImStr> = state.skins_imstr.iter().map(|s| s.as_ref()).collect();
                let id = ui.push_id(im_str!("skins"));
                if imgui::ComboBox::new(im_str!("")).build_simple_string(
                    &ui,
                    &mut self.skin_id,
                    &skins,
                ) {
                    state.skin_switched = true;
                }
                id.pop(&ui);
            }

            ui.separator();
            ui.text_colored(header_color, im_str!("Audio"));
            ui.separator();

            {
                ui.text(im_str!("Music volume"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("music_volume"));
                imgui::Slider::new(im_str!(""), 0.0..=1.0)
                    .display_format(im_str!("%.2f"))
                    .build(&ui, &mut self.music_volume);
                id.pop(&ui);

                ui.popup_modal(im_str!("Restart needed")).build(|| {
                    ui.text(im_str!(
                        "You need to restart the game to apply the settings"
                    ));
                    ui.separator();

                    if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                        ui.close_current_popup();
                    }
                    ui.same_line_with_spacing(0.0, 10.0);
                    if ui.button(im_str!("Quit the game"), [0.0, 0.0]) {
                        state.quit = true;
                    }
                });
            }

            menu.end(ui);
        }
    }
}
