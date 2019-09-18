use std::{collections::VecDeque, time::Duration};

use crate::{
    bag::Bag,
    blocks::Blocks,
    global::Global,
    holder::Holder,
    input::{Action, Input},
    matrix::{self, Locked, Matrix},
    piece::Piece,
    score::Score,
    utils,
};

use ggez::{
    graphics::{self, Color, Font, Scale},
    input::keyboard::KeyCode,
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

use rand::{thread_rng, RngCore};

pub struct Gameplay {
    input: Input,
    actions: VecDeque<Action>,
    pub actions_history: VecDeque<Action>,

    matrix: Matrix,
    bag: Bag,
    piece: Piece,
    holder: Holder,
    score: Score,

    game_over: bool,
    still: Duration,
    fall_interval: Duration,

    font: Font,
    blocks: Blocks,

    explosion: bool,
}

impl Gameplay {
    pub fn new(ctx: &mut Context, g: &mut Global) -> GameResult<Gameplay> {
        let repeat = Some((150, 50));
        let mut input = Input::new();
        input
            .bind(KeyCode::Right, Action::MoveRight, repeat)
            .bind(KeyCode::Left, Action::MoveLeft, repeat)
            .bind(KeyCode::Down, Action::MoveDown, repeat)
            .bind(KeyCode::Up, Action::RotateClockwise, None)
            .bind(KeyCode::X, Action::RotateClockwise, None)
            .bind(KeyCode::Z, Action::RotateCounterClockwise, None)
            .bind(KeyCode::Space, Action::HardDrop, None)
            .bind(KeyCode::LShift, Action::SoftDrop, None)
            .bind(KeyCode::C, Action::HoldPiece, None)
            .exclude(KeyCode::Right, KeyCode::Left)
            .exclude(KeyCode::Left, KeyCode::Right);

        let actions = VecDeque::new();
        let actions_history = VecDeque::new();

        let matrix = Matrix::new();

        let mut seed = [0u8; 32];
        thread_rng().fill_bytes(&mut seed);

        let mut bag = Bag::new(seed);
        let piece = Piece::new(bag.pop());
        let holder = Holder::default();
        let score = Score::default();

        let font = Font::new(ctx, utils::path(ctx, "fonts/game.ttf"))?;

        let blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);

        Ok(Gameplay {
            input,
            actions,
            actions_history,
            matrix,
            bag,
            piece,
            holder,
            score,
            game_over: false,
            still: Duration::new(0, 0),
            fall_interval: Duration::from_secs(1),
            font,
            blocks,
            explosion: false,
        })
    }

    fn reset_fall(&mut self) {
        if self.still > self.fall_interval {
            self.still -= self.fall_interval
        } else {
            self.still = Duration::new(0, 0);
        }
    }

    fn explode(&mut self) {
        self.explosion = true;
    }

    pub fn explosion(&mut self) -> bool {
        let result = self.explosion;
        self.explosion = false;
        result
    }

    pub fn update(&mut self, ctx: &mut Context, g: &Global) -> GameResult<()> {
        if g.imgui_state.game_over {
            self.actions.push_back(Action::GameOver);
        }

        if g.imgui_state.debug_t_spin_tower {
            self.matrix.debug_tower();
        }

        if g.settings_state.skin_switched {
            self.blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);
        }

        self.matrix.update(ctx);
        if self.game_over || self.matrix.blocked() || g.imgui_state.paused {
            return Ok(());
        }

        self.input.update(ctx);

        self.actions.extend(self.input.actions());
        self.actions_history.extend(&self.actions);

        while let Some(action) = self.actions.pop_front() {
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
                Action::SoftDrop => {
                    let rows = self.piece.fall(&self.matrix);
                    if rows > 0 {
                        self.reset_fall();
                        self.score.soft_drop(rows);
                    }
                }
                Action::HardDrop => {
                    let rows = self.piece.fall(&self.matrix);
                    self.score.hard_drop(rows);
                    self.actions.push_back(Action::LockPiece);
                }
                Action::HoldPiece => {
                    if let Some(shape) = self.holder.hold(self.piece.shape(), &mut self.bag) {
                        self.piece = Piece::new(shape);
                    }
                }
                Action::FallPiece => {
                    if !self.piece.shift(0, 1, &self.matrix) {
                        self.actions.push_back(Action::LockPiece);
                    }
                }
                Action::LockPiece => {
                    match self.matrix.lock(&self.piece) {
                        Locked::Collision => {
                            self.actions.push_back(Action::GameOver);
                        }
                        Locked::Success(rows) => {
                            if rows > 0 {
                                self.explode();
                                self.score.lock(rows, self.piece.t_spin());
                            } else {
                                self.score.reset_combo();
                            }

                            self.piece = Piece::new(self.bag.pop());
                            if self.matrix.collision(&self.piece) {
                                self.actions.push_back(Action::GameOver);
                            } else {
                                self.reset_fall();
                                self.holder.unlock();
                            }
                        }
                    };
                }
                Action::GameOver => {
                    self.game_over = true;
                    self.matrix.game_over();
                    self.explode();
                }
            };
        }

        self.still += timer::delta(ctx);

        if self.still >= self.fall_interval {
            self.still -= self.fall_interval;

            self.actions.push_back(Action::FallPiece);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, g: &Global) -> GameResult<()> {
        let coords = graphics::screen_coordinates(ctx);

        let block_size = g.settings.block_size;

        let position = Point2::new(
            (coords.w - (matrix::WIDTH * block_size) as f32) / 2.0,
            (coords.h - (matrix::HEIGHT * block_size) as f32) / 2.0,
        );

        let ui_block_size = ((block_size * 3) as f32 / 4.0) as i32;
        let ui_color = Color::new(0.8, 0.9, 1.0, 0.8);
        let ui_font = self.font;
        let ui_scale = Scale::uniform(ui_block_size as f32);

        self.holder.draw(
            ctx,
            position + Vector2::new(-3.25 * ui_block_size as f32, 0.0),
            &mut self.blocks,
            ui_block_size,
            ui_color,
            ui_font,
        )?;

        self.bag.draw(
            ctx,
            position + Vector2::new(((matrix::WIDTH + 1) * block_size) as f32, 0.0),
            &mut self.blocks,
            ui_block_size,
            ui_color,
            ui_font,
        )?;

        self.score.draw(
            ctx,
            position
                + Vector2::new(
                    block_size as f32 * (matrix::WIDTH + 1) as f32,
                    block_size as f32 * (matrix::HEIGHT - 2) as f32,
                ),
            ui_color,
            ui_font,
            ui_scale,
        )?;

        self.matrix
            .draw(ctx, position, &mut self.blocks, block_size)?;

        if !self.game_over {
            self.piece
                .draw(ctx, position, &mut self.blocks, block_size, 1.0)?;

            if g.settings.ghost_piece > 0.0 {
                let mut ghost = self.piece.clone();
                if ghost.fall(&self.matrix) >= ghost.grid().height {
                    ghost.draw(
                        ctx,
                        position,
                        &mut self.blocks,
                        block_size,
                        g.settings.ghost_piece,
                    )?;
                }
            }
        }

        Ok(())
    }
}
