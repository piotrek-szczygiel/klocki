use std::{fs, path::PathBuf};

use dirs;
use ggez::{conf::NumSamples, graphics::Image, Context, GameResult};
use imgui::{self, im_str, ComboBox, FontId, ImStr, ImString, Slider, Ui};
use serde::{Deserialize, Serialize};
use toml;

use crate::utils;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub graphics: Graphics,
    pub gameplay: Gameplay,
    pub audio: Audio,
    pub input: Input,
}

#[derive(Serialize, Deserialize)]
pub struct Graphics {
    pub window_size: (u32, u32),
    pub fullscreen: bool,
    pub vsync: bool,
    pub animated_background: bool,
    pub hide_menu: bool,
    pub multi_sampling: NumSamples,
}

#[derive(Serialize, Deserialize)]
pub struct Gameplay {
    pub block_size: i32,
    pub ghost_piece: u32,
    pub clear_delay: u32,
    pub skin: String,
    pub stack_outline: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Audio {
    pub music_volume: u32,
    pub sfx_volume: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub das: u32,
    pub arr: u32,
}

#[derive(Default)]
pub struct SettingsState {
    pub skins: Vec<PathBuf>,
    pub skins_imstr: Vec<ImString>,
    pub skin_id: usize,
    pub skin_switched: bool,
    pub restart: bool,
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
        if let Some(settings) = Settings::load() {
            settings
        } else {
            Settings {
                graphics: Graphics {
                    window_size: (800, 800),
                    fullscreen: false,
                    multi_sampling: NumSamples::Zero,
                    vsync: true,
                    animated_background: true,
                    hide_menu: false,
                },
                gameplay: Gameplay {
                    block_size: 43,
                    ghost_piece: 10,
                    clear_delay: 0,
                    skin: String::from("friends.png"),
                    stack_outline: true,
                },
                audio: Audio {
                    music_volume: 10,
                    sfx_volume: 50,
                },
                input: Input { das: 150, arr: 50 },
            }
        }
    }

    fn path() -> PathBuf {
        let mut path = dirs::data_local_dir().unwrap_or_default();
        path.push("klocki");
        path.push("config.toml");
        path
    }

    pub fn save(&self) {
        let toml = toml::to_string(self).unwrap();
        let path = Settings::path();
        fs::write(&path, toml).unwrap_or_else(|e| panic!("Unable to save settings: {:?}", e));
        log::info!("Saved settings to: {:?}", &path);
    }

    fn load() -> Option<Settings> {
        let path = Settings::path();

        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(settings) = toml::from_str(&contents) {
                log::info!("Loaded settings from: {:?}", &path);
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
            utils::path(ctx, state.skins[state.skin_id].to_str().unwrap()),
        )
    }

    pub fn draw(&mut self, state: &mut SettingsState, ui: &Ui, bold: FontId) {
        let pos = 120.0;
        let header_color = [0.6, 0.8, 1.0, 1.0];

        if let Some(menu) = ui.begin_menu(im_str!("Settings"), true) {
            ui.separator();

            let id = ui.push_font(bold);
            ui.text_colored(header_color, im_str!("Graphics"));
            id.pop(&ui);
            ui.separator();

            {
                let mut restart_popup = false;

                ui.text(im_str!("Fullscreen"));
                ui.same_line(pos);
                ui.checkbox(im_str!("<F11>"), &mut self.graphics.fullscreen);

                let mut sampling_id = SAMPLINGS
                    .iter()
                    .position(|&s| s == self.graphics.multi_sampling)
                    .unwrap();

                ui.text(im_str!("V-Sync"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("vsync"));
                if ui.checkbox(im_str!(""), &mut self.graphics.vsync) {
                    restart_popup = true;
                }
                id.pop(&ui);

                ui.text(im_str!("Background"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("background"));
                ui.checkbox(im_str!(""), &mut self.graphics.animated_background);
                id.pop(&ui);

                ui.text(im_str!("Hide menu"));
                ui.same_line(pos);
                if ui.checkbox(im_str!("<Left Alt>"), &mut self.graphics.hide_menu) {
                    ui.open_popup(im_str!("Menu visibility information"));
                }

                ui.text(im_str!("Sampling"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("sampling"));
                if ComboBox::new(im_str!("")).build_simple_string(
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
                    self.graphics.multi_sampling = SAMPLINGS[sampling_id];
                    restart_popup = true;
                }
                id.pop(&ui);

                if restart_popup {
                    ui.open_popup(im_str!("Restart needed"));
                }
            }

            ui.separator();
            let id = ui.push_font(bold);
            ui.text_colored(header_color, im_str!("Gameplay"));
            id.pop(&ui);
            ui.separator();

            {
                ui.text(im_str!("Ghost piece"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("ghost_piece"));
                Slider::new(im_str!(""), 0..=100).build(&ui, &mut self.gameplay.ghost_piece);
                id.pop(&ui);

                ui.text(im_str!("Block size"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("block_size"));
                Slider::new(im_str!(""), 16..=44).build(&ui, &mut self.gameplay.block_size);
                id.pop(&ui);

                ui.text(im_str!("Clear delay"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("clear_delay"));
                Slider::new(im_str!(""), 0..=2000).build(&ui, &mut self.gameplay.clear_delay);
                id.pop(&ui);

                ui.text(im_str!("Skin"));
                ui.same_line(pos);
                let skins: Vec<&ImStr> = state.skins_imstr.iter().map(|s| s.as_ref()).collect();
                let id = ui.push_id(im_str!("skins"));
                if ComboBox::new(im_str!("")).build_simple_string(&ui, &mut state.skin_id, &skins) {
                    state.skin_switched = true;

                    self.gameplay.skin = String::from(
                        state
                            .skins_imstr
                            .get(state.skin_id)
                            .unwrap_or(&ImString::new(""))
                            .to_str(),
                    );
                }
                id.pop(&ui);

                ui.text(im_str!("Stack outline"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("stack_outline"));
                ui.checkbox(im_str!(""), &mut self.gameplay.stack_outline);
                id.pop(&ui);
            }

            ui.separator();
            let id = ui.push_font(bold);
            ui.text_colored(header_color, im_str!("Audio"));
            id.pop(&ui);
            ui.separator();

            {
                ui.text(im_str!("Music"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("music"));
                Slider::new(im_str!(""), 0..=100).build(&ui, &mut self.audio.music_volume);
                id.pop(&ui);

                ui.text(im_str!("SFX"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("sfx"));
                Slider::new(im_str!(""), 0..=100).build(&ui, &mut self.audio.sfx_volume);
                id.pop(&ui);
            }

            ui.separator();
            let id = ui.push_font(bold);
            ui.text_colored(header_color, im_str!("Input"));
            id.pop(&ui);
            ui.separator();

            {
                ui.text(im_str!("DAS"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("das"));
                Slider::new(im_str!(""), 100..=500).build(&ui, &mut self.input.das);
                id.pop(&ui);

                ui.text(im_str!("ARR"));
                ui.same_line(pos);
                let id = ui.push_id(im_str!("arr"));
                Slider::new(im_str!(""), 5..=200).build(&ui, &mut self.input.arr);
                id.pop(&ui);
            }

            ui.popup_modal(im_str!("Restart needed")).build(|| {
                ui.text(im_str!(
                    "You need to restart the game to apply these settings"
                ));
                ui.separator();

                if ui.button(im_str!("Cancel"), [0.0, 0.0]) {
                    ui.close_current_popup();
                }
                ui.same_line_with_spacing(0.0, 10.0);
                if ui.button(im_str!("Restart the game"), [0.0, 0.0]) {
                    state.restart = true;
                }
            });

            menu.end(ui);
        }
    }
}
