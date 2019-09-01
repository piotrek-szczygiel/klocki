use crate::bag::Bag;
use crate::matrix::Matrix;

use ggez::nalgebra::Point2;
use ggez::*;

pub struct Game {
    matrix: Matrix,
    bag: Bag,
    background: graphics::Image,
    grid: graphics::Mesh,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let grid = &mut graphics::MeshBuilder::new();
        let grid_color = graphics::Color::new(0.3, 0.3, 0.3, 0.5);

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

        let background = graphics::Image::new(ctx, "background.png")?;
        let grid = grid.build(ctx)?;
        let matrix = Matrix::new(ctx)?;
        let bag = Bag::new();

        Ok(Game {
            grid,
            bag,
            background,
            matrix,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::draw(ctx, &self.background, graphics::DrawParam::new())?;

        graphics::draw(
            ctx,
            &self.grid,
            graphics::DrawParam::new().dest(Point2::new(205.0, 200.0)),
        )?;

        Ok(())
    }
}
