mod bag;
mod blocks;
mod game;
mod gameplay;
mod global;
mod holder;
mod imgui_wrapper;
mod input;
mod matrix;
mod particles;
mod piece;
mod score;
mod settings;
mod shape;
mod utils;

use crate::{game::Game, global::Global};
use std::ffi::OsStr;

use env_logger;
use log::{self, LevelFilter};

use ggez::{conf, event, filesystem, graphics, ContextBuilder, GameResult};

use imgui::ImString;

fn main() {
    std::env::set_var("WINIT_UNIX_BACKEND", "x11");

    env_logger::builder()
        .default_format_timestamp(false)
        .filter_module("ggez", LevelFilter::Warn)
        .filter_module("klocki", LevelFilter::Trace)
        .init();

    if let Some(err) = real_main().err() {
        log::error!("{}", err);
    }
}

fn real_main() -> GameResult {
    loop {
        let mut g = Global::new();

        const VERSION: &str = env!("CARGO_PKG_VERSION");

        log::debug!("Creating the context");
        let mut cb = ContextBuilder::new("klocki", "piotrek-szczygiel")
            .with_conf_file(false)
            .window_setup(
                conf::WindowSetup::default()
                    .title(&format!("Klocki v{}", VERSION))
                    .samples(g.settings.multi_sampling)
                    .vsync(false),
            )
            .window_mode(
                conf::WindowMode::default()
                    .dimensions(g.settings.width, g.settings.height)
                    .min_dimensions(450.0, 600.0)
                    .resizable(true),
            );

        // Read from resources directory on debug mode
        #[cfg(build = "debug")]
        {
            log::debug!("Adding resources path");
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let mut path = std::path::PathBuf::from(manifest_dir);
            path.push("resources");
            cb = cb.add_resource_path(path);
        }

        // Read from baked in zip file on release mode
        #[cfg(build = "release")]
        {
            log::debug!("Loading resources from baked archive");
            cb = cb.add_zipfile_bytes(include_bytes!("../target/resources.zip").to_vec());
        }

        let (ctx, event_loop) = &mut cb.build()?;

        graphics::set_window_icon(ctx, Some(utils::path(ctx, "icon.ico")))?;

        g.settings_state.skins = filesystem::read_dir(ctx, utils::path(ctx, "blocks"))?
            .filter(|p| p.extension().unwrap_or_else(|| OsStr::new("")) == "png")
            .collect();
        g.settings_state.skins_imstr = g
            .settings_state
            .skins
            .iter()
            .map(|s| ImString::from(String::from(s.file_name().unwrap().to_str().unwrap())))
            .collect();

        let game = &mut Game::new(ctx, g)?;

        log::info!("Starting the event loop");
        event::run(ctx, event_loop, game)?;

        log::info!("Saving the settings");
        game.g.settings.save().expect("Unable to save settings");

        if !game.g.settings_state.restart {
            break Ok(());
        }
    }
}
