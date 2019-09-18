use std::time::{Duration, Instant};

use crate::{
    gameplay::Gameplay, global::Global, imgui_wrapper::ImGuiWrapper, particles::ParticleAnimation,
    utils,
};

use ggez::{
    audio::{self, SoundSource},
    event::{self, EventHandler, KeyMods, MouseButton},
    graphics::{self, Image, Rect},
    input::keyboard::KeyCode,
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

pub struct Game {
    pub g: Global,
    gameplay: Gameplay,
    background: Image,
    particle_animation: ParticleAnimation,
    music: audio::Source,

    imgui_wrapper: ImGuiWrapper,
    is_fullscreen: bool,
    fullscreen_delay: Duration,
}

impl Game {
    pub fn new(ctx: &mut Context, mut g: Global) -> GameResult<Game> {
        let rect = graphics::screen_coordinates(ctx);
        let particle_animation = ParticleAnimation::new(130, 200.0, 80.0, rect.w, rect.h);

        let mut music = audio::Source::new(ctx, utils::path(ctx, "chiptronical.ogg"))?;
        music.set_repeat(true);
        music.set_volume(0.2);
        music.play()?;

        let mut app = Game {
            gameplay: Gameplay::new(ctx, &mut g)?,
            g,
            background: Image::new(ctx, utils::path(ctx, "background.jpg"))?,
            particle_animation,
            music,
            imgui_wrapper: ImGuiWrapper::new(ctx),
            is_fullscreen: false,
            fullscreen_delay: Duration::new(0, 0),
        };

        app.resize_event(ctx, app.g.settings.width, app.g.settings.height);

        Ok(app)
    }
}

impl EventHandler for Game {
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
            self.gameplay = Gameplay::new(ctx, &mut self.g)?;
        }

        if self.g.settings_state.restart {
            event::quit(ctx);
        }

        if self.g.settings.animated_background {
            self.particle_animation.update(ctx)?;
        }

        if (self.music.volume() - self.g.settings.music_volume).abs() > 0.01 {
            self.music.set_volume(self.g.settings.music_volume);
        }

        self.gameplay.update(ctx, &self.g)?;
        if self.gameplay.explosion() {
            self.particle_animation.explode(Point2::new(960.0, 540.0));
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

        self.particle_animation.draw(ctx)?;

        self.gameplay.draw(ctx, &self.g)?;
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
