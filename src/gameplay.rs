use std::{collections::VecDeque, time::Duration};

use ggez::{
    graphics::{self, Color, Font, Scale},
    input::{keyboard::KeyCode, mouse},
    nalgebra::{Point2, Vector2},
    timer, Context, GameResult,
};

use crate::{
    action::Action,
    bag::Bag,
    blocks::Blocks,
    global::Global,
    holder::Holder,
    input::Input,
    particles::Explosion,
    piece::Piece,
    popups::Popup,
    popups::Popups,
    replay::ReplayData,
    score::Score,
    stack::{Locked, Stack},
    utils,
};

#[derive(PartialEq)]
enum Countdown {
    Waiting,
    Ready,
    Finished,
}

pub struct Gameplay {
    interactive: bool,
    input: Input,
    action_duration: Duration,

    actions: VecDeque<Action>,
    replay: ReplayData,

    pub stack: Stack,
    bag: Bag,
    piece: Piece,
    piece_visible: bool,
    holder: Holder,
    score: Score,
    popups: Popups,

    game_over: bool,
    falling: Duration,
    fall_interval: Duration,

    piece_entering: Option<Duration>,

    font: Font,
    blocks: Blocks,

    explosion: Option<Explosion>,
    countdown: Countdown,
    countdown_switch: Duration,
}

impl Gameplay {
    pub fn new(
        ctx: &mut Context,
        g: &mut Global,
        interactive: bool,
        seed: &[u8; 32],
    ) -> GameResult<Gameplay> {
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

        let actions = VecDeque::new();
        let replay = ReplayData::new(seed);

        let stack = Stack::new(10, 20, 20);

        let mut bag = Bag::new(seed);
        let piece = Piece::new(bag.pop(), &stack);
        let holder = Holder::default();
        let score = Score::default();
        let popups = Popups::new(ctx)?;

        let font = Font::new(ctx, utils::path(ctx, "fonts/bold.ttf"))?;

        let blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);

        Ok(Gameplay {
            interactive,
            input,
            action_duration: Duration::new(0, 0),
            actions,
            replay,
            stack,
            bag,
            piece,
            piece_visible: true,
            holder,
            score,
            popups,
            game_over: false,
            falling: Duration::new(0, 0),
            fall_interval: Duration::from_secs(1),
            piece_entering: None,
            font,
            blocks,
            explosion: None,
            countdown: Countdown::Waiting,
            countdown_switch: Duration::new(0, 0),
        })
    }

    fn reset_fall(&mut self) {
        if self.falling > self.fall_interval {
            self.falling -= self.fall_interval
        } else {
            self.falling = Duration::new(0, 0);
        }
    }

    pub fn explode(&mut self, color: Color) {
        self.explosion = Some(Explosion {
            position: Point2::new(960.0, 540.0),
            color,
            strength: 30.0,
        });
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

    pub fn explosion(&mut self) -> Option<Explosion> {
        let result = self.explosion;
        self.explosion = None;
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

    pub fn paused(&self) -> bool {
        self.game_over || self.countdown != Countdown::Finished || self.stack.blocked()
    }

    fn process_action(&mut self, g: &mut Global, action: Action, sfx: bool) -> bool {
        match action {
            Action::HoldPiece => {
                if let Some(shape) = self.holder.hold(self.piece.shape(), &mut self.bag) {
                    self.piece = Piece::new(shape, &self.stack);
                    if sfx {
                        g.sfx.play("hold");
                    }
                } else if sfx {
                    g.sfx.play("holdfail");
                }
            }
            Action::FallPiece => {
                if !self.piece.shift(0, 1, &self.stack) && self.interactive {
                    self.action(Action::LockPiece, true);
                }
            }
            Action::LockPiece => {
                match self.stack.lock(
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
                            let t_spin = self.piece.t_spin(&self.stack);
                            self.score.lock(rows, t_spin);
                            self.popups.lock(
                                rows,
                                t_spin,
                                self.score.btb(),
                                self.score.combo(),
                                g.settings.gameplay.entry_delay.into(),
                            );

                            let color = if rows == 4 {
                                Color::new(0.0, 1.0, 1.0, 1.0)
                            } else if t_spin {
                                Color::new(1.0, 0.0, 1.0, 1.0)
                            } else {
                                Color::new(0.5, 0.5, 0.0, 1.0)
                            };

                            self.explode(color);
                        } else {
                            self.score.reset_combo();
                        }

                        if sfx {
                            match (rows, self.piece.t_spin(&self.stack)) {
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

                        self.piece_entering = Some(Duration::new(0, 0));
                        self.piece_visible = false;

                        return false;
                    }
                };
            }
            Action::GameOver => {
                self.game_over = true;
                self.stack.game_over();
                self.explode(Color::new(1.0, 0.0, 0.0, 1.0));

                let mut popup = Popup::new(Duration::from_secs(10));
                popup.add("Game Over", Color::new(0.9, 0.1, 0.2, 1.0), 4.0);
                self.popups.add(popup);

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
                let moved = self.piece.shift(1, 0, &self.stack);
                if moved && self.piece.touching_floor(&self.stack) {
                    self.reset_fall();
                }

                if sfx && moved {
                    g.sfx.play("move");
                }
            }
            Action::MoveLeft => {
                let moved = self.piece.shift(-1, 0, &self.stack);
                if moved && self.piece.touching_floor(&self.stack) {
                    self.reset_fall();
                }

                if sfx && moved {
                    g.sfx.play("move");
                }
            }
            Action::MoveDown => {
                if self.piece.shift(0, 1, &self.stack) {
                    self.reset_fall();

                    if sfx {
                        g.sfx.play("move");
                    }
                }
            }
            Action::RotateClockwise => {
                let rotated = self.piece.rotate(true, &self.stack);
                if rotated && self.piece.touching_floor(&self.stack) {
                    self.reset_fall();
                }

                if sfx && rotated {
                    g.sfx.play("rotate");
                }
            }
            Action::RotateCounterClockwise => {
                let rotated = self.piece.rotate(false, &self.stack);
                if rotated && self.piece.touching_floor(&self.stack) {
                    self.reset_fall();
                }

                if sfx && rotated {
                    g.sfx.play("rotate");
                }
            }
            Action::SoftDrop => {
                let rows = self.piece.fall(&self.stack);
                if rows > 0 {
                    self.reset_fall();
                    self.score.soft_drop(rows);
                }
            }
            Action::HardDrop => {
                let rows = self.piece.fall(&self.stack);
                self.score.hard_drop(rows);

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
            self.stack.debug_t_spin();
        }

        if g.imgui_state.debug_tetris_tower {
            self.stack.debug_tetris();
        }

        if g.settings_state.skin_switched {
            self.blocks = Blocks::new(g.settings.tileset(ctx, &g.settings_state)?);
        }

        if g.imgui_state.debug_click_to_place {
            self.debug_click_to_place(ctx, g);
        }

        if self.countdown != Countdown::Finished {
            self.countdown_switch += timer::delta(ctx);
            if self.countdown_switch >= Duration::from_secs(1) {
                self.countdown_switch = Duration::new(0, 0);

                let mut popup = Popup::new(Duration::from_secs(2));
                const COLOR: Color = Color::new(0.8, 0.9, 1.0, 1.0);

                match self.countdown {
                    Countdown::Waiting => {
                        self.countdown = Countdown::Ready;
                        popup.add("Ready", COLOR, 4.0);
                        if sfx {
                            g.sfx.play("ready");
                        }
                    }
                    Countdown::Ready => {
                        self.countdown = Countdown::Finished;
                        popup.add("Go", COLOR, 4.0);
                        if sfx {
                            g.sfx.play("go");
                        }
                    }
                    _ => (),
                }

                self.popups.add(popup);
            }
        }

        self.popups.update(
            ctx,
            (g.settings.gameplay.block_size * self.stack.width) as f32,
            (g.settings.gameplay.block_size * self.stack.height) as f32,
            g.settings.gameplay.block_size as f32,
        )?;

        self.stack.update(ctx, g)?;

        self.input.update(
            ctx,
            g.settings.input.das,
            g.settings.input.arr,
            self.paused() || g.imgui_state.paused || self.piece_entering.is_some(),
        );

        if self.paused() || g.imgui_state.paused {
            return Ok(());
        }

        let actions = self.input.actions();
        self.actions(&actions);

        self.action_duration += timer::delta(ctx);

        if self.piece_entering.is_none() {
            while let Some(action) = self.actions.pop_front() {
                self.replay.add(action, self.action_duration);
                self.action_duration = Duration::new(0, 0);

                if !self.process_action(g, action, sfx) {
                    break;
                }
            }
        }

        self.piece.update(ctx, &self.stack);

        if let Some(entering) = self.piece_entering.as_mut() {
            *entering += timer::delta(ctx);

            if *entering >= Duration::from_millis(g.settings.gameplay.entry_delay.into()) {
                self.piece_entering = None;
                self.piece_visible = true;

                self.piece = Piece::new(self.bag.pop(), &self.stack);
                if self.stack.collision(&self.piece) && self.interactive {
                    self.action(Action::GameOver, true);
                } else {
                    self.reset_fall();
                    self.holder.unlock();
                }
            }
        } else if self.interactive {
            if self.piece.locking() > Duration::from_millis(g.settings.gameplay.lock_delay.into()) {
                self.action(Action::LockPiece, true);
            } else {
                self.falling += timer::delta(ctx);

                if self.falling >= self.fall_interval {
                    self.falling -= self.fall_interval;

                    self.action(Action::FallPiece, true);
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context, g: &Global, position: Point2<f32>) -> GameResult<()> {
        let block_size = g.settings.gameplay.block_size;

        let next_block_size = block_size / 2;
        let holder_block_size = block_size * 3 / 4;
        let ui_color = Color::new(0.8, 0.9, 1.0, 0.8);
        let ui_scale = Scale::uniform(block_size as f32);

        self.holder.draw(
            ctx,
            position + Vector2::new(-6.0 * holder_block_size as f32, 0.0),
            &mut self.blocks,
            holder_block_size,
            ui_color,
            self.font,
        )?;

        self.bag.draw(
            ctx,
            position + Vector2::new((self.stack.width * block_size) as f32, 0.0),
            &mut self.blocks,
            next_block_size,
            ui_color,
            self.font,
        )?;

        self.score.draw(
            ctx,
            position
                + Vector2::new(
                    (block_size * self.stack.width) as f32 + next_block_size as f32,
                    (block_size * self.stack.height) as f32 - ui_scale.y * 3.0,
                ),
            ui_color,
            self.font,
            ui_scale,
        )?;

        // https://github.com/ggez/ggez/issues/664
        ggez::graphics::pop_transform(ctx);
        ggez::graphics::apply_transformations(ctx)?;

        self.stack
            .draw(ctx, position, &mut self.blocks, block_size)?;

        if self.piece_visible && !self.game_over {
            let alpha = if g.settings.gameplay.lock_delay > 0 {
                1.0 - self.piece.locking().as_millis() as f32
                    / g.settings.gameplay.lock_delay as f32
            } else {
                1.0
            };

            self.piece.draw(
                ctx,
                position,
                self.stack.vanish,
                &mut self.blocks,
                block_size,
                alpha,
            )?;

            if g.settings.gameplay.ghost_piece > 0 {
                let mut ghost = self.piece.clone();
                if ghost.fall(&self.stack) > 0 {
                    ghost.draw(
                        ctx,
                        position,
                        self.stack.vanish,
                        &mut self.blocks,
                        block_size,
                        g.settings.gameplay.ghost_piece as f32 / 100.0,
                    )?;
                }
            }
        }

        self.popups
            .draw(ctx, position, (block_size * self.stack.height) as f32)?;

        Ok(())
    }

    fn debug_click_to_place(&mut self, ctx: &mut Context, g: &Global) {
        if !mouse::button_pressed(ctx, mouse::MouseButton::Left) {
            return;
        }

        let mouse = utils::mouse_position_coords(ctx);
        let screen = graphics::screen_coordinates(ctx);
        let position_center = Vector2::new(
            (screen.w - (self.stack.width * g.settings.gameplay.block_size) as f32) / 2.0,
            (screen.h - (self.stack.height * g.settings.gameplay.block_size) as f32) / 2.0,
        );

        let position = mouse - position_center;
        let x = position.x / g.settings.gameplay.block_size as f32;
        let y = position.y / g.settings.gameplay.block_size as f32;

        if x < 0.0 || y < 0.0 {
            return;
        }

        let x = x as i32;
        let y = y as i32;

        if x >= self.stack.width || y >= self.stack.height {
            return;
        }

        let y = y + self.stack.vanish;
        self.stack.place_random(x as usize, y as usize);
    }
}
