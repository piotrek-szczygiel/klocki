#![allow(dead_code)]

mod bag;
mod blocks;
mod game;
mod imgui_wrapper;
mod input;
mod matrix;
mod particles;
mod piece;
mod shape;
mod utils;

use crate::{game::Game, imgui_wrapper::ImGuiWrapper, input::Input};

use env_logger;
use log::{self, LevelFilter};

use ggez::{
    conf,
    event::{self, EventHandler, KeyMods, MouseButton},
    graphics,
    input::keyboard::KeyCode,
    Context, ContextBuilder, GameResult,
};

fn main() {
    env_logger::builder()
        .default_format_timestamp(false)
        .filter_module("ggez", LevelFilter::Warn)
        .filter_module("tetris", LevelFilter::Trace)
        .init();

    if let Some(err) = real_main().err() {
        log::error!("{}", err);
    }
}

fn real_main() -> GameResult {
    log::debug!("Creating the context");

    let mut cb = ContextBuilder::new("tetris", "piotrek-szczygiel")
        .with_conf_file(false)
        .window_setup(
            conf::WindowSetup::default()
                .title("Tetris")
                .samples(conf::NumSamples::Four)
                .vsync(false),
        )
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0));

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
        cb = cb.add_zipfile_bytes(include_bytes!("../resources.zip").to_vec());
    }

    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0))?;

    let game = &mut Application::new(ctx)?;

    log::info!("Starting the event loop");
    event::run(ctx, event_loop, game)?;
    log::info!("Exiting the application");

    Ok(())
}

struct Application {
    game: game::Game,
    input: input::Input,
    imgui_wrapper: imgui_wrapper::ImGuiWrapper,
    is_fullscreen: bool,
}

impl Application {
    fn new(ctx: &mut Context) -> GameResult<Application> {
        let imgui = ImGuiWrapper::new(ctx);
        let mut input = input::Input::new();

        Ok(Application {
            game: game::Game::new(ctx, &mut input, &imgui)?,
            input,
            imgui_wrapper: imgui,
            is_fullscreen: false,
        })
    }
}

impl EventHandler for Application {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.imgui_wrapper.state.toggle_fullscreen {
            self.imgui_wrapper.state.toggle_fullscreen = false;
            self.is_fullscreen = !self.is_fullscreen;
            let fullscreen_type = if self.is_fullscreen {
                conf::FullscreenType::True
            } else {
                conf::FullscreenType::Windowed
            };
            graphics::set_fullscreen(ctx, fullscreen_type)?;
        }

        if self.imgui_wrapper.state.restart {
            self.input = Input::new();
            self.game = Game::new(ctx, &mut self.input, &self.imgui_wrapper)?;
        }

        self.input.update(ctx);
        self.game
            .update(ctx, &mut self.input, &self.imgui_wrapper)?;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        self.game.draw(ctx)?;
        self.imgui_wrapper.draw(ctx);
        graphics::present(ctx)?;
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::F11 => {
                self.imgui_wrapper.state.toggle_fullscreen = true;
            }
            KeyCode::D => self.imgui_wrapper.toggle_window(),
            KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((
            button == MouseButton::Left,
            button == MouseButton::Right,
            button == MouseButton::Middle,
        ));
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.imgui_wrapper.update_mouse_down((false, false, false));
    }
}
