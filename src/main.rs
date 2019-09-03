#![allow(dead_code)]

mod bag;
mod game;
mod imgui_wrapper;
mod matrix;
mod particles;
mod shape;
mod utils;

use env_logger;
use log;

use ggez::{
    conf,
    event::{self, EventHandler, KeyMods, MouseButton},
    graphics::{self, Color, DrawParam, Font, Image, Scale, Text, TextFragment},
    input::{self, keyboard::KeyCode},
    nalgebra::Point2,
    timer, Context, ContextBuilder, GameResult,
};

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
    input::mouse::set_cursor_hidden(ctx, true);

    let game = &mut Tetris::new(ctx)?;

    log::info!("starting the event loop");
    event::run(ctx, event_loop, game)
}

struct Cursor {
    image: Image,
    last_position: Point2<f32>,
    duration_static: f32,
}

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
}

struct Tetris {
    window_settings: WindowSettings,
    game: game::Game,
    imgui_wrapper: imgui_wrapper::ImGuiWrapper,
    cursor: Cursor,
}

impl Tetris {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let window_settings = WindowSettings {
            toggle_fullscreen: false,
            is_fullscreen: false,
        };

        Ok(Tetris {
            window_settings,
            game: game::Game::new(ctx)?,
            imgui_wrapper: imgui_wrapper::ImGuiWrapper::new(ctx),
            cursor: Cursor {
                image: Image::new(ctx, "cursor.png")?,
                last_position: Point2::new(0.0, 0.0),
                duration_static: 0.0,
            },
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
        let fps = Text::new(TextFragment {
            text: format!("FPS: {}", fps),
            color: Some(Color::new(0.5, 0.0, 0.0, 1.0)),
            font: Some(Font::default()),
            scale: Some(Scale::uniform(16.0)),
        });

        graphics::draw(ctx, &fps, DrawParam::new().dest(Point2::new(10.0, 30.0)))?;

        self.imgui_wrapper.render(ctx);

        let pos = utils::mouse_position_coords(ctx);
        if self.cursor.last_position == pos {
            self.cursor.duration_static += utils::dt(ctx);
        } else {
            self.cursor.duration_static = 0.0;
        }

        self.cursor.last_position = pos;

        let transparency = if self.cursor.duration_static > 3.0 {
            0.0
        } else if self.cursor.duration_static > 1.0 {
            1.0 - (self.cursor.duration_static - 1.0)
        } else {
            1.0
        };

        graphics::draw(
            ctx,
            &self.cursor.image,
            DrawParam::new()
                .dest(pos)
                .color(Color::new(1.0, 1.0, 1.0, transparency)),
        )?;

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
