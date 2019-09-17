mod bag;
mod blocks;
mod game;
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

use std::time::{Duration, Instant};

use crate::{game::Game, global::Global, imgui_wrapper::ImGuiWrapper};

use env_logger;
use log::{self, LevelFilter};

use ggez::{
    conf,
    event::{self, EventHandler, KeyMods, MouseButton},
    graphics::{self, Rect},
    input::keyboard::KeyCode,
    timer, Context, ContextBuilder, GameResult,
};

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
        let g = Global::new();

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

        let app = &mut Application::new(ctx, g)?;

        log::info!("Starting the event loop");
        event::run(ctx, event_loop, app)?;

        log::info!("Saving the settings");
        app.g.settings.save().expect("Unable to save settings");

        if !app.g.settings_state.restart {
            break Ok(());
        }
    }
}

struct Application {
    g: Global,
    game: game::Game,
    imgui_wrapper: imgui_wrapper::ImGuiWrapper,
    is_fullscreen: bool,
    fullscreen_delay: Duration,
}

impl Application {
    fn new(ctx: &mut Context, mut g: Global) -> GameResult<Application> {
        let mut app = Application {
            game: game::Game::new(ctx, &mut g)?,
            g,
            imgui_wrapper: ImGuiWrapper::new(ctx),
            is_fullscreen: false,
            fullscreen_delay: Duration::new(0, 0),
        };

        app.resize_event(ctx, app.g.settings.width, app.g.settings.height);

        Ok(app)
    }
}

impl EventHandler for Application {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let start = Instant::now();

        let fullscreen = self.g.settings.fullscreen;
        if fullscreen != self.is_fullscreen && self.fullscreen_delay > Duration::from_millis(300) {
            self.is_fullscreen = fullscreen;
            self.fullscreen_delay = Duration::new(0, 0);

            let fullscreen_type = if self.is_fullscreen {
                Some(graphics::window(ctx).get_current_monitor())
            } else {
                None
            };
            graphics::window(ctx).set_fullscreen(fullscreen_type);
        } else {
            self.fullscreen_delay += timer::delta(ctx);
        }

        if self.g.imgui_state.restart {
            self.game = Game::new(ctx, &mut self.g)?;
        }

        if self.g.settings_state.restart {
            event::quit(ctx);
        }

        self.game.update(ctx, &self.g)?;

        self.g.imgui_state.update.push(start.elapsed());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let start = Instant::now();

        graphics::clear(ctx, graphics::WHITE);
        self.game.draw(ctx, &self.g)?;
        self.imgui_wrapper.draw(ctx, &mut self.g);

        self.g.imgui_state.draw.push(start.elapsed());

        graphics::present(ctx)?;
        Ok(())
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

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.imgui_wrapper.update_mouse_pos(x, y);
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, y: f32) {
        self.imgui_wrapper.update_mouse_scroll(y);
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::F11 => self.g.settings.fullscreen ^= true,
            KeyCode::D => self.imgui_wrapper.toggle_window(),
            KeyCode::Escape => event::quit(ctx),
            KeyCode::LAlt => self.g.settings.hide_menu ^= true,
            _ => (),
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.g.settings.width = width;
        self.g.settings.height = height;

        let ratio = width / height;
        let width = 1080.0 * ratio;
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, 1080.0))
            .expect("Unable to change the coordinates");
    }
}
