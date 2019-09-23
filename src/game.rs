use std::{
    env, fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use chrono::Utc;
use dirs;
use ggez::{
    audio::{self, SoundSource},
    event::{self, EventHandler, KeyMods, MouseButton},
    graphics::{self, Image, Rect},
    input::keyboard::KeyCode,
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};
use rand::{thread_rng, RngCore};

use crate::{
    action::Action,
    gameplay::Gameplay,
    global::Global,
    imgui_wrapper::ImGuiWrapper,
    input::Input,
    matrix,
    particles::ParticleAnimation,
    replay::{Replay, ReplayData},
    utils,
};

pub struct Game {
    pub g: Global,
    input: Input,
    gameplay: Gameplay,
    game_over: bool,
    background: Image,
    particle_animation: ParticleAnimation,
    music: audio::Source,

    imgui_wrapper: ImGuiWrapper,
    is_fullscreen: bool,
    fullscreen_delay: Duration,

    replay: Option<Replay>,
}

impl Game {
    pub fn new(ctx: &mut Context, mut g: Global) -> GameResult<Game> {
        let mut input = Input::new();
        input
            .bind(KeyCode::Right, Action::MoveRight, true)
            .bind(KeyCode::Left, Action::MoveLeft, true)
            .bind(KeyCode::Down, Action::MoveDown, true)
            .bind(KeyCode::Up, Action::RotateClockwise, false)
            .bind(KeyCode::X, Action::RotateClockwise, false)
            .bind(KeyCode::Z, Action::RotateCounterClockwise, false)
            .bind(KeyCode::Space, Action::HardDrop, false)
            .bind(KeyCode::LShift, Action::SoftDrop, false)
            .bind(KeyCode::C, Action::HoldPiece, false)
            .exclude(KeyCode::Right, KeyCode::Left)
            .exclude(KeyCode::Left, KeyCode::Right);

        let mut replay = None;
        if let Some(path) = env::args().nth(1) {
            let path = PathBuf::from(path);
            if path.is_file() {
                if let Some(replay_data) = ReplayData::load(&path) {
                    if let Ok(r) = Replay::new(ctx, &mut g, replay_data) {
                        replay = Some(r);
                    }
                }
            }
        }

        let mut seed = [0u8; 32];
        thread_rng().fill_bytes(&mut seed);

        let gameplay = Gameplay::new(ctx, &mut g, true, &seed)?;

        let rect = graphics::screen_coordinates(ctx);
        let particle_animation = ParticleAnimation::new(130, 200.0, 80.0, rect.w, rect.h);

        let mut music = audio::Source::new(ctx, utils::path(ctx, "chiptronical.ogg"))?;
        music.set_repeat(true);
        music.set_volume(g.settings.audio.music_volume as f32 / 100.0);
        music.play()?;

        let mut path = dirs::data_local_dir().unwrap_or_default();
        path.push("klocki");
        path.push("replays");
        fs::create_dir_all(&path)
            .unwrap_or_else(|e| log::warn!("Unable to create directory {:?}: {:?}", &path, e));

        let mut app = Game {
            g,
            input,
            gameplay,
            game_over: false,
            background: Image::new(ctx, utils::path(ctx, "background.jpg"))?,
            particle_animation,
            music,
            imgui_wrapper: ImGuiWrapper::new(ctx),
            is_fullscreen: false,
            fullscreen_delay: Duration::new(0, 0),
            replay,
        };

        app.resize_event(
            ctx,
            app.g.settings.graphics.window_size.0 as f32,
            app.g.settings.graphics.window_size.1 as f32,
        );

        Ok(app)
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let start = Instant::now();

        let fullscreen = self.g.settings.graphics.fullscreen;
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
            let mut seed = [0u8; 32];
            thread_rng().fill_bytes(&mut seed);

            self.gameplay = Gameplay::new(ctx, &mut self.g, true, &seed)?;
            self.game_over = false;
        }

        if self.g.settings_state.restart {
            event::quit(ctx);
        }

        if self.g.settings.graphics.animated_background {
            self.particle_animation.update(ctx)?;
        }

        if (self.music.volume() * 100.0) as u32 != self.g.settings.audio.music_volume {
            self.music
                .set_volume(self.g.settings.audio.music_volume as f32 / 100.0);
        }

        if self.g.sfx.volume() != self.g.settings.audio.sfx_volume {
            self.g.sfx.set_volume(self.g.settings.audio.sfx_volume);
        }

        let mut gameplay = &mut self.gameplay;

        match &mut self.replay {
            Some(replay) => {
                replay.update(ctx);
                gameplay = &mut replay.gameplay;
            }
            None => {
                if !gameplay.blocked() {
                    self.input
                        .update(ctx, self.g.settings.input.das, self.g.settings.input.arr);
                    gameplay.actions(&self.input.actions());
                }
            }
        }

        gameplay.update(ctx, &mut self.g, true)?;

        if gameplay.explosion() {
            self.particle_animation.explode(Point2::new(960.0, 540.0));
        }

        if self.replay.is_none() {
            if self.gameplay.game_over() && !self.game_over {
                self.game_over = true;
                self.g.imgui_state.game_over_window = true;
                self.g.imgui_state.replay_score = self.gameplay.score();
            }

            if self.g.imgui_state.save_replay {
                self.g.imgui_state.save_replay = false;
                let mut path = dirs::data_local_dir().unwrap_or_default();
                path.push("klocki");
                path.push("replays");
                path.push(format!(
                    "Score {} - {}.klocki",
                    self.gameplay.score(),
                    Utc::now().format("%Y%m%d_%H%M%S"),
                ));

                self.gameplay.replay_data().save(&path);
                ReplayData::load(&path).unwrap();
            }
        }

        self.g.imgui_state.update.push(start.elapsed());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let start = Instant::now();

        graphics::clear(ctx, graphics::WHITE);

        let coords = graphics::screen_coordinates(ctx);
        let ratio = coords.w / coords.h;

        graphics::draw(
            ctx,
            &self.background,
            graphics::DrawParam::new().scale(Vector2::new(
                if ratio > (21.0 / 9.0) {
                    ratio / (21.0 / 9.0)
                } else {
                    1.0
                },
                1.0,
            )),
        )?;

        if self.g.settings.graphics.animated_background {
            self.particle_animation.draw(ctx)?;
        }

        let coords = graphics::screen_coordinates(ctx);
        let position_center = Point2::new(
            (coords.w - (matrix::WIDTH * self.g.settings.gameplay.block_size) as f32) / 2.0,
            (coords.h - (matrix::HEIGHT * self.g.settings.gameplay.block_size) as f32) / 2.0,
        );

        let gameplay = if let Some(replay) = &mut self.replay {
            &mut replay.gameplay
        } else {
            &mut self.gameplay
        };

        gameplay.draw(ctx, &self.g, position_center)?;

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
            KeyCode::F11 => self.g.settings.graphics.fullscreen ^= true,
            KeyCode::D => self.imgui_wrapper.toggle_window(),
            KeyCode::Escape => event::quit(ctx),
            KeyCode::LAlt => self.g.settings.graphics.hide_menu ^= true,
            _ => (),
        };
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.g.settings.graphics.window_size.0 = width as u32;
        self.g.settings.graphics.window_size.1 = height as u32;

        let ratio = width / height;
        let width = 1080.0 * ratio;
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, 1080.0))
            .expect("Unable to change the coordinates");
    }
}
