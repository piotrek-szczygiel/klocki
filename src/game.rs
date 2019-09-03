use crate::bag::Bag;
use crate::imgui_wrapper::State;
use crate::matrix::Matrix;
use crate::particles::ParticleAnimation;

use ggez::{
    graphics::{self, Image},
    nalgebra::Point2,
    Context, GameResult,
};

pub struct Game {
    matrix: Matrix,
    bag: Bag,
    particle_animation: ParticleAnimation,
    background: Image,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let background = graphics::Image::new(ctx, "background.png")?;
        let matrix = Matrix::new(ctx)?;
        let bag = Bag::new();

        let rect = graphics::screen_coordinates(ctx);
        let particle_animation = ParticleAnimation::new(100, 200.0, 70.0, rect.w, rect.h);

        Ok(Game {
            bag,
            particle_animation,
            background,
            matrix,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, state: &mut State) -> GameResult<()> {
        self.particle_animation.update(ctx)?;

        if state.debug_t_spin_tower {
            self.matrix.debug_tower();
        }

        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.background, graphics::DrawParam::new())?;

        self.particle_animation.draw(ctx)?;

        self.matrix.draw(ctx, Point2::new(100.0, 100.0))?;

        Ok(())
    }
}
