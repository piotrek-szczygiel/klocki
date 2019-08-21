use env_logger;
use ggez::event::{self, EventHandler, KeyMods};
use ggez::graphics::{self, Color, DrawParam, Image, Text};
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

    graphics::set_screen_coordinates(ctx, graphics::Rect::new(0.0, 0.0, 1920.0, 1080.0)).unwrap();

    let game = &mut Tetris::new(ctx)?;

    log::info!("starting the event loop");
    event::run(ctx, event_loop, game)
}

type Kick = [(i32, i32); 4];
type Kicks = [(Kick, Kick); 4];

const KICKS_JLSTZ: Kicks = [
    (
        [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
        [(1, 0), (1, -1), (0, 2), (1, 2)],
    ),
    (
        [(1, 0), (1, 1), (0, -2), (1, -2)],
        [(1, 0), (1, 1), (0, -2), (1, -2)],
    ),
    (
        [(1, 0), (1, -1), (0, 2), (1, 2)],
        [(-1, 0), (-1, -1), (0, 2), (-1, 2)],
    ),
    (
        [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
        [(-1, 0), (-1, 1), (0, -2), (-1, -2)],
    ),
];

const KICKS_I: Kicks = [
    (
        [(-2, 0), (1, 0), (-2, 1), (1, -2)],
        [(-1, 0), (2, 0), (-1, -2), (2, 1)],
    ),
    (
        [(-1, 0), (2, 0), (-1, -2), (2, 1)],
        [(2, 0), (-1, 0), (2, -1), (-1, 2)],
    ),
    (
        [(2, 0), (-1, 0), (2, -1), (-1, 2)],
        [(1, 0), (-2, 0), (1, 2), (-2, -1)],
    ),
    (
        [(1, 0), (-2, 0), (1, 2), (-2, -1)],
        [(-2, 0), (1, 0), (-2, 1), (1, -2)],
    ),
];

struct ShapeGrid {
    offset_x: i32,
    offset_y: i32,
    width: i32,
    height: i32,
    grid: [[i32; 4]; 4],
}

impl ShapeGrid {
    fn new(offset_x: i32, offset_y: i32, width: i32, height: i32, grid: [[i32; 4]; 4]) -> Self {
        ShapeGrid {
            offset_x,
            offset_y,
            width,
            height,
            grid,
        }
    }
}

struct Shape {
    shape_type: ShapeType,
    grids: [ShapeGrid; 4],
    kicks: Kicks,
}

enum ShapeType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl Shape {
    fn new(shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::I => Shape {
                shape_type: ShapeType::I,
                grids: [
                    ShapeGrid::new(
                        0,
                        1,
                        4,
                        1,
                        [[0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        2,
                        0,
                        1,
                        4,
                        [[0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 1, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        2,
                        4,
                        1,
                        [[0, 0, 0, 0], [0, 0, 0, 0], [1, 1, 1, 1], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        1,
                        4,
                        [[0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0], [0, 1, 0, 0]],
                    ),
                ],
                kicks: KICKS_I,
            },
            ShapeType::J => Shape {
                shape_type: ShapeType::J,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[2, 0, 0, 0], [2, 2, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 2, 2, 0], [0, 2, 0, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [2, 2, 2, 0], [0, 0, 2, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 2, 0, 0], [0, 2, 0, 0], [2, 2, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::L => Shape {
                shape_type: ShapeType::L,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 0, 3, 0], [3, 3, 3, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 3, 0, 0], [0, 3, 0, 0], [0, 3, 3, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [3, 3, 3, 0], [3, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[3, 3, 0, 0], [0, 3, 0, 0], [0, 3, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::O => Shape {
                shape_type: ShapeType::O,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        2,
                        [[4, 4, 0, 0], [4, 4, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::S => Shape {
                shape_type: ShapeType::S,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 5, 0, 0], [0, 5, 5, 0], [0, 0, 5, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [0, 5, 5, 0], [5, 5, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[5, 0, 0, 0], [5, 5, 0, 0], [0, 5, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::T => Shape {
                shape_type: ShapeType::T,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[0, 6, 0, 0], [6, 6, 6, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 6, 0, 0], [0, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [6, 6, 6, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 6, 0, 0], [6, 6, 0, 0], [0, 6, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
            ShapeType::Z => Shape {
                shape_type: ShapeType::Z,
                grids: [
                    ShapeGrid::new(
                        0,
                        0,
                        3,
                        2,
                        [[7, 7, 0, 0], [0, 7, 7, 0], [0, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        1,
                        0,
                        2,
                        3,
                        [[0, 0, 7, 0], [0, 7, 7, 0], [0, 7, 0, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        1,
                        3,
                        2,
                        [[0, 0, 0, 0], [7, 7, 0, 0], [0, 7, 7, 0], [0, 0, 0, 0]],
                    ),
                    ShapeGrid::new(
                        0,
                        0,
                        2,
                        3,
                        [[0, 7, 0, 0], [7, 7, 0, 0], [7, 0, 0, 0], [0, 0, 0, 0]],
                    ),
                ],
                kicks: KICKS_JLSTZ,
            },
        }
    }
}

struct WindowSettings {
    toggle_fullscreen: bool,
    is_fullscreen: bool,
}

struct Images {
    background: Image,
    outline: Image,
    blocks: [Image; 7],
}

struct Tetris {
    window_settings: WindowSettings,
    images: Images,
    grid: graphics::Mesh,
}

impl Tetris {
    fn new(ctx: &mut Context) -> GameResult<Tetris> {
        let window_settings = WindowSettings {
            toggle_fullscreen: false,
            is_fullscreen: false,
        };

        let images = Images {
            background: Image::new(ctx, "/background.png").unwrap(),
            outline: Image::new(ctx, "/outline.png").unwrap(),
            blocks: [
                Image::new(ctx, "/block_I.png").unwrap(),
                Image::new(ctx, "/block_J.png").unwrap(),
                Image::new(ctx, "/block_L.png").unwrap(),
                Image::new(ctx, "/block_O.png").unwrap(),
                Image::new(ctx, "/block_S.png").unwrap(),
                Image::new(ctx, "/block_T.png").unwrap(),
                Image::new(ctx, "/block_Z.png").unwrap(),
            ],
        };

        let grid = &mut graphics::MeshBuilder::new();
        let grid_color = Color::new(0.3, 0.3, 0.3, 0.5);

        for x in 1..=9 {
            let x = (x * 35) as f32;

            grid.line(
                &[Point2::new(x, 0.0), Point2::new(x, 700.0)],
                2.0,
                grid_color,
            )?;
        }

        for y in 1..=19 {
            let y = (y * 35) as f32;

            grid.line(
                &[Point2::new(0.0, y), Point2::new(350.0, y)],
                2.0,
                grid_color,
            )?;
        }

        let grid = grid.build(ctx)?;

        Ok(Tetris {
            window_settings,
            images,
            grid,
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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(
            ctx,
            &self.images.background,
            (Point2::new(0.0, 0.0), 0.0, graphics::WHITE),
        )?;

        let fps = timer::fps(ctx) as i32;
        let fps_display = Text::new(format!("FPS: {}", fps));
        graphics::draw(
            ctx,
            &fps_display,
            DrawParam::new().dest(Point2::new(10.0, 10.0)),
        )?;

        graphics::draw(
            ctx,
            &self.images.outline,
            DrawParam::new().dest(Point2::new(200.0, 200.0)),
        )?;

        graphics::draw(
            ctx,
            &self.grid,
            DrawParam::new().dest(Point2::new(205.0, 200.0)),
        )?;

        graphics::present(ctx)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        if let KeyCode::F11 = keycode {
            self.window_settings.toggle_fullscreen = true;
            self.window_settings.is_fullscreen = !self.window_settings.is_fullscreen;
        }
    }
}
