use crate::{
    bag::Bag,
    blocks::Blocks,
    imgui_wrapper::ImGuiWrapper,
    input::{Action, Input},
    matrix::Matrix,
    particles::ParticleAnimation,
    piece::Piece,
    utils,
};

use ggez::{
    audio::{self, SoundSource},
    graphics::{self, Image},
    input::keyboard::KeyCode,
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Game {
    matrix: Matrix,
    piece: Piece,
    bag: Bag,
    blocks: Blocks,
    particle_animation: ParticleAnimation,
    background: Image,
    theme: audio::Source,
}

impl Game {
    pub fn new(ctx: &mut Context, input: &mut Input, imgui: &ImGuiWrapper) -> GameResult<Game> {
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

        // Default is 150ms delay and 50ms interval
        let repeat = Some((150, 50));

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

        // let piece = Piece::new(ShapeType::T);
        let piece = Piece::new(bag.pop());

        Ok(Game {
            matrix,
            piece,
            bag,
            blocks,
            particle_animation,
            background,
            theme,
        })
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        input: &mut Input,
        imgui: &ImGuiWrapper,
    ) -> GameResult<()> {
        self.particle_animation.update(ctx)?;

        if imgui.state.debug_t_spin_tower {
            self.matrix.debug_tower();
        }

        if imgui.state.skin_switched {
            self.blocks = Blocks::new(imgui.tileset(ctx)?);
        }

        self.matrix.update(ctx);

        while let Some(action) = input.action() {
            match action {
                Action::MoveRight => {
                    self.piece.shift(1, 0, &self.matrix);
                }
                Action::MoveLeft => {
                    self.piece.shift(-1, 0, &self.matrix);
                }
                Action::MoveDown => {
                    self.piece.shift(0, 1, &self.matrix);
                }
                Action::RotateClockwise => {
                    self.piece.rotate(true, &self.matrix);
                }
                Action::RotateCounterClockwise => {
                    self.piece.rotate(false, &self.matrix);
                }
                Action::SoftFall => {
                    self.matrix.lock(&self.piece);
                    self.piece = Piece::new(self.bag.pop());
                }
                _ => (),
            };
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.background, graphics::DrawParam::new())?;

        self.particle_animation.draw(ctx)?;

        self.matrix
            .draw(ctx, Point2::new(200.0, 200.0), &mut self.blocks)?;

        self.piece
            .draw(ctx, Point2::new(200.0, 200.0), &mut self.blocks)?;

        Ok(())
    }
}
