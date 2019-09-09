use std::time::Duration;

use crate::{
    bag::Bag,
    blocks::{self, Blocks},
    imgui_wrapper::ImGuiWrapper,
    input::{Action, Input},
    matrix::{self, Matrix},
    particles::ParticleAnimation,
    piece::Piece,
    utils,
};

use ggez::{
    audio::{self, SoundSource},
    graphics::{self, Image},
    input::keyboard::KeyCode,
    nalgebra::Point2,
    timer, Context, GameResult,
};

pub struct Game {
    input: Input,

    matrix: Matrix,
    piece: Piece,
    bag: Bag,

    game_over: bool,
    still: Duration,
    fall_interval: Duration,

    blocks: Blocks,
    particle_animation: ParticleAnimation,
    background: Image,
    theme: audio::Source,
}

impl Game {
    pub fn new(ctx: &mut Context, imgui: &ImGuiWrapper) -> GameResult<Game> {
        let repeat = Some((150, 50));
        let mut input = Input::new();
        input
            .bind(KeyCode::Right, Action::MoveRight, repeat)
            .bind(KeyCode::Left, Action::MoveLeft, repeat)
            .bind(KeyCode::Down, Action::MoveDown, repeat)
            .bind(KeyCode::Up, Action::RotateClockwise, None)
            .bind(KeyCode::X, Action::RotateClockwise, None)
            .bind(KeyCode::Z, Action::RotateCounterClockwise, None)
            .bind(KeyCode::Space, Action::HardFall, None)
            .bind(KeyCode::LShift, Action::SoftFall, None)
            .bind(KeyCode::C, Action::HoldPiece, None)
            .exclude(KeyCode::Right, KeyCode::Left)
            .exclude(KeyCode::Left, KeyCode::Right);

        let matrix = Matrix::new(ctx)?;
        let mut bag = Bag::new();

        let blocks = Blocks::new(imgui.tileset(ctx)?);

        let rect = graphics::screen_coordinates(ctx);
        let particle_animation = ParticleAnimation::new(120, 200.0, 80.0, rect.w, rect.h);

        let background = Image::new(ctx, utils::path(ctx, "background.png"))?;

        let mut theme = audio::Source::new(ctx, utils::path(ctx, "main_theme.ogg"))?;
        theme.set_repeat(true);
        theme.set_volume(0.2);
        theme.play()?;

        let piece = Piece::new(bag.pop());

        Ok(Game {
            input,
            matrix,
            piece,
            bag,
            game_over: false,
            still: Duration::new(0, 0),
            fall_interval: Duration::from_secs(1),
            blocks,
            particle_animation,
            background,
            theme,
        })
    }

    fn lock_piece(&mut self) {
        if !self.matrix.lock(&self.piece) {
            self.game_over = true;
        } else {
            self.piece = Piece::new(self.bag.pop());
            if self.matrix.collision(&self.piece) {
                self.game_over = true;
            } else {
            self.reset_fall();
            }
        }
    }

    fn reset_fall(&mut self) {
        if self.still > self.fall_interval {
            self.still -= self.fall_interval;
        } else {
            self.still = Duration::new(0, 0);
        }
    }

    pub fn update(&mut self, ctx: &mut Context, imgui: &ImGuiWrapper) -> GameResult<()> {
        self.particle_animation.update(ctx)?;

        if imgui.state.debug_t_spin_tower {
            self.matrix.debug_tower();
        }

        if imgui.state.skin_switched {
            self.blocks = Blocks::new(imgui.tileset(ctx)?);
        }

        self.matrix.update(ctx);
        if self.game_over || self.matrix.blocked() {
            return Ok(());
        }

        self.input.update(ctx);

        while let Some(action) = self.input.action() {
            match action {
                Action::MoveRight => {
                    if self.piece.shift(1, 0, &self.matrix)
                        && self.piece.touching_floor(&self.matrix)
                    {
                        self.reset_fall();
                    }
                }
                Action::MoveLeft => {
                    if self.piece.shift(-1, 0, &self.matrix)
                        && self.piece.touching_floor(&self.matrix)
                    {
                        self.reset_fall();
                    }
                }
                Action::MoveDown => {
                    if self.piece.shift(0, 1, &self.matrix) {
                        self.reset_fall();
                    }
                }
                Action::RotateClockwise => {
                    if self.piece.rotate(true, &self.matrix)
                        && self.piece.touching_floor(&self.matrix)
                    {
                        self.reset_fall();
                    }
                }
                Action::RotateCounterClockwise => {
                    if self.piece.rotate(false, &self.matrix)
                        && self.piece.touching_floor(&self.matrix)
                    {
                        self.reset_fall();
                    }
                }
                Action::SoftFall => {
                    let rows = self.piece.fall(&self.matrix);
                    if rows > 0 {
                        self.reset_fall();
                    }
                }
                Action::HardFall => {
                    self.piece.fall(&self.matrix);
                    self.lock_piece();
                }
                _ => (),
            };
        }

        self.still += timer::delta(ctx);

        if self.still >= self.fall_interval {
            self.still -= self.fall_interval;

            if !self.piece.shift(0, 1, &self.matrix) {
                self.lock_piece();
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, imgui: &ImGuiWrapper) -> GameResult<()> {
        graphics::draw(ctx, &self.background, graphics::DrawParam::new())?;

        self.particle_animation.draw(ctx)?;

        let position = Point2::new(
            (1920 - matrix::WIDTH * blocks::BLOCK_SIZE) as f32 / 2.0,
            (1080 - matrix::HEIGHT * blocks::BLOCK_SIZE) as f32 / 2.0,
        );

        self.matrix.draw(ctx, position, &mut self.blocks)?;

        self.piece.draw(ctx, position, &mut self.blocks, 1.0)?;

        if imgui.state.ghost_piece {
            let mut ghost = self.piece.clone();
            if ghost.fall(&self.matrix) > ghost.get_grid().height {
                ghost.draw(ctx, position, &mut self.blocks, 0.1)?;
            }
        }

        Ok(())
    }
}
