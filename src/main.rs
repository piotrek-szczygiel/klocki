#![allow(dead_code)]

mod bag;
mod game;
mod imgui_wrapper;
mod matrix;
mod particles;
mod shape;

use env_logger;
use ggez::event::{self, EventHandler, KeyMods, MouseButton};
use ggez::graphics::{self, DrawParam, Text};
use ggez::*;
use input::keyboard::KeyCode;
use log;
use nalgebra::Point2;

use crate::imgui_wrapper::ImGuiWrapper;

fn main() -> GameResult {
    env_logger::init_from_env(
        env_logger::Env::default()
            .filter_or("LOG_LEVEL", "tetris,ggez")
            .write_style_or("LOG_STYLE", "always"),
    );

    let cb = ContextBuilder::new("tetris", "piotrek-szczygiel")
        .window_setup(
            conf::WindowSetup::default()
                .title("Tetris")
                .samples(conf::NumSamples::Sixteen)
                .vsync(false),
        )
        .window_mode(conf::WindowMode::default().dimensions(1600.0, 900.0))
        .add_zipfile_bytes(include_bytes!("../resources.zip").to_vec());

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
    imgui_wrapper: ImGuiWrapper,
}

impl Tetris {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let window_settings = WindowSettings {
            toggle_fullscreen: false,
            is_fullscreen: false,
        };

        let game = game::Game::new(ctx)?;

        let imgui_wrapper = ImGuiWrapper::new(ctx);

        Ok(Tetris {
            window_settings,
            game,
            imgui_wrapper,
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
            DrawParam::new().dest(Point2::new(10.0, 30.0)),
        )?;

        self.imgui_wrapper.render(ctx);

        graphics::present(ctx)?;

        Ok(())
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
        match keycode {
            KeyCode::F11 => {
                self.window_settings.toggle_fullscreen = true;
                self.window_settings.is_fullscreen = !self.window_settings.is_fullscreen;
            }
            KeyCode::D => self.imgui_wrapper.toggle_window(),
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
