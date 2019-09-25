use std::{collections::VecDeque, time::Duration};

use ggez::{
    graphics::{Color, Font, Scale},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

use crate::{
    action::Action,
    bag::Bag,
    blocks::Blocks,
    global::Global,
    holder::Holder,
    matrix::{Locked, Matrix},
    piece::Piece,
    popups::Popups,
    replay::ReplayData,
    score::Score,
    utils,
};

pub struct Gameplay {
    interactive: bool,
    action_duration: Duration,

    actions: VecDeque<Action>,
    replay: ReplayData,

    pub matrix: Matrix,
    bag: Bag,
    piece: Piece,
    holder: Holder,
    score: Score,
    popups: Popups,

    game_over: bool,
    still: Duration,
    fall_interval: Duration,

    font: Font,
    blocks: Blocks,

    explosion: bool,
}

impl Gameplay {
    pub fn new(
        ctx: &mut Context,
        g: &mut Global,
        interactive: bool,
        seed: &[u8; 32],
    ) -> GameResult<Gameplay> {
        let actions = VecDeque::new();
        let replay = ReplayData::new(seed);

        let matrix = Matrix::new(10, 20, 20);

        let mut bag = Bag::new(seed);
        let piece = Piece::new(bag.pop(), &matrix);
        let holder = Holder::default();
        let score = Score::default();
        let popups = Popups::new(ctx)?;

        let font = Font::new(ctx, utils::path(ctx, "fonts/bold.ttf"))?;

        let blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);

        Ok(Gameplay {
            interactive,
            action_duration: Duration::new(0, 0),
            actions,
            replay,
            matrix,
            bag,
            piece,
            holder,
            score,
            popups,
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

    pub fn explode(&mut self) {
        self.explosion = true;
    }

    pub fn action(&mut self, action: Action, immediate: bool) {
        if immediate {
            self.actions.push_front(action);
        } else {
            self.actions.push_back(action);
        }
    }

    pub fn actions(&mut self, actions: &[Action]) {
        for &action in actions {
            self.action(action, false);
        }
    }

    pub fn explosion(&mut self) -> bool {
        let result = self.explosion;
        self.explosion = false;
        result
    }

    pub fn replay_data(&self) -> &ReplayData {
        &self.replay
    }

    pub fn score(&self) -> i32 {
        self.score.score()
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    fn process_action(&mut self, g: &mut Global, action: Action, sfx: bool) -> bool {
        match action {
            Action::HoldPiece => {
                if let Some(shape) = self.holder.hold(self.piece.shape(), &mut self.bag) {
                    self.piece = Piece::new(shape, &self.matrix);
                    if sfx {
                        g.sfx.play("hold");
                    }
                } else if sfx {
                    g.sfx.play("holdfail");
                }
            }
            Action::FallPiece => {
                if !self.piece.shift(0, 1, &self.matrix) && self.interactive {
                    self.action(Action::LockPiece, true);
                }
            }
            Action::LockPiece => {
                match self.matrix.lock(
                    &self.piece,
                    Duration::from_millis(g.settings.gameplay.clear_delay.into()),
                ) {
                    Locked::Collision => {
                        if self.interactive {
                            self.action(Action::GameOver, true);
                        }
                    }
                    Locked::Success(rows) => {
                        if rows > 0 {
                            self.explode();
                            let t_spin = self.piece.t_spin(&self.matrix);
                            self.score.lock(rows, t_spin);
                            self.popups
                                .lock(rows, t_spin, self.score.btb(), self.score.combo());
                        } else {
                            self.score.reset_combo();
                        }

                        if sfx {
                            match (rows, self.piece.t_spin(&self.matrix)) {
                                (1, false) => g.sfx.play("erase1"),
                                (2, false) => g.sfx.play("erase2"),
                                (3, false) => g.sfx.play("erase3"),
                                (4, false) => g.sfx.play("erase4"),
                                (0, true) => g.sfx.play("tspin0"),
                                (1, true) => g.sfx.play("tspin1"),
                                (2, true) => g.sfx.play("tspin2"),
                                (3, true) => g.sfx.play("tspin3"),
                                _ => g.sfx.play("lock"),
                            }
                        }

                        self.piece = Piece::new(self.bag.pop(), &self.matrix);
                        if self.matrix.collision(&self.piece) && self.interactive {
                            self.action(Action::GameOver, true);
                        } else {
                            self.reset_fall();
                            self.holder.unlock();
                        }

                        if self.matrix.blocked() {
                            return false;
                        }
                    }
                };
            }
            Action::GameOver => {
                self.game_over = true;
                self.matrix.game_over();
                self.explode();

                if sfx {
                    g.sfx.play("gameover");
                }

                return false;
            }
            Action::MoveLeft
            | Action::MoveRight
            | Action::MoveDown
            | Action::RotateClockwise
            | Action::RotateCounterClockwise
            | Action::SoftDrop
            | Action::HardDrop => self.process_movement_action(g, action, sfx),
        };

        true
    }

    fn process_movement_action(&mut self, g: &mut Global, action: Action, sfx: bool) {
        match action {
            Action::MoveRight => {
                let moved = self.piece.shift(1, 0, &self.matrix);
                if moved && self.piece.touching_floor(&self.matrix) {
                    self.reset_fall();
                }

                if sfx && moved {
                    g.sfx.play("move");
                }
            }
            Action::MoveLeft => {
                let moved = self.piece.shift(-1, 0, &self.matrix);
                if moved && self.piece.touching_floor(&self.matrix) {
                    self.reset_fall();
                }

                if sfx && moved {
                    g.sfx.play("move");
                }
            }
            Action::MoveDown => {
                if self.piece.shift(0, 1, &self.matrix) {
                    self.reset_fall();

                    if sfx {
                        g.sfx.play("move");
                    }
                }
            }
            Action::RotateClockwise => {
                let rotated = self.piece.rotate(true, &self.matrix);
                if rotated && self.piece.touching_floor(&self.matrix) {
                    self.reset_fall();
                }

                if sfx && rotated {
                    g.sfx.play("rotate");
                }
            }
            Action::RotateCounterClockwise => {
                let rotated = self.piece.rotate(false, &self.matrix);
                if rotated && self.piece.touching_floor(&self.matrix) {
                    self.reset_fall();
                }

                if sfx && rotated {
                    g.sfx.play("rotate");
                }
            }
            Action::SoftDrop => {
                let rows = self.piece.fall(&self.matrix);
                if rows > 0 {
                    self.reset_fall();
                    self.score.soft_drop(rows);

                    if sfx {
                        g.sfx.play("softdrop");
                    }
                }
            }
            Action::HardDrop => {
                let rows = self.piece.fall(&self.matrix);
                self.score.hard_drop(rows);

                if sfx && rows > 0 {
                    g.sfx.play("harddrop");
                }

                if self.interactive {
                    self.action(Action::LockPiece, true);
                }
            }
            _ => (),
        };
    }

    pub fn update(&mut self, ctx: &mut Context, g: &mut Global, sfx: bool) -> GameResult {
        if g.imgui_state.game_over {
            self.action(Action::GameOver, true);
        }

        if g.imgui_state.debug_t_spin_tower {
            self.matrix.debug_t_spin();
        }

        if g.imgui_state.debug_tetris_tower {
            self.matrix.debug_tetris();
        }

        if g.settings_state.skin_switched {
            self.blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);
        }

        self.popups.update(
            ctx,
            (g.settings.gameplay.block_size * self.matrix.width) as f32,
            (g.settings.gameplay.block_size * self.matrix.height) as f32,
            g.settings.gameplay.block_size as f32,
        )?;

        self.matrix.update(ctx, g, sfx)?;

        if self.game_over || self.matrix.blocked() || g.imgui_state.paused {
            return Ok(());
        }

        self.action_duration += timer::delta(ctx);

        while let Some(action) = self.actions.pop_front() {
            self.replay.add(action, self.action_duration);
            self.action_duration = Duration::new(0, 0);

            if !self.process_action(g, action, sfx) {
                break;
            }
        }

        if self.interactive {
            self.still += timer::delta(ctx);

            if self.still >= self.fall_interval {
                self.still -= self.fall_interval;

                self.action(Action::FallPiece, true);
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, g: &Global, position: Point2<f32>) -> GameResult<()> {
        let block_size = g.settings.gameplay.block_size;

        let ui_block_size = block_size / 2;
        let ui_color = Color::new(0.8, 0.9, 1.0, 0.8);
        let ui_scale = Scale::uniform(block_size as f32);

        self.holder.draw(
            ctx,
            position + Vector2::new(-6.0 * ui_block_size as f32, 0.0),
            &mut self.blocks,
            ui_block_size,
            ui_color,
            self.font,
        )?;

        self.bag.draw(
            ctx,
            position + Vector2::new((self.matrix.width * block_size) as f32, 0.0),
            &mut self.blocks,
            ui_block_size,
            ui_color,
            self.font,
        )?;

        self.score.draw(
            ctx,
            position
                + Vector2::new(
                    (block_size * self.matrix.width) as f32 + ui_block_size as f32,
                    (block_size * self.matrix.height) as f32 - ui_scale.y * 3.0,
                ),
            ui_color,
            self.font,
            ui_scale,
        )?;

        // https://github.com/ggez/ggez/issues/664
        ggez::graphics::pop_transform(ctx);
        ggez::graphics::apply_transformations(ctx)?;

        self.matrix
            .draw(ctx, position, &mut self.blocks, block_size)?;

        if !self.game_over {
            self.piece.draw(
                ctx,
                position,
                self.matrix.vanish,
                &mut self.blocks,
                block_size,
                1.0,
            )?;

            if g.settings.gameplay.ghost_piece > 0 {
                let mut ghost = self.piece.clone();
                if ghost.fall(&self.matrix) > 0 {
                    ghost.draw(
                        ctx,
                        position,
                        self.matrix.vanish,
                        &mut self.blocks,
                        block_size,
                        g.settings.gameplay.ghost_piece as f32 / 100.0,
                    )?;
                }
            }
        }

        self.popups
            .draw(ctx, position, (block_size * self.matrix.height) as f32)?;

        Ok(())
    }
}
