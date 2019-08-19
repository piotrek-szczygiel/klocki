use std::{env, path};

use env_logger;
use ggez::{
    self, conf, conf::FullscreenType, event, event::EventHandler, event::KeyMods, filesystem,
    graphics, graphics::DrawParam, graphics::Text, input::keyboard::KeyCode, mint::Point2, timer,
    Context, ContextBuilder, GameResult,
};
use log;

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
        .window_setup(conf::WindowSetup::default().title("Tetris"))
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0))
        .add_resource_path(resource_dir);

    let (ctx, event_loop) = &mut cb.build()?;

    if let Ok(_conf) = filesystem::read_config(ctx) {
        log::info!("found existing conf file");
    } else {
        log::info!("no existing conf file found, saving one out");
        let c = conf::Conf::new();
        filesystem::write_config(ctx, &c)?;
    }

    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1280.0, 720.0)).unwrap();

    let game = &mut Tetris::new(ctx)?;
    event::run(ctx, event_loop, game)
}

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
    toggle_vsync: bool,
    is_vsync: bool,
}

struct Tetris {
    window_settings: WindowSettings,
}

impl Tetris {
    fn new(ctx: &mut Context) -> GameResult<Tetris> {
        log::debug!("resource path: {:?}", ctx.filesystem);

        Ok(Tetris {
            window_settings: WindowSettings {
                toggle_fullscreen: false,
                is_fullscreen: false,
                toggle_vsync: false,
                is_vsync: false,
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
            ggez::graphics::set_fullscreen(ctx, fullscreen_type)?;
            self.window_settings.toggle_fullscreen = false;
        }

        if self.window_settings.toggle_vsync {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let fps = timer::fps(ctx) as i32;
        let fps_display = Text::new(format!("FPS: {}", fps));
        graphics::draw(
            ctx,
            &fps_display,
            (Point2 { x: 5.0, y: 5.0 }, graphics::WHITE),
        )?;

        graphics::present(ctx)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::F11 => {
                self.window_settings.toggle_fullscreen = true;
                self.window_settings.is_fullscreen = !self.window_settings.is_fullscreen;
            }
            _ => (),
        }
    }
}
