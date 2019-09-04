use crate::{
    bag::Bag,
    blocks::Blocks,
    imgui_wrapper::ImGuiWrapper,
    input::{Action, Input},
    matrix::Matrix,
    particles::ParticleAnimation,
    utils,
};

use ggez::{
    graphics::{self, Image},
    input::keyboard::KeyCode,
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Game {
    matrix: Matrix,
    bag: Bag,
    blocks: Blocks,
    particle_animation: ParticleAnimation,
    background: Image,
}

impl Game {
    pub fn new(ctx: &mut Context, input: &mut Input, imgui: &ImGuiWrapper) -> GameResult<Self> {
        let background = Image::new(ctx, utils::path(ctx, "background.png"))?;
        let matrix = Matrix::new(ctx)?;
        let bag = Bag::new();

        let blocks = Blocks::new(imgui.tileset(ctx)?);

        let rect = graphics::screen_coordinates(ctx);
        let particle_animation = ParticleAnimation::new(100, 200.0, 70.0, rect.w, rect.h);

        input
            .bind_key(KeyCode::Space, Action::HardFall)
            .bind_key(KeyCode::LShift, Action::SoftFall);

        Ok(Game {
            matrix,
            bag,
            blocks,
            particle_animation,
            background,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, imgui: &ImGuiWrapper) -> GameResult<()> {
        self.particle_animation.update(ctx)?;

        if imgui.state.debug_t_spin_tower {
            self.matrix.debug_tower();
        }

        if imgui.state.skin_switched {
            self.blocks = Blocks::new(imgui.tileset(ctx)?);
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.background, graphics::DrawParam::new())?;

        self.particle_animation.draw(ctx)?;

        self.matrix
            .draw(ctx, Point2::new(200.0, 200.0), &mut self.blocks)?;

        Ok(())
    }
}
