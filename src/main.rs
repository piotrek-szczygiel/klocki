#![allow(dead_code)]
mod bag;
mod game;
mod matrix;
mod shape;

use env_logger;
use ggez::event::{self, EventHandler, KeyMods};
use ggez::graphics::{self, DrawParam, Text};
use ggez::*;
use input::keyboard::KeyCode;
use log;
use nalgebra::Point2;
use std::{env, path};

fn main() -> GameResult {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("MY_LOG_LEVEL", "tetris,ggez")
            .write_style_or("MY_LOG_STYLE", "always"),
    );

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ContextBuilder::new("tetris", "piotrek-szczygiel")
        .window_setup(conf::WindowSetup::default().title("Tetris").vsync(false))
        .window_mode(conf::WindowMode::default().dimensions(1600.0, 900.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0))?;

    let game = &mut Tetris::new(ctx)?;

    log::info!("starting the event loop");
    event::run(ctx, event_loop, game)
}

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
}

struct Tetris {
    window_settings: WindowSettings,
    game: game::Game,
}

impl Tetris {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let window_settings = WindowSettings {
            toggle_fullscreen: false,
            is_fullscreen: false,
        };

        let game = game::Game::new(ctx)?;

        Ok(Tetris {
            window_settings,
            game,
        })
    }
}

impl EventHandler for Tetris {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.window_settings.toggle_fullscreen {
            let fullscreen_type = if self.window_settings.is_fullscreen {
                conf::FullscreenType::True
            } else {
                conf::FullscreenType::Windowed
            };
            graphics::set_fullscreen(ctx, fullscreen_type)?;
            self.window_settings.toggle_fullscreen = false;
        }

        self.game.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.draw(ctx)?;

        let fps = timer::fps(ctx) as i32;
        let fps_display = Text::new(format!("FPS: {}", fps));
        graphics::draw(
            ctx,
            &fps_display,
            DrawParam::new().dest(Point2::new(10.0, 10.0)),
        )?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if let KeyCode::Escape = keycode {
            event::quit(ctx);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        if let KeyCode::F11 = keycode {
            self.window_settings.toggle_fullscreen = true;
            self.window_settings.is_fullscreen = !self.window_settings.is_fullscreen;
        }
    }
}
